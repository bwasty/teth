use std::collections::HashMap;

use ethereum_types::Address;
// use keccak_hasher::KeccakHasher;
// use kvdb::DBValue;
// use memorydb::MemoryDB;
// use patricia_trie_ethereum::TrieDBMut;
// use hash_db::HashDB;

use crate::lib::accountstate::AccountState;
use crate::lib::ONE_ETHER;

/// σ
///
/// The world state (_state_), is a mapping between addresses (160-bit identifiers) and account states.
#[derive(Debug, Default)]
pub struct WorldState {
    pub accounts: HashMap<Address, AccountState>,
    // pub account_db: TrieDBMut<'db>,
    // db: HashDB<KeccakHasher, DBValue>,
}

#[allow(dead_code)]
impl WorldState {
    pub fn new() -> Self {
        // let mut memdb = MemoryDB::<KeccakHasher, DBValue>::new();
        // let mut root = H256::new();
        // let mut t = TrieDBMut::new(&mut memdb, &mut root);

        // let db = HashDB::<KeccakHasher, DBValue>new();
        // let account_db =
        WorldState {
            accounts: HashMap::new(),
            // account_db:: TrieDBMut::new(&mut memdb, &mut root);
        }
    }

    /// An account is dead when its account state is non-existent or empty:
    ///
    /// DEAD(σ, a) ≡ σ[a] = ∅ ∨ EMPTY(σ, a)
    pub fn is_account_dead(&self, account: &AccountState) -> bool {
        account.is_empty() || !self.accounts.values().any(|x| x == account)
    }

    pub fn genesis_state() -> Self {
        let mut state = Self::new();
        let account = AccountState::new(*ONE_ETHER * 42u32);
        state.accounts.insert(
            Address::from("0xe94C35a6a32c759865dc49807f0ad3056848C8BF"),
            account,
        );

        let account = AccountState::new(*ONE_ETHER * 420u32);
        state.accounts.insert(Address::random(), account);

        let account = AccountState::new(*ONE_ETHER * 4200u32);
        state.accounts.insert(Address::random(), account);

        state
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
