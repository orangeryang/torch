use crate::U256 as u256;
use std::fmt;
use std::fmt::Formatter;

use super::random::random;
use super::seeds::{LAND, PEOPLE, PREFIX, SUFFIX, UNIQUE};

#[derive(Debug)]
pub struct CryptsAndCaverns {
    pub seed: u256,
    pub size: u32,
    pub name: String,
    pub affinity: String,
    pub legendary: u8,
    pub environment: u8,
    pub structure: u8,
    pub layout: Vec<Vec<u8>>,
    pub points: Vec<Vec<u8>>,
    pub doors: Vec<Vec<u8>>,
}

pub struct Settings {
    pub seed: u256,
    pub size: u32,
    pub length: u32,
    pub counter: u32,
}

pub struct Room {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
}

pub fn generate_map(seed: u256) -> CryptsAndCaverns {
    println!("We got the seed! {:#X}", seed);
    seed.build_setting().generate_cc()
}

impl Settings {
    pub fn generate_cc(mut self) -> CryptsAndCaverns {
        let (layout, points, doors, structure) = if self.random_add(0, 100) > 30 {
            self.generate_crypts()
        } else {
            self.generate_caverns()
        };

        let (name, affinity, legendary, environment) = self.generate_name();

        CryptsAndCaverns {
            seed: self.seed,
            size: self.size,
            name,
            affinity,
            legendary,
            environment,
            structure,
            layout,
            points,
            doors,
        }
    }

    fn random_shift(&mut self, min: u32, max: u32) -> u32 {
        let result = self.seed.random_shift(self.counter, min, max);
        self.counter += 1;
        result
    }

    fn random_add(&mut self, min: u32, max: u32) -> u32 {
        let result = self.seed.random_add(self.counter, min, max);
        self.counter += 1;
        result
    }

