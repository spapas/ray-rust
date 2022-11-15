pub mod tuples;
pub mod canon;
pub mod colors;
pub mod canvas;

use ndarray::Array2;
use ndarray_linalg::Array2;

fn main() {
    /*
    let x = 5;
    let y = 5;
    let mut test = Array2::<f64>::zeros((x, y));
    test[[0,1]] = 2.0;
    test[[1,1]] = 5.0;
    test[[1,0]] = 3.0;
    println!("{:?}", test);
    

    let mut test2 = Array2::<colors::Color>::default((x, y));
    test2[[3,3]] = colors::Color::color(0.5, 0.5, 0.5);
    println!("{:?}", test2);
     */
    //let c = canvas::Canvas::canvas(10, 20);
    //println!("{:?}", c);
    //println!("{:?}", c.to_ppm());
    //canon::runme();

    let mut test = Array2::<f64>::zeros((5, 5));
    test[[0,1]] = 2.0;
    test[[1,1]] = 5.0;
    test[[1,0]] = 3.0;
    println!("{:?}", test);
    let mut test2 = test.clone().reversed_axes();
    println!("{:?}", test2);
    let mut test3 = Array2::<f64>::zeros((5, 1));
    println!("{:?}", test3);
    println!("{:?}", test.dot(&test2));
    println!("{:?}", Array2::<f64>::eye(5));
    // println!("{:?}", test);

}
