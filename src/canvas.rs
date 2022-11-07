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

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.data[[x, y]]
    }
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
        assert!(ca.get_pixel(x, y) == co);
        assert!(ca.get_pixel(x-1, y) == Color::color(0.0, 0.0, 0.0));
    }
}
