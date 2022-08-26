# Polkapad Staking

## Building Locally

### ⚙️ Install Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### ⚒️ Add specific toolchains

```shell
rustup toolchain add nightly
rustup target add wasm32-unknown-unknown --toolchain nightly
```

... or ...

```shell
make init
```

### Clone Fungible Token contract repository

```shell
git clone 'TODO: link'
```

### Clone Staking contract repository

```shell
git clone 'TODO: link'
```

### Put the correct path to the Fungible Token folder in Cargo.toml 

```Cargo.toml
[dependencies]
...
staking-io = { path = "io" }
ft-io = { path = "../fungible-token/io" }
...
```

### 🏗️ Build

```shell
cargo build --release
```

... or ...

```shell
make build
```

### ✅ Run tests

```shell
cargo test --release
```

... or ...

```shell
make test
```

### 🚀 Run everything with one command

```shell
make all
```

... or just ...

```shell
make
```

## License

The source code is licensed under [GPL v3.0 license](LICENSE).