#ifndef RPM_VERSION_H
#define RPM_VERSION_H

#include <stdint.h>

#define RPM_VERSION_EQUAL 1
#define RPM_VERSION_LESS 0
#define RPM_VERSION_GREATER 2

int32_t rpm_version_compare(const char *a, const char *b);

#endif
