#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod racecar {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use traits::{Move, Racer};

    #[ink(storage)]
    pub struct Racecar {
        name: u8,
    }

    impl Racecar {
        #[ink(constructor)]
        pub fn new(name: u8) -> Self {
            Self { name }
        }
    }

    impl Racer for Racecar {
        #[ink(message)]
        fn take_turn(&mut self, track: Vec<Vec<Move>>, car_index: u64) -> Move {
            if car_index % 2 == 0 {
                Move::FireShell
            } else {
                Move::Accelerate
            }
        }
    }
}
