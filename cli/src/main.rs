use clap::{arg, value_parser, Command};
use clap_complete::{generate, Generator, Shell};
use nanopub::{error::NpError, get_np_server, Nanopub, ProfileBuilder};
use std::{error::Error, fs, io, path::Path};

// https://github.com/clap-rs/clap/blob/master/examples/git.rs
// cargo run -- sign tests/resources/nanopub_test_blank.trig -k tests/resources/id_rsa
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::new("nanopub")
        .bin_name("np")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Sign, publish, and check Nanopublications.")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sign")
                .about("Sign a Nanopub")
                .arg(arg!(<NANOPUB_FILE> "The file to sign"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The path to a private key used to sign. Found in ~/.nanopub by default")
                        .default_value("")
                )
                .arg(
                    arg!(-p --profile <PROFILE> "The path to a profile.yml file. Default: ~/.nanopub/profile.yml")
                        .default_value("")
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("publish")
                .about("Sign, publish, or check a Nanopublication (https://nanopub.net)")
                .arg(arg!(<NANOPUB_FILE> "The file to publish"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The path to a private key used to sign.")
                        .default_value("")
                )
                .arg(
                    arg!(-p --profile <PROFILE> "The path to a profile.yml file. Default: ~/.nanopub/profile.yml")
                        .default_value("")
                )
                .arg(
                    arg!(-t --test "To publish to the test server instead of the Nanopublication network.")
                )
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("check")
                .about("Check if a Nanopub is valid")
                .arg(arg!(<NANOPUB_FILE> "The file to check"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("completions")
                .about("Generates completion scripts for your shell")
                .arg(arg!([SHELL] "The shell to generate scripts for")
                    .value_parser(value_parser!(Shell)))
        );

    let matches = cmd.clone().get_matches();

    match matches.subcommand() {
        Some(("sign", sub)) => {
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            let key_file = sub.get_one::<String>("key").unwrap();
            let profile_file = sub.get_one::<String>("profile").unwrap();

            // Read RDF from file, and get profile from YAML file or key
            let np_rdf = fs::read_to_string(np_file)?;
            let profile = if !key_file.is_empty() {
                let privkey = fs::read_to_string(key_file)?;
                ProfileBuilder::new(privkey).build()?
            } else {
                ProfileBuilder::from_file(profile_file)?
            };
            println!("✍️  Signing {}", np_file);
            let np = Nanopub::new(&np_rdf)?.sign(&profile)?;
            println!("{}", np);

            // Prefix the nanopub filename with "signed."
            let path = Path::new(np_file);
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            let file_name = path
                .file_name()
                .ok_or_else(|| NpError(format!("Error getting filename from {:?}", path)))?
                .to_str()
                .ok_or_else(|| NpError(format!("Error getting filename from {:?}", path)))?;
            let new_file_name = format!("signed.{}", file_name);
            let signed_path = parent.join(new_file_name);
            println!(
                "📁 Signed Nanopub stored to {}",
                signed_path.to_str().ok_or_else(|| NpError(format!(
                    "Error getting signed path {:?}",
                    signed_path
                )))?
            );
            let _ = fs::write(signed_path, np.rdf()?);
        }
        Some(("publish", sub)) => {
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            let key_file = sub.get_one::<String>("key").unwrap();
            let profile_file = sub.get_one::<String>("profile").unwrap();
            let test_server = sub.get_flag("test");

            // Read RDF from file, and get profile from YAML file or key
            let np_rdf = fs::read_to_string(np_file)?;
            let profile = if !key_file.is_empty() {
                let privkey = fs::read_to_string(key_file)?;
                ProfileBuilder::new(privkey).build()?
            } else {
                ProfileBuilder::from_file(profile_file)?
            };
            if test_server {
                println!("🧪 Publishing {np_file} to test server");
                let _ = Nanopub::new(&np_rdf)?.publish(Some(&profile), None).await;
            } else {
                let server = get_np_server(true);
                println!("📬️ Publishing {np_file} to {server}");
                let _ = Nanopub::new(&np_rdf)?
                    .publish(Some(&profile), Some(server))
                    .await;
            }
        }
        Some(("check", sub)) => {
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            // Read RDF file
            let np_rdf = fs::read_to_string(np_file)?;
            println!("🔎 Checking {}", np_file);
            Nanopub::new(&np_rdf)?.check()?;
            // println!("{}", np);
        }
        Some(("completions", sub)) => {
            let shell = sub.get_one::<Shell>("SHELL").expect("required");
            eprintln!("Generating completion file for {shell}...");
            print_completions(shell.to_owned(), &mut cmd);
        }
        _ => {}
    }
    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
