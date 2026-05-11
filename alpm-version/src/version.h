#ifndef ALPM_VERSION_H
#define ALPM_VERSION_H

#define ALPM_VERSION_EQUAL 1
#define ALPM_VERSION_LESS 0
#define ALPM_VERSION_GREATER 2

int alpm_version_compare(const char *str1, const char *str2);

#endif
