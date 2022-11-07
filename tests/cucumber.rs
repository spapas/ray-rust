use cucumber::{World};
mod tuples;
mod colors;
mod canvas;


fn main() {
    futures::executor::block_on(colors::ColorsWorld::run("tests/features/colors.feature"));
    futures::executor::block_on(tuples::TuplesWorld::run("tests/features/tuples.feature"));
    futures::executor::block_on(canvas::CanvasWorld::run("tests/features/canvas.feature"));
}
