//! Gameboard controller.

use piston::input::GenericEvent;


use crate::Gameboard;

pub struct GameboardController {
    pub gameboard:Gameboard,
    pub cursor_pos: [f64; 2],
}

impl GameboardController {
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController{
            gameboard: gameboard,
            cursor_pos: [0.0; 2],
        }
    }
    
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: [f64; 2],  e:&E){
        use piston::input::{Button, MouseButton};
        
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args(){
            // Find coordinates relative to upper left corner.
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            
            // Check that coordinates are inside board boundaries.
            if !self.gameboard.is_finished()
             && x >= 0.0 && x <= size[0] 
             && y >= 0.0 && y <= size[1] 
             && self.gameboard.dice_roll != -1{
                let cell_x = (x / 60.) as usize;
                let cell_y = (y / 60.) as usize;
                println!("{}, {}", cell_x, cell_y);
                if (self.gameboard.active_player == 0 
                 && self.gameboard.grid_to_path_1.contains_key(&(cell_y as i8, cell_x as i8)))
                || (self.gameboard.active_player == 1 
                 && self.gameboard.grid_to_path_2.contains_key(&(cell_y as i8, cell_x as i8))){
                    let cell = self.gameboard.get_active_cell(cell_y as i8, cell_x as i8);
                    self.gameboard._move(cell);
                }
            // Check "Roll" button clicked.
            }else if !self.gameboard.is_finished()
             && x >= 0. && x <= 60. * 2. 
             && y <= (60.* 6.) && y >= (60. *4. ) 
             && self.gameboard.dice_roll == -1{
                    self.gameboard.roll();
            }
                // Check "Pass" button clicked.
             else if !self.gameboard.is_finished()
             && x >= 360. && x <= 480. 
             && y <= (60.* 6.) && y >= (60. *4. ){
                self.gameboard.pass_turn();
            } else {println!("{}, {}", x, y)}
        }
    }
}