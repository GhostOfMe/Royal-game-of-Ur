//! Gameboard view

use graphics::types::Color;
use graphics::*;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;

use crate::GameboardController;
use crate::gameboard::Player;


pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size_x: f64,
    pub size_y: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
    /// Player 1 color.
    pub player_1_color: Color,
    /// Player 2 color.
    pub player_2_color: Color,
    /// Player 1 board Color.
    pub player_1_board_color: Color,
    /// Player 2 board Color.
    pub player_2_board_color: Color,
    
}


impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0, 60.0],
            size_x: 60.0*8.0,
            size_y: 60.0*3.0,
            background_color: [0.8, 0.8, 0.8, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 2.0,
            section_edge_radius: 1.0,
            cell_edge_radius: 1.0,
            player_1_color: [0.8, 0.1, 0.1, 1.0],
            player_2_color: [0.1, 0.1, 0.8, 1.0],
            player_1_board_color: [1.0, 0.8, 0.8, 1.0],
            player_2_board_color: [0.8, 0.8, 1.0, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
    
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }
    
    /// Draw gameboard.
    pub fn draw<G: Graphics, C>(
        &self,
        controller: &GameboardController,
        glyphs: &mut C,
        c: &Context, 
        g: &mut G
    ) 
        where C: CharacterCache<Texture = G::Texture>
    {
        //use graphics::{Image, Line, Rectangle, Transformed};
        
        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size_x, settings.size_y,
        ];
        // Draw board background.
        Rectangle::new(settings.background_color)
           .draw(board_rect, &c.draw_state, c.transform, g);
        
        let (player_rect, player_color) = match controller.gameboard.active_player {
            Player::First => ([
                settings.position[0], settings.position[1],
                settings.size_x, settings.size_y * 2.0/3.0,
            ], settings.player_1_board_color),
            Player::Second => ([
                settings.position[0], settings.position[1] + 60.,
                settings.size_x, settings.size_y * 2.0/3.0,
            ], settings.player_2_board_color),
            
            };
        
        Rectangle::new(player_color)
                .draw(player_rect, &c.draw_state, c.transform, g);
        
        
        /*
        if controller.gameboard.active_player == Player::First {
            let player_rect = [
                settings.position[0], settings.position[1],
                settings.size_x, settings.size_y * 2.0/3.0,
            ];
            Rectangle::new(settings.player_1_board_color)
                .draw(player_rect, &c.draw_state, c.transform, g);
        
        }else {
            let player_rect = [
                settings.position[0], settings.position[1] + 60.,
                settings.size_x, settings.size_y * 2.0/3.0,
            ];
            Rectangle::new(settings.player_2_board_color)
                .draw(player_rect, &c.draw_state, c.transform, g);
        
        }
        */
        // Draw empty space.
        let mut black_rect = [
            settings.position[0]+settings.size_x/2.0, settings.position[1],
            settings.size_x/4.0, settings.size_y/3.0,
            ];
        
        Rectangle::new(settings.board_edge_color)
           .draw(black_rect, &c.draw_state, c.transform, g);
        
        black_rect[1] = black_rect[1]+settings.size_y*(2.0/3.0);
        
        Rectangle::new(settings.board_edge_color)
           .draw(black_rect, &c.draw_state, c.transform, g);
        
        // Draw horizontal lines.
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);
        for i in 0..3 {
            // Set up coordinates.
            let y = settings.position[1] + i as f64 / 3.0 * settings.size_y;
            
            let hline = [settings.position[0], y, settings.size_x+settings.position[0], y];
            section_edge.draw(hline, &c.draw_state, c.transform, g);
        }
        for i in 0..8 {
          // Set up coordinates.
            let x = settings.position[0] + i as f64 / 8.0 * settings.size_x;
            let y2 = settings.position[1] + settings.size_y;
        
            let vline = [x, settings.position[1], x, y2];
            section_edge.draw(vline, &c.draw_state, c.transform, g);
        }
    
        // Draw board edge.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
           .draw(board_rect, &c.draw_state, c.transform, g);
        
        // Draw first player's checkers.
        let cell_size = 60.0;
        for i in 1..15 {
            if controller.gameboard.player_1[i] !=0{
                let pos = [
                    settings.position[0] + controller.gameboard.path_to_grid_1[&(i as i8)].1 as f64 * cell_size + 12.,
                    settings.position[1] + controller.gameboard.path_to_grid_1[&(i as i8)].0 as f64 * cell_size + 100.
                ];
                if let Ok(character) = glyphs.character(90, '•') {
                    let ch_x = pos[0] as f64 + character.left();
                    let ch_y = pos[1] as f64 - character.top();
                    text::Text::new_color(settings.player_1_color, 90).draw("•",
                                                 glyphs,
                                                 &c.draw_state,
                                                 c.transform.trans(ch_x, ch_y),
                                                 g).ok();
                }
            }
        }

        // Draw unused checkers.
        for i in 0..controller.gameboard.player_1[0] as usize{
            
            let pos = [22.+ (i as f64 * 30.), 100.];
            
            if let Ok(character) = glyphs.character(90, '•') {
                let ch_x = pos[0] as f64 + character.left();
                let ch_y = pos[1] as f64 - character.top();
                text::Text::new_color(settings.player_1_color, 90).draw("•",
                                                 glyphs,
                                                 &c.draw_state,
                                                 c.transform.trans(ch_x, ch_y),
                                                 g).ok();
            }
        }
        
        for i in 0..controller.gameboard.player_1[15] as usize{
            let pos = [22. + 60. * 7.- (i as f64 * 30.), 100.];
            
            if let Ok(character) = glyphs.character(90, '•') {
                let ch_x = pos[0] as f64 + character.left();
                let ch_y = pos[1] as f64 - character.top();
                text::Text::new_color(settings.player_1_color, 90).draw("•",
                                                 glyphs,
                                                 &c.draw_state,
                                                 c.transform.trans(ch_x, ch_y),
                                                 g).ok();
            }
        }
        
        // Draw second player's checkers.
        for i in 1..15 {
            if controller.gameboard.player_2[i] == 1 {
                let pos = [
                    settings.position[0] + controller.gameboard.path_to_grid_2[&(i as i8)].1 as f64 * cell_size + 12.,
                    settings.position[1] + controller.gameboard.path_to_grid_2[&(i as i8)].0 as f64 * cell_size + 100.
                ];
                if let Ok(character) = glyphs.character(90, '•') {
                    let ch_x = pos[0] as f64 + character.left();
                    let ch_y = pos[1] as f64 - character.top();
                    text::Text::new_color(settings.player_2_color, 90).draw("•",
                                                 glyphs,
                                                 &c.draw_state,
                                                 c.transform.trans(ch_x, ch_y),
                                                 g).ok();
                }
            }
        }
        
        // Draw unused checkers.
        for i in 0..controller.gameboard.player_2[0] as usize{
            let pos = [22.+ (i as f64 * 30.), 340.];
            if let Ok(character) = glyphs.character(90, '•') {
                let ch_x = pos[0] as f64 + character.left();
                let ch_y = pos[1] as f64 - character.top();
                text::Text::new_color(settings.player_2_color, 90).draw("•",
                                                 glyphs,
                                                 &c.draw_state,
                                                 c.transform.trans(ch_x, ch_y),
                                                 g).ok();
            }
        }
        
        for i in 0..controller.gameboard.player_2[15] as usize{
            let pos = [22. + 60. * 7.- (i as f64 * 30.), 340.];
            if let Ok(character) = glyphs.character(90, '•') {
                let ch_x = pos[0] as f64 + character.left();
                let ch_y = pos[1] as f64 - character.top();
                text::Text::new_color(settings.player_2_color, 90).draw("•",
                                                 glyphs,
                                                 &c.draw_state,
                                                 c.transform.trans(ch_x, ch_y),
                                                 g).ok();
            }
        }
        
        
        // Draw Buttons.
        // "Roll" Button.
        let roll_button_rect = [
            settings.position[0], settings.position[1] +60.* 4.,
            settings.size_x/4.0, settings.size_x/4.0,
        ];
        
        let button_color;
        let text_color;
        
        if controller.gameboard.dice_roll == None {
            button_color = settings.background_color;
            text_color = settings.board_edge_color;
        }else {
            //text_color = [0.1, 0.1, 0.2, 1.0];
            text_color = settings.board_edge_color;
            button_color = [0.6, 0.6, 0.6, 1.0];
        }
        
        
        Rectangle::new(button_color)
           .draw(roll_button_rect, &c.draw_state, c.transform, g);
           
        // Draw Button's border.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
           .draw(roll_button_rect, &c.draw_state, c.transform, g);
        
        text::Text::new_color(text_color, 60).draw("Roll",
                                                     glyphs,
                                                     &c.draw_state,
                                                     c.transform.trans(16., 60.* 6.35),
                                                     g).ok();
        
        // "Pass" button.
        let pass_button_rect = [
            settings.position[0]+ 60.* 6., settings.position[1] +60.* 4.,
            settings.size_x/4.0, settings.size_x/4.0,
        ];
        
        
        Rectangle::new(settings.background_color)
           .draw(pass_button_rect, &c.draw_state, c.transform, g);
           
        // Draw Button's border.
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
           .draw(pass_button_rect, &c.draw_state, c.transform, g);
           
        text::Text::new_color(settings.board_edge_color, 60).draw("Pass",
                                                     glyphs,
                                                     &c.draw_state,
                                                     c.transform.trans(15.+ 60.* 5.9, 60.* 6.35),
                                                     g).ok();
           
        // Draw roll result.
        match controller.gameboard.dice_roll{
            Some(x) => {text::Text::new_color(settings.board_edge_color, 120)
                                                .draw(&format!("{}", 
                                                controller.gameboard.dice_roll.unwrap()),
                                             glyphs,
                                             &c.draw_state,
                                             c.transform.trans(10.+ 60.* 3.5, 60.* 6.6),
                                             g).ok();},
            None =>{},

        }
        
        // Draw Victory screen
        if controller.gameboard.is_finished() {
            
            let color = if controller.gameboard.player_1[15] == 7 {self.settings.player_1_color} 
                         else {self.settings.player_2_color};
            let text = if controller.gameboard.player_1[15] == 7 {"First player"} 
                         else {"Second player"} ;
            let trans = if controller.gameboard.player_1[15] == 7 {(20.+ 60.* 2., 60.* 4. - 45.)} 
                         else {(48.+ 60., 60.* 4. - 45.)};
            
            let rect = [
            settings.position[0]+ 60.* 1., settings.position[1] + 60.* 1.,
            settings.size_x -120., settings.size_y,
            ];
            
            Rectangle::new(color)
           .draw(rect, &c.draw_state, c.transform, g);
           
            text::Text::new_color(settings.board_edge_color, 45).draw(text,
                                             glyphs,
                                             &c.draw_state,
                                             c.transform.trans(trans.0, trans.1),
                                             g).ok();
            text::Text::new_color(settings.board_edge_color, 45).draw("wins!",
                                             glyphs,
                                             &c.draw_state,
                                             c.transform.trans(20.+ 60.* 3.,60.* 4. + 20.),
                                             g).ok();
            
        
        }
    }
}