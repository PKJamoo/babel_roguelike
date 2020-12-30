use std::convert::TryInto;
use std::cmp;

const VIEW_DIST: i32 = 10;
const VIEW_DIST_SQ: i32 = VIEW_DIST * VIEW_DIST;

pub struct Map {
    width: i32,
    height: i32,
    terrain: Vec<TileType>,
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
                 terrain: terrain};
  }

  pub fn get_tiles_in_view(&self, x: i32, y: i32) -> Vec<Tile> {
      let mut inView = Vec::new();
      for j in y-VIEW_DIST..y+VIEW_DIST+1 {
          for i in x-VIEW_DIST..x+VIEW_DIST+1 {
              // clamp values between [0, width/height)
              let col = cmp::min(cmp::max(i, 0), self.width-1);
              let row = cmp::min(cmp::max(j, 0), self.height-1);
              // Euclidean distance
              if (col - x).pow(2) + (row - y).pow(2) < VIEW_DIST_SQ {
                  let index: usize = (col + row * self.width).try_into().unwrap();
                  inView.push(Tile{x: col, y: row, tileType: self.terrain[index]});
              }
          }
      }
      return inView;
  }
}

pub struct Tile {
    pub x: i32,
    pub y: i32,
    pub tileType: TileType,
}

#[derive(Copy, Clone)]
pub enum TileType {
    Ground,
    Water,
    Wall
}