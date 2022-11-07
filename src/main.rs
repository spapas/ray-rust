pub mod tuples;
pub mod canon;



fn main() {
    let t = tuples::Tuple{x: 1.0, y: 1.0, z: 1.0, w: 1.0};
    println!("Hello, world! {t:?}");
    canon::runme();

}
