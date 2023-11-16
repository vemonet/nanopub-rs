use clap::{arg, Command};
use nanopub::{profile::get_default_profile_path, Nanopub, NpProfile};
use std::{fs, path::Path};

// https://github.com/clap-rs/clap/blob/master/examples/git.rs
// cargo run -- sign tests/resources/nanopub_test_blank.trig -k tests/resources/id_rsa
fn main() {
    // let default_profile = get_default_profile_path();
    let cmd = Command::new("nanopub")
        .bin_name("np")
        // .version("1.0")
        // .author("Vincent Emonet. <vincent.emonet@gmail.com>")
        .about("Sign and publish Nanopublications")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("sign")
                .about("Sign a Nanopub")
                .arg(arg!(<NANOPUB_FILE> "The file to sign"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The private key used to sign. Found in ~/.nanopub by default")
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
                .about("Sign and publish a Nanopub")
                .arg(arg!(<NANOPUB_FILE> "The file to publish"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The private key used to sign.")
                        .default_value("")
                )
                .arg(
                    arg!(-p --profile <PROFILE> "The path to a profile.yml file. Default: ~/.nanopub/profile.yml")
                        .default_value("")
                )
                .arg_required_else_help(true),
        ).subcommand(
            Command::new("check")
                .about("Check if a Nanopub is valid")
                .arg(arg!(<NANOPUB_FILE> "The file to check"))
                .arg_required_else_help(true),
        );

    let matches = cmd.get_matches();

    // TODO: get ORCID from ~/.nanopub/profile.yml
    match matches.subcommand() {
        Some(("sign", sub)) => {
            let orcid = "http://orcid.org/0000-0000-0000-0000";
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            let key_file = sub.get_one::<String>("key").unwrap();
            let profile_file = sub.get_one::<String>("profile").unwrap();
            println!("profile_file {}", profile_file);

            // Read files
            let np_rdf = fs::read_to_string(np_file).unwrap();

            let profile = if !key_file.is_empty() {
                let private_key = fs::read_to_string(key_file).unwrap();
                NpProfile::new(orcid, "", &private_key, None).unwrap()
            } else if !profile_file.is_empty() {
                NpProfile::from_file(profile_file).unwrap()
            } else {
                NpProfile::from_file(&get_default_profile_path()).unwrap()
            };
            println!("Signing {} with {}", np_file, key_file);
            let np = Nanopub::sign(np_rdf.as_str(), profile).unwrap();
            println!("{}", np);

            // Prefix the nanopub filename with "signed."
            let path = Path::new(np_file);
            let parent = path.parent().unwrap_or_else(|| Path::new(""));
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let new_file_name = format!("signed.{}", file_name);
            let signed_path = parent.join(new_file_name);
            let _ = fs::write(signed_path, np.get_rdf());
        }
        Some(("publish", sub)) => {
            let orcid = "http://orcid.org/0000-0000-0000-0000";
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            let key_file = sub.get_one::<String>("key").unwrap();
            let profile_file = sub.get_one::<String>("profile").unwrap();

            // Read files
            let np_rdf = fs::read_to_string(np_file).unwrap();

            let profile = if !key_file.is_empty() {
                let private_key = fs::read_to_string(key_file).unwrap();
                NpProfile::new(orcid, "", &private_key, None).unwrap()
            } else if !profile_file.is_empty() {
                NpProfile::from_file(profile_file).unwrap()
            } else {
                NpProfile::from_file(&get_default_profile_path()).unwrap()
            };
            println!("Publishing {} with {}", np_file, key_file);
            let np = Nanopub::publish(np_rdf.as_str(), profile, None).unwrap();
            println!("{}", np);
        }
        Some(("check", sub)) => {
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            // Read RDF file
            let np_rdf = fs::read_to_string(np_file).unwrap();
            println!("Checking {}", np_file);
            let np = Nanopub::check(np_rdf.as_str()).unwrap();
            println!("{}", np);
        }
        // TODO: verify
        _ => {}
    }
}
