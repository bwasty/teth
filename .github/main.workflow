workflow "CI" {
  on = "push"
  resolves = ["Deploy to GitHub Pages"]
}

action "Build, Lint, Test" {
  uses = "icepuma/rust-action@master"
  args = "cargo build && cargo clippy -- -Dwarnings && cargo test && cargo doc"
}

action "Deploy to GitHub Pages" {
  needs = "Build, Lint, Test"
  uses = "maxheld83/ghpages@v0.1.1"
  env = {
    BUILD_DIR = "target/doc/"
  }
  secrets = ["GH_PAT"]
}
