#![cfg_attr(not(feature = "std"), no_std)]

use ink::prelude::vec::Vec;

#[derive(Debug, PartialEq, Eq, Clone, scale::Encode, scale::Decode)]
#[cfg_attr(
    feature = "std",
    derive(ink::storage::traits::StorageLayout, scale_info::TypeInfo)
)]
pub enum Move {
    Accelerate,
    FireShell,
    Empty,
}

#[ink::trait_definition]
pub trait Racer {
    #[ink(message)]
    fn take_turn(&mut self, track: Vec<Vec<Move>>, car_index: u64, lap: u64) -> Move;
}
