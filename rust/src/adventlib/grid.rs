use std::cmp;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x: x, y: y }
    }

    pub fn origin() -> Point {
        Point::new(0, 0)
    }

    pub fn neighbors4(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x, self.y + 1),
        ]
    }

    pub fn neighbors8(&self) -> Vec<Point> {
        vec![
            Point::new(self.x - 1, self.y - 1),
            Point::new(self.x, self.y - 1),
            Point::new(self.x + 1, self.y - 1),
            Point::new(self.x + 1, self.y),
            Point::new(self.x + 1, self.y + 1),
            Point::new(self.x, self.y + 1),
            Point::new(self.x - 1, self.y + 1),
            Point::new(self.x - 1, self.y),
        ]
    }

    pub fn manhattan_dist_to(&self, other: &Point) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn vec_subtract(&self, other: &Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn vec_add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Hash, Debug)]
pub struct Point3d {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl Point3d {
    pub fn new(x: i64, y: i64, z: i64) -> Point3d {
        Point3d { x: x, y: y, z: z }
    }

    pub fn origin() -> Point3d {
        Point3d::new(0, 0, 0)
    }

    pub fn manhattan_dist_to(&self, other: &Point3d) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    pub fn vec_add(&self, other: &Point3d) -> Point3d {
        Point3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn as_vector(&self) -> Point {
        match self {
            Direction::Up => Point::new(0, -1),
            Direction::Down => Point::new(0, 1),
            Direction::Left => Point::new(-1, 0),
            Direction::Right => Point::new(1, 0),
        }
    }

    pub fn is_horizontal(&self) -> bool {
        *self == Direction::Left || *self == Direction::Right
    }

    pub fn is_vertical(&self) -> bool {
        *self == Direction::Up || *self == Direction::Down
    }

    pub fn turn_corner(&self, c_type: char) -> Direction {
        if self.is_vertical() {
            return match c_type {
                '/' => self.turn_right(),
                _ => self.turn_left(),
            };
        }
        return match c_type {
            '/' => self.turn_left(),
            _ => self.turn_right(),
        };
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

pub struct SparseGrid<TContents> {
    grid_contents: HashMap<Point, TContents>,
    min_x: i64,
    min_y: i64,
    max_x: i64,
    max_y: i64,
}

impl<TContents: PartialEq> SparseGrid<TContents> {
    pub fn new() -> SparseGrid<TContents> {
        SparseGrid {
            grid_contents: HashMap::<Point, TContents>::new(),
            min_x: std::i64::MAX,
            min_y: std::i64::MAX,
            max_x: std::i64::MIN,
            max_y: std::i64::MIN,
        }
    }

    pub fn get(&self, location: &Point) -> Option<&TContents> {
        self.grid_contents.get(location)
    }

    pub fn insert(&mut self, location: Point, value: TContents) {
        self.grid_contents.insert(location, value);

        self.min_x = cmp::min(self.min_x, location.x);
        self.min_y = cmp::min(self.min_y, location.y);
        self.max_x = cmp::max(self.max_x, location.x);
        self.max_y = cmp::max(self.max_y, location.y);
    }

    pub fn len(&self) -> usize {
        self.grid_contents.keys().len()
    }

    pub fn find_location_of(&self, value: &TContents) -> Option<Point> {
        self.grid_contents
            .iter()
            .filter(|(_, v)| **v == *value)
            .map(|(&k, _)| k)
            .next()
    }

    pub fn print(&self, cell_renderer: &dyn (Fn(Option<&TContents>) -> char)) {
        println!("{}", self.render_to_string(cell_renderer));
    }

    pub fn render_to_string(&self, cell_renderer: &dyn (Fn(Option<&TContents>) -> char)) -> String {
        let number_of_chars = (self.max_y - self.min_y) * (self.max_x - self.min_x + 1);
        let mut output = String::with_capacity(number_of_chars as usize);

        for i in self.min_y..=self.max_y {
            for j in self.min_x..=self.max_x {
                let contents = self.grid_contents.get(&Point::new(j, i));
                output.push_str(&format!("{}", cell_renderer(contents)));
            }
            output.push_str("\n");
        }
        return output;
    }

    pub fn iter(&self) -> Iter<Point, TContents> {
        self.grid_contents.iter()
    }
}
