use std::{collections::HashSet, env, fs::write, process::exit};

use clap::Parser;
use clap_stdin::MaybeStdin;
use env_logger::Env;
use log::{debug, error, info};
use semver::{Version, VersionReq};
use what_version_core::what_version;

#[derive(Debug, Parser)]
pub struct Args {
    pub version_requirements: MaybeStdin<String>,
    pub versions: MaybeStdin<String>,
}

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("debug")).init();

    let args = Args::parse();
    let parsed_version_reqs: Vec<String> =
        serde_json::from_str(&args.version_requirements).expect("Failed to parse JSON");
    let parsed_versions: Vec<String> =
        serde_json::from_str(&args.versions).expect("Failed to parse JSON");
    debug!("Parsed version requirements: {:?}", &parsed_version_reqs);
    debug!("Parsed versions: {:?}", &parsed_versions);

    let versions = parsed_versions
        .into_iter()
        .map(|v| {
            if let Some(stripped) = v.strip_prefix("v") {
                return Version::parse(stripped).expect("version is semver compatible");
            }
            Version::parse(&v).expect("version is semver compatible")
        })
        .collect();
    let version_reqs = parsed_version_reqs
        .iter()
        .filter_map(|ver| VersionReq::parse(ver).ok())
        .collect::<HashSet<VersionReq>>();

    let version = what_version(version_reqs, versions);
    if let Ok(chosen_version) = version {
        info!("Chosen Version: '{}'", &chosen_version);
        if env::var("GITHUB_ACTIONS").is_ok() {
            let github_output_path = env::var("GITHUB_OUTPUT").unwrap();
            write(
                github_output_path,
                format!("WHAT_VERSION={}", chosen_version),
            )
            .unwrap();
        }
    } else {
        error!("Could not find a version compatible with given version requirements, aborting...");
        exit(1);
    }
}
