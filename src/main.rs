use clap::{Arg, Command};
use clap::builder::PossibleValuesParser;
use semver::{Version};

fn main() -> anyhow::Result<()> {
    let matches = clap::Command::new("semver")
        .version("1.0.0")
        .subcommand(
            Command::new("bump")
                .arg(Arg::new("level")
                    .value_parser(PossibleValuesParser::new(&[
                        "major",
                        "minor",
                        "patch"
                    ])))
                .arg(Arg::new("version"))
        )
        .get_matches();
    match matches.subcommand().unwrap() {
        ("bump", matches) => {
            let level = matches.value_of("level").unwrap();
            let version = matches.value_of("version").unwrap();
            let mut version = Version::parse(version)?;
            match level {
                "major" => {
                    version.major += 1;
                    version.minor = 0;
                    version.patch = 0;
                }
                "minor" => {
                    version.minor += 1;
                    version.patch = 0;
                }
                "patch" => {
                    version.patch += 1;
                }
                _ => panic!("Unrecognized level"),
            };
            println!("{}", version);
        }
        _ => {
            panic!("Unknown subcommand");
        }
    }
    Ok(())
}