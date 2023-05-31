use crate::types::Vector;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

pub fn angle_cos<K>(u: &Vector<K>, v: &Vector<K>) -> f32
where
    K: Add<Output = K> + Mul<Output = K> + Sub<Output = K> + Copy,
    K: Into<f32>,
{
    if u.size() != v.size() {
        panic!("Vectors must have the same dimension to be able to compute there cosines");
    }

    if u.positions.len() == 0 || v.positions.len() == 0 {
        panic!("One the Vectors are 0");
    }
    let dot_product: K = u.clone().dot(v.clone());
    let u_norm = u.clone().norm();
    let v_norm = v.clone().norm();

    (dot_product.into() / (u_norm * v_norm)) as f32
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn angle_cos_with_0() {
        let u = Vector::from(&[1., 0.]);
        let v = Vector::from(&[1., 0.]);
        assert_eq!(angle_cos(&u, &v), 1.0);

        let u = Vector::from(&[1., 0.]);
        let v = Vector::from(&[0., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.0);
    }

    #[test]
    fn angle_cos_with_negative() {
        let u = Vector::from(&[-1., 1.]);
        let v = Vector::from(&[1., -1.]);
        assert_eq!(angle_cos(&u, &v), -1.0000001)
    }

    #[test]
    fn angle_cos_basics() {
        let u = Vector::from(&[2., 1.]);
        let v = Vector::from(&[4., 2.]);
        assert_eq!(angle_cos(&u, &v), 1.0);

        let u = Vector::from(&[1., 2., 3.]);
        let v = Vector::from(&[4., 5., 6.]);
        assert_eq!(angle_cos(&u, &v), 0.9746319);
    }
}
