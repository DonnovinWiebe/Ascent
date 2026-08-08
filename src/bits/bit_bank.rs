use serde::{Deserialize, Serialize};

use crate::bits::bit_wallet::BitWallet;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitBank {
    pub wallets: Vec<BitWallet>,
}