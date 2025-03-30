use std::{collections::HashSet, env, fs::write};

use clap::Parser;
use clap_stdin::MaybeStdin;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Debug, Parser)]
pub struct Args {
    pub version_requirements: MaybeStdin<String>,
    pub versions: MaybeStdin<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Match {
    pub path: String,
    pub version_req: String,
}

fn main() {
    let args = Args::parse();
    let parsed_version_reqs: Vec<Match> =
        serde_json::from_str(&args.version_requirements).expect("Failed to parse JSON");
    let parsed_versions: Vec<String> =
        serde_json::from_str(&args.versions).expect("Failed to parse JSON");

    dbg!(&parsed_version_reqs);
    dbg!(&parsed_versions);

    let versions = parsed_versions
        .into_iter()
        .map(|v| {
            if v.starts_with("v") {
                return Version::parse(&v[1..]).expect("version is semver compatible");
            }
            Version::parse(&v).expect("version is semver compatible")
        })
        .collect();
    let version_reqs = parsed_version_reqs
        .iter()
        .filter_map(|ver| VersionReq::parse(ver.version_req.as_str()).ok())
        .collect::<HashSet<VersionReq>>();

    let version = what_version::what_version(version_reqs, versions);
    if let Ok(chosen_version) = version {
        println!("Chosen Version: {}", &chosen_version);
        if env::var("GITHUB_ACTIONS").is_ok() {
            let github_output_path = env::var("GITHUB_OUTPUT").unwrap();
            write(
                github_output_path,
                format!("WHAT_VERSION={}", chosen_version),
            )
            .unwrap();
        }
    } else {
        eprintln!("No valid versions found");
    }
}
