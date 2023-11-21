# ⌨️ Use from the CLI

Publish Nanopubs from your terminal using the binary for your platform.

## 📥️ Install

Download the binary adapted to your platform from the repository [Releases page](https://github.com/vemonet/nanopub-rs/releases):

- 🐧 Linux
- 🍎 MacOS
- 🪟 Windows

Rename the binary to `np` and put it in your path.

You can do it easily on Linux, MacOS, and Windows WSL, by using our install script:

```bash
curl -sSL https://raw.github.com/vemonet/nanopub-rs/main/scripts/install-binary.sh | bash
```

## ✍️ Sign

Sign a Nanopub, by default it will try to use the profile available at `~/.nanopub/profile.yml`

```bash
np sign nanopub.trig
```

Sign a Nanopub, using a specific private key file:

```bash
np sign nanopub.trig -k ~/.nanopub/id_rsa
```

Sign a Nanopub, using a `profile.yml` file:

```bash
np sign nanopub.trig -p ~/.nanopub/profile.yml
```

## 📬️ Publish

Check and publish a signed nanopub, or sign and publish an unsigned nanopub:

```bash
np publish signed.nanopub.trig
```

You can use the same `-p` and `-k` options that are available for the `np sign` command

## ✅ Check

Check if a signed nanopub is valid. It will check the Trusty hash, and signature based on the public key:

```bash
np check signed.nanopub.trig
```