    fn generate_crypts(&mut self) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>, u8) {
        let (rooms, floor) = self.generate_rooms();
        let mut hallways = self.generate_hallways(&rooms);

        let mut layout = floor.clone();
        layout.add(&hallways);

        hallways.subtract(&floor);

        let count = hallways.count();
        let doors = if count > 0 {
            self.genera_points(&hallways, 40 / (count as f64).sqrt() as u32)
        } else {
            self.new_map()
        };
        let points = self.genera_points(&floor, 12 / ((self.size - 6) as f64).sqrt() as u32);

        (layout, points, doors, 0)
    }

    fn generate_caverns(&mut self) -> (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>, u8) {
        let cavern = self.generate_cavern();
        let mut count = cavern.count();
        if count <= 6 {
            count = 7;
        }

        let mut points = self.genera_points(&cavern, 12 / ((count - 6) as f64).sqrt() as u32);
        let doors = self.genera_points(&cavern, 40 / (count as f64).sqrt() as u32);

        points.subtract(&doors);

        (cavern, points, doors, 1)
    }

    fn generate_rooms(&mut self) -> (Vec<Room>, Vec<Vec<u8>>) {
        let size = self.size;
        let size_divided_by_three = size / 3;
        let mut num_of_rooms = self.random_add(size_divided_by_three, size);
        let mut rooms: Vec<Room> = Vec::new();
        let mut floor: Vec<Vec<u8>> = self.new_map();

        let mut safety_check: u16 = 257;
        while num_of_rooms > 0 && safety_check > 0 {
            let width = self.random_add(2, size_divided_by_three);
            let height = self.random_add(2, size_divided_by_three);
            let x = self.random_add(1, size - 1 - width);
            let y = self.random_add(1, size - 1 - height);
            let current = Room {
                x,
                y,
                width,
                height,
            };
            if current.is_valid_room(&rooms) {
                floor.mark_the_floor(&current);
                rooms.push(current);
                num_of_rooms -= 1;
            }
            safety_check -= 1;
        }

        while rooms.len() < num_of_rooms as usize {
            rooms.push(Room {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            });
        }

        (rooms, floor)
    }

    fn generate_hallways(&mut self, rooms: &Vec<Room>) -> Vec<Vec<u8>> {
        let mut hallways = self.new_map();

        if rooms.is_empty() {
            return self.new_map();
        }

        let mut pre_x = rooms[0].x + rooms[0].width / 2;
        let mut pre_y = rooms[0].y + rooms[0].height / 2;

        for i in 1..rooms.len() - 1 {
            let cur_x = rooms[i].x + rooms[i].width / 2;
            let cur_y = rooms[i].y + rooms[i].height / 2;

            if cur_x == pre_x {
                hallways.connect_vertical(cur_y, pre_y, cur_x);
            } else if cur_y == pre_y {
                hallways.connect_horizontal(cur_x, pre_x, cur_y);
            } else {
                if self.random_add(1, 2) == 1 {
                    hallways.connect_horizontal(cur_x, pre_x, cur_y);
                    hallways.connect_vertical(cur_y, pre_y, cur_x);
                } else {
                    hallways.connect_vertical(cur_y, pre_y, cur_x);
                    hallways.connect_horizontal(cur_x, pre_x, cur_y);
                }
            }

            pre_x = cur_x;
            pre_y = cur_y;
        }

        hallways
    }

    fn new_map(&self) -> Vec<Vec<u8>> {
        vec![vec![0; self.size as usize]; self.size as usize]
    }

    fn genera_points(&mut self, map: &Vec<Vec<u8>>, probability: u32) -> Vec<Vec<u8>> {
        let mut points = self.new_map();

        let mut prob = self.random_add(0, probability);
        if prob == 0 {
            prob = 1;
        }
        for i in 0..map.len() {
            for j in 0..map[i].len() {
                if map[i][j] == 1 && self.random_add(1, 100) <= prob {
                    points[i][j] = 1;
                }
            }
        }
        points
    }

    fn generate_cavern(&mut self) -> Vec<Vec<u8>> {
        let holes = self.size / 2;

        let x = self.random_add(0, self.size);
        let y = self.random_add(0, self.size);
        self.explore_in_cavern(self.new_map(), holes, 0, 0, x, y)
    }

    fn generate_name(&mut self) -> (String, String, u8, u8) {
        let unique_seed = self.seed.random_shift(15, 0, 10000);

        #[allow(unused_assignments)]
        let mut name = String::new();
        let mut affinity = String::from("none");
        let mut legendary = 0;

        if unique_seed < 17 {
            legendary = 1;
            name = UNIQUE[unique_seed as usize].to_owned();
        } else {
            let base_seed = self.seed.random_shift(16, 0, 38);

            if unique_seed <= 300 {
                // Person's Name + Base Land
                let people_seed = self.seed.random_shift(23, 0, 12);
                name = PEOPLE[people_seed as usize].to_owned() + " " + LAND[base_seed as usize];
            } else if unique_seed <= 1800 {
                // Prefix + Base Land + Suffix
                let suffix_random = self.seed.random_shift(27, 0, 59);
                affinity = String::from(SUFFIX[suffix_random as usize]);
                name = PREFIX[self.seed.random_shift(42, 0, 29) as usize].to_owned()
                    + " "
                    + LAND[base_seed as usize]
                    + " of "
                    + &affinity;
            } else if unique_seed <= 4000 {
                // Base Land + Suffix
                affinity = String::from(SUFFIX[self.seed.random_shift(51, 0, 59) as usize]);
                name = LAND[base_seed as usize].to_owned() + " of " + &affinity;
            } else if unique_seed <= 6500 {
                // Prefix + Base Land
                name = PREFIX[self.seed.random_shift(59, 0, 29) as usize].to_owned()
                    + " "
                    + LAND[base_seed as usize];
            } else {
                // Base Land
                name = LAND[base_seed as usize].to_owned();
            }
        }

        let rand = self.seed.random_shift(8, 0, 100);
        let environment = if rand >= 70 {
            0
        } else if rand >= 45 {
            1
        } else if rand >= 25 {
            2
        } else if rand >= 13 {
            3
        } else if rand >= 4 {
            4
        } else {
            5
        };

        (name, affinity, legendary, environment)
    }

    fn explore_in_cavern(
        &mut self,
        cavern: Vec<Vec<u8>>,
        holes: u32,
        mut last_direction: u32,
        #[allow(unused_assignments)] mut next_direction: u32,
        mut x: u32,
        mut y: u32,
    ) -> Vec<Vec<u8>> {
        if last_direction == 0 {
            let new_direction = self.random_shift(1, 4);
            last_direction = new_direction;
            next_direction = new_direction;
        } else {
            let direction_seed = self.random_shift(0, 100);
            if direction_seed <= 25 {
                next_direction = if last_direction == 3 {
                    0
                } else {
                    last_direction + 1
                };
            } else if direction_seed <= 50 {
                next_direction = if last_direction == 0 {
                    3
                } else {
                    last_direction - 1
                };
            } else {
                next_direction = last_direction;
            }
        }

        (x, y) = if next_direction == 0 {
            (if x > 0 { x - 1 } else { x }, y)
        } else if next_direction == 1 {
            (x, y + 1)
        } else if next_direction == 2 {
            (x + 1, y)
        } else {
            // next_direction == 3
            (x, if y > 0 { y - 1 } else { y })
        };

        if x > 0 && x < self.size && y > 0 && y < self.size {
            self.explore_in_cavern(cavern, holes, last_direction, next_direction, x, y)
        } else if holes > 1 {
            x = self.random_add(0, self.size);
            y = self.random_add(0, self.size);
            self.explore_in_cavern(cavern, holes - 1, last_direction, next_direction, x, y)
        } else {
            cavern
        }
    }
}

