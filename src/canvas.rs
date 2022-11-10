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
        let mut line_len: u32 = 0;
        s.push_str("P3\n");
        s.push_str(&format!("{} {}\n", self.width, self.height));
        s.push_str("255\n");
        for y in 0..self.height {
            
            for x in 0..self.width {
                
                let c = self.pixel_at(x, y);
                let r = (c.red * 256.0) as u8;
                let g = (c.green * 256.0) as u8;
                let b = (c.blue * 256.0) as u8;
                s = fix_line(s, r, &mut line_len);
                s = fix_line(s, g, &mut line_len);
                s = fix_line(s, b, &mut line_len);
                //line.push_str(&format!("{} {} {} ", r, g, b));
                //println!("line: {}", line.len());
                
                
            }
            s = s.trim().to_string();
            s.push_str("\n");
            line_len = 0;
        }

        s
    }
}

fn fix_line(mut l: String, v: u8,  line_len: &mut u32 ) -> String {
    let s = &format!("{}", v);
    l.push_str(s);
    *line_len = *line_len + 1 + s.len() as u32;

    let should_split = *line_len >= 68;
    if should_split {
        l.push_str("\n");
        *line_len = 0;
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
        let mut line_len = 0;
        l = fix_line(l, 1, &mut line_len);
        assert_eq!(l.len(), 2);
        l = fix_line(l, 2, &mut line_len);
        assert_eq!(l.len(), 4);
        l = fix_line(l, 3, &mut line_len);
        assert_eq!(l.len(), 6);
        l = fix_line(l, 4, &mut line_len);
        assert_eq!(l.len(), 8);
        
        assert_eq!(l.trim(), "1 2 3 4");
    }

}
