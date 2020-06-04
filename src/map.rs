use rltk::{RGB, Rltk, RandomNumberGenerator, Point, Algorithm2D };
use std::vec::Vec;
use super::{Rect};
use std::cmp::{min, max};


#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}


pub struct Map {
    pub tiles : Vec<TileType>,
    pub rooms : Vec<Rect>,
    pub width : i32,
    pub height: i32
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map{
    fn is_opaque(&self, idx:usize) -> bool {
        self.tiles[idx as usize] == TileType::Wall
    }
}

impl Map {
        
    fn apply_room_to_map(&mut self, room: &Rect){
        for y in room.y1 + 1 ..=room.y2 {
            for x in room.x1 + 1 ..= room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1:i32, x2: i32, y: i32){

        for x in min(x1, x2) ..= max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_vertical_tunnel(&mut self, y1:i32, y2: i32, x: i32) {
        for y in min(y1, y2) ..= max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < 80*50 {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    pub fn xy_idx(&mut self, x: i32, y: i32) -> usize {
        (y as usize * self.width as usize) + x as usize
    }
}

pub fn new_map_rooms_and_corridors() -> Map {
    let mut map = Map {
        tiles: vec![TileType::Wall; 80*50],
        rooms: Vec::new(),
        width: 80,
        height: 50
    };

    const MAX_ROOMS : i32 = 30;
    const MIN_SIZE : i32 = 6;
    const MAX_SIZE : i32 = 10;

    let mut rng = RandomNumberGenerator::new();

    for _ in 0..MAX_ROOMS {
        let w: i32 = rng.range(MIN_SIZE, MAX_SIZE);
        let h: i32 = rng.range(MIN_SIZE, MAX_SIZE);
        let x: i32 = rng.roll_dice(1, 80 - w - 1) - 1;
        let y: i32 = rng.roll_dice(1, 50 - h - 1) - 1;

        let new_room = Rect::new(x, y, w, h);
        let mut ok = true;

        for other_room in map.rooms.iter() {
            if new_room.interset(other_room){
                ok = false
            }
        }

        if ok {
            map.apply_room_to_map(&new_room);

            if !map.rooms.is_empty() {
                let (new_x, new_y) = new_room.center();
                let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();

                if rng.range(0, 1) == 1 {
                    map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                    map.apply_vertical_tunnel(prev_y, new_y, new_x);
                } else {
                    map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                    map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                }
            }

            map.rooms.push(new_room);
        }

    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut Rltk){
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0.,0.,0.), rltk::to_cp437('.'));
            },
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0.0, 1.0, 0.0), RGB::from_f32(0.,0.,0.), rltk::to_cp437('#'))
            }
        }

        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }   
}