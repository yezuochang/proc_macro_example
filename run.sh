rm -rf ~/.rustup
rm -rf ~/.cargo
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup install nightly-2021-12-20
rustup default nightly-2021-12-20
rustup component add rust-src
rustup component add rust-src
rustup component add rust-analyzer-preview
cargo clean
cargo build -p macro_crate
cargo build