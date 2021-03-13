use std::ops::{Index, IndexMut};
use serde::{Serialize, Deserialize};
use bracket_geometry::prelude::Point;

type TileMapLocation = usize;

#[derive(Clone, Serialize, Deserialize)]
pub struct TileMap<T> {
    pub dim: Point,
    storage: Vec<T>
}

impl <T> TileMap<T> {
    pub fn new<F>(dim: Point, init: F) -> Self
    where F: Fn(Point) -> T {
        TileMap {
            dim: dim,
            storage: (0 .. (dim.x * dim.y)).map(|i| {
                let x = i % dim.x;
                let y = i / dim.x;
                init(Point::new(x, y))
            }).collect()
        }
    }

    pub fn is_valid(&self, pos: Point) -> bool {
        pos.x >= 0
            && pos.y >= 0
            && pos.x < self.dim.x
            && pos.y < self.dim.y
    }

    pub fn to_location(&self, pos: Point) -> TileMapLocation {
        (pos.y * self.dim.x + pos.x) as usize
    }

    pub fn from_location(&self, loc: TileMapLocation) -> Point {
        Point::new(loc % self.dim.x as usize, loc / self.dim.x as usize)
    }
}

impl <T> Index<Point> for TileMap<T> {
    type Output = T;

    fn index(&self, pos: Point) -> &Self::Output {
        &self.storage[(pos.y * self.dim.x + pos.x) as usize]
    }
}

impl <T> IndexMut<Point> for TileMap<T> {
    fn index_mut(&mut self, pos: Point) -> &mut Self::Output {
        &mut self.storage[(pos.y * self.dim.x + pos.x) as usize]
    }
}

impl <T> Index<TileMapLocation> for TileMap<T> {
    type Output = T;

    fn index(&self, loc: TileMapLocation) -> &Self::Output {
        &self.storage[loc]
    }
}

impl <T> IndexMut<TileMapLocation> for TileMap<T> {
    fn index_mut(&mut self, loc: TileMapLocation) -> &mut Self::Output {
        &mut self.storage[loc]
    }
}
