use std::ops::Add;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Cartesian {
    pub x: i32,
    pub y: i32,
}

impl Cartesian {
    pub fn new(x: i32, y: i32) -> Cartesian {
        Cartesian { x: x, y: y }
    }

    /// Creates a list of points around `self` excluding diagonal
    pub fn neigh4(&self) -> Vec<Cartesian> {
        let x = self.x;
        let y = self.y;
        vec![
            Cartesian::new(x - 1, y),
            Cartesian::new(x, y + 1),
            Cartesian::new(x + 1, y),
            Cartesian::new(x, y - 1),
        ]
    }

    /// Creates a list of points around `self` including diagonal
    pub fn neigh8(&self) -> Vec<Cartesian> {
        let x = self.x;
        let y = self.y;
        vec![
            Cartesian::new(x - 1, y),
            Cartesian::new(x - 1, y + 1),
            Cartesian::new(x, y + 1),
            Cartesian::new(x + 1, y + 1),
            Cartesian::new(x + 1, y),
            Cartesian::new(x + 1, y - 1),
            Cartesian::new(x, y - 1),
            Cartesian::new(x - 1, y - 1),
        ]
    }

    /// Calculate the manhattan distance between two points
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        let x_dist = (self.x - other.x).abs() as usize;
        let y_dist = (self.y - other.y).abs() as usize;
        x_dist + y_dist
    }
}

impl FromStr for Cartesian {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.trim_matches(|p| p == '(' || p == ')' )
                                 .split(',')
                                 .map(|t| t.trim())
                                 .collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Cartesian { x: x_fromstr, y: y_fromstr })
    }
}

impl Add for Cartesian {
    type Output = Cartesian;

    fn add(self, other: Cartesian) -> Cartesian {
        Cartesian {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<'a> Add for &'a Cartesian {
    type Output = Cartesian;

    fn add(self, other: Self) -> Cartesian {
        Cartesian {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[test]
fn test_cartesian() {
    let a = Cartesian::new(1, 1);
    let b = Cartesian::new(2, 2);
    let c = Cartesian::new(3, 3);

    assert_eq!(&a + &b, c);
    assert_eq!(a + b, c);
}
