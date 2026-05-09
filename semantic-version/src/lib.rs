use regex::Regex;
use semver::{Prerelease, Version};
use std::borrow::Cow;
use std::cmp::Ordering;
use std::sync::LazyLock;

static SEMVER_PATTERN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?x)
        (?P<major>0|[1-9]\d*)
        (\.
        (?P<minor>0|[1-9]\d*)
        (\.
            (?P<patch>0|[1-9]\d*)
        )?
        (?:[-\.](?P<prerelease>
            (?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)
            (?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*
        ))?
        (?:-\+(?P<build>
            [0-9a-zA-Z-]+
            (?:\.[0-9a-zA-Z-]+)*
        ))?
        )?
        ",
    )
    .unwrap()
});

static KNOWN_PRERELEASE_STR: LazyLock<[&'static str; 7]> =
    LazyLock::new(|| ["dev", "final", "release", "alpha", "beta", "rc", "latest"]);

#[inline(always)]
fn get_named_capture<'a>(captured: &'a regex::Captures, name: &str) -> Option<Cow<'a, str>> {
    match captured.name(name) {
        Some(capture) => {
            let s = capture.as_str();
            if s.chars().any(|c| c.is_uppercase()) {
                Some(Cow::Owned(s.to_lowercase()))
            } else {
                Some(Cow::Borrowed(s))
            }
        }
        None => None,
    }
}

#[inline(always)]
fn get_number_capture<'a>(captured: &'a regex::Captures, name: &str) -> Result<u64, String> {
    let s = match captured.name(name) {
        Some(capture) => capture.as_str(),
        None => "0",
    };
    Ok(s.parse::<u64>().map_err(|e| format!("{} {}", name, e))?)
}

fn convert_to_semver(version: &str) -> Result<Version, String> {
    if ["0", "0.0", "0.0.0"].iter().any(|z| z == &version) {
        return Ok(Version::new(0, 0, 0));
    }

    if let Ok(mut semver) = Version::parse(version) {
        // println!("use semver crate parse success: {:?}", semver);
        // 1.0.0-alpha.1 keep prerelease
        // 1.0.0-201801010 remove prerelease
        if KNOWN_PRERELEASE_STR
            .iter()
            .any(|known| semver.pre.as_str().contains(known))
        {
            return Ok(semver);
        }
        // if !KNOWN_PRERELEASE_STR.contains(&semver.pre.as_str()) {
        semver.pre = Prerelease::EMPTY;
        // }
        return Ok(semver);
    }

    // println!("use semver crate parse failure");
    if let Some(captured) = SEMVER_PATTERN.captures(version) {
        let major = get_number_capture(&captured, "major")?;
        let minor = get_number_capture(&captured, "minor")?;
        let patch = get_number_capture(&captured, "patch")?;
        let mut version = Version::new(major, minor, patch);
        if let Some(v) = get_named_capture(&captured, "prerelease") {
            if KNOWN_PRERELEASE_STR.contains(&v.as_ref()) {
                if let Ok(pre) = Prerelease::new(&v) {
                    version.pre = pre;
                }
            }
        }
        // println!(
        //     "capture major: {}, minor: {}, patch: {}, prerelease: {:?}",
        //     major, minor, patch, prerelease
        // );
        // let build = get_named_capture(&captured, "build");
        return Ok(version);
    }

    Err("invalid semver".to_string())
}

pub fn compare_version(a: &str, b: &str) -> Result<Ordering, String> {
    let va = convert_to_semver(a).map_err(|e| format!("SemVer parse error '{}': {}", a, e))?;
    let vb = convert_to_semver(b).map_err(|e| format!("SemVer parse error '{}': {}", b, e))?;
    // println!("{:#?}, {:#?}", va, vb);

    // Ok(va.cmp(&vb))
    Ok(va.cmp_precedence(&vb))
}
