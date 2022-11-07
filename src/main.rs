pub mod tuples;
pub mod canon;
pub mod colors;

use ndarray::Array2;


fn main() {
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
}
