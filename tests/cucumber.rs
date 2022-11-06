use cucumber::{given, then, when, World};
use ray::Tuple;
use std::collections::HashMap;
//use cucumber::Parameter;

#[derive(Debug, Default, World)]
pub struct TuplesWorld {
    tuples: HashMap<String, Tuple>,
}


#[then(expr = "{word}.{word} = {float}")]
fn check_tuple(world: &mut TuplesWorld, tuple_name: String, attr: String, v: f64) {
    let z: &Tuple = &world.tuples[&tuple_name];
    let tv = match attr.as_str() {
        "x" => z.x,
        "y" => z.y,
        "z" => z.z,
        "w" => z.w,
        _ => panic!("Expected x,y or z!"),
    };
    assert!(tv == v)
}

#[then(expr = "{word} is a {word}")]
fn check_is_something(world: &mut TuplesWorld, tuple_name: String, what: String) {
    let z: &Tuple = &world.tuples[&tuple_name];
    if what == "point" {
        assert!(z.is_point());
        assert!(!z.is_vector());
    } else if what == "vector" {
        assert!(!z.is_point());
        assert!(z.is_vector());
    }
}

#[then(expr = "{word} is not a {word}")]
fn check_is_not_something(world: &mut TuplesWorld, tuple_name: String, what: String) {
    let z: &Tuple = &world.tuples[&tuple_name];
    if what == "point" {
        assert!(!z.is_point());
        assert!(z.is_vector());
    } else if what == "vector" {
        assert!(z.is_point());
        assert!(!z.is_vector());
    }
}

#[given(expr = "{word} a {tuple}")]
fn given_a_custom_tuple(world: &mut TuplesWorld, name: String, t: Tuple) {
    world.tuples.insert(name, t);
}

fn main() {
    // You may choose any executor you like (`tokio`, `async-std`, etc.).
    // You may even have an `async` main, it doesn't matter. The point is that
    // Cucumber is composable. :)
    futures::executor::block_on(TuplesWorld::run("tests/features"));
}
