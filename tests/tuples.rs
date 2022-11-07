use cucumber::{given, then, World};
use ray::Tuple;
use std::collections::HashMap;
//use cucumber::Parameter;

#[derive(Debug, Default, World)]
pub struct TuplesWorld {
    tuples: HashMap<String, Tuple>,
}

#[then(expr = "{word}.{word} = {float}")]
fn check_tuple_attr(world: &mut TuplesWorld, tuple_name: String, attr: String, v: f64) {
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

#[given(expr = "{word} is a point {float}, {float}, {float}")]
fn given_a_point(world: &mut TuplesWorld, name: String, x: f64, y: f64, z: f64) {
    let t = Tuple::point(x, y, z);
    world.tuples.insert(name, t);
}

#[given(expr = "{word} is a vector {float}, {float}, {float}")]
fn given_a_vector(world: &mut TuplesWorld, name: String, x: f64, y: f64, z: f64) {
    let t = Tuple::vector(x, y, z);
    world.tuples.insert(name, t);
}

#[then(expr = "{word} + {word} = {tuple}")]
fn check_add(world: &mut TuplesWorld, tuple1_name: String, tuple2_name: String, result: Tuple) {
    let t1: Tuple = world.tuples[&tuple1_name].clone();
    let t2: Tuple = world.tuples[&tuple2_name].clone();
    let t3 = t1 + t2;
    assert!(t3.x == result.x);
    assert!(t3.y == result.y);
    assert!(t3.z == result.z);
    assert!(t3.w == result.w);
}

#[then(expr = "{word} = {tuple}")]
fn check_tuple(world: &mut TuplesWorld, tuple_name: String, result: Tuple) {
    let t: Tuple = world.tuples[&tuple_name].clone();

    assert!(t.x == result.x);
    assert!(t.y == result.y);
    assert!(t.z == result.z);
    assert!(t.w == result.w);
}

#[then(expr = "- {word} = {tuple}")]
fn check_tuple_neg(world: &mut TuplesWorld, tuple_name: String, result: Tuple) {
    let t: Tuple = -world.tuples[&tuple_name].clone();

    assert!(t.x == result.x);
    assert!(t.y == result.y);
    assert!(t.z == result.z);
    assert!(t.w == result.w);
}

#[then(expr = "{word} - {word} = vector {float}, {float}, {float}")]
fn check_sub(world: &mut TuplesWorld, tuple1_name: String, tuple2_name: String, x: f64, y: f64, z: f64) {
    let t1: Tuple = world.tuples[&tuple1_name].clone();
    let t2: Tuple = world.tuples[&tuple2_name].clone();
    let t3 = t1 - t2;
    assert!(t3.is_vector());
    assert!(t3.x == x);
    assert!(t3.y == y);
    assert!(t3.z == z);
}

#[then(expr = "{word} * {float} = {tuple}")]
fn check_mul(world: &mut TuplesWorld, tuple_name: String, x: f64, result: Tuple) {
    let t: Tuple = world.tuples[&tuple_name].clone() * x;

    assert!(t.x == result.x);
    assert!(t.y == result.y);
    assert!(t.z == result.z);
    assert!(t.w == result.w);
}


#[then(expr = "{word} \\/ {float} = {tuple}")]
fn check_div(world: &mut TuplesWorld, tuple_name: String, x: f64, result: Tuple) {
    let t: Tuple = world.tuples[&tuple_name].clone() / x;

    assert!(t.x == result.x);
    assert!(t.y == result.y);
    assert!(t.z == result.z);
    assert!(t.w == result.w);
}

#[then(expr = "magnitude\\({word}\\) = {float}")]
fn check_magnitude(world: &mut TuplesWorld, tuple_name: String, result: f64) {
    let t: Tuple = world.tuples[&tuple_name].clone();
    assert!(t.magnitude() == result);
}

#[then(expr = "normalize\\({word}\\) = vector {float}, {float}, {float}")]
fn check_normalize(world: &mut TuplesWorld, tuple_name: String, x: f64, y: f64, z: f64) {
    let t: Tuple = world.tuples[&tuple_name].clone();
    let t2 = t.normalize();
    assert!(t2.is_vector());
    assert!(t2.x == x);
    assert!(t2.y == y);
    assert!(t2.z == z);

    assert!(t2.magnitude() == 1.0);
}

#[then(expr = "dot\\({word}, {word}\\) = {float}")]
fn check_dot(world: &mut TuplesWorld, tuple1_name: String, tuple2_name: String, res: f64) {
    let t1: Tuple = world.tuples[&tuple1_name].clone();
    let t2: Tuple = world.tuples[&tuple2_name].clone();

    assert!(t1.dot(&t2) == res);
}

#[then(expr = "cross\\({word}, {word}\\) = vector {float}, {float}, {float}")]
fn check_cross(world: &mut TuplesWorld, tuple1_name: String, tuple2_name: String, x: f64, y: f64, z: f64) {
    let t1: Tuple = world.tuples[&tuple1_name].clone();
    let t2: Tuple = world.tuples[&tuple2_name].clone();
    
    let t3 = Tuple::vector(x, y, z);
    
    assert!(t1.cross(&t2) == t3);
}


