#include "version.h"
#include <ctype.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <string.h>

typedef struct {
  char *ptr;
  int len;
} apk_blob_t;

#define APK_BLOB_PTR_LEN(p, l)                                                 \
  ((apk_blob_t){.ptr = (char *)(p), .len = (int)(l)})
#define APK_BLOB_IS_NULL(b) ((b).ptr == NULL || (b).len == 0)

static apk_blob_t apk_blob_from_cstr(const char *s) {
  return APK_BLOB_PTR_LEN(s ? s : "", s ? (int)strlen(s) : 0);
}

static int apk_blob_compare(apk_blob_t a, apk_blob_t b) {
  int minlen = a.len < b.len ? a.len : b.len;
  int cmp = memcmp(a.ptr, b.ptr, minlen);
  if (cmp != 0)
    return cmp;
  return a.len - b.len;
}

static int apk_blob_sort(apk_blob_t a, apk_blob_t b) {
  int cmp = apk_blob_compare(a, b);
  if (cmp < 0)
    return -1;
  if (cmp > 0)
    return 1;
  return 0;
}

static uint64_t apk_blob_pull_uint(apk_blob_t *b, int base) {
  uint64_t n = 0;
  while (b->len > 0 && isdigit((unsigned char)b->ptr[0])) {
    n = n * base + (b->ptr[0] - '0');
    b->ptr++;
    b->len--;
  }
  return n;
}

static void apk_blob_spn(apk_blob_t b, int (*ctype)(int), apk_blob_t *out,
                         apk_blob_t *rest) {
  int i = 0;
  while (i < b.len && ctype((unsigned char)b.ptr[i]))
    i++;
  *out = APK_BLOB_PTR_LEN(b.ptr, i);
  rest->ptr = b.ptr + i;
  rest->len = b.len - i;
}

static int apk_blob_pull_blob_match(apk_blob_t *b, apk_blob_t match) {
  if (b->len < match.len)
    return 0;
  if (memcmp(b->ptr, match.ptr, match.len) != 0)
    return 0;
  b->ptr += match.len;
  b->len -= match.len;
  return 1;
}

static int apk_ctype_version_suffix(int c) { return (c >= 'a' && c <= 'z'); }
static int apk_ctype_hexdigit(int c) {
  return (c >= '0' && c <= '9') || (c >= 'a' && c <= 'f') ||
         (c >= 'A' && c <= 'F');
}

enum PARTS {
  TOKEN_INITIAL_DIGIT,
  TOKEN_DIGIT,
  TOKEN_LETTER,
  TOKEN_SUFFIX,
  TOKEN_SUFFIX_NO,
  TOKEN_COMMIT_HASH,
  TOKEN_REVISION_NO,
  TOKEN_END,
  TOKEN_INVALID,
};

#define DECLARE_SUFFIXES(func)                                                 \
  func(INVALID, "") func(ALPHA, "alpha") func(BETA, "beta") func(PRE, "pre")   \
      func(RC, "rc") func(NONE, "") func(CVS, "cvs") func(SVN, "svn")          \
          func(GIT, "git") func(HG, "hg") func(P, "p")

#define SUFFIX_ENUM(n, str) SUFFIX_##n,
enum { SUFFIX_ENUM_START = -1, DECLARE_SUFFIXES(SUFFIX_ENUM) };

struct token_state {
  unsigned int token;
  unsigned int suffix;
  uint64_t number;
  apk_blob_t value;
};

