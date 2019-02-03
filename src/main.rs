use std::collections::HashMap;

mod lib;
use lib::*;

fn main() {
    let world = WorldState { accounts: HashMap::new() };
    dbg!(world);

    let acc = AccountState::default();
    dbg!(acc);
}