impl u256 {
    fn random_shift(self, shift: u32, min: u32, max: u32) -> u32 {
        random(self << shift, min, max)
    }

    fn random_add(self, add: u32, min: u32, max: u32) -> u32 {
        random(self + add, min, max)
    }

    pub fn build_setting(self) -> Settings {
        let size = self.random_shift(4, 8, 25);
        let length = size ^ 2 / 256 + 1;

        Settings {
            seed: self,
            size,
            length,
            counter: 0,
        }
    }
}

trait Map {
    fn connect_horizontal(&mut self, current_x: u32, previous_x: u32, y: u32);
    fn connect_vertical(&mut self, current_y: u32, previous_y: u32, x: u32);
    fn mark_the_floor(&mut self, current: &Room);
    fn subtract(&mut self, other: &Vec<Vec<u8>>);
    fn add(&mut self, other: &Vec<Vec<u8>>);
    fn count(&self) -> u32;
}

impl Map for Vec<Vec<u8>> {
    fn connect_horizontal(&mut self, current_x: u32, previous_x: u32, y: u32) {
        let (min, max) = if current_x < previous_x {
            (current_x, previous_x)
        } else {
            (previous_x, current_x)
        };
        for x in min..max {
            self[x as usize][y as usize] = 1;
        }
    }

    fn connect_vertical(&mut self, current_y: u32, previous_y: u32, x: u32) {
        let (min, max) = if current_y < previous_y {
            (current_y, previous_y)
        } else {
            (previous_y, current_y)
        };
        for y in min..max {
            self[x as usize][y as usize] = 1;
        }
    }

    fn mark_the_floor(&mut self, current: &Room) {
        for x in current.x..current.x + current.width {
            for y in current.y..current.y + current.height {
                self[x as usize][y as usize] = 1;
            }
        }
    }

    fn subtract(&mut self, other: &Vec<Vec<u8>>) {
        for i in 0..self.len() {
            for j in 0..self[i].len() {
                self[i][j] &= !other[i][j];
            }
        }
    }

    fn add(&mut self, other: &Vec<Vec<u8>>) {
        for i in 0..self.len() {
            for j in 0..self[i].len() {
                self[i][j] |= other[i][j];
            }
        }
    }

    fn count(&self) -> u32 {
        let mut count = 0;
        for i in 0..self.len() {
            for j in 0..self[i].len() {
                if self[i][j] == 1 {
                    count += 1;
                }
            }
        }
        count
    }
}

impl Room {
    fn is_valid_room(&self, rooms: &Vec<Room>) -> bool {
        for room in rooms {
            if room.x - 1 < self.x + self.width
                && room.x + room.width + 1 > self.x
                // bad code here
                && room.y - 1 < self.x + self.height
                && room.y + room.height > self.y
            {
                return false;
            }
        }
        true
    }
}

// impl fmt::Debug for CryptsAndCaverns{
//     fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
//         for inner_vec in &cc.layout {
//             let inner_vec_str: Vec<String> = inner_vec.iter().map(|&x| x.to_string()).collect();
//             let joined_str = inner_vec_str.join(" ");
//             println!("{}", joined_str);
//         }
//     }
// }
