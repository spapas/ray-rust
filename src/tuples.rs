use core::str::FromStr;
use cucumber::Parameter;
use regex::Regex;
use std::ops::Add;

#[derive(Debug, Default, Clone, Parameter)]
#[param(name = "tuple", regex = r"tuple\(.+, .+, .+, .+\)")]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w != 1.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }
}

impl FromStr for Tuple {
    type Err = String;
    // https://doc.rust-lang.org/std/str/trait.FromStr.html#examples
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"tuple\((.+), (.+), (.+), (.+)\)").unwrap();
        let caps = re.captures(s).unwrap();
        let x: f64 = caps.get(1).map_or("", |m| m.as_str()).parse().unwrap();
        let y: f64 = caps.get(2).map_or("", |m| m.as_str()).parse().unwrap();
        let z: f64 = caps.get(3).map_or("", |m| m.as_str()).parse().unwrap();
        let w: f64 = caps.get(4).map_or("", |m| m.as_str()).parse().unwrap();
        Ok(Tuple { x, y, z, w })
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let t: Tuple = "tuple(1, 2.3, 3, -4)".parse().unwrap();
        assert!(t.x == 1.0);
        assert!(t.y == 2.3);
        assert!(t.z == 3.0);
        assert!(t.w == -4.0);
        //let t2: Tuple = " tuple(3, -2, 5, 1)".parse().unwrap();
    }

    #[test]
    fn test_add() {
        let t1: Tuple = "tuple(1, 2.3, 3, -4)".parse().unwrap();
        let t2: Tuple = "tuple(2, 0, 1.1, 4)".parse().unwrap();
        let t3 =  t1 + t2;

        assert!(t3.x == 3.0);
        assert!(t3.y == 2.3);
        assert!(t3.z == 4.1);
        assert!(t3.w == 0.0);
        //let t2: Tuple = " tuple(3, -2, 5, 1)".parse().unwrap();
    }
}
