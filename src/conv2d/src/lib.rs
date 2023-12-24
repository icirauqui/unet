extern crate nalgebra as na;
extern crate ndarray as nd;


struct Conv2d {
    /*
    Perform 2D convolution on input feature map with a set of kernels.
    :param kernels: (kernel_dim, kernel_dim, channels, output_channels)
    :param stride: stride of the convolution
    :param padding: padding of the input feature map
     */
    kernel: nd::Array4<f64>,
    stride: usize,
    padding: usize
}


impl Conv2d {
    fn new(kernel: nd::Array4<f64>, stride: usize, padding: usize) -> Conv2d {
        Conv2d{ kernel, stride, padding }
    }

    fn forward(&self, input_feature_map: &nd::Array3<f64>) -> nd::Array3<f64> {
        let (h, w, d) = (input_feature_map.shape()[0], input_feature_map.shape()[1], input_feature_map.shape()[2]);
        let (f, _, _, n) = (self.kernel.shape()[0], self.kernel.shape()[1], self.kernel.shape()[2], self.kernel.shape()[3]);
        
        let out_height = (h - f + 2 * self.padding) / self.stride + 1;
        let out_width = (w - f + 2 * self.padding) / self.stride + 1;
        
        let mut output_feature_map = nd::Array3::<f64>::zeros((out_height, out_width, n));

        let padded_input = {
            let mut temp = nd::Array3::<f64>::zeros((h+2*self.padding, w+2*self.padding, d));
            let (ph, pw) = (self.padding, self.padding);
        }
    }

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
