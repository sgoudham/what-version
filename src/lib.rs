use std::collections::HashSet;

use semver::{Version, VersionReq};

/// Determines the highest version from a list of versions that satisfies all given version requirements.
///
/// ## Arguments
///
/// * `version_requirements` - A `HashSet` of `VersionReq` specifying the version constraints.
/// * `versions` - A `Vec` of `Version` representing the available versions to choose from.
///
/// ## Returns
///
/// * `Ok(Version)` - The highest version that satisfies all the given requirements.
/// * `Err(())` - If no version satisfies all the given requirements.
///
/// ## Examples
///
/// ```rust
/// use std::collections::HashSet;
/// use semver::{Version, VersionReq};
/// use what_version::what_version;
///
/// let version_requirements: HashSet<VersionReq> = vec![
///     "^2.0".parse().unwrap(),
///     ">=2.1".parse().unwrap(),
///     "<2.3".parse().unwrap(),
/// ]
/// .into_iter()
/// .collect();
///
/// let versions: Vec<Version> = vec![
///     "2.5.1", "2.5.0", "2.4.0", "2.3.0", "2.2.0", "2.1.1", "2.1.0", "2.0.2", "2.0.1", "2.0.0",
/// ]
/// .into_iter()
/// .map(|ver| ver.parse().unwrap())
/// .collect();
///
/// let result = what_version(version_requirements, versions);
///
/// assert_eq!(result.ok(), Some(Version::parse("2.2.0").unwrap()));
/// ```
pub fn what_version(
    version_requirements: HashSet<VersionReq>,
    versions: Vec<Version>,
) -> Result<Version, ()> {
    versions
        .iter()
        .filter(|ver| version_requirements.iter().all(|req| req.matches(ver)))
        .max_by(|arg0: &&semver::Version, other: &&semver::Version| {
            Version::cmp_precedence(*arg0, *other)
        })
        .cloned()
        .ok_or(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const VERSIONS: [&str; 14] = [
        "2.5.1", "2.5.0", "2.4.0", "2.3.0", "2.2.0", "2.1.1", "2.1.0", "2.0.2", "2.0.1", "2.0.0",
        "1.1.4", "1.1.2", "1.1.1", "1.0.3",
    ];

    #[yare::parameterized(
        all_same = { vec!["2.5.1", "2.5.1", "2.5.1"], Some("2.5.1") },
        all_caret = { vec!["^2", "^2", "^2"], Some("2.5.1") },
        all_wildcard = { vec!["*", "*", "*"], Some("2.5.1") },
        one_tilde = { vec!["~1.1.1", "^1", "1"], Some("1.1.4") },
        one_wildcard = { vec!["1.*", "^1", "1.1.0"], Some("1.1.4") },
        one_equals = { vec!["=2.2.0", "^2", "2.1.0"], Some("2.2.0") },
        upper_bound = { vec!["^2.0", "<2.5"], Some("2.4.0") },
        all_ranges = { vec!["^2.0", ">=2.1", "<2.3"], Some("2.2.0") },
        lower_bound = { vec!["^1.0", "<1.1"], Some("1.0.3") },
        no_requirements = { vec![], Some("2.5.1") }
    )]
    fn happy(version_reqs: Vec<&str>, expected: Option<&str>) {
        let version_requirements: HashSet<VersionReq> = version_reqs
            .into_iter()
            .map(|req| req.parse().unwrap())
            .collect();
        let versions: Vec<Version> = VERSIONS
            .into_iter()
            .map(|ver| ver.parse().unwrap())
            .collect();
        let expected = expected.map(|ver| ver.parse::<Version>().unwrap());

        let actual = what_version(version_requirements, versions);

        assert_eq!(actual.ok(), expected);
    }

    #[yare::parameterized(
        no_match = { vec!["^3"], None },
        conflicting_ranges = { vec!["^2.0", "<2.0"], None },
        invalid_version = { vec!["=3.0.0", "^2"], None },
    )]
    fn sad(version_reqs: Vec<&str>, expected: Option<&str>) {
        let version_requirements: HashSet<VersionReq> = version_reqs
            .into_iter()
            .map(|req| req.parse().unwrap())
            .collect();
        let versions: Vec<Version> = VERSIONS
            .into_iter()
            .map(|ver| ver.parse().unwrap())
            .collect();
        let expected = expected.map(|ver| ver.parse::<Version>().unwrap());

        let actual = what_version(version_requirements, versions);

        assert_eq!(actual.ok(), expected);
    }
}
