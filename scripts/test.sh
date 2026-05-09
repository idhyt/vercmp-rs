cd apk-version; cargo test -- test_apk_version_compare --show-output; cd ..
cd debian-version; cargo test -- test_debian_version_compare --show-output; cd ..
cd rpm-version; cargo test -- test_rpm_version_compare --show-output; cd ..
cd semantic-version; cargo test -- test_semver_compare --show-output; cd ..
