use crate::matrix::Matrix;

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    pixel_size: f64,
    field_of_view: f64,
    transform: Matrix,
}

#[cfg(test)]
mod tests {
    use super::*;
}
