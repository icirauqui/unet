extern crate nalgebra as na;



struct Conv2d {
    kernels: Vec<na::DMatrix<f32>>,
    stride: usize,
    padding: usize,
    channels: usize,
}


impl Conv2d {
    fn new(kernels: Vec<na::DMatrix<f32>>, stride: usize, padding: usize, channels: usize) -> Conv2d {
        Conv2d {
            kernels,
            stride,
            padding,
            channels,
        }
    }

    fn forward(&self, input: &na::DMatrix<f32>) -> na::DMatrix<f32> {
        let mut output = na::DMatrix::<f32>::zeros(input.nrows(), input.ncols());
        for i in 0..input.nrows() {
            for j in 0..input.ncols() {
                output[(i, j)] = input[(i, j)] + 1.0;
            }
        }
        output
    }
}


pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
