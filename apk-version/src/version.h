#ifndef APK_VERSION_H
#define APK_VERSION_H

#define APK_VERSION_EQUAL    0
#define APK_VERSION_LESS     1
#define APK_VERSION_GREATER  2

/**
 * 比较两个 APK 版本字符串
 * @return APK_VERSION_EQUAL, APK_VERSION_LESS, APK_VERSION_GREATER,
 *         或 (APK_VERSION_LESS | APK_VERSION_GREATER) 表示不可比较
 */
int apk_version_compare(const char *str1, const char *str2);

#endif
