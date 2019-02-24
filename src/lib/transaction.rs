use ethereum_types::{Address, U256};
use lazy_static::lazy_static;
use rlp::{Encodable, RlpStream};

use crate::lib::{Wei, FEES, WorldState};

/// ECDSA signature
#[derive(Debug, Default)]
pub struct Signature {
    /// T<sub>w</sub>
    pub v: u8,
    /// T<sub>r</sub>
    pub r: U256,
    /// T<sub>s</sub>
    pub s: U256,
}

lazy_static! {
    static ref SECP256K1N: U256 = U256::from_dec_str(
        "115792089237316195423570985008687907852837564279074904382605163141518161494337"
    )
    .unwrap();
}

#[allow(dead_code)]
impl Signature {
    pub fn new(v: u8, r: U256, s: U256) -> Self {
        Self { v, r, s }
    }

    /// We declare that an ECDSA signature is invalid unless all the following conditions are true:  
    ///     0 < r < secp256k1n  
    ///     0 < s < secp256k1n÷2+1  
    ///     v ∈ {27, 28}  
    /// where:  
    ///     secp256k1n = 115792089237316195423570985008687907852837564279074904382605163141518161494337
    pub fn is_valid(&self) -> bool {
        self.r > U256::zero()
            && self.r < *SECP256K1N
            && self.s > U256::zero()
            && self.s < *SECP256K1N / 2 + 1
            && self.v == 27
            || self.v == 28
    }
}

/// T
///
/// A transaction (formally, T) is a single cryptographically-signed instruction constructed by an
/// actor externally to the scope of Ethereum. While it is assumed that the ultimate external actor
/// will be human in nature, software tools will be used in its construction and dissemination.
/// There are two types of transactions: those which result in message calls and those which
/// result in the creation of new accounts with associated code (known informally as ‘contract
/// creation’).
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct Transaction {
    /// A scalar value equal to the number of transactions sent by the sender; formally T<sub>n</sub>.
    pub nonce: U256,
    /// A scalar value equal to the number of Wei to be paid per unit of _gas_ for all computation costs incurred
    /// as a result of the execution of this transaction; formally T<sub>p</sub>.
    pub gas_price: Wei,
    /// A scalar value equal to the maximum amount of gas that should be used in executing this transaction.
    /// This is paid up-front, before any computation is done and may not be increased later; formally T<sub>g</sub>.
    pub gas_limit: U256,
    /// The 160-bit address of the message call’s recipient or, for a contract creation transaction, ∅,
    /// used here to denote the only member of B<sub>0</sub>; formally T<sub>t</sub>.
    pub to: Option<Address>,
    /// A scalar value equal to the number of Wei to be transferred to the message call’s recipient or,
    /// in the case of contract creation, as an endowment to the newly created account; formally T<sub>v</sub>.
    pub value: Wei,

    /// Values corresponding to the signature of the transaction and used to determine the sender of the transaction;
    /// formally T<sub>w</sub>, T<sub>r</sub> and T<sub>s</sub>. This is expanded in Appendix F.
    pub signature: Signature,

    /// An unlimited size byte array specifying the EVM-code for the account initialisation procedure, formally
    /// T<sub>i</sub>.  
    /// `init` is an EVM-code fragment; it returns the `body`, a second fragment of code that executes each time the
    /// account receives a message call (either through a transaction or due to the internal execution of code).
    /// `init` is executed only once at account creation and gets discarded immediately thereafter.  
    /// Empty for message calls.
    pub init: Option<Vec<u8>>,

    /// An unlimited size byte array specifying the input data of the message call, formally T<sub>d</sub>.  
    /// Empty for contract creation transactions.
    pub data: Option<Vec<u8>>,
}

#[allow(dead_code)]
impl Transaction {
    /// S
    ///
    /// Appendix F specifies the function, S, which maps transactions to the sender, and happens through
    /// the ECDSA of the SECP-256k1 curve, using the hash of the transaction (excepting the latter three
    /// signature fields) as the datum to sign. For the present we simply assert that the sender of a
    /// given transaction T can be represented with S(T).
    pub fn sender(&self) -> Address {
        unimplemented!() // TODO!
    }

    /// Section 6 (beginning)
    pub fn validate(&self) {
        // TODO!!:
        // (1) The transaction is well-formed RLP, with no additional trailing bytes;
        // (2) the transaction signature is valid;
        // (3) the transaction nonce is valid (equivalent to the sender account’s current nonce);
        // (4) the gas limit is no smaller than the intrinsic gas, g0, used by the transaction; and
        // (5) the sender account balance contains at least the cost, v0, required in up-front payment.

        // See also Equation 58:
        // S(T) != ∅ ∧
        // σ[S(T)] != ∅ ∧
        // Tn = σ[S(T)]n ∧
        // g0 <= Tg ∧
        // v0 <= σ[S(T)]b ∧
        // Tg <= B_Hl−l(B_R)u
        //  -> he sum of the transaction’s gas limit, Tg, and the gas utilised in this block prior,
        //     given by l(BR)u, must be no greater than the block’s gasLimit, BHl
        unimplemented!()
    }

    /// Section 6.2: We define intrinsic gas g<sub>0</sub>, the amount of gas this transaction requires to be paid prior
    /// to execution, as follows:
    pub fn intrinsic_gas(&self) -> u64 {
        let mut g0: u64 = 0;
        let data_or_code = if let Some(init) = &self.init {
            init
        } else if let Some(data) = &self.data {
            data
        } else {
            unreachable!()
        };

        g0 += data_or_code
            .iter()
            .map(|i| {
                if *i == 0 {
                    FEES.tx_data_zero
                } else {
                    FEES.tx_data_non_zero
                }
            })
            .sum::<u64>();

        if self.to.is_none() {
            g0 += FEES.tx_create;
        }

        g0 += FEES.transaction;

        g0
    }

    /// v<sub>0</sub> (Equation 57)
    pub fn up_front_cost(&self) -> U256 {
        self.gas_limit * self.gas_price + self.value
    }

    /// Section 6.2
    pub fn execute(&self, state: &mut WorldState) {
        let mut sender_account = state.accounts[&self.sender()].clone();
        // Equation 60
        sender_account.balance -= self.gas_limit * self.gas_price;
        // Equationn 61
        sender_account.nonce += 1.into();

        // => checkpoint state σ0

        // gas available for the proceeding computation (Equation 63)
        let g = self.gas_limit - self.intrinsic_gas();
    
    
    }

    // TODO!: refund counter, self destructed accounts... equation 64
    // TODO!: g* ... Equation 65
}

/// Equation 15
impl Encodable for Transaction {
    fn rlp_append(&self, s: &mut RlpStream) {
        s.append(&self.nonce);
        s.append(&self.gas_price);
        s.append(&self.gas_limit);
        s.append(&self.to);
        s.append(&self.value);
        if self.to.is_none() {
            s.append(&self.init);
        } else {
            s.append(&self.data);
        }
        s.append(&self.signature.v);
        s.append(&self.signature.r);
        s.append(&self.signature.s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore] // TODO!!: missing implementation...
    fn test_transaction() {
        let t = Transaction::default();
        assert_eq!(t.sender(), Address::zero());
    }

    #[test]
    fn test_signature() {
        let sig = Signature::new;
        let s = sig(0, 0.into(), 0.into());
        assert!(!s.is_valid());

        let s1 = sig(27, 1.into(), 1.into());
        assert!(s1.is_valid())
    }
}
