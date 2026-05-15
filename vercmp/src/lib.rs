/// PURL Type Enum（Based on Package URL）
///
/// reference: https://github.com/package-url/purl-spec/tree/main/types
#[derive(Debug)]
pub enum PurlType {
    /// description: Alpine Linux APK-based packages
    /// examples: ['pkg:apk/alpine/curl@7.83.0-r0?arch=x86', 'pkg:apk/alpine/apk@2.12.9-r3?arch=x86']
    Apk,

    /// description: Arch Linux packages and other users of the libalpm/pacman package manager.
    /// examples: ['pkg:alpm/arch/pacman@6.0.1-1?arch=x86_64', 'pkg:alpm/arch/python-pip@21.0-1?arch=any', 'pkg:alpm/arch/containers-common@1:0.47.4-4?arch=x86_64']
    Alpm,

    /// description: Bitbucket-based packages
    /// examples: ['pkg:bitbucket/birkenfeld/pygments-main@244fd47e07d1014f0aed9c']
    Bitbucket,

    /// description: Bitnami-based packages
    /// examples: ['pkg:bitnami/wordpress?distro=debian-12', 'pkg:bitnami/wordpress@6.2.0?distro=debian-12', 'pkg:bitnami/wordpress@6.2.0?arch=arm64&distro=debian-12', 'pkg:bitnami/wordpress@6.2.0?arch=arm64&distro=photon-4']
    Bitnami,

    /// description: Bazel modules as specified at https://bazel.build/external/module
    /// examples: ['pkg:bazel/rules_java@7.8.0', 'pkg:bazel/curl@8.8.0.bcr.1', 'pkg:bazel/curl@8.8.0?repository_url=https://example.org/bazel-registry', 'pkg:bazel/rules_java@8.5.0#java/runfiles', 'pkg:bazel/rules_java@8.5.0#java/runfiles:runfiles', 'pkg:bazel/rules_go@0.48.0#go']
    Bazel,

    /// description: Cargo packages for Rust
    /// examples: ['pkg:cargo/rand@0.7.2', 'pkg:cargo/clap@2.33.0', 'pkg:cargo/structopt@0.3.11']
    Cargo,

    /// description: Chrome Browser Extensions. Note: there are currently no officially documented APIs, further there appears to be no way to query different versions of a package - there only seems to be responses on the latest version. To this end the version component of a chrome purl can be optional. Two known data sources are a sitemap which can be crawled to discover (some) extensions at https://chromewebstore.google.com/sitemap, and an 'updatecheck' API which you can read about here https://github.com/Rob--W/crxviewer , and perhaps here https://github.com/chromium/chromium/blob/main/docs/updater/protocol_3_1.md
    /// examples: ['pkg:chrome-extension/hlepfoohegkhhmjieoechaddaejaokhf@25.7.1', 'pkg:chrome-extension/dncgedbnidfkppmdgfgidcepclnokpkb@6.0.2.3611', 'pkg:chrome-extension/kanfjhdeebkfgkbmnfknhejpadhlmiab@0.6', 'pkg:chrome-extension/dlpngalgnefjeiefhmpklpfiohadpglk@1', 'pkg:chrome-extension/dlpngalgnefjeiefhmpklpfiohadpglk']
    ChromeExtension,

    /// description: Composer PHP packages
    /// examples: ['pkg:composer/laravel/laravel@5.5.0']
    Composer,

    /// description: CocoaPods pods
    /// examples: ['pkg:cocoapods/AFNetworking@4.0.1', 'pkg:cocoapods/MapsIndoors@3.24.0', 'pkg:cocoapods/ShareKit@2.0#Twitter', 'pkg:cocoapods/GoogleUtilities@7.5.2#NSData+zlib']
    Cocoapods,

    /// description: Conan C/C++ packages. The purl is designed to closely resemble the Conan-native <package-name>/<package-version>@<user>/<channel> syntax for package references as specified in https://docs.conan.io/en/1.46/cheatsheet.html#package-terminology
    /// examples: ['pkg:conan/openssl@3.0.3', 'pkg:conan/openssl.org/openssl@3.0.3?user=bincrafters&channel=stable', 'pkg:conan/openssl.org/openssl@3.0.3?arch=x86_64&build_type=Debug&compiler=Visual%20Studio&compiler.runtime=MDd&compiler.version=16&os=Windows&shared=True&rrev=93a82349c31917d2d674d22065c7a9ef9f380c8e&prev=b429db8a0e324114c25ec387bfd8281f330d7c5c']
    Conan,

