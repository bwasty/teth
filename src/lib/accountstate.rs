use ethereum_types::{H256, U256};
use rlp::{Encodable, RlpStream};
use tiny_keccak::keccak256;

use crate::lib::types::Wei;

/// σ[a]
#[derive(Debug, PartialEq, Clone)]
pub struct AccountState {
    /// A scalar value equal to the number of transactions sent from this address or,
    /// in the case of accounts with associated code, the number of contract-creations made by
    /// this account. For account of address a in state σ, this would be formally denoted σ[a]<sub>n</sub>.
    pub nonce: U256,
    /// A scalar value equal to the number of Wei owned by this address. Formally denoted σ[a]<sub>b</sub>.
    pub balance: Wei,
    /// A 256-bit hash of the root node of a Merkle Patricia tree that encodes the storage contents
    /// of the account (a mapping between 256-bit integer values), encoded into the trie as a mapping
    /// from the Keccak 256-bit hash of the 256-bit integer keys to the RLP-encoded 256-bit integer
    /// values. The hash is formally denoted σ[a]<sub>s</sub>.
    /// TODO?: It shall be understood that σ[a]<sub>s</sub> is not a ‘physical’ member of the account and does
    /// not contribute to its later serialisation.
    pub storage_root: H256,
    /// The hash of the EVM code of this account — this is the code that gets executed should this
    /// address receive a message call; it is immutable and thus, unlike all other fields, cannot
    /// be changed after construction. All such code fragments are contained in the state database
    /// under their corresponding hashes for later retrieval. This hash is formally denoted σ[a]<sub>c</sub>,
    /// and thus the code may be denoted as b, given that KEC(b) = σ[a]<sub>c</sub>.
    pub code_hash: H256,
}

impl Default for AccountState {
    fn default() -> Self {
        AccountState {
            nonce: 0.into(),
            balance: 0.into(),
            storage_root: keccak256(&[]).into(),
            code_hash: keccak256(&[]).into(),
        }
    }
}

#[allow(dead_code)]
impl AccountState {
    /// If the codeHash field is the Keccak-256 hash of the empty string, i.e. σ[a]<sub>c</sub> = KEC(()),
    /// then the node represents a simple account, sometimes referred to as a “non-contract” account.
    pub fn has_code(&self) -> bool {
        self.code_hash != keccak256(&[]).into()
    }

    /// An account is empty when it has no code, zero nonce and zero balance:
    ///
    /// EMPTY(σ, a) ≡ σ[a]<sub>c</sub> = KEC(()) ∧ σ[a]<sub>n</sub> = 0 ∧ σ[a]<sub>b</sub> = 0
    pub fn is_empty(&self) -> bool {
        self.nonce.is_zero() && self.balance.is_zero() && !self.has_code()
    }
}

/// p(a) ≡ KEC(a), RLP(σ[a]n, σ[a]b, σ[a]s, σ[a]c))
impl Encodable for AccountState {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.balance);
        s.append(&self.storage_root);
        s.append(&self.code_hash);
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

        acc.code_hash = keccak256(&[1, 2, 3]).into();
        assert!(acc.has_code());
        assert!(!acc.is_empty());
    }
}
