use std::time::{SystemTime, UNIX_EPOCH};

mod canvas;
mod materials;
mod math;
mod matrix;
mod parsers;
mod patterns;
mod rays;
mod renderer;
mod scene;
mod shapes;
mod tuples;

fn main() {
    println!("Rendering...");
    let start = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time isn't working today");
    renderer::render_scene();
    let end = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time is still on leave");

    println!(
        "Rendering complete. Execution time: {:?} seconds",
        end.as_secs() - start.as_secs()
    );
}
