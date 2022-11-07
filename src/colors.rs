use core::str::FromStr;
use cucumber::Parameter;
use std::cmp::Eq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Default, Clone, Copy, Parameter)]
#[param(name = "color", regex = r"color\(.+, .+, .+\)")]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl FromStr for Color {
    type Err = String;
    // https://doc.rust-lang.org/std/str/trait.FromStr.html#examples
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut colors = s
            .trim_start_matches("color(")
            .trim_end_matches(")")
            .split(',')
            .map(|s| s.trim().parse::<f64>().unwrap());

        let (red, green, blue): (f64, f64, f64) = match (colors.next(), colors.next(), colors.next()) {
            (Some(r), Some(g), Some(b)) => {
                (r, g, b)
            }
            _ => panic!(),
        };

        Ok(Color { red, green, blue })
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.green == other.green && self.blue == other.blue
    }
}
impl Eq for Color {}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let t: Color = "color(1, 2.3, 3)".parse().unwrap();
        assert!(t.red == 1.0);
        assert!(t.green == 2.3);
        assert!(t.blue == 3.0);
        //let t2: Tuple = " tuple(3, -2, 5, 1)".parse().unwrap();
    }

    #[test]
    fn test_add() {
        let t1: Color = "color(1, 2.3, 3)".parse().unwrap();
        let t2: Color = "color(2, 0, 1.1)".parse().unwrap();
        let t3 = t1 + t2;

        assert!(t3.red == 3.0);
        assert!(t3.green == 2.3);
        assert!(t3.blue == 4.1);
    }

    #[test]
    fn test_sub() {
        let t1: Color = "color(0.8, 0.6, 0.75)".parse().unwrap();
        let t2: Color = "color(0.8, 0.1, 0.25)".parse().unwrap();
        let t3 = t1 - t2;
        
        assert!(t3.red == 0.0);
        assert!(t3.green == 0.5);
        assert!(t3.blue == 0.5);
    }

    #[test]
    fn test_mul() {
        let t1: Color = "color(1, 2.3, 3)".parse().unwrap();
        let t3 = t1 * 2.0;

        assert!(t3.red == 2.0);
        assert!(t3.green == 4.6);
        assert!(t3.blue == 6.0);
    }

    #[test]
    fn test_mul_h() {
        let t1: Color = "color(1, 0.2, 0.4)".parse().unwrap();
        let t2: Color = "color(0.9, 1, 0.5)".parse().unwrap();
        let t3 = t1 * t2;
        println!("t3: {:?}", t3);
        assert!(t3.red == 0.9);
        assert!(t3.green == 0.2);
        assert!(t3.blue == 0.2);
    }
}
