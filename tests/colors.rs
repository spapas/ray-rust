use cucumber::{given, then, World};
use ray::Color;
use std::collections::HashMap;

#[derive(Debug, Default, World)]
pub struct ColorsWorld {
    colors: HashMap<String, Color>,
}

#[given(expr = "{word} is a {color}")]
fn given_a_custom_color(world: &mut ColorsWorld, name: String, t: Color) {
    world.colors.insert(name, t);
}

#[then(expr = "{word}.{word} = {float}")]
fn check_color_attr(world: &mut ColorsWorld, tuple_name: String, attr: String, v: f64) {
    let z: &Color = &world.colors[&tuple_name];
    let tv = match attr.as_str() {
        "red" => z.red,
        "green" => z.green,
        "blue" => z.blue,
        _ => panic!("Expected red green blue!"),
    };
    assert!(tv == v)
}

#[then(expr = "{word} + {word} = {color}")]
fn check_add(world: &mut ColorsWorld, c1_name: String, c2_name: String, result: Color) {
    let t1: Color = world.colors[&c1_name].clone();
    let t2: Color = world.colors[&c2_name].clone();
    let t3 = t1 + t2;
    assert!(t3 == result);
    assert!(t3.red == result.red);
    assert!(t3.green == result.green);
    assert!(t3.blue == result.blue);
}

#[then(expr = "{word} - {word} = {color}")]
fn check_sub(world: &mut ColorsWorld, c1_name: String, c2_name: String, result: Color) {
    let t1: Color = world.colors[&c1_name].clone();
    let t2: Color = world.colors[&c2_name].clone();
    let t3 = t1 - t2;
    assert!(t3.red == result.red);
    assert!(t3.green == result.green);
    assert!(t3.blue == result.blue);
    assert!(t3 == result);
}

#[then(expr = "{word} * {float} = {color}")]
fn check_mul(world: &mut ColorsWorld, cname: String, x: f64, result: Color) {
    let t: Color = world.colors[&cname].clone() * x;

    assert!(t.red == result.red);
    assert!(t.green == result.green);
    assert!(t.blue == result.blue);
    assert!(t == result);
}

#[then(expr = "{word} *h {word} = {color}")]
fn check_mul_co(world: &mut ColorsWorld, c1_name: String, c2_name: String, result: Color) {
    let t1: Color = world.colors[&c1_name].clone();
    let t2: Color = world.colors[&c2_name].clone();
    let t3 = t1 * t2;
    assert!(t3 == result);
    assert!(t3.red == result.red);
    assert!(t3.green == result.green);
    assert!(t3.blue == result.blue);
}