use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    x: i64,
    y: i64,
}

impl From<(i64, i64)> for Vec2 {
    fn from(value: (i64, i64)) -> Vec2 {
        Vec2 {
            x: value.0,
            y: value.1,
        }
    }
}

pub struct Map {
    size: Vec2,
    tiles: Vec<Tile>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in 0..self.size.y {
            for col in 0..self.size.x {
                write!(f, "{:?}", self.get((col, row).into()))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn new(size: Vec2) -> Self {
        let num_tiles = size.x * size.y;
        Self {
            size,
            tiles: (0..num_tiles)
                .into_iter()
                .map(|_| Default::default())
                .collect(),
        }
    }

    fn parse(input: &[u8]) -> Self {
        let mut cols = 0;
        let mut rows = 1;

        for &c in input.trim_ascii_end().iter() {
            if c == b'\n' {
                rows += 1;
                cols = 0;
            } else {
                cols += 1;
            }
        }

        let mut map = Self::new((cols, rows).into());
        let mut iter = input.iter().copied();
        for y in 0..map.size.y {
            for x in 0..map.size.x {
                let tile = match iter.next() {
                    Some(b'#') => Tile::Tree,
                    Some(b'.') => Tile::Open,
                    Some(b'\n') => break,
                    _ => panic!("Invalid map tile"),
                };
                map.set((x, y).into(), tile);
            }
            iter.next(); // skip newline 
        }
        map
    }

    /// Calculates the resulting position after applying the map
    /// size constrains to `pos`.
    /// Logical vertical bounds and horizonal wrapping in both sides.
    fn normalize_pos(&self, pos: Vec2) -> Option<Vec2> {
        if pos.y < 0 || pos.y >= self.size.y {
            None
        } else {
            let x = pos.x % self.size.x;
            let x = if x < 0 { self.size.x + x } else { x };
            Some((x, pos.y).into())
        }
    }

    /// Calculates the linear index into the `tiles` array
    fn index(&self, pos: Vec2) -> Option<usize> {
        self.normalize_pos(pos)
            .map(|pos| (pos.y * self.size.x + pos.x) as usize)
    }

    fn set(&mut self, pos: Vec2, tile: Tile) {
        if let Some(i) = self.index(pos) {
            self.tiles[i] = tile;
        }
    }

    fn get(&self, pos: Vec2) -> Tile {
        self.index(pos).map(|i| self.tiles[i]).unwrap_or_default()
    }
}

#[derive(Default, Copy, Clone, PartialEq)]
pub enum Tile {
    #[default]
    Open,
    Tree,
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Open => write!(f, "."),
            Self::Tree => write!(f, "#"),
        }
    }
}

pub fn main() {
    let input = include_bytes!("../input.txt");
    let map = Map::parse(input);
    let trees = (0..map.size.y)
        .map(|y| Vec2::from((y * 3, y)))
        .filter(|&pos| map.get(pos) == Tile::Tree)
        .count();
    println!("Solution: {trees}");
}

#[cfg(test)]
mod tests {
    use crate::Map;

    #[test]
    fn normalize_pos() {
        let m = Map::new((2, 2).into());
        assert_eq!(m.normalize_pos((0, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((1, 0).into()), Some((1, 0).into()));
        assert_eq!(m.normalize_pos((2, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((-1, 0).into()), Some((1, 0).into()));
        assert_eq!(m.normalize_pos((-2, 0).into()), Some((0, 0).into()));
        assert_eq!(m.normalize_pos((0, -1).into()), None);
        assert_eq!(m.normalize_pos((0, 2).into()), None);
    }

    #[test]
    fn test_index() {
        let m = Map::new((3, 5).into());
        assert_eq!(m.index((0, 0).into()), Some(0));
        assert_eq!(m.index((2, 0).into()), Some(2));
        assert_eq!(m.index((0, 1).into()), Some(3));
        assert_eq!(m.index((2, 1).into()), Some(5));
    }
}
