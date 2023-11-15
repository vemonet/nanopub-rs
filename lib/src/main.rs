use clap::{arg, Command};
use nanopub::Nanopub;
use std::{fs, path::Path};

// https://github.com/clap-rs/clap/blob/master/examples/git.rs
// cargo run -- sign tests/resources/nanopub_test_blank.trig -k tests/resources/id_rsa
fn main() {
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
                        .default_value("~/.nanopub/id_rsa")
                )
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("publish")
                .about("Publish a Nanopub")
                .arg(arg!(<NANOPUB_FILE> "The file to publish"))
                .arg(
                    arg!(-k --key <PRIVATE_KEY> "The private key used to sign. Found in ~/.nanopub by default")
                        .default_value("~/.nanopub/id_rsa")
                )
                .arg_required_else_help(true),
        );

    let matches = cmd.get_matches();

    // TODO: get ORCID from ~/.nanopub/profile.yml
    match matches.subcommand() {
        Some(("sign", sub)) => {
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            let key_file = sub.get_one::<String>("key").unwrap();
            let np_rdf = fs::read_to_string(np_file).unwrap();
            let private_key = fs::read_to_string(key_file).unwrap();
            let orcid = "http://orcid.org/0000-0000-0000-0000";
            println!("Signing {} with {}", np_file, key_file);
            let np =
                Nanopub::new(np_rdf.as_str(), private_key.as_str(), orcid, None, None).unwrap();
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
            let np_file = sub.get_one::<String>("NANOPUB_FILE").expect("required");
            let key_file = sub.get_one::<String>("key").unwrap();
            let np_rdf = fs::read_to_string(np_file).unwrap();
            let private_key = fs::read_to_string(key_file).unwrap();
            let orcid = "http://orcid.org/0000-0000-0000-0000";
            println!("Signing {} with {}", np_file, key_file);
            let np =
                Nanopub::new(np_rdf.as_str(), private_key.as_str(), orcid, None, None).unwrap();
            println!("{}", np);
        }
        // TODO: verify
        _ => {}
    }
}
