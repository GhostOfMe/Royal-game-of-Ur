use std::collections::HashMap;

use rand::{self, Rng};

/// Stores board information
pub struct Gameboard {
    pub cells: [[u8; 8]; 3],
    pub player_1: [i8; 16],
    pub player_2: [i8; 16],
    pub active_player: i8,
    pub dice_roll: i8,
    pub grid_to_path_1: HashMap<(i8, i8), i8>,
    pub grid_to_path_2: HashMap<(i8, i8), i8>,
    pub path_to_grid_1: HashMap<i8, (i8, i8)>,
    pub path_to_grid_2: HashMap<i8, (i8, i8)>,
    }
    



impl Gameboard {
    pub fn new() -> Gameboard {
        let mut g = Gameboard {
        cells: [[0; 8];3],
        player_1: [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        player_2: [7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        active_player: 0,
        dice_roll: -1,
        grid_to_path_1: HashMap::new(),
        grid_to_path_2: HashMap::new(),
        path_to_grid_2: HashMap::new(),
        path_to_grid_1: HashMap::new(),
        };
        g.populate();
        return g;
        
    }
    
    pub fn is_player_1(&self, ind: usize) -> bool {
        return self.player_1[ind] == 1;
    }
    
    pub fn is_player_2(&self, ind: usize) -> bool {
        return self.player_2[ind] == 1;
    }
    
    pub fn roll(&mut self){
        //roll d2 x 4
        let mut rng = rand::thread_rng();
        let mut result: i8 = 0;
        
        for i in 0..4{
            let num: i8 = rng.gen_range(0, 2);
            println!("{} die: {}", i, num);
            result +=  num;
            }
        
        self.dice_roll = result;
        //Pass turn if roll is 0.
        if self.dice_roll == 0{
            self.pass_turn();
        }
        println!("Total: {}", self.dice_roll);
    }
    
    pub fn _move(&mut self, _i: usize) {
        if self.active_player == 0 
         // Check array boundary.
         && (_i + self.dice_roll as usize) <16
         // Check destination cell is free.
         && (self.player_1[_i+self.dice_roll as usize] != 1 
            ||_i + self.dice_roll as usize == 15)
            
         // Check target cell has checker.
         && self.player_1[_i] > 0 {
            self.player_1[_i] -= 1;
            self.player_1[_i+self.dice_roll as usize] += 1;
            
            // knock out check
            if self.player_2[_i+self.dice_roll as usize] == 1 
                            && (_i+self.dice_roll as usize) > 4
                            && (_i+self.dice_roll as usize) < 13 {
                self.player_2[_i+self.dice_roll as usize] = 0;
                self.player_2[0] +=1;
            }
            self.pass_turn();
        
        }else if self.active_player == 1  
              && (_i + self.dice_roll as usize) < 16
         // Check destination cell is free.
              && (self.player_2[_i+self.dice_roll as usize] != 1 
                ||_i + self.dice_roll as usize == 15)
              && self.player_2[_i] > 0 {
            self.player_2[_i] -= 1;
            self.player_2[_i+self.dice_roll as usize] += 1;
            
            if self.player_1[_i+self.dice_roll as usize] == 1 
                             && (_i+self.dice_roll as usize) > 4
                             && (_i+self.dice_roll as usize) < 13 {
                self.player_1[_i+self.dice_roll as usize] = 0;
                self.player_1[0] +=1;
            }
            self.pass_turn();
        }
    }
    
    pub fn pass_turn(&mut self) {
        self.dice_roll = -1;
        println!("{}", self.dice_roll);
        if self.active_player == 0 {
            self.active_player = 1;
            println!{"Second player's turn."};
        } else {
            self.active_player = 0;
            println!{"First player's turn."};
        }
    }
    
    pub fn is_finished(&self) -> bool {
        return (self.player_1[15] == 7) || (self.player_2[15] == 7)
    }
    
    
    pub fn get_active_cell(&self, x: i8, y: i8) -> usize{
        if self.active_player == 0 {
            return self.grid_to_path_1[&(x, y)] as usize;
        }else {
            return self.grid_to_path_2[&(x, y)] as usize;
        }
    }
    
    
    pub fn populate(&mut self) {
        // Populate hash map for the first player
        self.grid_to_path_1.insert((0, 0), 4);
        self.grid_to_path_1.insert((0, 1), 3);
        self.grid_to_path_1.insert((0, 2), 2);
        self.grid_to_path_1.insert((0, 3), 1);
        self.grid_to_path_1.insert((0, 4), 0);
        self.grid_to_path_1.insert((0, 5), 15);
        self.grid_to_path_1.insert((0, 6), 14);
        self.grid_to_path_1.insert((0, 7), 13);
        self.grid_to_path_1.insert((1, 0), 5);
        self.grid_to_path_1.insert((1, 1), 6);
        self.grid_to_path_1.insert((1, 2), 7);
        self.grid_to_path_1.insert((1, 3), 8);
        self.grid_to_path_1.insert((1, 4), 9);
        self.grid_to_path_1.insert((1, 5), 10);
        self.grid_to_path_1.insert((1, 6), 11);
        self.grid_to_path_1.insert((1, 7), 12);
        // Populate hash map for the second player
        self.grid_to_path_2.insert((2, 0), 4);
        self.grid_to_path_2.insert((2, 1), 3);
        self.grid_to_path_2.insert((2, 2), 2);
        self.grid_to_path_2.insert((2, 3), 1);
        self.grid_to_path_2.insert((2, 4), 0);
        self.grid_to_path_2.insert((2, 5), 15);
        self.grid_to_path_2.insert((2, 6), 14);
        self.grid_to_path_2.insert((2, 7), 13);
        self.grid_to_path_2.insert((1, 0), 5);
        self.grid_to_path_2.insert((1, 1), 6);
        self.grid_to_path_2.insert((1, 2), 7);
        self.grid_to_path_2.insert((1, 3), 8);
        self.grid_to_path_2.insert((1, 4), 9);
        self.grid_to_path_2.insert((1, 5), 10);
        self.grid_to_path_2.insert((1, 6), 11);
        self.grid_to_path_2.insert((1, 7), 12);
        
        for (k, v) in self.grid_to_path_1.iter() {
            self.path_to_grid_1.insert(*v, *k);
        }
        
        for (k, v) in self.grid_to_path_2.iter() {
            self.path_to_grid_2.insert(*v, *k);
        }
    }
}