use crate::types::Vector;

pub fn cross_product<K>(u: &Vector<K>, v: &Vector<K>) -> Vector<K>
where
    K: Copy + std::ops::Mul<Output = K> + std::ops::Sub<Output = K> + std::ops::Add<Output = K>,
{
    let x = u.positions[1] * v.positions[2] - u.positions[2] * v.positions[1];
    let y = u.positions[2] * v.positions[0] - u.positions[0] * v.positions[2];
    let z = u.positions[0] * v.positions[1] - u.positions[1] * v.positions[0];

    Vector::from(&[x, y, z])
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn cross_product_null() {
        let u = Vector::from(&[0., 0., 0.]);
        let v = Vector::from(&[0., 0., 0.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([0., 0., 0.]));

        let u = Vector::from(&[1., 0., 0.]);
        let v = Vector::from(&[0., 0., 0.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([0., 0., 0.]));

        let u = Vector::from(&[1., 1., 1.]);
        let v = Vector::from(&[0., 0., 0.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([0., 0., 0.]));

        let u = Vector::from(&[1., 1., 1.]);
        let v = Vector::from(&[1., 1., 1.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([0., 0., 0.]));
    }

    #[test]
    fn cross_product_bool() {
        let u = Vector::from(&[0., 0., 1.]);
        let v = Vector::from(&[1., 0., 0.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([0., 1., 0.]));

        let u = Vector::from(&[1., 0., 0.]);
        let v = Vector::from(&[0., 1., 0.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([0., 0., 1.]));
    }

    #[test]
    fn cross_product_positive() {
        let u = Vector::from(&[1., 2., 3.]);
        let v = Vector::from(&[4., 5., 6.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([-3., 6., -3.]));

        let u = Vector::from(&[8., 7., -4.]);
        let v = Vector::from(&[3., 2., 1.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([15., -20., -5.]));
    }

    #[test]
    fn cross_product_negative() {
        let u = Vector::from(&[4., 2., -3.]);
        let v = Vector::from(&[-2., -5., 16.]);
        let res = cross_product(&u, &v);

        assert_eq!(res.positions, Vec::from([17., -58., -16.]));
    }
}
