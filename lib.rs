#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod parabolica {
    use ink::prelude::vec;
    use ink::prelude::vec::Vec;

    #[ink(storage)]
    pub struct Parabolica {
        racers: Vec<AccountId>,
        length: u64,
        track: Vec<Vec<bool>>,
        coins: u64,
        current_lap: u64,
        manual_kill: bool,
    }

    impl Parabolica {
        #[ink(constructor)]
        pub fn new(racers: u64, laps: u64) -> Self {
            let init_track: Vec<Vec<bool>> = vec![vec![false; racers as usize]; laps as usize];
            Self {
                racers: Vec::new(),
                length: laps,
                track: init_track,
                coins: 10000,
                current_lap: 0,
                manual_kill: false,
            }
        }

        #[ink(message)]
        pub fn lap(&mut self) {
            self.current_lap = self.current_lap + 1;
            if self.racers.len() != 3 {
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
            // if self.current_lap > self.length {
            //     return false;
            // }
            !self.manual_kill
        }

        #[ink(message)]
        pub fn should_kill(&self) -> bool {
            self.manual_kill
        }

        #[ink(message)]
        pub fn register_racer(&mut self, racer: AccountId) {
            let mut new_racers = self.racers.clone();
            new_racers.push(racer);
            self.racers = new_racers;
        }

        #[ink(message)]
        pub fn set_manual_kill(&mut self) {
            self.manual_kill = true;
        }

        #[ink(message)]
        pub fn get_num_racers(&self) -> u64 {
            self.racers.len() as u64
        }

        #[ink(message)]
        pub fn get_curr_lap(&self) -> u64 {
            self.current_lap
        }

        #[ink(message)]
        pub fn get_track(&self) -> Vec<Vec<bool>> {
            self.track.clone()
        }

        pub fn calculate_elo(&self) -> u64 {
            // inner function for ELO calc
            42
        }
    }
}
