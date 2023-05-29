mod utils;

use wasm_bindgen::prelude::*;
use rand::{Rng};
use std::fmt;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;



#[derive(Copy, Clone, Debug, PartialEq)]
enum BaseTile{
    Bomb,
    NearbyBombs(u8)
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum UserTile{
    Flag,
    Cleared,
    NotCleared
}


#[wasm_bindgen]
#[derive(Debug)]
pub struct Board{
    width: usize,
    height: usize,
    mines: u32,
    flags: u32,
    bomb_info: Vec<Vec<BaseTile>>,
    user_input: Vec<Vec<UserTile>>
}

#[wasm_bindgen]
impl Board{
    pub fn new(w: usize, h: usize) -> Board{
        Board { 
            width: h, 
            height: w,
            mines: 0,
            flags: 0, 
            bomb_info: vec![vec![BaseTile::NearbyBombs(0); w]; h], 
            user_input: vec![vec![UserTile::NotCleared; w]; h] 
        }
    }

    pub fn init(&mut self, num_mines: u32, first_point_x: i32, first_point_y: i32) -> (){
        let mut rng = rand::thread_rng();
        self.mines = num_mines;
        self.flags = 0;
        
        for _i in 0..num_mines {
            let mut unique_bomb: bool = false;
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            while !unique_bomb{
                x = rng.gen_range(0..(self.width as i32));
                y = rng.gen_range(0..(self.height as i32));
                if (x == first_point_x && y == first_point_y)|| x+1 == first_point_x || x-1 == first_point_x
                || y+1 == first_point_y || y-1 == first_point_y 
                || self.bomb_info[x as usize][y as usize] == BaseTile::Bomb{
                    unique_bomb = false;
                }
                else{
                    unique_bomb = true;
                }
            }
            self.bomb_info[x as usize][y as usize] = BaseTile::Bomb;

        }
        self.calculate_nearby_bombs();
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get_flags(&self) -> u32 {
        self.flags
    }

    fn calculate_nearby_bombs(&mut self) -> (){
        for x in 0..(self.width as i32) {
            for y in 0..(self.height as i32) {
                if let BaseTile::NearbyBombs(mut val) = self.bomb_info[x as usize][y as usize]{
                    if x == 0 && y == 0{
                        if self.bomb_info[x as usize + 1][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if x == (self.width as i32 - 1) && y == (self.height as i32 - 1){
                        if self.bomb_info[x as usize - 1][y as usize - 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize - 1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if x == 0 && y == (self.height as i32 - 1){
                        if self.bomb_info[x as usize + 1][y as usize - 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize - 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if y == 0 && x == (self.width as i32 - 1){
                        if self.bomb_info[x as usize - 1][y as usize + 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize + 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if x == 0{
                        if self.bomb_info[x as usize +1][y as usize + 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if x == (self.width as i32 - 1){
                        if self.bomb_info[x as usize -1][y as usize + 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if y == 0{
                        if self.bomb_info[x as usize +1][y as usize + 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize + 1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else if y == (self.height as i32 - 1){
                        if self.bomb_info[x as usize - 1][y as usize - 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize - 1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize - 1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    else{
                        if self.bomb_info[x as usize + 1][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize +1][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize +1] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize] == BaseTile::Bomb{
                            val+=1;
                        }
                        if self.bomb_info[x as usize -1][y as usize -1] == BaseTile::Bomb{
                            val+=1;
                        }
                    }
                    self.bomb_info[x as usize][y as usize] = BaseTile::NearbyBombs(val);
                }

            }
        }

    }

    pub fn flag(&mut self, x: i32, y: i32) -> (){
        if self.user_input[x as usize][y as usize] == UserTile::Cleared{
            return
        }
        else if self.user_input[x as usize][y as usize] == UserTile::Flag{
            self.user_input[x as usize][y as usize] = UserTile::NotCleared;
            self.flags -= 1;
        }
        else{
            self.user_input[x as usize][y as usize] = UserTile::Flag;
            self.flags += 1;
        }
        
    }

    pub fn clear(&mut self, x: i32, y: i32) -> bool{
        if x < 0 || y < 0 || x >= self.width as i32 || y >= self.height as i32{
            return true;
        }
        if self.user_input[x as usize][y as usize] == UserTile::Cleared
        || self.user_input[x as usize][y as usize] == UserTile::Flag{
            return true;
        }
        if self.bomb_info[x as usize][y as usize] == BaseTile::Bomb{
            self.user_input = vec![vec![UserTile::Cleared; self.height]; self.width];
            return false;
        }
        else{
            self.user_input[x as usize][y as usize] = UserTile::Cleared;
            if self.bomb_info[x as usize][y as usize] == BaseTile::NearbyBombs(0){
                self.clear(x+1, y+1);
                self.clear(x+1, y);
                self.clear(x+1, y-1);
                self.clear(x, y+1 );
                self.clear(x, y-1 );
                self.clear(x-1, y+1);
                self.clear(x-1, y);
                self.clear(x-1, y-1);
            }
            return true;
        }

    }

    pub fn render(&self) -> String {
        self.to_string()
    }


}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in 0..(self.width){
            for y in 0..(self.height){
                let user_tile = self.user_input[x][y];
                let base_tile = self.bomb_info[x][y];
                let symbol;
                match (base_tile, user_tile) {
                    (_, UserTile::NotCleared) => symbol = 'n',
                    (_, UserTile::Flag) => symbol = 'f',
                    (BaseTile::NearbyBombs(val), UserTile::Cleared) =>{
                        match val {
                            1 => symbol = '1',
                            2 => symbol = '2',
                            3 => symbol = '3',
                            4 => symbol = '4',
                            5 => symbol = '5',
                            6 => symbol = '6',
                            7 => symbol = '7',
                            8 => symbol = '8',
                            _ => symbol = '0'
                        }
                    },
                    (BaseTile::Bomb, UserTile::Cleared) => symbol = 'b'
                }
                write!(f, "{}", symbol)?;
            }
        }

        Ok(())
    }
}


