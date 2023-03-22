#![cfg_attr(not(feature = "std"), no_std)]

pub use self::racecar::{
    Racecar,
    RacecarRef,
}

#[ink::contract]
mod racecar {
    use ink::prelude::vec::Vec;
    use ink::prelude::string::String;
    use traits::{
        Racer,
        Move,
    };

    #[ink(storage)]
    pub struct Racecar {
        name: u64,
    }

    impl Racecar {
        #[ink(constructor)]
        pub fn new(name: u64) -> Self {
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
