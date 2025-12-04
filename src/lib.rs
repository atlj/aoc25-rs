use std::str::FromStr;

#[derive(Debug)]
pub struct Map<Tile>
where
    Tile: std::fmt::Debug,
{
    cells: Vec<Vec<Tile>>,
}

impl<Tile> Map<Tile>
where
    Tile: std::fmt::Debug,
{
    pub fn new(cells: Vec<Vec<Tile>>) -> Self {
        Map { cells }
    }

    /// Returns the tile at given `Pos`. Or `None` if `at` is out of bounds.
    pub fn get_tile<'a>(&'a self, at: &Pos) -> Option<&'a Tile> {
        self.cells.get(at.y).and_then(|row| row.get(at.x))
    }

    pub fn get_tile_mut<'a>(&'a mut self, at: &Pos) -> Option<&'a mut Tile> {
        self.cells.get_mut(at.y).and_then(|row| row.get_mut(at.x))
    }

    /// Returns all tiles marked with `X`:
    /// XXX
    /// XOX
    /// XXX
    pub fn get_tiles_ring1<'a>(&'a self, of: &Pos) -> impl Iterator<Item = (Pos, &'a Tile)> {
        enum Delta {
            Decrement,
            Zero,
            Increment,
        }

        const DELTAS: [(Delta, Delta); 8] = [
            // TOP
            (Delta::Decrement, Delta::Decrement),
            (Delta::Zero, Delta::Decrement),
            (Delta::Increment, Delta::Decrement),
            // MIDDLE
            (Delta::Decrement, Delta::Zero),
            (Delta::Increment, Delta::Zero),
            // BOTTOM
            (Delta::Decrement, Delta::Increment),
            (Delta::Zero, Delta::Increment),
            (Delta::Increment, Delta::Increment),
        ];

        DELTAS.iter().filter_map(|(delta_x, delta_y)| {
            let x = match delta_x {
                Delta::Decrement => of.x.checked_sub(1)?,
                Delta::Zero => of.x,
                Delta::Increment => of.x + 1,
            };

            let y = match delta_y {
                Delta::Decrement => of.y.checked_sub(1)?,
                Delta::Zero => of.y,
                Delta::Increment => of.y + 1,
            };

            let pos = Pos { x, y };
            let tile = self.get_tile(&pos)?;

            Some((pos, tile))
        })
    }

    pub fn iter(&self) -> impl Iterator<Item = (Pos, &Tile)> {
        self.cells.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, tile)| (Pos { x, y }, tile))
        })
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Pos, &mut Tile)> {
        self.cells.iter_mut().enumerate().flat_map(|(y, row)| {
            row.iter_mut()
                .enumerate()
                .map(move |(x, tile)| (Pos { x, y }, tile))
        })
    }
}

impl<Err, Tile> FromStr for Map<Tile>
where
    Err: std::fmt::Display,
    Tile: std::fmt::Debug + TryFrom<char, Error = Err>,
{
    type Err = Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cells = s
            .lines()
            .map(|row| {
                row.chars()
                    .map(Tile::try_from)
                    .collect::<Result<Vec<_>, Self::Err>>()
            })
            .collect::<Result<Vec<_>, Self::Err>>()?;

        Ok(Self { cells })
    }
}

#[derive(Clone, Debug)]
pub struct Pos {
    x: usize,
    y: usize,
}
