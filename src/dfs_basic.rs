use std::fmt;
use std::{thread, time};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Tile {
    row: u8,
    col: u8,
    val: u8,
}

#[derive(Clone, PartialEq, Eq)]
pub struct Tilemap {
    width: u8,
    height: u8,
    tiles: Vec<Vec<Tile>>,
}

impl Tilemap {
    pub fn new(rows: u8, colums: u8) -> Tilemap {
        let width = colums;
        let height = rows;

        let mut tiles = Vec::new();

        for r in 0..height {
            let mut vec_inner = Vec::new();
            for c in 0..width {
                let new_tile = Tile {
                    row: r,
                    col: c,
                    val: 0,
                };
                vec_inner.push(new_tile);
            }
            tiles.push(vec_inner);
        }

        Tilemap {
            width,
            height,
            tiles,
        }
    }

    pub fn run_dfs(&mut self,start_row:u8, start_col:u8) {
        let mut tile_stack = Vec::new();
        self.tiles[start_row as usize][start_col as usize].val=1;
        tile_stack.push(self.tiles[start_row as usize][start_col as usize].clone());

        let mut next = tile_stack.last().cloned().unwrap();

        while tile_stack.len() > 0{
            match self.get_unexplored(next.row, next.col){
                Ok((r,c)) =>{
                    self.tiles[r as usize][c as usize].val=1;
                    if let Some(last) = tile_stack.last_mut(){
                        last.val = 1;
                    }
                    tile_stack.push(self.tiles[r as usize][c as usize].clone());
                    next = tile_stack.last().cloned().unwrap();
                    println!("{}", self);
                },
                Err(()) => {
                    break;
                    //backtrack
                }
            };
        }

    }

    fn get_unexplored(&self, row: u8, col: u8) -> Result<(u8,u8),()>{
        for dr in [self.height-1,0,1].iter().cloned(){
            for dc in [self.width-1,0,1].iter().cloned(){
                let nr = (dr + row)  % self.height;
                let nc = (dc + col) % self.width;
                if self.tiles[nr as usize][nc as usize].val == 0{
                    return Ok((nr, nc));
                }                
            }
        }
        Err(())
    }
}

impl fmt::Display for Tilemap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        print!("\x1B[2J");
        thread::sleep(time::Duration::from_millis(100));
        write!(f, "\n")?;
        for r in 0..self.height as usize {
            for c in 0..self.width as usize {
                let symbol = match self.tiles[r][c].val {
                    0 => '◻',
                    1 => '◼',
                    _ => '◆',
                };
                write!(f, "{0:>2}", symbol)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

pub fn run() {
    let mut tilemap = Tilemap::new(10, 10);
    println!("{}", tilemap);
    tilemap.run_dfs(9, 2);
}
