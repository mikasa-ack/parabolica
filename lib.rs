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
        current_lap: u64,
        manual_kill: bool,
    }

    impl Parabolica {
        #[ink(constructor)]
        pub fn new(players: u64, laps: u64) -> Self {
            let init_track: Vec<Vec<bool>> = vec![vec![false; players as usize]; laps as usize];
            Self {
                players: Vec::new(),
                length: laps,
                track: init_track,
                current_lap: 0,
                manual_kill: false,
            }
        }

        #[ink(message)]
        pub fn lap(&mut self) {
            if self.players.len() != 3 {
                return;
            }

            let next_track = self.track.clone();

            // calculate the ELO of each racer for this round
            // advance accordingly
            self.track = next_track;
        }

        /// Returns `true` if the autonomous call should be executed.
        #[ink(message)]
        pub fn should_run(&self) -> bool {
            if self.current_lap > self.length {
                return false;
            }
            !self.manual_kill
        }

        #[ink(message)]
        pub fn should_kill(&self) -> bool {
            self.manual_kill
        }

        #[ink(message)]
        pub fn register_racer(&self, racer: AccountId) {
            let players = self.players;
            players.append(racer);
            self.players = players;
        }

        #[ink(message)]
        pub fn set_manual_kill(&mut self) {
            self.manual_kill = true;
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
