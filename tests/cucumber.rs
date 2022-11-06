use ray::Tuple;
use cucumber::{given, when, then, World};
use std::collections::HashMap;
use cucumber::Parameter;

#[derive(Parameter)]
#[param(name = "tuple", regex = "tuple(3, -2, 5, 1)")]
enum Tuple {}


#[derive(Debug, Default, World)]
pub struct TuplesWorld {
    tuple: Tuple,
    x: f64,
    y: f64,
    z: f64,
    w: f64,
    tuples: HashMap<String, Tuple>
}

//#[given(expr = "a tuple {float}, {float}, {float}, {float}")]
fn given_a_tuple(world: &mut TuplesWorld, x: f64, y: f64, z: f64, w: f64) {
    world.tuple = Tuple{x: x, y: y, z: z, w: w};
    
    world.x = x;
    world.y = y;
    world.z = z;
    world.w = w;
}

#[then(expr = "a.{word} = {float}")]
fn check_tuple(world: &mut TuplesWorld, attr: String, v: f64) {
    let tv = match attr.as_str() {
        "x" => world.tuple.x,
        "y" => world.tuple.y,
        "z" => world.tuple.z,
        "w" => world.tuple.w,
        _ => panic!("Expected x,y or z!")
    };

    assert!(v == tv)
}

#[then("a is a point")]
fn check_is_point(world: &mut TuplesWorld) {
    assert!(world.tuple.is_point());
    assert!(!world.tuple.is_vector());
}

#[then("a is not a point")]
fn check_is_not_point(world: &mut TuplesWorld) {
    assert!(!world.tuple.is_point());
    assert!(world.tuple.is_vector());
}

#[then("a is a vector")]
fn check_is_vector(world: &mut TuplesWorld) {
    assert!(world.tuple.is_vector());
    assert!(!world.tuple.is_point());
}

#[then("a is not a vector")]
fn check_is_not_vector(world: &mut TuplesWorld) {
    assert!(!world.tuple.is_vector());
    assert!(world.tuple.is_point());
}

#[given(expr = "{word} a tuple {float}, {float}, {float}, {float}")]
fn given_a_custom_tuple(world: &mut TuplesWorld, name: String, x: f64, y: f64, z: f64, w: f64) {
    world.tuples.insert(name, Tuple{x: x, y: y, z: z, w: w});
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TuplesWorld::run("tests/features"));
}