    /// description: Perl package distributions published on CPAN
    /// examples: ['pkg:cpan/GDT/URI-PackageURL', 'pkg:cpan/OALDERS/libwww-perl@6.76', 'pkg:cpan/DROLSKY/DateTime@1.55?repository_url=backpan.perl.org']
    Cpan,

    /// description: conda is for Conda packages
    /// examples: ['pkg:conda/absl-py@0.4.1?build=py36h06a4308_0&channel=main&subdir=linux-64&type=tar.bz2']
    Conda,

    /// description: CRAN R packages
    /// examples: ['pkg:cran/A3@1.0.0', 'pkg:cran/rJava@1.0-4', 'pkg:cran/caret@6.0-88']
    Cran,

    /// description: Debian packages, Debian derivatives, and Ubuntu packages
    /// examples: ['pkg:deb/debian/curl@7.50.3-1?arch=i386&distro=jessie', 'pkg:deb/debian/dpkg@1.19.0.4?arch=amd64&distro=stretch', 'pkg:deb/ubuntu/dpkg@1.19.0.4?arch=amd64', 'pkg:deb/debian/attr@1:2.4.47-2?arch=source', 'pkg:deb/debian/attr@1:2.4.47-2%2Bb1?arch=amd64']
    Deb,

    /// description: for Docker images
    /// examples: ['pkg:docker/cassandra@latest', 'pkg:docker/smartentry/debian@dc437cc87d10', 'pkg:docker/customer/dockerimage@sha256%3A244fd47e07d10?repository_url=gcr.io']
    Docker,

    /// description: GitHub-based packages
    /// examples: ['pkg:github/package-url/purl-spec@244fd47e07d1004', 'pkg:github/package-url/purl-spec@244fd47e07d1004#everybody/loves/dogs']
    Github,

    /// description: RubyGems
    /// examples: ['pkg:gem/ruby-advisory-db-check@0.12.4', 'pkg:gem/jruby-launcher@1.1.2?platform=java']
    Gem,

    /// description: The generic type is for plain, generic packages that do not fit anywhere else such as for "upstream-from-distro" packages. In particular this is handy for a plain version control repository such as a bare git repo in combination with a vcs_url.
    /// examples: ['pkg:generic/openssl@1.1.10g', 'pkg:generic/openssl@1.1.10g?download_url=https://openssl.org/source/openssl-1.1.0g.tar.gz&checksum=sha256:de4d501267da', 'pkg:generic/bitwarderl?vcs_url=git%2Bhttps://git.fsfe.org/dxtr/bitwarderl%40cc55108da32']
    Generic,

    /// description: Go packages
    /// examples: ['pkg:golang/github.com/gorilla/context@234fd47e07d1004f0aed9c', 'pkg:golang/google.golang.org/genproto#googleapis/api/annotations', 'pkg:golang/github.com/gorilla/context@234fd47e07d1004f0aed9c#api']
    Golang,

    /// description: Hugging Face ML models
    /// examples: ['pkg:huggingface/distilbert-base-uncased@043235d6088ecd3dd5fb5ca3592b6913fd516027', 'pkg:huggingface/microsoft/deberta-v3-base@559062ad13d311b87b2c455e67dcd5f1c8f65111?repository_url=https://hub-ci.huggingface.co']
    Huggingface,

    /// description: Haskell packages
    /// examples: ['pkg:hackage/a50@0.5', 'pkg:hackage/AC-HalfInteger@1.2.1', 'pkg:hackage/3d-graphics-examples@0.0.0.2']
    Hackage,

    /// description: Hex packages
    /// examples: ['pkg:hex/jason@1.1.2', 'pkg:hex/acme/foo@2.3.', 'pkg:hex/phoenix_html@2.13.3#priv/static/phoenix_html.js', 'pkg:hex/bar@1.2.3?repository_url=https://myrepo.example.com']
    Hex,

    /// description: Julia packages
    /// examples: ['pkg:julia/Dates@1.9.0?uuid=ade2ca70-3891-5945-98fb-dc099432e06a', 'pkg:julia/Dates?uuid=ade2ca70-3891-5945-98fb-dc099432e06a', 'pkg:julia/RegisterQD@0.3.1?uuid=ac24ea0c-1830-11e9-18d4-81f172323054', 'pkg:julia/RegisterQD@0.3.1?uuid=ac24ea0c-1830-11e9-18d4-81f172323054&repository_url=https://github.com/HolyLab/HolyLabRegistry']
    Julia,

