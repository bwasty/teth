use lazy_static::lazy_static;

/// Fee Schedule (Appendix G)
///
/// The fee schedule G is a tuple of 31 scalar values corresponding to the relative costs, in gas, of a number of abstract operations that a transaction may effect.
pub struct FeeSchedule {
    /// Paid by all contract-creating transactions after the Homestead transition.
    pub tx_create: u64,
    /// Paid for every zero byte of data or code for a transaction.
    pub tx_data_zero: u64,
    /// Paid for every non-zero byte of data or code for a transaction.
    pub tx_data_non_zero: u64,
    /// Paid for every transaction.
    pub transaction: u64,
}

impl FeeSchedule {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for FeeSchedule {
    fn default() -> Self {
        Self {
            tx_create: 32000,
            tx_data_zero: 4,
            tx_data_non_zero: 68,
            transaction: 21000,
        }
    }
}

lazy_static! {
    /// lazy_static for `FeeSchedule`
    pub static ref FEES: FeeSchedule = FeeSchedule::default();
}
