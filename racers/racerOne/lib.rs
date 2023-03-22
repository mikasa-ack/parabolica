#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod racerOne {
    use ink::prelude::string::String;
    use racer::Move;

    #[ink(storage)]
    pub struct RacerOne {
        name: String,
    }

    impl RacerOne {
        #[ink(constructor)]
        pub fn new(name: String) -> Self {
            Self { name }
        }

        #[ink(message)]
        pub fn take_turn(&mut self, track: Vec<Vec<bool>>, carIndex: u64) -> Move {
            self.value = !self.value;
        }
    }
}