    /// description: Lua packages installed with LuaRocks
    /// examples: ['pkg:luarocks/luasocket@3.1.0-1', 'pkg:luarocks/hisham/luafilesystem@1.8.0-1', 'pkg:luarocks/username/packagename@0.1.0-1?repository_url=https://example.com/private_rocks_server/']
    Luarocks,

    /// description: MLflow ML models (Azure ML, Databricks, etc.)
    /// examples: ['pkg:mlflow/creditfraud@3?repository_url=https://westus2.api.azureml.ms/mlflow/v1.0/subscriptions/a50f2011-fab8-4164-af23-c62881ef8c95/resourceGroups/TestResourceGroup/providers/Microsoft.MachineLearningServices/workspaces/TestWorkspace', 'pkg:mlflow/trafficsigns@10?model_uuid=36233173b22f4c89b451f1228d700d49&run_id=410a3121-2709-4f88-98dd-dba0ef056b0a&repository_url=https://adb-5245952564735461.0.azuredatabricks.net/api/2.0/mlflow']
    Mlflow,

    /// description: PURL type for Maven JARs and related artifacts.
    /// examples: ['pkg:maven/org.apache.xmlgraphics/batik-anim@1.9.1', 'pkg:maven/org.apache.xmlgraphics/batik-anim@1.9.1?type=pom', 'pkg:maven/org.apache.xmlgraphics/batik-anim@1.9.1?classifier=sources', 'pkg:maven/org.apache.xmlgraphics/batik-anim@1.9.1?type=zip&classifier=dist', 'pkg:maven/net.sf.jacob-projec/jacob@1.14.3?classifier=x86&type=dll', 'pkg:maven/net.sf.jacob-projec/jacob@1.14.3?classifier=x64&type=dll', 'pkg:maven/groovy/groovy@1.0?repository_url=https://maven.google.com']
    Maven,

    /// description: NuGet .NET packages
    /// examples: ['pkg:nuget/EnterpriseLibrary.Common@6.0.1304']
    Nuget,

    /// description: PURL type for npm packages.
    /// examples: ['pkg:npm/foobar@12.3.1', 'pkg:npm/%40angular/animation@12.3.1', 'pkg:npm/mypackage@12.4.5?vcs_url=git://host.com/path/to/repo.git%404345abcd34343']
    Npm,

    /// description: For artifacts stored in registries that conform to the OCI Distribution Specification https://github.com/opencontainers/distribution-spec including container images built by Docker and others
    /// examples: ['pkg:oci/debian@sha256%3A244fd47e07d10?repository_url=docker.io/library/debian&arch=amd64&tag=latest', 'pkg:oci/debian@sha256%3A244fd47e07d10?repository_url=ghcr.io/debian&tag=bullseye', 'pkg:oci/static@sha256%3A244fd47e07d10?repository_url=gcr.io/distroless/static&tag=latest', 'pkg:oci/hello-wasm@sha256:244fd47e07d10?tag=v1']
    Oci,

    /// description: Opam packages
    /// examples: ['pkg:opam/ocaml-base-compiler@5.2.0', 'pkg:opam/git@3/16.1']
    Opam,

    /// description: BEAM/OTP applications written in Elixir, Erlang, Gleam and other BEAM languages
    /// examples: ['pkg:otp/erts@10.6.3?platform=linux&arch=amd64&repository_url=https:%2F%2Fgithub.com%2Ferlang%2Fotp&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Ferlang%2Fotp.git', 'pkg:otp/stdlib@3.11.2?repository_url=https:%2F%2Fgithub.com%2Ferlang%2Fotp&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Ferlang%2Fotp.git', 'pkg:otp/crypto@4.6.4?platform=darwin&arch=x86_64&repository_url=https:%2F%2Fgithub.com%2Ferlang%2Fotp&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Ferlang%2Fotp.git', 'pkg:otp/elixir@1.10.0?repository_url=https:%2F%2Fgithub.com%2Felixir-lang%2Felixir&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Felixir-lang%2Felixir.git', 'pkg:otp/eex@1.10.0?repository_url=https:%2F%2Fgithub.com%2Felixir-lang%2Felixir&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Felixir-lang%2Felixir.git', 'pkg:otp/logger@1.10.0?repository_url=https:%2F%2Fgithub.com%2Felixir-lang%2Felixir&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Felixir-lang%2Felixir.git', 'pkg:otp/rebar@3.13.0?repository_url=https:%2F%2Fgithub.com%2Ferlang%2Frebar3&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Ferlang%2Frebar3.git', 'pkg:otp/hex@2.1.1?repository_url=https:%2F%2Fgithub.com%2Fhexpm%2Fhex&vcs_url=git%2Bhttps:%2F%2Fgithub.com%2Fhexpm%2Fhex.git']
    Otp,

