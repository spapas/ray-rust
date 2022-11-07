use core::str::FromStr;
use cucumber::Parameter;
use regex::Regex;
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::cmp::Eq;

#[derive(Debug, Default, Clone, Copy, Parameter)]
#[param(name = "tuple", regex = r"tuple\(.+, .+, .+, .+\)")]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Tuple {
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Tuple {
        let m: f64 = self.magnitude();
        Tuple {
            x: self.x / m,
            y: self.y / m,
            z: self.z / m,
            w: self.w / m,
        }
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 1.0 }
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple { x, y, z, w: 0.0 }
    }

    pub fn dot(&self, other: &Tuple) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: &Tuple) -> Tuple {
        Tuple::vector(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
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

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.w == other.w
    }
}
impl Eq for Tuple {}

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

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;

    fn neg(self) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, other: f64) -> Tuple {
        Tuple {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, other: f64) -> Tuple {
        Tuple {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
            w: self.w / other,
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
        let t3 = t1 + t2;

        assert!(t3.x == 3.0);
        assert!(t3.y == 2.3);
        assert!(t3.z == 4.1);
        assert!(t3.w == 0.0);
    }

    #[test]
    fn test_sub() {
        let t1: Tuple = "tuple(1, 2.3, 3, -4)".parse().unwrap();
        let t2: Tuple = "tuple(2, 0, 1.0, 4)".parse().unwrap();
        let t3 = t1 - t2;

        assert!(t3.x == -1.0);
        assert!(t3.y == 2.3);
        assert!(t3.z == 2.0);
        assert!(t3.w == -8.0);
    }

    #[test]
    fn test_mul() {
        let t1: Tuple = "tuple(1, 2.3, 3, -4)".parse().unwrap();
        let t3 = t1 * 2.0;

        assert!(t3.x == 2.0);
        assert!(t3.y == 4.6);
        assert!(t3.z == 6.0);
        assert!(t3.w == -8.0);
    }

    #[test]
    fn test_div() {
        let t1: Tuple = "tuple(1, 2.4, 3, -4)".parse().unwrap();
        let t3 = t1 / 2.0;

        assert!(t3.x == 0.5);
        assert!(t3.y == 1.2);
        assert!(t3.z == 1.5);
        assert!(t3.w == -2.0);
    }

    #[test]
    fn test_magnitude() {
        let t1: Tuple = "tuple(1, 0, 0, 0)".parse().unwrap();
        let t2: Tuple = "tuple(3, 4, 0, 0)".parse().unwrap();
        assert!(t1.magnitude() == 1.0);
        assert!(t2.magnitude() == 5.0);
    }

    #[test]
    fn test_normalize() {
        
        let t0: Tuple = "tuple(3, 4, 0, 0)".parse().unwrap();
        let t1: Tuple = t0.normalize();
        assert!(t1.x == 0.6);
        assert!(t1.y == 0.8);
        assert!(t1.z == 0.0);
        assert!(t1.w == 0.0);

        assert!(t1.magnitude() == 1.0);
    }

    #[test]
    fn test_dot() {
        let t0: Tuple = Tuple::vector(1.0, 2.0,3.0);
        let t1: Tuple = Tuple::vector(2.0, 3.0,4.0);
        assert!(20.0 == t0.dot(&t1));
    }

    #[test]
    fn test_eq() {
        let t0: Tuple = Tuple::vector(1.0, 2.0,3.0);
        let t1: Tuple = Tuple::vector(2.0, 3.0,4.0);
        let t2: Tuple = Tuple{x: 1.0, y: 2.0, z: 3.0, w: 0.0};
        assert!(t0 != t1);
        assert!(t0 == t2);
    }

    #[test]
    fn test_cross() {
        let t0: Tuple = Tuple::vector(1.0, 2.0,3.0);
        let t1: Tuple = Tuple::vector(2.0, 3.0,4.0);
        let t01 = t0.cross(&t1);
        let t10 = t1.cross(&t0);
        assert!(t01 == Tuple::vector(-1.0, 2.0, -1.0)); 
        assert!(t10 == Tuple::vector(1.0, -2.0, 1.0)); 
    }
}
