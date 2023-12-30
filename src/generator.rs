use crate::U256 as u256;

use super::random::random;

pub struct CryptsAndCaverns {
    pub seed: u256,
    pub map: Vec<Vec<u8>>,
    pub name: String,
    pub enviroment: u8,
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

impl CryptsAndCaverns {
    pub fn new() -> CryptsAndCaverns {
        CryptsAndCaverns {
            seed: u256::zero(),
            map: Vec::new(),
            name: String::new(),
            enviroment: 0,
        }
    }
}

pub fn generate_map(seed: u256) -> CryptsAndCaverns {
    println!("We got the seed! {:#X}", seed);

    let mut settings: Settings = seed.build_setting();


    CryptsAndCaverns::new()
}

impl u256 {
    pub fn random_shift(self, shift: u32, min: u32, max: u32) -> u32 {
        random(self << shift, min, max)
    }

    pub fn random_add(self, add: u32, min: u32, max: u32) -> u32 {
        random(self + add, min, max)
    }

    pub fn build_setting(self) -> Settings {
        let size = self.random_shift(4, 8, 25) as u32;
        let length = size ^ 2 / 256 + 1;

        Settings {
            seed: self,
            size,
            length,
            counter: 0,
        }
    }
}

impl Settings {
    pub fn random_shift(&mut self, min: u32, max: u32) -> u32 {
        let result = self.seed.random_shift(self.counter, min, max);
        self.counter += 1;
        result
    }

    pub fn random_add(&mut self, min: u32, max: u32) -> u32 {
        let result = self.seed.random_add(self.counter, min, max);
        self.counter += 1;
        result
    }

    pub fn generate_cc(mut self) -> CryptsAndCaverns {
        if self.random_add(0, 100) > 30 {
            let (rooms, floor) = self.generate_rooms();
        }


        CryptsAndCaverns::new()
    }

    pub fn generate_rooms(&mut self) -> (Vec<Room>, Vec<Vec<u8>>) {
        let min_rooms = self.size / 3;
        let max_rooms = self.size;
        let min_room_size = 2_u32;
        let max_room_size = self.size / 3;

        let mut num_of_rooms = self.random_add(min_rooms, max_rooms);
        let mut rooms: Vec<Room> = Vec::new();
        let mut floor: Vec<Vec<u8>> = vec![vec![0; self.size as usize]; self.size as usize];

        let mut safety_check: u16 = 256;
        while num_of_rooms > 0 && safety_check > 0 {
            let width = self.random_add(min_room_size, max_room_size);
            let height = self.random_add(min_room_size, max_room_size);
            let x = self.random_add(1, self.size - 1 - width);
            let y = self.random_add(1, self.size - 1 - height);
            let current = Room { x, y, width, height };
            if is_valid_room(&current, &rooms) {
                mark_the_floor(&mut floor, &current);
                rooms.push(current);
                num_of_rooms -= 1;
            }
            safety_check -= 1;
        }

        (rooms, floor)
    }

    pub fn generate_hallways(&mut self, rooms: &Vec<Room>) -> Vec<Vec<u8>> {
        if rooms.is_empty() {
            return vec![vec![0; self.size as usize]; self.size as usize];
        }

        let pre_x = rooms[0].x + rooms[0].width / 2;
        let pre_y = rooms[0].y + rooms[0].height / 2;

        for i in 1..rooms.len() - 1 {
            let cur_x = rooms[i].x + rooms[i].width / 2;
            let cur_y = rooms[i].y + rooms[i].height / 2;

            if cur_x == pre_x {
                todo!()
            }
        }

        vec![vec![0; self.size as usize]; self.size as usize]
    }
}

fn mark_the_floor(layout: &mut Vec<Vec<u8>>, current: &Room) {
    for x in current.x..current.x + current.width {
        for y in current.y..current.y + current.height {
            layout[x as usize][y as usize] = 1;
        }
    }
}

fn is_valid_room(current: &Room, rooms: &Vec<Room>) -> bool {
    for room in rooms {
        if room.x - 1 < current.x + current.width
            && room.x + room.width + 1 > current.x
            && room.y - 1 < current.y + current.height
            && room.y + room.height > current.y {
            return false;
        }
    }
    true
}