    /// description: Dart and Flutter pub packages
    /// examples: ['pkg:pub/characters@1.2.0', 'pkg:pub/flutter@0.0.0']
    Pub,

    /// description: Python packages
    /// examples: ['pkg:pypi/django@1.11.1', 'pkg:pypi/django@1.11.1?file_name=Django-1.11.1.tar.gz', 'pkg:pypi/django@1.11.1?file_name=Django-1.11.1-py2.py3-none-any.whl', 'pkg:pypi/django-allauth@12.23']
    Pypi,

    /// description: QNX packages
    /// examples: ['pkg:qpkg/blackberry/com.qnx.sdp@7.0.0.SGA201702151847', 'pkg:qpkg/blackberry/com.qnx.qnx710.foo.bar.qux@0.0.4.01449T202205040833L']
    Qpkg,

    /// description: RPM packages
    /// examples: ['pkg:rpm/fedora/curl@7.50.3-1.fc25?arch=i386&distro=fedora-25', 'pkg:rpm/centerim@4.22.10-1.el6?arch=i686&epoch=1&distro=fedora-25']
    Rpm,

    /// description: PURL type for ISO-IEC 19770-2 Software Identification (SWID) tags.
    /// examples: ['pkg:swid/Acme/example.com/Enterprise+Server@1.0.0?tag_id=75b8c285-fa7b-485b-b199-4745e3004d0d', 'pkg:swid/Fedora@29?tag_id=org.fedoraproject.Fedora-29', 'pkg:swid/Adobe%2BSystems%2BIncorporated/Adobe%2BInDesign@CC?tag_id=CreativeCloud-CS6-Win-GM-MUL']
    Swid,

    /// description: Swift packages
    /// examples: ['pkg:swift/github.com/Alamofire/Alamofire@5.4.3', 'pkg:swift/github.com/RxSwiftCommunity/RxFlow@2.12.4']
    Swift,

    /// description: VS Code Extension packages
    /// examples: ['pkg:vscode-extension/ms-python/python@2023.25.10292213', 'pkg:vscode-extension/muhammad-sammy/csharp@2.15.30?repository_url=https://open-vsx.org', 'pkg:vscode-extension/golang/go@0.39.1?platform=win32-x64']
    VscodeExtension,

    /// description: Yocto Project recipes
    /// examples: ['pkg:yocto/core/glibc@2.35', 'pkg:yocto/core/glibc@2.35&repository_url=https:%2F%2Fgit.openembedded.org%2Fopenembedded-core&layer_version=kirkstone', 'pkg:yocto/core/glibc@2.35&repository_url=https:%2F%2Fgit.openembedded.org%2Fopenembedded-core&layer_version=25ba9895b9', 'pkg:yocto/core/glibc@2.35&repository_url=https:%2F%2Fgit.openembedded.org%2Fopenembedded-core&layer_version=25ba9895b98715adb66a06e50f644aea2e2c9eb6', 'pkg:yocto/xilinx/u-boot-xlnx-uenv@1.0.0', 'pkg:yocto/odroid-layer/emmc@1.0.0?layer_version=4e07fab&repository_url=https:%2F%2Fgithub.com%2Fakuster%2Fmeta-odroid']
    Yocto,
}

