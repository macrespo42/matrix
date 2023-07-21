use crate::types::Vector;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

pub fn linear_combination<K>(u: &[Vector<K>], coefs: &[K]) -> Vector<K>
where
    K: Add<Output = K> + Mul<Output = K> + Sub<Output = K> + Copy,
{
    if u.len() != coefs.len() {
        panic!("For linear combination you must have same size of vectors and coefs");
    }
    let vector_size: usize = u[0].positions.len();
    for elt in u {
        if elt.positions.len() != vector_size {
            panic!("For linear combination vector must have the same size");
        }
    }

    let mut scaled_vectors: Vec<Vector<K>> = Vec::new();

    for i in 0..u.len() {
        let mut cpy = u[i].clone();
        cpy.scl(coefs[i]);
        scaled_vectors.push(cpy);
    }

    let mut linear_combination_result: Vec<K> = Vec::new();

    for column_index in 0..scaled_vectors[0].size() {
        let mut computed_point: K = scaled_vectors[0].positions[0];
        for (row_index, _row_value) in scaled_vectors.iter().enumerate() {
            if row_index == 0 {
                computed_point = scaled_vectors[row_index].positions[column_index];
            } else {
                computed_point = computed_point + scaled_vectors[row_index].positions[column_index];
            }
        }
        linear_combination_result.push(computed_point);
    }

    let result: Vector<K> = Vector {
        positions: linear_combination_result,
    };

    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn linear_combination_square_vectors() {
        let e1 = Vector::from(&[1., 0., 0.]);
        let e2 = Vector::from(&[0., 1., 0.]);
        let e3 = Vector::from(&[0., 0., 1.]);

        let linear_combined = linear_combination::<f32>(&[e1, e2, e3], &[10., -2., 0.5]);
        assert_eq!(linear_combined.positions, &[10., -2., 0.5]);
    }

    #[test]
    fn linear_combination_unsquared_vector() {
        let v1 = Vector::from(&[1., 2., 3.]);
        let v2 = Vector::from(&[0., 10., -100.]);

        let linear_combined = linear_combination::<f32>(&[v1, v2], &[10., -2.]);
        assert_eq!(linear_combined.positions, &[10., 0., 230.]);
    }

    #[test]
    fn linear_combination_basics() {
        let v1 = Vector::from(&[-42., 42.]);

        let linear_combined = linear_combination::<f32>(&[v1], &[-1.]);
        assert_eq!(linear_combined.positions, &[42., -42.]);

        let v1 = Vector::from(&[-42.]);
        let linear_combined =
            linear_combination::<f32>(&[v1.clone(), v1.clone(), v1.clone()], &[-1., 1., 0.]);
        assert_eq!(linear_combined.positions, &[0.]);

        let v1 = Vector::from(&[-42., 42.]);
        let v2 = Vector::from(&[1., 3.]);
        let v3 = Vector::from(&[10., 20.]);
        let linear_combined = linear_combination::<f32>(&[v1, v2, v3], &[1., -10., -1.]);
        assert_eq!(linear_combined.positions, &[-62., -8.]);

        let v1 = Vector::from(&[-42., 100., -69.5]);
        let v2 = Vector::from(&[1., 3., 5.]);
        let linear_combined = linear_combination::<f32>(&[v1, v2], &[1., -10.]);
        assert_eq!(linear_combined.positions, &[-52., 70., -119.5]);
    }
}
