# sewerhash-rs

[![Crates.io](https://img.shields.io/crates/v/sewerhash-rs.svg)](https://crates.io/crates/sewerhash-rs)
[![Docs.rs](https://docs.rs/sewerhash-rs/badge.svg)](https://docs.rs/sewerhash-rs)
[![CI](https://github.com/Sewer56/sewerhash-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/Sewer56/sewerhash-rs/actions)

-----------------------

## Additional Setup Required!!

Additional setup might be required.  
Once you do the stuff under this text, delete this section.  

### Setup Documentation

If you don't have a `docs/Reloaded` folder, an error occurred, and you might need to add it manually as submodule:

```
git submodule add https://github.com/Reloaded-Project/Reloaded.MkDocsMaterial.Themes.R2.git docs/Reloaded
```

Once you make your first commit to GitHub, you'll need to do the following:
- Go to `Settings -> Pages` in your repo and select `gh-pages` branch to enable GitHub Pages.


### Setup API Key

You'll need to set up an API key to publish to `crates.io` on tag.

- Go to `Settings -> Secrets -> Actions` in your repo and add environment secret named `CRATES_IO_TOKEN`.  
- Paste your API key generated in [crates.io (Account Settings -> API Tokens)][crates-io-key].  


-----------------------

## About

A custom vectorised variant of FNV-1a, extremely fast hash implemethat trades a bit of quality for a lot of speed.

You can learn more about this project in the [dedicated documentation page][docs].

## Development

How to develop this project.

***Clone this Repository:***
```bash
# When cloning, make sure symlinks are enabled
git clone -c core.symlinks=true https://github.com/Sewer56/sewerhash-rs.git
```

***Install Rust:***
- Install the [Rust Toolchain.][rust-toolchain]***Setup IDE***
- This repository is fully with VSCode. [Guidance below](#visual-studio-code-integration).

### Visual Studio Code Integration

`Code`/`VSCode` is the de-facto Rust development environment.  

The following extensions are required:  
- [rust-analyzer][rust-analyzer] for Rust support.  
- [coverage-gutters][coverage-gutters] for Coverage support.  
- [CodeLLDB][codelldb] for debugging.  
- [crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates) easier dependency management.  

The VSCode configuration in Reloaded projects (`.vscode`) contain the following:  
- Run Rust linter `clippy` on Save.  
- Run code format `rustfmt` on Save.  
- Tasks for common operations (generate documentation, active CI/CD etc.).  

These configurations are in the `.vscode` folder; and the tasks can be ran via `Ctrl+Shift+P -> Run Task`.  

#### Test Coverage

To run Coverage, run task (`Ctrl+Shift+P -> Run Task`), you should see: 

| Task                   | Description                                                                |
| ---------------------- | -------------------------------------------------------------------------- |
| Cargo Watch Tarpaulin  | Automatically runs tests and updates coverage on save.                     |
| Generate Code Coverage | Manually generate code coverage (`cobertura.xml`, `tarpaulin-report.html`) |

The `tarpaulin-report.html` file can be opened in VSCode (`Show Preview`) for a live view.

For GUI integration, run action `Coverage Gutter: Watch` (in `Ctrl+Shift+P` actions menu).

## Debugging Benchmarks

If you wish to debug benchmarks in VSCode, go to `Run and Debug` Menu and generate the launch 
profiles, you should get one for debugging benchmarks.

## Profiling Benchmarks

### Linux/OSX

Execute the following:

```
cargo bench --bench my_benchmark --profile profile -- --profile-time 10
```

This should give you a flamegraph in `target/criterion/<method_name>/profile`. You can open that flamegraph in a web browser.

### Windows

Execute the following:

```
cargo bench --bench my_benchmark --no-run --profile profile
```

Navigate to the executable listed in the commandline:

```
target/profile/deps/my_benchmark-eced832ac8f31257.exe
```

And run with command `my_benchmark-eced832ac8f31257.exe --bench --profile-time 10` under an external profiler, such as Visual Studio.  

![example](./assets/profile_example.png)

## Optimizing for Size when Creating C Libraries

1. Add `"cdylib"` crate type to `Cargo.toml` (if not already present)

```
[lib]
crate-type = ["cdylib"]
```

Install `cargo-bloat`, `nightly toolchain` and `build-std`:

```
cargo install cargo-bloat
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

Run `cargo-bloat` the following command to calculate package size:

```
cargo +nightly bloat --profile profile -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-unknown-linux-gnu -n 100
```

Change `--target` if needed for your platform.  
This should produce binaries more appropriate for dynamic linking from C.  

## File Layout

The following is the expected file layout for your project:

```
.vscode/
docs/
src/
Cargo.toml
mkdocs.yml
```

The `docs` folder, and `mkdocs.yml` contain [MkDocs Material documentation][mkdocs-material] for your project.  
The `src` folder should contains all source code for your project.  

`Cargo.toml` should be in the root of the project.

## Contributing

See [CONTRIBUTING](CONTRIBUTING.MD) for guidance on how to contribute to this project.  

## License

Licensed under [LGPL v3](./LICENSE).  

[Learn more about Reloaded's general choice of licensing for projects.][reloaded-license].  

[codelldb]: https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb
[coverage-gutters]: https://marketplace.visualstudio.com/items?itemName=ryanluker.vscode-coverage-gutters 
[docs]: https://sewerhash-rs.github.io/sewerhash-rs
[mkdocs-material]: https://squidfunk.github.io/mkdocs-material/
[reloaded-license]: https://reloaded-project.github.io/Reloaded.MkDocsMaterial.Themes.R2/Pages/license/
[rust-analyzer]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer
[rust-toolchain]: https://www.rust-lang.org/tools/install
