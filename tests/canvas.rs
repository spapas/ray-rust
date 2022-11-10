use cucumber::{given, then, when, World};
use ray::{Canvas, Color};
use std::collections::HashMap;
use cucumber::gherkin::Step;

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvases: HashMap<String, Canvas>,
    colors: HashMap<String, Color>,
    ppms: HashMap<String, String>,
}

#[given(expr = "{word} is a canvas\\({int}, {int}\\)")]
fn given_a_custom_tuple(world: &mut CanvasWorld, name: String, w: usize, h: usize) {
    world.canvases.insert(name, Canvas::canvas(w, h));
}

#[then(expr = "{word}.{word} = {int}")]
fn check_canvas_attr(world: &mut CanvasWorld, cname: String, attr: String, v: usize) {
    let z: &Canvas = &world.canvases[&cname];
    let tv = match attr.as_str() {
        "width" => z.width,
        "height" => z.height,
        _ => panic!("Expected width or height!"),
    };
    assert!(tv == v)
}

#[then(expr = "every pixel of {word} is {color}")]
fn check_canvas_color(world: &mut CanvasWorld, cname: String, c: Color) {
    let z: &Canvas = &world.canvases[&cname];
    z.data.iter().for_each(|x| assert!(*x == c));
}

#[given(expr = "{word} is a {color}")]
fn given_a_custom_color(world: &mut CanvasWorld, name: String, t: Color) {
    world.colors.insert(name, t);
}

#[when(expr = "write_pixel\\({word}, {int}, {int}, {word}\\)")]
fn write_color(world: &mut CanvasWorld, cname: String, x: usize, y: usize, c: String) {
    let ca: &mut Canvas = &mut world.canvases.get_mut(&cname).unwrap();
    let co: &Color = &world.colors[&c];
    ca.write_pixel(x, y, co.clone())
}

#[then(expr = "pixel_at\\({word}, {int}, {int}\\) = {word}")]
fn check_canvas_pixel(world: &mut CanvasWorld, cname: String, x: usize, y: usize, colorname: String) {
    let ca: &Canvas = &world.canvases[&cname];
    let co: &Color = &world.colors[&colorname];
    assert!(ca.pixel_at(x, y) == *co)
}

#[when(expr = "{word} is canvas_to_ppm\\({word}\\)")]
fn canvas_to_ppm(world: &mut CanvasWorld, pname: String, cname: String) {
    let ca: &Canvas = &world.canvases[&cname];
    let ppm = ca.to_ppm();
    world.ppms.insert(pname, ppm);
}

fn ftf2(l: &str, lfrom: usize, lto: usize ) -> impl Iterator<Item=&str> {
    let lfrom_ok = lfrom - 1;
    let lto_ok = lto - 1;
    l.lines().skip(lfrom_ok).take(lto_ok - lfrom_ok + 1)  

}

#[then(expr = "lines {int}-{int} of {word} are")]
fn check_ppm_lines(world: &mut CanvasWorld, step: &Step, lfrom: usize, lto: usize, pname: String) {
    let ppm: &String = &world.ppms[&pname];
    let l1 = step.docstring().unwrap().lines();
    let l2 = ftf2(ppm, lfrom, lto);
    println!("l1: {:?}", step.docstring().unwrap());
    println!("l2: {:?}", ppm);
    for (x,y) in l1.zip(l2) { 
        println!("x: {:?}, y: {:?}", x, y);
        assert_eq!(x,  y);
    }
}


#[when(expr = "every pixel of {word} is set to {color}")]
fn canvas_set_color(world: &mut CanvasWorld, cname: String, color: Color) {
    let ca: &mut Canvas = &mut world.canvases.get_mut(&cname).unwrap();
    for x in 0..ca.width {
        for y in 0..ca.height {
            ca.write_pixel(x, y, color);
        }
    }
}

#[then(expr = "{word} ends with a newline character")]
fn check_ppm_newline(world: &mut CanvasWorld, pname: String) {
    let ppm: &String = &world.ppms[&pname];
    
    assert!(ppm.ends_with("\n"))
    
}


