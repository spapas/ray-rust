use std::fs::File;
use std::io::prelude::*;

use crate::tuples::Tuple;
use crate::canvas::Canvas;
use crate::colors::Color;

#[derive(Debug)]
pub struct Proj {
    pub p: Tuple,
    pub v: Tuple
}

pub struct Env {
    pub g: Tuple,
    pub w: Tuple
}

pub fn tick(e: &Env, p: &Proj) -> Proj {
    let pos = p.p + p.v;
    let vel = p.v + e.g + e.w;
    Proj{p: pos, v: vel}
}

pub fn runme() {
    let e = Env{g: Tuple::vector(0.0, -0.1, 0.0), w: Tuple::vector(-0.03, 0.0, 0.0)};
    let mut p = Proj{p: Tuple::point(0.0, 1.0, 1.0), v: Tuple::vector(1.0, 1.8, 0.0).normalize() * 11.25};
    
    let mut c = Canvas::canvas(900, 550);
    let red = Color::color(1.0, 0.0, 0.0);
    
    while p.p.y > 0.0 {
        let x = p.p.x as usize;
        let y = (550.0 - p.p.y) as usize;
        c.write_pixel(x, y, red);
        p = tick(&e, &p);
        
    }
    let ppm = c.to_ppm();
    let mut file = File::create("foo.ppm").unwrap();
    file.write_all(ppm.as_bytes()).unwrap();
    
}