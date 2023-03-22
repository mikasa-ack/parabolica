#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod racecar {
    use ink::prelude::string::String;
    use traits::{
        Racer,
        Move,
    };

    #[ink(storage)]
    pub struct Racecar {
        name: String,
    }

    impl Racecar {
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Racer for Racecar {
        #[ink(message)]
        pub fn take_turn(&mut self, track: Vec<Vec<bool>>, carIndex: u64) -> Move {
            Move::FireShell
        }
    }
}
