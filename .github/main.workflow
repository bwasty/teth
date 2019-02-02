workflow "Build" {
  on = "push"
  resolves = ["Test"]
}

action "Test" {
  uses = "icepuma/rust-action@master"
  args = "cargo build && cargo clippy -- -Dwarnings && cargo test"
}
