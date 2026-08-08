use std::str::FromStr;

use schrod::Schrod::{self, Pass};
use serde::{Deserialize, Serialize};
use slip44::Coin;
use uuid::Uuid;

use crate::{bits::{bit::{Bit, BitTypes}, bit_wallet::BitWallet}, vault::transaction::Date};

/// Manages a collection of `BitWallet`s.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BitBank {
    /// The collection of `BitWallet`s.
    pub wallets: Vec<BitWallet>,
}
impl BitBank {
    // initializing
    /// Creates a new `BitBank`.
    #[must_use]
    pub fn new() -> BitBank {
        BitBank { wallets: Vec::new() }
    }
    


    // wallet management
    /// Adds a new `BitWallet`.
    #[must_use]
    pub fn add_wallet(&mut self, name: &str, coin_string: &str) -> Schrod<()> {
        let new_wallet_result = BitWallet::new_from_raw_parts(name, coin_string);
        if new_wallet_result.is_fail() {
            return new_wallet_result
                .convert("BitBank::add_wallet()")
                .fail("Failed to add wallet.", "BitBank::add_wallet()")
        }
        let new_wallet = new_wallet_result.wont_fail("This is past an is_fail() guard clause.", "BitBank::add_wallet()");
        
        self.wallets.push(new_wallet);
        Pass(())
    }

    /// Edits the `name` of a `BitWallet`.
    #[must_use]
    pub fn edit_name(&mut self, wallet: Uuid, new_name: &str) -> Schrod<()> {
        // gets the wallet
        let wallet_result = self.get_wallet_mut(wallet);
        if wallet_result.is_fail() {
            return wallet_result
                .convert("BitBank::edit_name()")
                .fail("Failed to edit a wallet name.", "BitBank::edit_name()")
        }
        let wallet = wallet_result.wont_fail("This is past an is_fail() guard clause.", "BitBank::edit_name()");

        // edits
        let result = wallet.edit_name(new_name);
        if result.is_fail() {
            return result
                .fail("Failed to edit a wallet name.", "BitBank::edit_name()")
        }
        Pass(())
    }

    /// Edits the `coin` of a `BitWallet`.
    #[must_use]
    pub fn edit_coin(&mut self, wallet: Uuid, new_coin_string: &str) -> Schrod<()> {
        // gets the wallet
        let wallet_result = self.get_wallet_mut(wallet);
        if wallet_result.is_fail() {
            return wallet_result
                .convert("BitBank::edit_coin()")
                .fail("Failed to edit a wallet coin.", "BitBank::edit_coin()")
        }
        let wallet = wallet_result.wont_fail("This is past an is_fail() guard clause.", "BitBank::edit_coin()");

        // edits
        let result = wallet.edit_coin(new_coin_string);
        if result.is_fail() {
            return result
                .fail("Failed to edit a wallet coin.", "BitBank::edit_coin()")
        }
        Pass(())
    }

    /// Removes a `BitWallet`.
    pub fn remove_wallet(&mut self, id: Uuid) {
        self.wallets.retain(|wallet| wallet.get_id() != id);
    }

    /// Adds a `Bit` to a `BitWallet`.
    #[must_use]
    pub fn add_bit(&mut self, wallet: Uuid, amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date, bit_type: BitTypes) -> Schrod<()> {
        // gets the wallet
        let wallet_result = self.get_wallet_mut(wallet);
        if wallet_result.is_fail() {
            return wallet_result
                .convert("BitBank::add_bit()")
                .fail("Failed to add a Bit.", "BitBank::add_bit()")
        }
        let wallet = wallet_result.wont_fail("This is past an is_fail() guard clause.", "BitBank::add_bit()");

        // adds the bit
        let result = wallet.add_bit_from_raw_parts(amount_string, coin_value_amount_string, coin_value_currency_string, date, bit_type);
        if result.is_fail() {
            return result
                .convert("BitBank::add_bit()")
                .fail("Failed to add a Bit.", "BitBank::add_bit()")
        }
        Pass(())
    }

    /// Edits a `Bit` in a `BitWallet`.
    #[must_use]
    pub fn edit_bit(&mut self, wallet: Uuid, bit: Uuid, amount_string: &str, coin_value_amount_string: &str, coin_value_currency_string: &str, date: Date, bit_type: BitTypes) -> Schrod<()> {
        // gets the wallet
        let wallet_result = self.get_wallet_mut(wallet);
        if wallet_result.is_fail() {
            return wallet_result
                .convert("BitBank::edit_bit()")
                .fail("Failed to edit a Bit.", "BitBank::edit_bit()")
        }
        let wallet = wallet_result.wont_fail("This is past an is_fail() guard clause.", "BitBank::edit_bit()");

        // edits the bit
        let result = wallet.edit_bit_with_raw_parts(bit, amount_string, coin_value_amount_string, coin_value_currency_string, date, bit_type);
        if result.is_fail() {
            return result
                .convert("BitBank::edit_bit()")
                .fail("Failed to edit a Bit.", "BitBank::edit_bit()")
        }
        Pass(())
    }

    /// Removes a `Bit` from a `BitWallet`.
    #[must_use]
    pub fn remove_bit(&mut self, wallet: Uuid, bit: Uuid) -> Schrod<()> {
        // gets the wallet
        let wallet_result = self.get_wallet_mut(wallet);
        if wallet_result.is_fail() {
            return wallet_result
                .convert("BitBank::remove_bit()")
                .fail("Failed to remove a Bit.", "BitBank::remove_bit()")
        }
        let wallet = wallet_result.wont_fail("This is past an is_fail() guard clause.", "BitBank::remove_bit()");

        // removes the bit
        wallet.remove_bit(bit);
        Pass(())
    }

    
    
    // data retrieval
    /// Gets an immutable reference to a `BitWallet`.
    #[must_use]
    pub fn get_wallet(&self, id: Uuid) -> Schrod<&BitWallet> {
        for i in 0..self.wallets.len() {
            if self.wallets[i].get_id() == id { return Pass(&self.wallets[i]) }
        }

        Schrod::new_fail("Failed to get wallet.", "BitBank::get_wallet()")
    }
    
    #[must_use]
    /// Gets a mutable reference to a `BitWallet`.
    pub fn get_wallet_mut(&mut self, id: Uuid) -> Schrod<&mut BitWallet> {
        for i in 0..self.wallets.len() {
            if self.wallets[i].get_id() == id { return Pass(&mut self.wallets[i]) }
        }

        Schrod::new_fail("Failed to get wallet.", "BitBank::get_wallet_mut()")
    }
}