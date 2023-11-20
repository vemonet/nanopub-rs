---
title: 'Write once, run everywhere: exploring the use of Rust and WebAssembly to implement the Nanopublication signing protocol'
title_short: 'Write once, run everywhere'
tags:
  - nanopublications
  - cross-platform compilation
  - FAIR software
  - Rust
authors:
  - name: Vincent Emonet
    orcid: 0000-0002-1501-1082
    affiliation: 1
  - name: Shashank Chakravarthy
    orcid: 0000-0003-3336-8402
    affiliation: 2

affiliations:
  - name: Department of Advanced Computing Sciences, Maastricht University, The Netherlands
    index: 1
  - name: Brightlands Institute for Smart Society, Maastricht University, The Netherlands
    index: 2

date: 2 November 2023
cito-bibliography: paper.bib
event: BioHackEU23
biohackathon_name: "BioHackathon Europe 2023"
biohackathon_url:   "https://biohackathon-europe.org/"
biohackathon_location: "Barcelona, Spain, 2023"
group: Project 35
# URL to project git repo --- should contain the actual paper.md:
git_url: https://github.com/vemonet/nanopub-rs
# This is the short authors description that is used at the
# bottom of the generated paper (typically the first two authors):
authors_short: Vincent Emonet, Shashank Chakravarthy
---

# Introduction

In the realm of making software libraries FAIR (Findable, Accessible, Interoperable, and Reusable) [@wilkinson2016fair], one crucial yet often overlooked aspect is the diversity of programming languages and platforms. To truly qualify as *reusable* and *interoperable*, a library should seamlessly run across various languages and environments. Imagine a data scientist effortlessly installing a library via `pip` for Python projects or a web developer using `npm` to integrate the same library into a web application running in the browser.

The challenge, however, lies in the arduous and time-consuming process of re-implementing a library in multiple languages. Not only does it demand significant effort and expertise, but it also multiplies the potential for bugs, making future maintenance an intricate task. The ideal solution is to have a single-language implementation that transcends these limitations.

Enter the Nanopublication library, a prime example that meets these requisites. Currently used in Java and Python, there's an emerging need for its direct utilization in web browsers through JavaScript. This gave rise to our mission: to develop a versatile, cross-platform library empowering researchers to sign and publish Nanopublications. These Nanopublications, defined in the Resource Description Framework (RDF), are authenticated and traceable through the user's ORCID-linked private key. They are subsequently published across a decentralized network of Nanopublication servers.

Nanopublications are commonly used in biomedical and semantic web communities for publishing research-related data and metadata in a free, open, and trusted platform. There is currently an official implementation of the Nanopublication signing process in Java, and we have been working on a Python implementation, but there is is no JavaScript implementation that can perform signing in the browser yet, which would enable developers to implement the signing process on the web without the need for hosting a server.

Our solution involves developing a Rust library that compiles seamlessly across diverse platforms, including browsers via WebAssembly. In  addition, we will define language bindings and publish libraries to  package registries for popular languages such as JavaScript and Python.

This solution will enable developers to easily sign and publish Nanopublications from the browser, and will eliminate the need for maintaining and updating multiple versions for different languages, which requires extensive testing to ensure all cases are covered.

By removing these barriers, we aim to empower developers to harness the  full potential of Nanopublications and foster a more accessible,  interconnected, and collaborative research ecosystem.

# Discussion

## Why Rust?

In selecting a programming language for our project, we sought qualities that would enhance our development experience and result in a robust, cross-platform library. We identified several key criteria:

- **Package Management**: We needed a language with a package manager that would streamline library reuse and save us from reinventing the wheel. Additionally, the language should simplify the integration of external libraries without the headache of manual compilation.
- **Safety and Reliability**: It was essential to choose a language that encourages safe and reliable software development practices. Safety is paramount, especially when working on projects with potential security and performance implications.
- **Modern Development Tools**: Our ideal language would provide modern development tools for formatting, testing, and documentation generation. These tools boost productivity and code quality.
- **Language Bindings**: Given our goal of enabling use across multiple languages, we needed a language that facilitated the definition of bindings to popular languages. Furthermore, the language had to compile to WebAssembly (wasm) to cater to web-based applications.

We considered options like C, C++, and Zig, all of which can compile to binary or wasm and support language bindings. However, these languages presented certain challenges:

- **C and C++**: These languages lack official package managers and require extensive effort to locate, configure, and compile external dependencies. They are known for their potential pitfalls in terms of safety, especially for developers without extensive experience. As Bjarne Stroustrup, the creator of C++, once said: "C makes it easy to shoot yourself in the foot; C++ makes it harder, but when you do it blows your whole leg off"
- **Zig**: While Zig is a promising candidate, it currently lacks an official package manager, and there is a limited availability of libraries.

Ultimately, Rust emerged as the most compelling choice. Renowned for its performance and memory safety, Rust is designed to prevent developers from creating unsafe programs. Moreover, the Rust community is vibrant, and a wealth of high-quality libraries are readily available. Notably, the Sophia library [@champin2020sophia] offers excellent support for working with RDF-compliant data.

## For Which Type of Developers?

During this project, we, two developers with no prior Rust experience, took on the challenge. Our backgrounds were in higher-level programming languages like Python, JavaScript, Java, and Ruby. The transition to Rust was surprisingly smooth, and within less than a week, we educated ourselves on the language and produced a working proof of concept. This proof of concept could be compiled into both a `pip` package for Python and an `npm` package via WebAssembly.

Our experience as newcomers to Rust revealed some key insights:

- Rust introduces novel concepts like borrowing, but the compiler is incredibly supportive. This assistance prevents us from becoming stuck on issues and instills confidence in the correctness of our code.
- While Rust's approach to error handling may appear perplexing initially, it makes sense in the long run, ultimately enhancing code reliability.
- Compilation to wasm was a satisfying experience. Tools like `wasm-pack` generate a clean directory with the minimum files needed, avoiding any weird additional boilerplate files.

# Conclusion

In conclusion, we recommend the use of Rust for building research software intended to run across diverse platforms. This recommendation holds particularly true when anticipating the deployment of software in various environments, such as web applications (JavaScript-based and browser-executed) or machine learning environments (typically Python-based).

Our journey with Rust was an enlightening experience, revealing that the learning curve is far less daunting than initially anticipated. The language's strong ecosystem and tooling made the process not only manageable but also fascinating and rewarding. Rust, with its blend of performance, safety, and cross-platform capabilities, is a powerful choice for researchers and developers aiming to create truly versatile and interoperable software solutions.

# Future work

* Complete the nanopublication signing protocol implementation
* Add Java bindings
* Discuss with the responsible of the Nanopublication project on how to best integrate this new library to the current Nanopublication ecosystem

# GitHub repositories

* https://github.com/vemonet/nanopub-rs

# Acknowledgements
We would like to particularly acknowledge Thomas Pellissier Tanon who inspired us to pursue this project thanks to his Oxigraph triplestore implementation in Rust.

We would like to thank the ELIXIR BioHackathon organisers and Chateauform Campus Belloch for hosting this event. And all the BioHackathon participants for the great interactions.

# References
