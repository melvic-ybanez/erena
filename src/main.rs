use std::time::{SystemTime, UNIX_EPOCH};

mod tuples;
mod math;
mod canvas;
mod matrix;
mod rays;
mod shapes;
mod materials;
mod renderer;
mod scene;
mod patterns;
mod parsers;

fn main() {
    println!("Rendering...");
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH).expect("Time isn't working today");
    renderer::render_scene();
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH).expect("Time is still on leave");

    println!("Rendering complete. Execution time: {:?} seconds", end.as_secs() - start.as_secs());
}
