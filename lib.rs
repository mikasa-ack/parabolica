#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod parabolica {
    use ink::prelude::vec;
    use ink::prelude::vec::Vec;
    use traits::Move;

    #[ink(storage)]
    pub struct Parabolica {
        racers: Vec<AccountId>,
        length: u64,
        track: Vec<Vec<Move>>,
        coins: u64,
        current_lap: u64,
        manual_kill: bool,
    }

    impl Parabolica {
        #[ink(constructor)]
        pub fn new(racers: u64, laps: u64) -> Self {
            let init_track: Vec<Vec<Move>> = vec![vec![Move::Empty; racers as usize]; laps as usize];
            Self {
                racers: Vec::new(),
                length: laps,
                track: init_track,
                coins: 30000,
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
            //[[Empty, Empty, Empty], [Empty, Empty, Empty]
            for row in 0..self.track.len() {
                for col in 0..self.track[0].len() {
                    //AccountId of racer
                    let track_view = self.track.clone();
                    let racer_move = self.racers[col].take_turn(track_view, col);
                    next_track[row][col] = racer_move;
                }
            }

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
        pub fn get_track(&self) -> Vec<Vec<Move>> {
            self.track.clone()
        }
    }
}
