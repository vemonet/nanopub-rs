# 📜 Changelog

All notable changes to this project will be documented in this file.

## [0.1.4](https://github.com/vemonet/nanopub-rs/compare/v0.1.3..0.1.4) - 2024-05-30

### ⛰️ Features

- [**breaking**] Update the `NpProfile` struct to make ORCID and name optional, and use the builder pattern to build a profile. Remove dependency on serde_yaml (YAML profile file parsed manually) - ([c3e30fa](https://github.com/vemonet/nanopub-rs/commit/c3e30fa250dfd3a9cdfe67dd71735a15ee6bf48a))

### 🐛 Bug Fixes

- Fix python build in workflow - ([90a6a83](https://github.com/vemonet/nanopub-rs/commit/90a6a836527c99607e22d9718d5f2f04669ac47a))

### 📚 Documentation

- Small improvements - ([6b0c599](https://github.com/vemonet/nanopub-rs/commit/6b0c599f1f9d303266c37c722e24abcbcef1ff8b))

### 🛠️ Miscellaneous Tasks

- Improve dependency setup - ([09a5e08](https://github.com/vemonet/nanopub-rs/commit/09a5e086591be4b064c977b34b39c8701bc6b940))

### 🧪 Testing

- Improve JS tests to test profile with default args - ([44bc7e6](https://github.com/vemonet/nanopub-rs/commit/44bc7e61d97f61658652a33fe55d6ad8bb81f0b8))

## [0.1.3](https://github.com/vemonet/nanopub-rs/compare/v0.1.2..v0.1.3) - 2024-03-31

### ⚙️ Continuous Integration

- Improve build process to use a matrix and add binary for windows aarch64 - ([ab8f3ba](https://github.com/vemonet/nanopub-rs/commit/ab8f3baecee5b7d542761f3ffab02b0124bf1d6c))
- Fix aarch64 binary builds - ([888c734](https://github.com/vemonet/nanopub-rs/commit/888c73443707dca018674aca269d99ab0dd37031))
- Fix linux aarch64 build - ([24e8fc5](https://github.com/vemonet/nanopub-rs/commit/24e8fc56e3e5e062108c5a6dc78d3d09c15a9530))

### 🐛 Bug Fixes

- Fix wheel building on windows aarch64 - ([a7ef4a5](https://github.com/vemonet/nanopub-rs/commit/a7ef4a570c9fe38ff7d29f88af0194d137e43049))
- Add content type header to nanopub server request (fix error when running test with jest and nodejs). Add env variable to try fix build on aarch64 - ([c464e63](https://github.com/vemonet/nanopub-rs/commit/c464e63abe0f30f34a27d9b99d94972411d9ae48))

### 📚 Documentation

- Improve error msg display in playground page - ([dfa0d44](https://github.com/vemonet/nanopub-rs/commit/dfa0d443e0442758d73c773f2d58c1a06c035242))
- Add loader icon for editor - ([a13b4fb](https://github.com/vemonet/nanopub-rs/commit/a13b4fb2ec7599660dae8b07238c26be3bca6ea9))

### 🚜 Refactor

- Creating a new intro nanopub now fails if the ORCID or name of the introduced Profile are empty - ([2a7fae5](https://github.com/vemonet/nanopub-rs/commit/2a7fae51c161c4eb888e71bb49d82793d06a3af2))

### 🛠️ Miscellaneous Tasks

- Improve the list of prefixes used when building thenanopub, using https://schema.org/ instead of http://schema.org/ - ([b534f7f](https://github.com/vemonet/nanopub-rs/commit/b534f7f038fa20f8882684d965b2f9bad8204014))
- Upgrade reqwest to 0.12 - ([a9f3eaa](https://github.com/vemonet/nanopub-rs/commit/a9f3eaa0ea36ba0a9022011b2e516caec99f6299))
- Use rust-toolchain.toml for components - ([2a380c8](https://github.com/vemonet/nanopub-rs/commit/2a380c85085728d8a53190c6ee8866fd03d38703))
- Bump version to 0.1.3 - ([0cc3a18](https://github.com/vemonet/nanopub-rs/commit/0cc3a18e272fed9c4438c7b9b656f9a421b10336))

### 🧪 Testing

- Test with reqwest native-tls - ([5cae658](https://github.com/vemonet/nanopub-rs/commit/5cae658d3afc37fbca2b0441c0d464c375d135b0))

## [0.1.2](https://github.com/vemonet/nanopub-rs/compare/v0.1.1..v0.1.2) - 2024-03-07

### ⚙️ Continuous Integration

- Improve wheel building in build workflow, use the maturin gh action inspired from pydantic2 - ([d26d008](https://github.com/vemonet/nanopub-rs/commit/d26d008ef73abbabc0b540ec3e778e11460f6d2d))
- Fix wheel working dir - ([e3e0f30](https://github.com/vemonet/nanopub-rs/commit/e3e0f30bdc321034f47121a5e6e03b9eff682339))

### 🚜 Refactor

- Rename get_rdf() function to rdf() - ([bb28299](https://github.com/vemonet/nanopub-rs/commit/bb282990662ae0fdb284c6e005f6bab58d329cf6))

### 🛠️ Miscellaneous Tasks

- Bump version to 0.1.2 - ([85d2bde](https://github.com/vemonet/nanopub-rs/commit/85d2bdef310e0d378a307d2dacf0dc6d8b1e4002))

## [0.1.1](https://github.com/vemonet/nanopub-rs/compare/v0.1.0..v0.1.1) - 2024-03-06

### ⛰️ Features

- Add python bindings for generating private/public `KeyPair` and static method `Nanopub.publish_intro()` to publish an introduction - ([d02ef65](https://github.com/vemonet/nanopub-rs/commit/d02ef652e33d3d48a0ea70f11c912234b2442a4d))

### 🐛 Bug Fixes

- Fix tokio dev dependency for when runninv cov tests - ([de2a109](https://github.com/vemonet/nanopub-rs/commit/de2a1097979e4e8e8d76f6aeff2193d6238c14d7))

### 📚 Documentation

- Migrate docs from mdbook to material for mkdocs - ([d2785dc](https://github.com/vemonet/nanopub-rs/commit/d2785dc327be5bf875ca8c279a66f0f0d04bf12b))
- Add service worker to playground to speed up loading assets - ([e2f6bfc](https://github.com/vemonet/nanopub-rs/commit/e2f6bfc108da52af7afd2a0d7234e214e26c6107))

### 🛠️ Miscellaneous Tasks

- Bump version to 0.1.1 - ([b2ff457](https://github.com/vemonet/nanopub-rs/commit/b2ff4579880f524e08f12cf3bd3af94ed391c45e))

## [0.1.0](https://github.com/vemonet/nanopub-rs/compare/v0.0.18..v0.1.0) - 2024-03-04

### ⚙️ Continuous Integration

- Fix test workflow, add cargo deny - ([d618203](https://github.com/vemonet/nanopub-rs/commit/d618203c7ba74c636ff97ec407c90aa62a1732bc))
- Add git cliff to generate changelog - ([69d5de9](https://github.com/vemonet/nanopub-rs/commit/69d5de97f288db9daeade7c5115861d1bd59fc63))
- Fix release script - ([bfbc7ac](https://github.com/vemonet/nanopub-rs/commit/bfbc7ac15712af618674922c5e73598904ffad66))

### ⛰️ Features

- Now supports null for profile and string in JS np.publish(). Related to https://github.com/vemonet/nanopub-rs/issues/4 - ([63093e4](https://github.com/vemonet/nanopub-rs/commit/63093e4c89b4b0eb18b9836c425121b03d50bd24))
- Add support for re-signing already signed Nanopubs. Related to https://github.com/vemonet/nanopub-rs/issues/4 - ([e8a37d9](https://github.com/vemonet/nanopub-rs/commit/e8a37d932496198b5c546b651ca0b101831f3362))
- Add fetch static function to JS bindings - ([c5635da](https://github.com/vemonet/nanopub-rs/commit/c5635dabc9b9a997d1e2fa52cc55ec4416cf7212))
- [**breaking**] Change the python API to use instantiated Nanopub objects instead of static functions on the Nanopub class to check/sign/publish - ([9e34dda](https://github.com/vemonet/nanopub-rs/commit/9e34dda7eb46e2ec431665e54943fd6860b78ecc))

### 🐛 Bug Fixes

- Fix handling of published URI - ([2840850](https://github.com/vemonet/nanopub-rs/commit/284085011a053a0f4cd8fdf2be2bf623bf9e6f56))

### 📚 Documentation

- Fix rust codeblocks in docs - ([2c82215](https://github.com/vemonet/nanopub-rs/commit/2c8221521e5541a453471db313c630ff83b2ced4))

### 🛠️ Miscellaneous Tasks

- Delete bump.sh script, now replaced by release.sh - ([0f72924](https://github.com/vemonet/nanopub-rs/commit/0f729249967d97a7518b34867b7c6da516001c5f))
- Cleanup dev dependencies for python build - ([2aff338](https://github.com/vemonet/nanopub-rs/commit/2aff338047afaaffa951ce1af68775d1660b8669))
- Bump version to 0.1.0 - ([7841e1d](https://github.com/vemonet/nanopub-rs/commit/7841e1d05fba9be19451abf473a4b6eecd06d0a9))

### 🧪 Testing

- Add tests for python and JS with pytest and jest. Update some gh actions versions - ([fdca2f7](https://github.com/vemonet/nanopub-rs/commit/fdca2f73d834b7b42fb57255c2a1a8805a515ef2))

## [0.0.18](https://github.com/vemonet/nanopub-rs/compare/v0.0.17..v0.0.18) - 2024-02-01

## [0.0.17](https://github.com/vemonet/nanopub-rs/compare/v0.0.16..v0.0.17) - 2024-01-30

### 🐛 Bug Fixes

- Fix issue with keypair in demo - ([0f6d0bc](https://github.com/vemonet/nanopub-rs/commit/0f6d0bc92ac9e84faa1336e06a5720d750a96c9b))
- Fix docs - ([b77cbe4](https://github.com/vemonet/nanopub-rs/commit/b77cbe4b898e965d221e9c566b1297793af29ef8))
- Fix rsa crate version - ([62cabd2](https://github.com/vemonet/nanopub-rs/commit/62cabd2ce681a1cbc1f2daa9ce09edcbc331ba84))
- Fix nanopub java ddl in workflow - ([9d7568a](https://github.com/vemonet/nanopub-rs/commit/9d7568af1ddf645f28ea6b9b361ccf01a4fe68e4))

## [0.0.16](https://github.com/vemonet/nanopub-rs/compare/v0.0.15..v0.0.16) - 2023-12-01

### 🐛 Bug Fixes

- Fix docs examples - ([08150af](https://github.com/vemonet/nanopub-rs/commit/08150af5ad7d26e61b7b5c70aa15a229ac76a6e6))
- Fix print - ([898b41d](https://github.com/vemonet/nanopub-rs/commit/898b41dd68334e879b85ab7dfe6bd9f759301b6a))
- Fix docs - ([f87a372](https://github.com/vemonet/nanopub-rs/commit/f87a372638e0337dd0fcdf4a0ca29dff1893231a))

## [0.0.15](https://github.com/vemonet/nanopub-rs/compare/v0.0.14..v0.0.15) - 2023-11-29

### 🐛 Bug Fixes

- Fix orcid init - ([8c72842](https://github.com/vemonet/nanopub-rs/commit/8c72842f03ae06f3fde2914184e99f2983fefcda))
- Fix main np server URL - ([229cde6](https://github.com/vemonet/nanopub-rs/commit/229cde6a751c887bfa411070ff4bcd7b894e00a9))
- Fix tests - ([e2b2866](https://github.com/vemonet/nanopub-rs/commit/e2b2866ea16f49ff6b1109f420070e00e75c49f5))

## [0.0.14](https://github.com/vemonet/nanopub-rs/compare/v0.0.13..v0.0.14) - 2023-11-29

### 🐛 Bug Fixes

- Fix tests - ([6c2feb8](https://github.com/vemonet/nanopub-rs/commit/6c2feb8818bb8db1edc5da80d01540011576f8b2))

## [0.0.13](https://github.com/vemonet/nanopub-rs/compare/v0.0.12..v0.0.13) - 2023-11-27

## [0.0.12](https://github.com/vemonet/nanopub-rs/compare/v0.0.11..v0.0.12) - 2023-11-27

### 🐛 Bug Fixes

- Fix demo example - ([021607c](https://github.com/vemonet/nanopub-rs/commit/021607c784dcd573570dcee6f3d8ad967be11bfd))
- Fix KeyPair import in demo - ([0e48f56](https://github.com/vemonet/nanopub-rs/commit/0e48f564268681a71889014cfb5da86a655a4864))
- Fix wasm init for pub intro - ([2501c4b](https://github.com/vemonet/nanopub-rs/commit/2501c4b0dcb45bf005c3654dc97199427ac81d62))
- Fix demo intro pub - ([7a8b177](https://github.com/vemonet/nanopub-rs/commit/7a8b177b4631e7b3c32668a38edd65b0b908bd9c))

## [0.0.11](https://github.com/vemonet/nanopub-rs/compare/v0.0.10..v0.0.11) - 2023-11-27

### 🐛 Bug Fixes

- Fix demo - ([f933d78](https://github.com/vemonet/nanopub-rs/commit/f933d780e34ed3fbcdc83096f84d869770f84bef))

## [0.0.10](https://github.com/vemonet/nanopub-rs/compare/v0.0.9..v0.0.10) - 2023-11-24

### 🐛 Bug Fixes

- Fix workflow upload - ([3df9d75](https://github.com/vemonet/nanopub-rs/commit/3df9d759c832f36cabea02dadfb03710e96e5d05))
- Fix docs - ([e7b6855](https://github.com/vemonet/nanopub-rs/commit/e7b6855bf887e4a74539ae58e214b537903f332d))
- Fix - ([5ab51c1](https://github.com/vemonet/nanopub-rs/commit/5ab51c1d2637ee96f38913ed0d3b67119937fa48))

## [0.0.9](https://github.com/vemonet/nanopub-rs/compare/v0.0.8..v0.0.9) - 2023-11-21

## [0.0.8](https://github.com/vemonet/nanopub-rs/compare/v0.0.7..v0.0.8) - 2023-11-21

### 🐛 Bug Fixes

- Fix binary install script - ([a07b07a](https://github.com/vemonet/nanopub-rs/commit/a07b07a04580ff5b64b9726a188e6ce33abb18d1))
- Fix dep - ([c9f9db0](https://github.com/vemonet/nanopub-rs/commit/c9f9db05dbe38f40b5cce1d538f51450cf24e124))
- Fix dep - ([a112208](https://github.com/vemonet/nanopub-rs/commit/a11220875c314dc13055f44fed94ee5901792e22))

## [0.0.7](https://github.com/vemonet/nanopub-rs/compare/v0.0.6..v0.0.7) - 2023-11-20

### 🐛 Bug Fixes

- Fix js pkg license - ([fc6e4c0](https://github.com/vemonet/nanopub-rs/commit/fc6e4c0977788d4d69870049d1fa8e0429486d7e))
- Fix js docs - ([2bcf5f5](https://github.com/vemonet/nanopub-rs/commit/2bcf5f54525e6788df445637cd54f68c1e15ce4a))

## [0.0.6](https://github.com/vemonet/nanopub-rs/compare/v0.0.5..v0.0.6) - 2023-11-20

### 🐛 Bug Fixes

- Fix npm publish bump 0.0.6 - ([3d3a0e5](https://github.com/vemonet/nanopub-rs/commit/3d3a0e59cae8b91f374adfa8fd1999fcf01cd8f8))

## [0.0.5](https://github.com/vemonet/nanopub-rs/compare/v0.0.4..v0.0.5) - 2023-11-20

### 🐛 Bug Fixes

- Fix npm publish bump 0.0.5 - ([bcccf74](https://github.com/vemonet/nanopub-rs/commit/bcccf74e8cae1805ddb3982f2700022c7a31afee))

## [0.0.4](https://github.com/vemonet/nanopub-rs/compare/v0.0.3..v0.0.4) - 2023-11-20

### 🐛 Bug Fixes

- Fix codecov workflow - ([54e20eb](https://github.com/vemonet/nanopub-rs/commit/54e20eb70d365f10dcad569c3a34558a5d36fdd0))

## [0.0.3](https://github.com/vemonet/nanopub-rs/compare/v0.0.2..v0.0.3) - 2023-11-20

### 🐛 Bug Fixes

- Fix bump script and bump to 0.0.3 - ([69ee8fa](https://github.com/vemonet/nanopub-rs/commit/69ee8faa0d741b99abee18b07ca5a2d7c203f5ba))

## [0.0.2](https://github.com/vemonet/nanopub-rs/compare/v0.0.1..v0.0.2) - 2023-11-20

### 🐛 Bug Fixes

- Fix issues with pkg versions - ([6822464](https://github.com/vemonet/nanopub-rs/commit/68224641ada479575ee4d9830d31e88e1aa90f5f))

## [0.0.1](https://github.com/vemonet/nanopub-rs/tree/v0.0.1) - 2023-11-20

### 🐛 Bug Fixes

- Fix docs - ([8e53b47](https://github.com/vemonet/nanopub-rs/commit/8e53b471fef87d1991ac3358b9e470ac34151ae6))
- Fix doc workflow - ([5fac322](https://github.com/vemonet/nanopub-rs/commit/5fac322f5f0bbd7b81973aa2871ba7a07dd68aea))
- Fix error msg js - ([2b244c4](https://github.com/vemonet/nanopub-rs/commit/2b244c4acc39b22c6dccea9aad37622c95dc8242))
- Fix ref - ([05fa3b9](https://github.com/vemonet/nanopub-rs/commit/05fa3b956586251bc96e0b704e20b5863a04a262))
- Fix ref - ([9522b91](https://github.com/vemonet/nanopub-rs/commit/9522b913eaee1159288312557895a6df80508893))
- Fix wheel build - ([cf88924](https://github.com/vemonet/nanopub-rs/commit/cf889246563419f60b134d8d7b83a9885f334c87))
- Fix python dev deps - ([10bb8ee](https://github.com/vemonet/nanopub-rs/commit/10bb8ee242097e0c829352089fa627e23debfded))
- Fix python dev deps - ([7b168f9](https://github.com/vemonet/nanopub-rs/commit/7b168f9d0a086d559a272df138e6ecef894b4a97))
- Fix workflow - ([282d923](https://github.com/vemonet/nanopub-rs/commit/282d923500016cf8559aeee5c89b7c975996a4e6))
- Fix wheel build - ([dd9d97b](https://github.com/vemonet/nanopub-rs/commit/dd9d97ba32d3eabd4059e3901632bfeb4e9773e3))
- Fix conf - ([53b40ca](https://github.com/vemonet/nanopub-rs/commit/53b40ca304b8f7eb16e0fa92de345d6a8438d55e))
- Fix workflow - ([4cfaab3](https://github.com/vemonet/nanopub-rs/commit/4cfaab304d574c9524fe0f31b81082b837670364))
- Fix workflow - ([78af5b2](https://github.com/vemonet/nanopub-rs/commit/78af5b2a105cd684832e3f7e8998a4f51491a581))
- Fix reqwest for wasm - ([d2b7400](https://github.com/vemonet/nanopub-rs/commit/d2b74008df3e16c26a86a1c6beca71f597c267b7))
- Fix wasm bindgen future import - ([3c214d9](https://github.com/vemonet/nanopub-rs/commit/3c214d97867fc617a8548a65078a6d7031f6ed29))
- Fix wasm - ([2feca44](https://github.com/vemonet/nanopub-rs/commit/2feca44fa8e8554948b22be78bbc014f9fa426d8))
- Fix profile pointer - ([dfba31c](https://github.com/vemonet/nanopub-rs/commit/dfba31cd883a9c75d18669741678d3a2ba75033d))
- Fix workflow - ([872eca6](https://github.com/vemonet/nanopub-rs/commit/872eca6ee25879f2fe6e569d66316ec8214e2a4f))
- Fix bench - ([3190c74](https://github.com/vemonet/nanopub-rs/commit/3190c74e2089011bb1e9b51945fd47fba452441d))
- Fix export benchmark md - ([09c99e1](https://github.com/vemonet/nanopub-rs/commit/09c99e115774ac1b5f5bd3f6e6ab51fc7889d688))
- Fix test - ([8473e8d](https://github.com/vemonet/nanopub-rs/commit/8473e8d08fb6817bd1b37a597b874b98a1741cc1))
- Fix install tarpaulin - ([b8bad05](https://github.com/vemonet/nanopub-rs/commit/b8bad05dd47b5807a01fe0e5a12d568476b6c6b9))
- Fix wasm - ([cc0e45c](https://github.com/vemonet/nanopub-rs/commit/cc0e45ca518325fd3a41a21c1b60cf74361b44b1))
- Fix wasm - ([8ea54d1](https://github.com/vemonet/nanopub-rs/commit/8ea54d1660c15b5b5e458598434c0dcdec107787))
- Fix wasm - ([4e375e4](https://github.com/vemonet/nanopub-rs/commit/4e375e47768607d165d494a39c9a4174d37d6940))
- Fix sophia version - ([d623ec7](https://github.com/vemonet/nanopub-rs/commit/d623ec7ff5b37fdeef140823cad7a40a6f6c7ac3))
- Fix version and delete lib/README.md - ([e4c0835](https://github.com/vemonet/nanopub-rs/commit/e4c08355ebcbee67e796434e798c3ece0c0ec146))
- Fix docs outside pkg - ([21ef352](https://github.com/vemonet/nanopub-rs/commit/21ef3521c71a75ab28c556db080a7c75639bba2e))

### 📚 Documentation

- Doc - ([9d98002](https://github.com/vemonet/nanopub-rs/commit/9d980024c0fad3179bd7a719115283274c216c6c))
- Docs - ([6ea7b08](https://github.com/vemonet/nanopub-rs/commit/6ea7b08337d8e344b0cc5a12ba4090ab3c77cb98))
- Docs - ([c95bcd3](https://github.com/vemonet/nanopub-rs/commit/c95bcd381b6929a60bd99acfde361d275d3444e1))
- Docs - ([8bd0e63](https://github.com/vemonet/nanopub-rs/commit/8bd0e63fbd238ee4267c10d90780eac54f7041eb))

<!-- generated by git-cliff -->
