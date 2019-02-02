mod lib;
use lib::*;

fn main() {
    let world = WorldState { accounts: vec![] };
    dbg!(world);

    let acc = AccountState::default();
    dbg!(acc);
}
