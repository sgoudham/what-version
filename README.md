<h1 align="center">
    what-version
    <br>
    ( <a href="https://github.com/sgoudham/what-version/actions/workflows/build.yml"><img src="https://github.com/sgoudham/what-version/actions/workflows/build.yml/badge.svg"></a> )
</h1>

Determines the highest [semver](https://docs.rs/semver/latest/semver/) version
from a list of versions that satisfies all given version requirements.

## Usage

1. Add the library to your `Cargo.toml`:

   ```shell
   cargo add what-version-core
   ```

2. Call the `what_version()` function with a list of versions and version requirements.

   ```rust
   let versions = vec![
       Version::parse("1.0.0").unwrap(),
       Version::parse("1.1.0").unwrap(),
       Version::parse("1.2.3").unwrap(),
       Version::parse("1.6.0").unwrap(),
       Version::parse("2.0.0").unwrap(),
   ];

   let requirements = vec![
       VersionReq::parse(">=1.1.0").unwrap(),
       VersionReq::parse("<2.0.0").unwrap(),
   ]
   .into_iter()
   .collect::<HashSet<_>>();

   match what_version(requirements, versions) {
       Ok(chosen_version) => println!("Chosen Version: {}", chosen_version),
       Err(_) => eprintln!("No valid version found"),
   }
   ```
