use tcod::colors::*;
use tcod::console::*;
use super::{SCREEN_HEIGHT, SCREEN_WIDTH, GameState};
use crate::map::TileType;

pub struct Tcod {
    pub root: Root,
    pub con: Offscreen,
}

// struct for all menu interfaces, i.e. lists of different options to choose from.
#[derive(Clone)]
pub struct Menu {
    pub cursor_index: i16,
    pub menu_size: i16,
    pub options: Vec<String>
}

impl Tcod {

    pub fn render_main_menu(&mut self, menu: Menu) {
        self.con.set_default_background(BLACK);
        self.con.set_default_foreground(WHITE);
        self.con.set_alignment(TextAlignment::Center);
        self.con.clear();
        self.con.print(SCREEN_WIDTH/2, SCREEN_HEIGHT/2, "BABEL");
        for (pos, option) in menu.options.iter().enumerate() {
            self.con.print(SCREEN_WIDTH/2, SCREEN_HEIGHT/2 + 1 + pos as i32, option);
        }
        self.con.put_char(SCREEN_WIDTH/2 - 5, SCREEN_HEIGHT/2 + 1 + (menu.cursor_index as i32), '>', BackgroundFlag::None);

        blit( &self.con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT),
        &mut self.root, (0, 0), 1.0, 1.0,);
        self.root.flush();
    }

    pub fn render_game(&mut self, game: &mut GameState) {

        // for map -> render all tiles in view
        // for entities -> render all objects in view
        self.con.set_default_foreground(WHITE);
        self.con.clear();

        // Render map
        for tile in game.current_level.visited.iter() {
          let tileChar: char;
          let tileColor: Color;
          match tile.tileType {
              TileType::Ground => {tileChar = '.'; tileColor = LIGHT_GREY},
              TileType::Water => {tileChar = '*'; tileColor = BLUE},
              TileType::Wall => {tileChar = 'X'; tileColor = LIGHT_GREY},
          }
          self.con.put_char_ex(tile.x, tile.y, tileChar, tileColor, BLACK);
        }
        for tile in game.current_level.get_tiles_in_view(game.player_x, game.player_y).iter() {
          let tileChar: char;
          let tileColor: Color;
          match tile.tileType {
              TileType::Ground => {tileChar = '.'; tileColor = LIGHTEST_GREY},
              TileType::Water => {tileChar = '*'; tileColor = LIGHT_BLUE},
              TileType::Wall => {tileChar = 'X'; tileColor = WHITE},
          }
          self.con.put_char_ex(tile.x, tile.y, tileChar, tileColor, BLACK);
        }

        // Render player
        self.con.put_char(game.player_x, game.player_y, '@', BackgroundFlag::None);


        blit( &self.con, (0, 0), (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut self.root, (0, 0), 1.0, 1.0,
        );

        self.root.flush();
    }
}