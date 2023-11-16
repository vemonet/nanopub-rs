# Getting Started

The `nanopub` crate aims to provide a comprehensive toolkit to check, sign, and publish [Nanopublications](https://nanopub.net).

It is available under multiple forms, for all platforms (Linux, MacOS, Windows):

- ⌨️ Binary with a CLI for use in the terminal
- 🦀 Crate for Rust
- 🐍 Pip package `nanopub_sign` for Python
- 📦️ NPM package `@nanopub/sign` for JavaScript (compiled to WebAssembly) in the browser or with NodeJS

## ⌨️ Use from the CLI

Download the binary adapted to your platform:

- Linux
- MacOS
- Windows

Rename the binary to `np` and put it in your path

> TODO: add one-liner to install for every platform

### ✍️ Sign

Signing a Nanopub, by default it will try to use the profile available at `~/.nanopub/profile.yml`

```bash
np sign nanopub.trig
```

Signing a Nanopub, using a specific private key file:

```bash
np sign nanopub.trig -k ~/.nanopub/id_rsa
```

Signing a Nanopub, using a `profile.yml` file:

```bash
np sign nanopub.trig -p ~/.nanopub/profile.yml
```

### 📬️ Publish

Check and publish a signed nanopub, or sign and publish an unsigned nanopub:

```bash
np publish signed.nanopub.trig
```

You can use the same `-p` and `-k` options that are available for the `np sign` command

### ✅ Check

Check if a signed nanopub is valid. It will check the Trusty hash, and signature based on the public key:

```bash
np check signed.nanopub.trig
```

## 🦀 Use with Rust

You can use the Rust crate to easily sign, publish, or check a Nanopub:

```rust
use std::fs;
use nanopub::{Nanopub, NpProfile};
let public_key = r#"MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAo2NYLBcZmpOkjgqLiT7hMxzRaK5KhYYHxxN2gCAMfmOaulAxAzPUNBJLIXjX3yQeIj6rAH4haWNAEUGPdiua/D+Pmu/Hrva3mK29lsWW9ajyZr0e12erDdaBw+3XfxMkKCZkLJjina6mi0W80e7Wa3+dsrypMDVl3CFYVvLsXu4lIMYqI2aVvbKyqCv6hUaWlGUip+2f84LQx/RSZGGwbBjwzKqe/Cs7frCW/lNlvsAkkst+IyFMcekEW875+rnsXP3phcP9Q1Ocu8wbnYYAu5lZPL19YFDSso2Qc5TpkXK3rawDYH36rOX8f0zBzdcbZAPx9btSCgXyqMpP8U4TCwIDAQAB"#;
let private_key = r#"MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCjY1gsFxmak6SOCouJPuEzHNForkqFhgfHE3aAIAx+Y5q6UDEDM9Q0EksheNffJB4iPqsAfiFpY0ARQY92K5r8P4+a78eu9reYrb2WxZb1qPJmvR7XZ6sN1oHD7dd/EyQoJmQsmOKdrqaLRbzR7tZrf52yvKkwNWXcIVhW8uxe7iUgxiojZpW9srKoK/qFRpaUZSKn7Z/zgtDH9FJkYbBsGPDMqp78Kzt+sJb+U2W+wCSSy34jIUxx6QRbzvn6uexc/emFw/1DU5y7zBudhgC7mVk8vX1gUNKyjZBzlOmRcretrANgffqs5fx/TMHN1xtkA/H1u1IKBfKoyk/xThMLAgMBAAECggEAECuG0GZA3HF8OaqFgMG+W+agOvH04h4Pqv4cHjYNxnxpFcNV9nEssTKWSOvCwYy7hrwZBGV3PQzbjFmmrxVFs20+8yCD7KbyKKQZPVC0zf84bj6NTNgvr6DpGtDxINxuGaMjCt7enqhoRyRRuZ0fj2gD3Wqae/Ds8cpDCefkyMg0TvauHSUj244vGq5nt93txUv1Sa+/8tWZ77Dm0s5a3wUYB2IeAMl5WrO2GMvgzwH+zT+4kvNWg5S0Ze4KE+dG3lSIYZjo99h14LcQS9eALC/VBcAJ6pRXaCTT/TULtcLNeOpoc9Fu25f0yTsDt6Ga5ApliYkb7rDhV+OFrw1sYQKBgQDCE9so+dPg7qbp0cV+lbb7rrV43m5s9Klq0riS7u8m71oTwhmvm6gSLfjzqb8GLrmflCK4lKPDSTdwyvd+2SSmOXySw94zr1Pvc7sHdmMRyA7mH3m+zSOOgyCTTKyhDRCNcRIkysoL+DecDhNo4Fumf71tsqDYogfxpAQhn0re8wKBgQDXhMmmT2oXiMnYHhi2k7CJe3HUqkZgmW4W44SWqKHp0V6sjcHm0N0RT5Hz1BFFUd5Y0ZB3JLcah19myD1kKYCj7xz6oVLb8O7LeAZNlb0FsrtD7NU+Hciywo8qESiA7UYDkU6+hsmxaI01DsttMIdG4lSBbEjA7t4IQC5lyr7xiQKBgQCN87YGJ40Y5ZXCSgOZDepz9hqX2KGOIfnUv2HvXsIfiUwqTXs6HbD18xg3KL4myIBOvywSM+4ABYp+foY+Cpcq2btLIeZhiWjsKIrw71+Q/vIe0YDb1PGf6DsoYhmWBpdHzR9HN+hGjvwlsYny2L9Qbfhgxxmsuf7zeFLpQLijjwKBgH7TD28k8IOk5VKec2CNjKd600OYaA3UfCpP/OhDl/RmVtYoHWDcrBrRvkvEEd2/DZ8qw165Zl7gJs3vK+FTYvYVcfIzGPWA1KU7nkntwewmf3i7V8lT8ZTwVRsmObWU60ySJ8qKuwoBQodki2VX12NpMN1wgWe3qUUlr6gLJU4xAoGAet6nD3QKwk6TTmcGVfSWOzvpaDEzGkXjCLaxLKh9GreM/OE+h5aN2gUoFeQapG5rUwI/7Qq0xiLbRXw+OmfAoV2XKv7iI8DjdIh0F06mlEAwQ/B0CpbqkuuxphIbchtdcz/5ra233r3BMNIqBl3VDDVoJlgHPg9msOTRy13lFqc="#;

let np_rdf = fs::read_to_string("./tests/resources/simple1-rsa.trig").unwrap();
let orcid = "https://orcid.org/0000-0000-0000-0000";
let profile = NpProfile::new(orcid, "", &private_key, None).unwrap();

let np = Nanopub::sign(&np_rdf, profile).unwrap();
let checked_np = Nanopub::check(&np.rdf).unwrap();
```

## 🛠️ Contributing

Checkout our repository [README](https://github.com/vemonet/nanopub-rs) for more details on how to develop on this project.
