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

    if u.positions.is_empty() || v.positions.is_empty() {
        panic!("One the Vectors are 0");
    }
    let dot_product: K = u.dot(v.clone());
    let u_norm = u.norm();
    let v_norm = v.norm();

    dot_product.into() / (u_norm * v_norm)
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

    #[test]
    fn angle_cos_complete() {
        let u = Vector::from(&[8., 7.]);
        let v = Vector::from(&[3., 2.]);
        assert_eq!(angle_cos(&u, &v), 0.99145424);

        let u = Vector::from(&[1., 1.]);
        let v = Vector::from(&[1., 1.]);
        assert_eq!(angle_cos(&u, &v), 1.0000001);

        let u = Vector::from(&[4., 2.]);
        let v = Vector::from(&[1., 1.]);
        assert_eq!(angle_cos(&u, &v), 0.94868326);

        let u = Vector::from(&[-7., 3.]);
        let v = Vector::from(&[6., 4.]);
        assert_eq!(angle_cos(&u, &v), -0.54626775);
    }

    #[test]
    fn cosine_proof() {
        let u = Vector::from(&[8., 7.]);
        let v = Vector::from(&[3., 2.]);
        println!("test with\n{}\n{}", u, v);
        println!("{}", angle_cos(&u, &v));
        assert_eq!(angle_cos(&u, &v), 0.99145424);
        let norm_u_f32 = u.norm();
        let norm_calculator_u_f32 = 10.630145812735 as f32;
        assert_eq!(norm_u_f32, norm_calculator_u_f32);
        let norm_v_f32 = v.norm();
        let norm_calculator_v_f32 = 3.605551275464 as f32;
        assert_eq!(norm_v_f32, norm_calculator_v_f32);
        let dot_f32 = u.dot(v);
        let dot_calculator = 38 as f32;
        assert_eq!(dot_f32, dot_calculator);
        println!(
        "norm u : {} | norm from calculator online {}\nnorm v {} | norm v from calculator online {}\ndot {} | dot on calculator {}",
        norm_u_f32, norm_calculator_u_f32, norm_v_f32, norm_calculator_v_f32,dot_f32, dot_calculator
    );
        let dot_norm = norm_u_f32 * norm_v_f32;
        let dot_norm_calculator = norm_calculator_u_f32 * norm_calculator_v_f32;
        assert_eq!(dot_norm, dot_norm_calculator);
        println!(
            "dot product on norm {:?} | dot product with value of generator {}",
            (dot_norm),
            (norm_calculator_u_f32 * norm_calculator_v_f32)
        );
        let angle_cos_result = dot_f32 / dot_norm;
        let angle_cos_calculator = dot_calculator / dot_norm_calculator;
        assert_eq!(angle_cos_result, angle_cos_calculator);
        println!(
            "angle cos result : {} | angle cos result calculator : {}",
            angle_cos_result, angle_cos_calculator
        );
        println!("------------------------------------------------------");
    }
}
