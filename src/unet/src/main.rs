extern crate conv2d;
extern crate nalgebra as na;


fn simulate_image(width: usize, height: usize) -> na::DMatrix<f32> {
    let mut image = na::DMatrix::<f32>::zeros(width, height);
    for i in 0..width {
        for j in 0..height {
            image[(i, j)] = (i + j) as f32;
        }
    }
    image
}


fn main() {
    println!("Hello, world!");

    let image = simulate_image(10, 10);
    println!("image: {:?}", image);
}
