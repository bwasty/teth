use std::collections::HashMap;
use ethereum_types::{Address};
// use patricia_trie_ethereum::TrieDB;

use crate::lib::accountstate::AccountState;

/// σ
///
/// The world state (_state_), is a mapping between addresses (160-bit identifiers) and account states.
#[derive(Debug)]
pub struct WorldState {
    pub accounts: HashMap<Address, AccountState>,
    // pub trie: TrieDB,
}

#[allow(dead_code)]
impl WorldState {
    /// An account is dead when its account state is non-existent or empty:
    ///
    /// DEAD(σ, a) ≡ σ[a] = ∅ ∨ EMPTY(σ, a)
    pub fn is_account_dead(&self, account: &AccountState) -> bool {
        account.is_empty() || !self.accounts.values().any(|x| x == account)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_state() {
        let empty_acc = AccountState::default();
        let non_empty_acc = AccountState {
            balance: 1.into(),
            ..Default::default()
        };
        let non_empty_but_dead_acc = AccountState {
            balance: 2.into(),
            ..Default::default()
        };
        let world = WorldState {
            accounts: vec![
                (Address::random(), empty_acc.clone()),
                (Address::random(), non_empty_acc.clone()),
            ]
            .into_iter()
            .collect(),
        };
        assert!(world.is_account_dead(&empty_acc));
        assert!(!world.is_account_dead(&non_empty_acc));
        assert!(world.is_account_dead(&non_empty_but_dead_acc));
    }
}
