workflow "Build" {
  on = "push"
  resolves = ["Build"]
}

action "Build" {
  uses = "icepuma/rust-action@master"
  args = "cargo build && cargo clippy -- -Dwarnings && cargo test"
}
