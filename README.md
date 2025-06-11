# Rust Study Repogitory

## Getting Started

다음 명령어를 통해 install 합니다
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

환경변수 설정
```
# ~/.zshrc
export RUSTUP_HOME="$HOME/.cargo"
export PATH="$RUSTUP_HOME/bin:$PATH"
```

```sh
rustup default stable
rustc --version
```

vscode `rust-analyzer` 설치


```sh
cargo new hello_cargo
cargo build
cargo run # Hello, wrold!
```