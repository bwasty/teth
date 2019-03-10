use ethereum_types::U256;
use lazy_static::lazy_static;

/// Alias for `ethereum_types::U256`
pub type Wei = U256;

lazy_static! {
    /// lazy_static for one Ether in Wei.
    pub static ref ONE_ETHER: Wei = Wei::from(10).pow(18.into());
}
