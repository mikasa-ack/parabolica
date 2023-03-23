#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod parabolica {
    use ink::prelude::vec;
    use ink::prelude::vec::Vec;
    use racecar::RacecarRef;
    use traits::{Move, Racer};

    #[ink(storage)]
    pub struct Parabolica {
        racers: Vec<CarData>,
        length: u64,
        track: Vec<Vec<Move>>,
        coins: u64,
        current_lap: u64,
        manual_kill: bool,
    }

    #[derive(Debug, Clone, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    struct CarData {
        racer: RacecarRef,
        speed: u64,
        y: u64,
    }
    impl CarData {
        pub fn accelerate(&mut self) {
            self.speed += 1
        }
        pub fn advance(&mut self) {
            self.y += self.speed;
        }
    }
    impl Parabolica {
        #[ink(constructor)]
        pub fn new(racers: u64, laps: u64) -> Self {
            let init_track: Vec<Vec<Move>> =
                vec![vec![Move::Empty; racers as usize]; laps as usize];
            Self {
                racers: Vec::new(),
                length: laps,
                track: init_track,
                coins: 30000,
                current_lap: 0,
                manual_kill: false,
            }
        }
        fn fire_shell(&mut self, shooter_id: usize, shooter_y: u64) {
            let mut idx = 0;
            let mut distance = u64::MAX;
            for (id, car) in self.racers.iter().enumerate() {
                if id != shooter_id && car.y > shooter_y && car.y < distance {
                    idx = id;
                    distance = car.y;
                }
            }
            self.racers[idx].speed = 1;
        }
        fn advance_players(&mut self) {
            self.racers.iter_mut().for_each(|racer| racer.advance())
        }

        #[ink(message)]
        pub fn lap(&mut self) {
            if self.racers.len() != 3 {
                return;
            }
            for racer in self.racers.iter() {
                if racer.y >= self.length {
                    return;
                }
            }
            let curr_lap = self.current_lap + 1;
            self.current_lap = curr_lap;
            let mut next_track = self.track.clone();
            //[[Empty, Empty, Empty], [Empty, Empty, Empty]
            for racer in 0..self.track[curr_lap as usize].len() {
                let track_view = self.track.clone();
                let racer_move = self.racers[racer].racer.take_turn(track_view, racer as u64);
                next_track[curr_lap as usize][racer] = racer_move.clone();
                match racer_move {
                    Move::Accelerate => self.racers[racer].accelerate(),
                    Move::FireShell => self.fire_shell(racer, self.racers[racer].y),
                    Move::Empty => (),
                };
            }

            self.track = next_track;
            self.advance_players();
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
        pub fn register_racer(&mut self, racer_hash: Hash) {
            let n_racers = self.racers.len();
            if n_racers >= 3 {
                return;
            }

            let racer = RacecarRef::new(n_racers as u8)
                .code_hash(racer_hash)
                .endowment(0)
                .salt_bytes([0xDE, 0xAD, 0xBE, 0xEF])
                .instantiate();

            let mut new_racers = self.racers.clone();
            new_racers.push(CarData {
                racer: racer,
                speed: 0,
                y: 0,
            });
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
        #[ink(message)]
        pub fn get_positions(&self) -> Vec<u64> {
            self.racers.iter().map(|racer| racer.y).collect()
        }
    }
}
