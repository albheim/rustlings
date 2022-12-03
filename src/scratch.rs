use std::f32;

fn main() {
    let radius = 5.00f32;

    let area = std::f32::consts::pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}