impl PurlType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "alpm" => PurlType::Alpm,
            "apk" => PurlType::Apk,
            "bazel" => PurlType::Bazel,
            "bitbucket" => PurlType::Bitbucket,
            "bitnami" => PurlType::Bitnami,
            "cargo" => PurlType::Cargo,
            "chrome-extension" => PurlType::ChromeExtension,
            "cocoapods" => PurlType::Cocoapods,
            "composer" => PurlType::Composer,
            "conan" => PurlType::Conan,
            "conda" => PurlType::Conda,
            "cpan" => PurlType::Cpan,
            "cran" => PurlType::Cran,
            "deb" => PurlType::Deb,
            "docker" => PurlType::Docker,
            "gem" => PurlType::Gem,
            "generic" | "arbitrary" | "unknown" => PurlType::Generic,
            "github" => PurlType::Github,
            "golang" | "go" => PurlType::Golang,
            "hackage" => PurlType::Hackage,
            "hex" => PurlType::Hex,
            "huggingface" => PurlType::Huggingface,
            "julia" => PurlType::Julia,
            "luarocks" => PurlType::Luarocks,
            "maven" => PurlType::Maven,
            "mlflow" => PurlType::Mlflow,
            "npm" => PurlType::Npm,
            "nuget" => PurlType::Nuget,
            "oci" => PurlType::Oci,
            "opam" => PurlType::Opam,
            "otp" => PurlType::Otp,
            "pub" => PurlType::Pub,
            "pypi" | "pip" => PurlType::Pypi,
            "qpkg" => PurlType::Qpkg,
            "rpm" => PurlType::Rpm,
            "swid" => PurlType::Swid,
            "swift" => PurlType::Swift,
            "vscode-extension" => PurlType::VscodeExtension,
            "yocto" => PurlType::Yocto,
            _ => PurlType::Generic,
        }
    }

    /// 获取版本比较器类型（用于路由到具体的版本实现）
    pub fn version_scheme(&self) -> VersionScheme {
        match self {
            // 使用 RPM 版本比较规则的
            PurlType::Rpm => VersionScheme::Rpm,

            // 使用 APK 版本比较规则的
            PurlType::Apk => VersionScheme::Apk,

            // 使用 ALPM 版本比较规则的（Alpine Linux）
            PurlType::Alpm => VersionScheme::Alpm,

            // 使用 Debian 规则的
            PurlType::Deb => VersionScheme::Deb,

            // 使用 SemVer 规范的
            PurlType::Cargo
            | PurlType::Npm
            | PurlType::Pypi
            | PurlType::Gem
            | PurlType::Golang
            | PurlType::Hex
            | PurlType::Nuget
            | PurlType::Maven
            | PurlType::Conda
            | PurlType::Cran
            | PurlType::Cpan
            | PurlType::Composer
            | PurlType::Cocoapods
            | PurlType::Conan
            | PurlType::Hackage
            | PurlType::Julia
            | PurlType::Luarocks
            | PurlType::Opam
            | PurlType::Pub
            | PurlType::Swift
            | PurlType::Bazel => VersionScheme::Semantic,

            // 使用 Docker/OCI 版本规则（通常是标签，类似 SemVer 但更宽松）
            PurlType::Docker | PurlType::Oci => VersionScheme::Docker,
            // 使用通用/兜底规则
            _ => VersionScheme::Generic,
        }
    }
}

/// 版本比较方案类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VersionScheme {
    /// APK 版本比较
    Apk,
    /// ALPM 版本比较
    Alpm,
    /// Debian 版本比较
    Deb,
    /// Docker 标签版本（宽松 SemVer）
    Docker,
    /// 非标/通用比较
    Generic,
    /// RPM 版本比较
    Rpm,
    /// SemVer 规范
    Semantic,
}

impl VersionScheme {
    pub fn from_purl(purl: &str) -> Result<Self, String> {
        let t = get_purl_type_fast(purl, false)?;
        Ok(PurlType::from_str(t).version_scheme())
    }

