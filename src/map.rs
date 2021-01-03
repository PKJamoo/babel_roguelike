use std::convert::TryInto;
use std::cmp;
use std::collections::HashSet;

const VIEW_DIST: i32 = 9;
const VIEW_DIST_SQ: i32 = VIEW_DIST * VIEW_DIST;

pub struct Map {
    width: i32,
    height: i32,
    terrain: Vec<TileType>,
    pub visited: HashSet<Tile>,
    pub visible: HashSet<Tile>
}

// TODO: handle maps that are bigger than the screen width (i.e. handle wrapping)
impl Map {
  pub fn new(width: i32, height: i32) -> Map {
      let mut terrain = Vec::with_capacity((width * height).try_into().unwrap());
      for j in 0..height {
          for i in 0..width {
              let tile: TileType;
              if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
                  tile = TileType::Wall;
              } else if j == 18 && (i == 38 || i == 39 || i == 40 || i == 41 || i == 42) {
                  tile = TileType::Wall;
              } else if j == 32 && (i == 38 || i == 39 || i == 41 || i == 42) {
                  tile = TileType::Wall;
              } else if (i - 20).pow(2) + (j - 20).pow(2) < 100 {
                  tile = TileType::Water;
              } else {
                  tile = TileType::Ground;
              }
              terrain.push(tile);
          }
      }
      return Map{width: width,
                 height: height,
                 terrain: terrain,
                 visited: HashSet::new(),
                 visible: HashSet::new()};
  }

  pub fn can_move_to(&self, x: i32, y: i32) -> bool {
      return self.terrain[self.get_index(x, y)] != TileType::Wall;
  }

  pub fn tile_in_view(&self, x: i32, y: i32) -> bool{
    return self.visible.contains(&Tile{x: x, y: y, tile_type: TileType::Ground})
  }

  pub fn get_tiles_in_view(&mut self, x: i32, y: i32) -> HashSet<Tile> {
      let mut in_view = HashSet::new();
      // clamp values between [0, width/height)
      let row_start = cmp::max(y-VIEW_DIST, 0);
      let row_end = cmp::min(y+VIEW_DIST, self.height - 1);
      let col_start = cmp::max(x-VIEW_DIST, 0);
      let col_end = cmp::min(x+VIEW_DIST, self.width - 1);

      for row in row_start..row_end+1 {
          self.find_visible_tiles_between(x, y, col_start, row, VIEW_DIST_SQ, &mut in_view);
          self.find_visible_tiles_between(x, y, col_end, row, VIEW_DIST_SQ, &mut in_view);
      }

      for col in col_start..col_end+1 {
          self.find_visible_tiles_between(x, y, col, row_start, VIEW_DIST_SQ, &mut in_view);
          self.find_visible_tiles_between(x, y, col, row_end, VIEW_DIST_SQ, &mut in_view);
      }

      return in_view;
  }

  fn find_visible_tiles_between(&self, start_x: i32, start_y: i32, x1: i32, y1: i32, length_sq: i32, visible: &mut HashSet<Tile>) {
    // Bresenham line algorithm (http://members.chello.at/~easyfilter/bresenham.html)
    let mut x0 = start_x;
    let mut y0 = start_y;
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sign_x = if x0 < x1 { 1 } else { -1 };
    let sign_y = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut e2;

    while self.get_distance_sq(start_x, start_y, x0, y0) < length_sq {
        let tile_type = self.terrain[self.get_index(x0, y0)];
        visible.insert(Tile{x: x0, y: y0, tile_type: tile_type});
        if self.tile_blocks_vision(tile_type) || (x0 == x1 && y0 == y1) {
            break;
        }
        e2 = 2*err;
        if e2 >= dy {
            err += dy;
            x0 += sign_x;
        }
        if e2 <= dx {
            err += dx;
            y0 += sign_y;
        }
    }
  }

  fn get_distance_sq(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
      // Euclidean distance
      return (x0 - x1).pow(2) + (y0 - y1).pow(2);
  }

  fn get_index(&self, x0: i32, y0: i32) -> usize {
      return (x0 + y0 * self.width).try_into().unwrap();
  }

  fn tile_blocks_vision(&self, tile_type: TileType) -> bool {
      return tile_type == TileType::Wall;
  }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}


#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub enum TileType {
    Ground,
    Water,
    Wall
}