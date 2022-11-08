use ndarray::{Array2, ArrayBase, Dim, OwnedRepr};

use crate::colors::Color;

#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: ArrayBase<OwnedRepr<Color>, Dim<[usize; 2]>>,
}

impl Canvas {
    pub fn canvas(w: usize, h: usize) -> Canvas {
        Canvas {
            width: w,
            height: h,
            data: Array2::<Color>::default((w, h)),
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        self.data[[x, y]] = c;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.data[[x, y]]
    }

    pub fn to_ppm(&self) -> String {
        let mut s = String::new();
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");
        for y in 0..self.height {
            let mut line = String::new();
            for x in 0..self.width {
                let c = self.pixel_at(x, y);
                let r = (c.red * 256.0) as u8;
                let g = (c.green * 256.0) as u8;
                let b = (c.blue * 256.0) as u8;
                line = fix_line(line, r);
                line = fix_line(line, g);
                line = fix_line(line, b);
                //line.push_str(&format!("{} {} {} ", r, g, b));
                //println!("line: {}", line.len());
                
                
            }
            s.push_str(&format!("{}\n", line.trim()));
        }

        s
    }
}

fn fix_line(mut l: String, v: u8) -> String {
    l.push_str(&format!("{}", v));
    let len = l.len();
    let should_split = len >= 70;
    if should_split {
        l.push_str("\n");
    } else {
        l.push_str(" ");
    }
    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fn() {
        let c = Canvas::canvas(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.data.shape(), &[10, 20]);
        assert_eq!(c.data[[0, 0]], Color::color(0.0, 0.0, 0.0));
        assert_eq!(c.data[[5, 5]], Color::color(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_write_pixel() {
        let mut ca = Canvas::canvas(10, 20);
        let co = Color::color(0.5, 0.5, 0.5);
        let x = 2;
        let y = 2;
        ca.write_pixel(x, y, co);
        assert!(ca.pixel_at(x, y) == co);
        assert!(ca.pixel_at(x-1, y) == Color::color(0.0, 0.0, 0.0));
    }

    
    #[test]
    fn test_fix_line() {
        let mut l = String::new();
        l = fix_line(l, 1);
        l = fix_line(l, 2);
        l = fix_line(l, 3);
        
        assert_eq!(l.trim(), "1 2 3");
    }

}
