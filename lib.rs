#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod parabolica {
    use ink::prelude::vec;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Parabolica {
        players: Vec<AccountId>,
        length: u64,
        track: Vec<Vec<bool>>,
    }

    impl Parabolica {
        #[ink(constructor)]
        pub fn new(players: u64, laps: u64) -> Self {
            let init_track: Vec<Vec<bool>> = vec![vec![false; players as usize]; laps as usize];
            Self {
                players: Vec::new(),
                length: laps,
                track: init_track,
            }
        }

        #[ink(message)]
        pub fn lap(&mut self) {
            let next_track = self.track.clone();

            // calculate the ELO of each racer for this round
            // advance accordingly
            self.track = next_track;
        }

        #[ink(message)]
        pub fn get(&self) -> Vec<Vec<bool>> {
            self.track.clone()
        }

        pub fn calculate_elo(&self) -> u64 {
            // inner function for ELO calc
            42
        }
    }
}
