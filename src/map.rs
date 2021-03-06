use std::cmp;
use std::collections::HashSet;

const VIEW_DIST: i32 = 9;
const VIEW_DIST_SQ: i32 = VIEW_DIST * VIEW_DIST;

pub struct Map {
    pub width: i32,
    pub height: i32,
    terrain: Vec<TileType>,
    pub visited: HashSet<Tile>,
    pub visible: HashSet<Tile>,
    pub blocked: Vec<bool>
}

// TODO: handle maps that are bigger than the screen width (i.e. handle wrapping)
impl Map {
  pub fn new(width: i32, height: i32) -> Map {
      let mut terrain = vec![TileType::Ground; (width * height) as usize];
      for j in 0..height {
          for i in 0..width {
              if i == 0 || j == 0 || i == width - 1 || j == height - 1 {
                terrain[(i + j*width) as usize] = TileType::Wall;
              } else if j == 18 && (i == 38 || i == 39 || i == 40 || i == 41 || i == 42) {
                terrain[(i + j*width) as usize] = TileType::Wall;
              } else if j == 32 && (i == 38 || i == 39 || i == 41 || i == 42) {
                terrain[(i + j*width) as usize] = TileType::Wall;
              } else if (i - 20).pow(2) + (j - 20).pow(2) < 100 {
                terrain[(i + j*width) as usize] = TileType::Water;
              }
          }
      }
      return Map{width: width,
                 height: height,
                 terrain: terrain,
                 visited: HashSet::new(),
                 visible: HashSet::new(),
                 blocked: vec![false; (width * height) as usize]};
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

    while self.get_distance_sq(start_x, start_y, x0, y0) < length_sq as f32 {
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

  pub fn get_distance_sq(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> f32 {
      // Euclidean distance
    let dx = (cmp::max(x0, x1) - cmp::min(x0, x1)) as f32;
    let dy = (cmp::max(y0, y1) - cmp::min(y0, y1)) as f32;
      return (dx * dx) + (dy * dy);
  }

  pub fn get_index(&self, x0: i32, y0: i32) -> usize {
      return (y0 as usize * self.width as usize) + x0 as usize;
  }

  fn tile_blocks_vision(&self, tile_type: TileType) -> bool {
      return tile_type == TileType::Wall;
  }

  // sets blocked vec based on wall locations, entity locations are set in the mapblocking system
  pub fn set_tile_blocked(&mut self) {
    for (i, tile) in self.terrain.iter().enumerate() {
        if tile == &TileType::Wall || tile == &TileType::Water {
            self.blocked[i] = true;
        }
        else {
            self.blocked[i] = false;
        }
    }
  }

pub fn is_exit_valid(&self, x:i32, y:i32) -> bool {
    if x < 1 || x > self.width-1 || y < 1 || y > self.height-1 { return false; }
    let idx = self.get_index(x, y);
    !self.blocked[idx]
}

pub fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
    let w = self.width as usize;
    let p1 = (idx1 % w, idx1 / w);
    let p2 = (idx2 % w, idx2 / w);
    self.get_distance_sq(p1.0 as i32, p1.1 as i32, p2.0 as i32, p2.1 as i32)
}

pub fn get_available_exits(&self, idx:usize) -> Vec<(usize, f32)> {
    let mut exits = Vec::new();
    let x = idx as i32 % self.width;
    let y = idx as i32 / self.width;
    let w = self.width as usize;

    // Cardinal directions
    if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
    if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
    if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
    if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

    // Diagonals
    if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
    if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
    if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
    if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }

    exits
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