static int suffix_value(apk_blob_t suf) {
#define SUFFIX_DEFINE(n, str) char suffix_##n[sizeof(str)];
#define SUFFIX_ASSIGN(n, str) str,
#define SUFFIX_INDEX(n, str)                                                   \
  [SUFFIX_##n] = offsetof(struct suffix_literals, suffix_##n),
  static const struct suffix_literals {
    DECLARE_SUFFIXES(SUFFIX_DEFINE)
  } suffixes = {DECLARE_SUFFIXES(SUFFIX_ASSIGN)};
  static const unsigned short suffix_indexes[] = {
      DECLARE_SUFFIXES(SUFFIX_INDEX) sizeof(suffixes)};
  int val;

  if (suf.len == 0)
    return SUFFIX_INVALID;
  switch (suf.ptr[0]) {
  case 'a':
    val = SUFFIX_ALPHA;
    break;
  case 'b':
    val = SUFFIX_BETA;
    break;
  case 'c':
    val = SUFFIX_CVS;
    break;
  case 'g':
    val = SUFFIX_GIT;
    break;
  case 'h':
    val = SUFFIX_HG;
    break;
  case 'p':
    val = suf.len > 1 ? SUFFIX_PRE : SUFFIX_P;
    break;
  case 'r':
    val = SUFFIX_RC;
    break;
  case 's':
    val = SUFFIX_SVN;
    break;
  default:
    return SUFFIX_INVALID;
  }
  char *ptr = (char *)&suffixes + suffix_indexes[val];
  unsigned short len = suffix_indexes[val + 1] - suffix_indexes[val] - 1;
  if (apk_blob_compare(suf, APK_BLOB_PTR_LEN(ptr, len)) != 0)
    return SUFFIX_INVALID;
  return val;
}

static int token_cmp(struct token_state *ta, struct token_state *tb) {
  uint64_t a, b;
  int r;

  switch (ta->token) {
  case TOKEN_DIGIT:
    if (ta->value.ptr[0] == '0' || tb->value.ptr[0] == '0') {
      goto use_string_sort;
    }
  case TOKEN_INITIAL_DIGIT:
  case TOKEN_SUFFIX_NO:
  case TOKEN_REVISION_NO:
    a = ta->number;
    b = tb->number;
    break;
  case TOKEN_LETTER:
    a = ta->value.ptr[0];
    b = tb->value.ptr[0];
    break;
  case TOKEN_SUFFIX:
    a = ta->suffix;
    b = tb->suffix;
    break;
  use_string_sort:
  default:
    r = apk_blob_sort(ta->value, tb->value);
    if (r < 0)
      return APK_VERSION_LESS;
    if (r > 0)
      return APK_VERSION_GREATER;
    return APK_VERSION_EQUAL;
  }
  if (a < b)
    return APK_VERSION_LESS;
  if (a > b)
    return APK_VERSION_GREATER;
  return APK_VERSION_EQUAL;
}

static void token_parse_digits(struct token_state *t, apk_blob_t *b) {
  char *start = b->ptr;
  t->number = apk_blob_pull_uint(b, 10);
  t->value = APK_BLOB_PTR_LEN(start, b->ptr - start);
  if (t->value.len == 0)
    t->token = TOKEN_INVALID;
}

static void token_first(struct token_state *t, apk_blob_t *b) {
  t->token = TOKEN_INITIAL_DIGIT;
  token_parse_digits(t, b);
}

static void token_next(struct token_state *t, apk_blob_t *b) {
  if (b->len == 0) {
    t->token = TOKEN_END;
    return;
  }
  switch (b->ptr[0]) {
  case 'a' ... 'z':
    if (t->token > TOKEN_DIGIT)
      goto invalid;
    t->value = APK_BLOB_PTR_LEN(b->ptr, 1);
    t->token = TOKEN_LETTER;
    b->ptr++, b->len--;
    break;
  case '.':
    if (t->token > TOKEN_DIGIT)
      goto invalid;
    b->ptr++, b->len--;
  case '0' ... '9':
    switch (t->token) {
    case TOKEN_INITIAL_DIGIT:
    case TOKEN_DIGIT:
      t->token = TOKEN_DIGIT;
      break;
    case TOKEN_SUFFIX:
      t->token = TOKEN_SUFFIX_NO;
      break;
    default:
      goto invalid;
    }
    token_parse_digits(t, b);
    break;
  case '_':
    if (t->token > TOKEN_SUFFIX_NO)
      goto invalid;
    b->ptr++, b->len--;
    apk_blob_spn(*b, apk_ctype_version_suffix, &t->value, b);
    t->suffix = suffix_value(t->value);
    if (t->suffix == SUFFIX_INVALID)
      goto invalid;
    t->token = TOKEN_SUFFIX;
    break;
  case '~':
    if (t->token >= TOKEN_COMMIT_HASH)
      goto invalid;
    b->ptr++, b->len--;
    apk_blob_spn(*b, apk_ctype_hexdigit, &t->value, b);
    if (t->value.len == 0)
      goto invalid;
    t->token = TOKEN_COMMIT_HASH;
    break;
  case '-':
    if (t->token >= TOKEN_REVISION_NO)
      goto invalid;
    if (!apk_blob_pull_blob_match(b, APK_BLOB_PTR_LEN("-r", 2)))
      goto invalid;
    t->token = TOKEN_REVISION_NO;
    token_parse_digits(t, b);
    break;
  invalid:
  default:
    t->token = TOKEN_INVALID;
    break;
  }
}

static int apk_version_compare_core(apk_blob_t a, apk_blob_t b) {
  struct token_state ta, tb;

  if (APK_BLOB_IS_NULL(a) || APK_BLOB_IS_NULL(b)) {
    if (APK_BLOB_IS_NULL(a) && APK_BLOB_IS_NULL(b))
      return APK_VERSION_EQUAL;
    return APK_VERSION_LESS | APK_VERSION_GREATER | APK_VERSION_EQUAL;
  }

  for (token_first(&ta, &a), token_first(&tb, &b);
       ta.token == tb.token && ta.token < TOKEN_END;
       token_next(&ta, &a), token_next(&tb, &b)) {
    int r = token_cmp(&ta, &tb);
    if (r != APK_VERSION_EQUAL)
      return r;
  }

  if (ta.token == tb.token)
    return APK_VERSION_EQUAL;

  if (ta.token == TOKEN_SUFFIX && ta.suffix < SUFFIX_NONE)
    return APK_VERSION_LESS;
  if (tb.token == TOKEN_SUFFIX && tb.suffix < SUFFIX_NONE)
    return APK_VERSION_GREATER;
  if (ta.token > tb.token)
    return APK_VERSION_LESS;
  if (tb.token > ta.token)
    return APK_VERSION_GREATER;
  return APK_VERSION_EQUAL;
}

int apk_version_compare(const char *str1, const char *str2) {
  apk_blob_t a = apk_blob_from_cstr(str1);
  apk_blob_t b = apk_blob_from_cstr(str2);
  return apk_version_compare_core(a, b);
}
