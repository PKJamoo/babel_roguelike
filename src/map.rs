use std::convert::TryInto;
use std::cmp;
use std::collections::HashSet;

const VIEW_DIST: i32 = 10;
const VIEW_DIST_SQ: i32 = VIEW_DIST * VIEW_DIST;

pub struct Map {
    width: i32,
    height: i32,
    terrain: Vec<TileType>,
    pub visited: HashSet<Tile>
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
                 visited: HashSet::new()};
  }

  pub fn get_tiles_in_view(&mut self, x: i32, y: i32) -> Vec<Tile> {
      let mut in_view = Vec::new();
      // clamp values between [0, width/height)
      let row_start = cmp::max(y-VIEW_DIST, 0);
      let row_end = cmp::min(y+VIEW_DIST + 1, self.height);
      let col_start = cmp::max(x-VIEW_DIST, 0);
      let col_end = cmp::min(x+VIEW_DIST + 1, self.width);
      for row in row_start..row_end {
          for col in col_start..col_end {
              // Euclidean distance
              if self.get_distance(col, row, x, y) < VIEW_DIST_SQ {
                  let index: usize = (col + row * self.width).try_into().unwrap();
                  let tile = Tile{x: col, y: row, tile_type: self.terrain[index]};
                  in_view.push(tile);
                  self.visited.insert(tile);
              }
          }
      }
      return in_view;
  }

  fn get_distance(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
      return (x0 - x1).pow(2) + (y0 - y1).pow(2);
  }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tile_type: TileType,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
pub enum TileType {
    Ground,
    Water,
    Wall
}