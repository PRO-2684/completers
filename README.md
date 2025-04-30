# completers

[![GitHub License](https://img.shields.io/github/license/PRO-2684/completers?logo=opensourceinitiative)](https://github.com/PRO-2684/completers/blob/main/LICENSE)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/PRO-2684/completers/release.yml?logo=githubactions)](https://github.com/PRO-2684/completers/blob/main/.github/workflows/release.yml)
[![GitHub Release](https://img.shields.io/github/v/release/PRO-2684/completers?logo=githubactions)](https://github.com/PRO-2684/completers/releases)
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/PRO-2684/completers/total?logo=github)](https://github.com/PRO-2684/completers/releases)
[![Crates.io Version](https://img.shields.io/crates/v/completers?logo=rust)](https://crates.io/crates/completers)
[![Crates.io Total Downloads](https://img.shields.io/crates/d/completers?logo=rust)](https://crates.io/crates/completers)
[![docs.rs](https://img.shields.io/docsrs/completers?logo=rust)](https://docs.rs/completers)

Rust-native shell completion.

## ⚙️ Automatic Releases Setup

1. [Create a new GitHub repository](https://github.com/new) with the name `completers` and push this generated project to it.
2. Enable Actions for the repository, and grant "Read and write permissions" to the workflow [here](https://github.com/PRO-2684/completers/settings/actions).
3. [Generate an API token on crates.io](https://crates.io/settings/tokens/new), with the following setup:

    - `Name`: `completers`
    - `Expiration`: `No expiration`
    - `Scopes`: `publish-new`, `publish-update`
    - `Crates`: `completers`

4. [Add a repository secret](https://github.com/PRO-2684/completers/settings/secrets/actions) named `CARGO_TOKEN` with the generated token as its value.
5. Consider removing this section and updating this README with your own project information.

## 📥 Installation

### Using [`binstall`](https://github.com/cargo-bins/cargo-binstall)

```shell
cargo binstall completers
```

### Downloading from Releases

Navigate to the [Releases page](https://github.com/PRO-2684/completers/releases) and download respective binary for your platform. Make sure to give it execute permissions.

### Compiling from Source

```shell
cargo install completers
```

## 💡 Examples

TODO

## 📖 Usage

TODO

## 🎉 Credits

TODO
