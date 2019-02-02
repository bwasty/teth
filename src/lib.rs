use tiny_keccak::keccak256;

/// σ
#[derive(Debug)]
pub struct WorldState {
    pub accounts: Vec<AccountState>
}

#[allow(dead_code)]
impl WorldState {
    /// An account is dead when its account state is non-existent or empty:
    /// 
    /// DEAD(σ, a) ≡ σ[a] = ∅ ∨ EMPTY(σ, a)
    pub fn is_account_dead(&self, account: &AccountState) -> bool {
        account.is_empty() || !self.accounts.contains(account)
    }
}

pub type Hash = [u8; 32];

/// σ[a]
#[derive(Debug, Default, PartialEq)]
pub struct AccountState {
    /// A scalar value equal to the number of transactions sent from this address or, 
    /// in the case of accounts with associated code, the number of contract-creations made by 
    /// this account. For account of address a in state σ, this would be formally denoted σ[a]n.
    pub nonce: u64,
    /// A scalar value equal to the number of Wei owned by this address. Formally denoted σ[a]b.
    pub balance: u64, 
    /// A 256-bit hash of the root node of a Merkle Patricia tree that encodes the storage contents 
    /// of the account (a mapping between 256-bit integer values), encoded into the trie as a mapping 
    /// from the Keccak 256-bit hash of the 256-bit integer keys to the RLP-encoded 256-bit integer 
    /// values. The hash is formally denoted σ[a]s.
    /// TODO?: It shall be understood that σ[a]s is not a ‘physical’ member of the account and does 
    /// not contribute to its later serialisation.
    pub storage_root: Hash,
    /// The hash of the EVM code of this account — this is the code that gets executed should this 
    /// address receive a message call; it is immutable and thus, unlike all other fields, cannot 
    /// be changed after construction. All such code fragments are contained in the state database 
    /// under their corresponding hashes for later retrieval. This hash is formally denoted σ[a]c, 
    /// and thus the code may be denoted as b, given that KEC(b) = σ[a]c.
    pub code_hash: Hash,
}

#[allow(dead_code)]
impl AccountState {
    /// If the codeHash field is the Keccak-256 hash of the empty string, i.e. σ[a]c = KEC(()), 
    /// then the node represents a simple account, sometimes referred to as a “non-contract” account.
    pub fn has_code(&self) -> bool {
        self.code_hash == keccak256(&[])
    }

    /// An account is empty when it has no code, zero nonce and zero balance:
    /// 
    /// EMPTY(σ, a) ≡ σ[a]c = KEC(()) ∧ σ[a]n = 0 ∧ σ[a]b = 0
    pub fn is_empty(&self) -> bool {
        self.nonce == 0 && self.balance == 0 && !self.has_code()
    }
}

// TODO!: world-state collapse function L_S
// This function, LS, is used alongside the trie function to provide a short identity (hash) of the world state

// TODO!: v is the account validity function...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_account_state() {
        let mut acc = AccountState::default();
        assert!(!acc.has_code());
        assert!(acc.is_empty());

        acc.code_hash = keccak256(&[1, 2, 3]);
        dbg!(acc.code_hash);
        assert!(acc.has_code());
        assert!(!acc.is_empty());
    }

    #[test]
    fn test_world_state() {
        let world = WorldState { accounts: vec![AccountState::default()] };
        assert!(world.is_account_dead(&AccountState::default()));
    }
}
