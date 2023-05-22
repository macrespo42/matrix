pub fn lerp<V>(u: V, v: V, t: f32) -> V
where
    V: std::ops::Add<Output = V>
        + std::ops::Sub<Output = V>
        + std::ops::Mul<f32, Output = V>
        + Clone,
{
    let mut result: V = v - u.clone();
    result = result * t;
    result = result + u;
    result
}

#[cfg(test)]

mod tests {
    use super::*;
    use crate::types::{Matrix, Vector};

    #[test]
    fn linear_interpolation_should_be_zero() {
        assert_eq!(lerp(0., 1., 0.), 0.);
    }

    #[test]
    fn linear_interpolation_test_with_floats() {
        assert_eq!(lerp(0., 1., 1.), 1.0);
        assert_eq!(lerp(0., 1., 0.5), 0.5);
        assert_eq!(lerp(21., 42., 0.3), 27.3);
    }

    #[test]
    fn linear_interpolation_test_with_vectors() {
        let linear_interpoled = lerp(Vector::from(&[2., 1.]), Vector::from(&[4., 2.]), 0.3);
        assert_eq!(linear_interpoled.positions[0], 2.6);
        assert_eq!(linear_interpoled.positions[1], 1.3);
    }

    #[test]
    fn linear_interpolation_test_with_matrix() {
        let linear_interpoled = lerp(
            Matrix::from(&[&[2., 1.], &[3., 4.]]),
            Matrix::from(&[&[20., 10.], &[30., 40.]]),
            0.5,
        );
        assert_eq!(linear_interpoled.positions[0], Vec::from([11., 5.5]));
        assert_eq!(linear_interpoled.positions[1], Vec::from([16.5, 22.]));
    }
}
