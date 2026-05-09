#!/usr/bin/env python3
import rpm
import sys

def parse_rpm_version(ver_string):
    """
    解析 RPM 版本字符串，返回 (epoch, version, release)
    格式： [epoch:]version[-release]
    """
    try:
        # 使用 rpm 模块的 vercmp 相关功能
        # 注意：这需要 python3-rpm 包
        epoch = None
        version = ver_string
        release = None

        # 分离 epoch
        if ':' in ver_string:
            epoch_part, rest = ver_string.split(':', 1)
            epoch = int(epoch_part)
            version = rest
        else:
            epoch = 0  # 默认 epoch 为 0

        # 分离 release
        if '-' in version:
            version, release = version.split('-', 1)

        return {
            'epoch': epoch,
            'version': version,
            'release': release if release else '(none)',
            'full_string': ver_string
        }
    except Exception as e:
        return {'error': str(e)}

if __name__ == "__main__":
    if len(sys.argv) != 2:
        print("Usage: parse_rpm_version.py <version_string>")
        print("Example: parse_rpm_version.py '1:2.3.4-5.el8'")
        sys.exit(1)

    result = parse_rpm_version(sys.argv[1])
    if 'error' in result:
        print(f"Error: {result['error']}")
    else:
        print(f"完整版本: {result['full_string']}")
        print(f"  Epoch:   {result['epoch']}")
        print(f"  Version: {result['version']}")
        print(f"  Release: {result['release']}")