    /// 比较两个版本字符串
    ///
    /// # Arguments
    /// * `a` - 第一个版本字符串
    /// * `b` - 第二个版本字符串
    ///
    /// # Returns
    /// * `Ok(Ordering)` - 比较结果
    /// * `Err(String)` - 错误信息
    pub fn compare(&self, a: &str, b: &str) -> Result<std::cmp::Ordering, String> {
        match self {
            VersionScheme::Semantic => semantic_version::compare_version(a, b),

            VersionScheme::Deb => Ok(debian_version::compare_version(a, b)),

            VersionScheme::Apk => apk_version::compare_version(a, b),

            VersionScheme::Rpm => rpm_version::compare_version(a, b),

            VersionScheme::Alpm => alpm_version::compare_version(a, b),

            // Fallback matching use the `version_compare` crate
            VersionScheme::Docker | VersionScheme::Generic => {
                use version_compare::{Cmp, Version};
                let v1 = Version::from(a)
                    .ok_or_else(|| format!("Fallback compare invalid version string: {}", a))?;
                let v2 = Version::from(b)
                    .ok_or_else(|| format!("Fallback compare invalid version string: {}", b))?;
                let ord = match v1.compare(v2) {
                    Cmp::Lt => std::cmp::Ordering::Less,
                    Cmp::Eq => std::cmp::Ordering::Equal,
                    Cmp::Gt => std::cmp::Ordering::Greater,
                    // _ => unreachable!(),
                    _ => return Err(format!("Unreachable compare {} vs {}", a, b)),
                };
                Ok(ord)
            }
        }
    }
}

fn get_purl_type_fast(purl: &str, strict: bool) -> Result<&str, String> {
    let bytes = purl.as_bytes();
    // 检查 "pkg:" 前缀
    if bytes.len() < 5 || &bytes[0..4] != b"pkg:" {
        return Err("PURL missing required `scheme` component".to_string());
    }

    let mut i = 4; // 跳过 "pkg:"
    let len = bytes.len();
    // 查找第一个 '/' 的位置
    while i < len && bytes[i] != b'/' {
        if strict {
            let c = bytes[i];
            // type 只能包含：a-z, 0-9, -, _, .
            let is_valid = matches!(c, b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.');
            if !is_valid {
                return Err("PURL type contains invalid character".to_string());
            }
        }
        i += 1;
    }
    // 检查有效性：
    // 1. i == 4: 没有 type（如 "pkg:/..."）
    // 2. i == len: 没有找到 '/'（如 "pkg:cargo"）
    // 3. i + 1 >= len: '/' 后面没有 name（如 "pkg:cargo/"）
    if i == 4 || i == len || i + 1 >= len {
        return Err("PURL missing required `type` component".to_string());
    }

    let t = unsafe { std::str::from_utf8_unchecked(&bytes[4..i]) };
    Ok(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_purls() {
        let valid_cases = [
            ("pkg:deb/debian/curl@7.50.3-1", "deb"),
            ("pkg:docker/cassandra@sha256:244fd47e", "docker"),
            ("pkg:gem/jruby-launcher@1.1.2", "gem"),
            ("pkg:cargo/serde@1.0.0", "cargo"),
            ("pkg:maven/org.foo/bar", "maven"),
            ("pkg:generic/foo#subpath", "generic"),
            ("pkg:python/django@3.2", "python"),
            ("pkg:npm/foo@1.0.0", "npm"),
        ];

        for (purl, expected) in valid_cases {
            let t = get_purl_type_fast(purl, false);
            assert!(t.is_ok());
            assert_eq!(t.unwrap(), expected);
        }
    }

    #[test]
    fn test_invalid_purls() {
        let invalid_cases = [
            "pkg:cargo/",       // 缺少 name
            "pkg:cargo",        // 没有斜杠
            "pkg:/serde@1.0.0", // type 为空
            "pkg:",             // 什么都没有
            "pkg:deb/",         // 缺少 name
            "invalid",          // 不是 purl
            "pkg:",             // 不完整
            "",                 // 空字符串
        ];

        for purl in invalid_cases {
            let t = get_purl_type_fast(purl, false);
            assert!(t.is_err());
        }
    }

    #[test]
    fn test_strict_validation() {
        // 严格模式会拒绝非法字符
        assert!(
            get_purl_type_fast("pkg:Deb/debian/curl", true).is_err() // 大写 D
        );
        assert!(
            get_purl_type_fast("pkg:d👀b/debian/curl", true).is_err() // 包含非法字符
        );
        assert_eq!(
            get_purl_type_fast("pkg:deb/debian/curl", true).unwrap(), // 全小写
            "deb"
        );
    }

    #[test]
    fn test_edge_cases() {
        // 边界情况
        assert!(get_purl_type_fast("pkg:a/", false).is_err()); // name 为空
        assert_eq!(get_purl_type_fast("pkg:a/b", false).unwrap(), "a"); // 最小有效格式
        assert_eq!(
            get_purl_type_fast("pkg:abc123/def", false).unwrap(),
            "abc123"
        );
    }
}

#[cfg(feature = "generate-capi")]
mod capi;
