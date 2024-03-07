# ⚔️ Introduction

[![crates.io](https://img.shields.io/crates/v/nanopub.svg)](https://crates.io/crates/nanopub)
[![PyPI](https://img.shields.io/pypi/v/nanopub-sign)](https://pypi.org/project/nanopub-sign/)
[![npm](https://img.shields.io/npm/v/@nanopub/sign)](https://www.npmjs.com/package/@nanopub/sign)

This project aims to provide a comprehensive cross-platform toolkit to sign, publish, and verify **[Nanopublications](https://nanopub.net)**.

Whether you're a developer looking to integrate nanopub functionalities into your application or a researcher seeking an efficient way to handle nanopublications, `nanopub-rs` offers a suite of tools tailored to meet  your needs.

## 🪄 Nanopub management

<ul>
    <li style="list-style-type: '✍️'">
        &nbsp;<b>Sign & Publish</b> nanopubs using a RSA private key. Customize your workflow with a <code>profile.yml</code> file.
    </li>
    <li style="list-style-type: '🔍'">
        &nbsp;<b>Verify</b>: ensure the integrity of nanopubs by checking their validity, whether they are signed or unsigned.
    </li>
    <li style="list-style-type: '📡'">
        &nbsp;<b>Fetch</b> nanopubs from the network using their URI.
    </li>
</ul>

## 📦️ Packaged for multiple interfaces

This library is packaged for easy use across various interfaces and languages:

<ul>
    <li style="list-style-type: '🦀'">
        &nbsp;<a href="rust.md"><b>Rust developers</b></a>: available as a Rust crate <code>nanopub</code>.
    </li>
    <li style="list-style-type: '🐍'">
        &nbsp;<a href="python.md"><b>Python programmers</b></a>: available as a Python pip package <code>nanopub-sign</code>.
    </li>
    <li style="list-style-type: '🌐'">
        &nbsp;<a href="javascript.md"><b>Web developers</b></a>: available as a NPM package <code>@nanopub/sign</code>, compiled to <a href="https://webassembly.org/">WebAssembly</a>, for browser integrations with JavaScript, or NodeJS.
    </li>
    <li style="list-style-type: '⌨️'">
        &nbsp;<a href="cli.md"><b>Terminal enthusiasts</b></a>: binary with a Command Line Interface (CLI) for straightforward terminal operations.
    </li>
</ul>


## ⚔️ Cross-platform support

It runs seamlessly on:

<ul>
    <li style="list-style-type: '🦊'">&nbsp;Web browsers
    <li style="list-style-type: '🐧'">&nbsp;Linux
    <li style="list-style-type: '🍎'">&nbsp;MacOS
    <li style="list-style-type: '🪟'">&nbsp;Windows
</ul>


## 🧩 RDF serialization support

The library handles most RDF serializations supporting quads, including TriG, Nquads, and JSON-LD.

## 📝 Automated metadata creation

When you sign a nanopub, if it has not already been defined in the pubinfo graph yet:

<ul>
    <li style="list-style-type: '🕓'">
        &nbsp;<b>Automatic timestamp</b>: the <code>dct:created</code> attribute is used to add the date and time of creation.
    </li>
    <li style="list-style-type: '🪪'">
        &nbsp;<b>Creator identification</b>: if an ORCID is provided in the profile, it's added using <code>dct:creator</code>. The library also checks for ORCID set with  <code>prov:wasAttributedTo</code> or <code>pav:createdBy</code>.
    </li>
</ul>

## 👆 Interactive playground

Experience `nanopub-rs` in action! Visit the **[playground page](playground.html)** to sign Nanopublications, or generate and register a new key pair, directly in your browser using the NPM version of this library.

> **💡 Facing a bug or have suggestions?** We welcome your input and feedback! If you encounter any issues or have ideas to enhance this tool, please [create an issue](https://github.com/vemonet/nanopub-rs/issues) on our GitHub repository.
