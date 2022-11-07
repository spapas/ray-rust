use cucumber::{given, then, when, World};
use ray::{Canvas, Color};
use std::collections::HashMap;

#[derive(Debug, Default, World)]
pub struct CanvasWorld {
    canvases: HashMap<String, &mut Canvas>,
    colors: HashMap<String, Color>,
}

#[given(expr = "{word} is a canvas\\({int}, {int}\\)")]
fn given_a_custom_tuple(world: &mut CanvasWorld, name: String, w: usize, h: usize) {
    world.canvases.insert(name, &Canvas::canvas(w, h));
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
    let ca: &Canvas = &mut world.canvases.get_mut(&cname).unwrap();
    let co: &Color = &world.colors[&c];
    ca.write_pixel(x, y, co)

}