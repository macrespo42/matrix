use crate::Vector;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Matrix<K> {
    pub positions: Vec<Vec<K>>,
}

impl<K: std::ops::Add<Output = K> + Copy> std::ops::Add for Matrix<K> {
    type Output = Matrix<K>;

    fn add(self, other: Matrix<K>) -> Matrix<K> {
        let mut result = self;
        for (row, other_row) in result.positions.iter_mut().zip(other.positions.iter()) {
            for (column, &other_column) in row.iter_mut().zip(other_row.iter()) {
                *column = *column + other_column;
            }
        }
        result
    }
}

impl<K: std::ops::Sub<Output = K> + Copy> std::ops::Sub for Matrix<K> {
    type Output = Matrix<K>;

    fn sub(self, other: Matrix<K>) -> Matrix<K> {
        let mut result = self;
        for (row, other_row) in result.positions.iter_mut().zip(other.positions.iter()) {
            for (column, &other_column) in row.iter_mut().zip(other_row.iter()) {
                *column = *column - other_column;
            }
        }
        result
    }
}

impl<K: std::ops::Mul<f32, Output = K> + Copy> std::ops::Mul<f32> for Matrix<K> {
    type Output = Matrix<K>;

    fn mul(self, scalar: f32) -> Matrix<K> {
        let mut result = self;
        for row in result.positions.iter_mut() {
            for value in row.iter_mut() {
                *value = *value * scalar;
            }
        }
        result
    }
}

impl<
        K: Copy
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>
            + std::cmp::PartialOrd,
    > Matrix<K>
{
    fn column_size(&self) -> usize {
        self.positions[0].len()
    }

    fn row_size(&self) -> usize {
        self.positions.len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.column_size(), self.row_size())
    }

    pub fn is_square(&self) -> bool {
        self.positions.len() == self.positions[0].len()
    }

    pub fn reshape_to_vector(&self) -> Vec<K> {
        let mut returned_vector: Vec<K> = Vec::new();
        for row in self.positions.iter() {
            returned_vector.extend(row.clone());
        }
        returned_vector
    }

    pub fn from(matrix: &[&[K]]) -> Self {
        let mut positions = Vec::with_capacity(matrix.len());
        for row in matrix {
            positions.push(row.to_vec());
        }
        Matrix { positions }
    }

    fn same_size(&self, v: &Matrix<K>) {
        if self.positions.len() != v.positions.len() {
            panic!("Matrix do not have same size!");
        }
        for (index, point) in self.positions.iter().enumerate() {
            if point.len() != v.positions[index].len() {
                panic!("Matrix rows do not have same size!");
            }
        }
    }

    pub fn add(&mut self, v: &Matrix<K>) {
        self.same_size(v);
        for (row_index, row) in v.positions.iter().enumerate() {
            for (col_index, column) in row.iter().enumerate() {
                self.positions[row_index][col_index] =
                    self.positions[row_index][col_index] + *column;
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<K>) {
        self.same_size(v);
        for (row_index, row) in v.positions.iter().enumerate() {
            for (col_index, column) in row.iter().enumerate() {
                self.positions[row_index][col_index] =
                    self.positions[row_index][col_index] - *column;
            }
        }
    }

    pub fn scl(&mut self, a: K) {
        for row in self.positions.iter_mut() {
            for column in row.iter_mut() {
                *column = *column * a;
            }
        }
    }

    pub fn mul_vec(&mut self, vec: Vector<K>) -> Vector<K> {
        if self.column_size() != vec.size() {
            panic!("The number of columns in this Matrix must equals the number of rows in vec");
        }
        let mut result: Vector<K> = Vector::from(&[]);

        for row in self.positions.iter() {
            let row_to_vector: Vector<K> = Vector::from(row);
            result.positions.push(row_to_vector.dot(vec.clone()));
        }
        result
    }

    pub fn mul_mat(&mut self, mat: Matrix<K>) -> Matrix<K> {
        if self.row_size() != mat.column_size() {
            println!(
                "self row: {} | mat column: {}",
                self.row_size(),
                mat.column_size()
            );
            panic!("The number of columns in this Matrix must equals the number of rows in mat");
        }
        let mut mat_rotated: Matrix<K> = Matrix::from(&[]);
        for column in 0..mat.column_size() {
            let mut mat_rotated_row: Vec<K> = Vec::new();
            for row in 0..mat.row_size() {
                mat_rotated_row.push(mat.clone().positions[row][column]);
            }
            mat_rotated.positions.push(mat_rotated_row);
        }

        let mut result: Matrix<K> = Matrix::from(&[]);
        for row in self.clone().positions {
            let mut result_row: Vec<K> = Vec::new();
            for rotated_mat_row in mat_rotated.clone().positions {
                let rotated_mat_to_vec: Vector<K> = Vector::from(&rotated_mat_row);
                let product: K = Vector::from(&row).dot(rotated_mat_to_vec);
                result_row.push(product);
            }
            result.positions.push(result_row);
        }
        result
    }

    pub fn trace(&mut self) -> K {
        if self.shape() == (0, 0) {
            panic!("Can't trace an empty matrix");
        }
        let mut result: K = self.positions[0][0];
        let mut col_index: usize = 1;
        let mut row_index: usize = 1;
        while col_index < self.column_size() && row_index < self.row_size() {
            result = result + self.positions[row_index][col_index];
            col_index += 1;
            row_index += 1;
        }

        result
    }

    pub fn transpose(&mut self) -> Matrix<K> {
        let mut mat_rotated: Matrix<K> = Matrix::from(&[]);
        for column in 0..self.column_size() {
            let mut mat_rotated_row: Vec<K> = Vec::new();
            for row in 0..self.row_size() {
                mat_rotated_row.push(self.clone().positions[row][column]);
            }
            mat_rotated.positions.push(mat_rotated_row);
        }
        mat_rotated
    }

    fn find_pivot(&mut self, row: usize, column: usize) -> (K, usize)
    where
        K: PartialEq + Default,
    {
        let zero: K = K::default();
        let mut max: K = self.positions[row][column];
        let mut max_row: usize = row;
        for row_index in row..self.row_size() {
            if self.positions[row_index][column] != zero && self.positions[row_index][column] > max
            {
                max = self.positions[row_index][column];
                max_row = row_index;
            }
        }
        (max, max_row)
    }

    pub fn row_echelon(&mut self) -> Matrix<K>
    where
        K: PartialEq
            + Default
            + std::ops::Div<Output = K>
            + std::ops::Neg<Output = K>
            + Copy
            + std::fmt::Display,
    {
        let mut row_echelon_form: Matrix<K> = self.clone();
        let zero = K::default();

        let mut row_index: usize = 0;
        let mut column_index: usize = 0;

        while column_index < row_echelon_form.column_size()
            && row_index < row_echelon_form.row_size()
        {
            let (pivot, pivot_row) = row_echelon_form.find_pivot(row_index, column_index);

            if pivot != zero {
                for i in 0..row_echelon_form.column_size() {
                    row_echelon_form.positions[pivot_row][i] =
                        row_echelon_form.positions[pivot_row][i] / pivot;
                }
            }

            if pivot_row != row_index {
                row_echelon_form.positions.swap(row_index, pivot_row);
            }

            row_index += 1;

            for scaled_row_index in row_index..row_echelon_form.row_size() {
                let mut scaled_pivot: Vector<K> =
                    Vector::from(&row_echelon_form.clone().positions[row_index - 1]);

                scaled_pivot
                    .scl(row_echelon_form.clone().positions[scaled_row_index][column_index]);

                let mut scaled_row: Vector<K> =
                    Vector::from(&row_echelon_form.clone().positions[scaled_row_index]);
                scaled_row.sub(&scaled_pivot.clone());

                row_echelon_form.positions[scaled_row_index] = scaled_row.clone().positions;
            }
            column_index += 1;
        }
        row_echelon_form
    }

    fn determinant_2(&mut self) -> K {
        return (self.positions[0][0] * self.positions[1][1])
            - (self.positions[0][1] * self.positions[1][0]);
    }

    fn determinant_3(&mut self) -> K {
        let a: K = self.positions[0][0]
            * Matrix::from(&[
                &[self.positions[1][1], self.positions[1][2]],
                &[self.positions[2][1], self.positions[2][2]],
            ])
            .determinant_2();
        let b: K = self.positions[0][1]
            * Matrix::from(&[
                &[self.positions[1][0], self.positions[1][2]],
                &[self.positions[2][0], self.positions[2][2]],
            ])
            .determinant_2();
        let c: K = self.positions[0][2]
            * Matrix::from(&[
                &[self.positions[1][0], self.positions[1][1]],
                &[self.positions[2][0], self.positions[2][1]],
            ])
            .determinant_2();
        return a - b + c;
    }

    fn determinant_4(&mut self) -> K {
        let a: K = self.positions[0][0]
            * Matrix::from(&[
                &[
                    self.positions[1][1],
                    self.positions[1][2],
                    self.positions[1][3],
                ],
                &[
                    self.positions[2][1],
                    self.positions[2][2],
                    self.positions[2][3],
                ],
                &[
                    self.positions[3][1],
                    self.positions[3][2],
                    self.positions[3][3],
                ],
            ])
            .determinant_3();

        let b: K = self.positions[0][1]
            * Matrix::from(&[
                &[
                    self.positions[1][0],
                    self.positions[1][2],
                    self.positions[1][3],
                ],
                &[
                    self.positions[2][0],
                    self.positions[2][2],
                    self.positions[2][3],
                ],
                &[
                    self.positions[3][0],
                    self.positions[3][2],
                    self.positions[3][3],
                ],
            ])
            .determinant_3();

        let c: K = self.positions[0][2]
            * Matrix::from(&[
                &[
                    self.positions[1][0],
                    self.positions[1][1],
                    self.positions[1][3],
                ],
                &[
                    self.positions[2][0],
                    self.positions[2][1],
                    self.positions[2][3],
                ],
                &[
                    self.positions[3][0],
                    self.positions[3][1],
                    self.positions[3][3],
                ],
            ])
            .determinant_3();

        let d: K = self.positions[0][3]
            * Matrix::from(&[
                &[
                    self.positions[1][0],
                    self.positions[1][1],
                    self.positions[1][2],
                ],
                &[
                    self.positions[2][0],
                    self.positions[2][1],
                    self.positions[2][2],
                ],
                &[
                    self.positions[3][0],
                    self.positions[3][1],
                    self.positions[3][2],
                ],
            ])
            .determinant_3();
        return a - b + c - d;
    }

    // fn null_determinant(&mut self) -> bool
    // where
    //     K: Default,
    // {
    //     for (index, row) in self.positions.iter().enumerate() {
    //         let first = self.positions[index][0];
    //         // check if a row is filled with zero
    //         if row
    //             .iter()
    //             .all(|&item| item == first && item == K::default())
    //         {
    //             return true;
    //         }
    //         // check if row in double
    //         for row_index in index..self.row_size() {
    //             if row_index != index && self.positions[row_index] == *row {
    //                 return true;
    //             }
    //             // check if row proportional
    //         }
    //     }
    //     //columns in double
    //     //if columns proportional
    //     //if a column is filled is zero
    //     false
    // }

    pub fn determinant(&mut self) -> K
    where
        K: Default,
    {
        if !self.is_square() {
            panic!("The matrix must be square to compute is determinant");
        }

        if self.row_size() == 0 || self.column_size() == 0 {
            return K::default();
        }

        if self.row_size() == 2 {
            return self.determinant_2();
        } else if self.row_size() == 3 {
            return self.determinant_3();
        } else if self.row_size() == 4 {
            return self.determinant_4();
        } else {
            panic!("Matrix determinant are available only for matrix of n <= 4");
        }
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[").expect("can't write in stdout");
        for row in self.positions.iter() {
            write!(f, "[").expect("can't write in stdout");
            for (index, point) in row.iter().enumerate() {
                write!(f, "{}", point).expect("can't write in stdout");
                if index < row.len() - 1 {
                    write!(f, ",").expect("can't write in stdout");
                }
            }
            writeln!(f, "]").expect("can't write in stdout");
        }
        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_add() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
        u.add(&v);
        assert_eq!(Vec::from([8.0, 6.0]), u.positions[0]);
        assert_eq!(Vec::from([1.0, 6.0]), u.positions[1]);

        let mut u = Matrix::from(&[&[0, 0], &[0, 0]]);
        let v = Matrix::from(&[&[0, 0], &[0, 0]]);
        u.add(&v);
        assert_eq!(Vec::from([0, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 0]), u.positions[1]);

        let mut u = Matrix::from(&[&[1, 0], &[0, 1]]);
        let v = Matrix::from(&[&[0, 0], &[0, 0]]);
        u.add(&v);
        assert_eq!(Vec::from([1, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 1]), u.positions[1]);

        let mut u = Matrix::from(&[&[1, 1], &[1, 1]]);
        let v = Matrix::from(&[&[1, 1], &[1, 1]]);
        u.add(&v);
        assert_eq!(Vec::from([2, 2]), u.positions[0]);
        assert_eq!(Vec::from([2, 2]), u.positions[1]);

        let mut u = Matrix::from(&[&[21, 21], &[21, 21]]);
        let v = Matrix::from(&[&[21, 21], &[21, 21]]);
        u.add(&v);
        assert_eq!(Vec::from([42, 42]), u.positions[0]);
        assert_eq!(Vec::from([42, 42]), u.positions[1]);
    }

    #[test]
    fn matrix_sub() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        let v = Matrix::from(&[&[7., 4.], &[-2., 2.]]);
        u.sub(&v);
        assert_eq!(Vec::from([-6.0, -2.0]), u.positions[0]);
        assert_eq!(Vec::from([5.0, 2.0]), u.positions[1]);

        let mut u = Matrix::from(&[&[0, 0], &[0, 0]]);
        let v = Matrix::from(&[&[0, 0], &[0, 0]]);
        u.sub(&v);
        assert_eq!(Vec::from([0, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 0]), u.positions[1]);

        let mut u = Matrix::from(&[&[1, 0], &[0, 1]]);
        let v = Matrix::from(&[&[0, 0], &[0, 0]]);
        u.sub(&v);
        assert_eq!(Vec::from([1, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 1]), u.positions[1]);

        let mut u = Matrix::from(&[&[1, 1], &[1, 1]]);
        let v = Matrix::from(&[&[1, 1], &[1, 1]]);
        u.sub(&v);
        assert_eq!(Vec::from([0, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 0]), u.positions[1]);

        let mut u = Matrix::from(&[&[21, 21], &[21, 21]]);
        let v = Matrix::from(&[&[21, 21], &[21, 21]]);
        u.sub(&v);
        assert_eq!(Vec::from([0, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 0]), u.positions[1]);
    }

    #[test]
    fn matrix_scale() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        u.scl(2.);
        assert_eq!(Vec::from([2.0, 4.0]), u.positions[0]);
        assert_eq!(Vec::from([6.0, 8.0]), u.positions[1]);

        let mut u = Matrix::from(&[&[1, 0], &[0, 1]]);
        u.scl(1);
        assert_eq!(Vec::from([1, 0]), u.positions[0]);
        assert_eq!(Vec::from([0, 1]), u.positions[1]);

        let mut u = Matrix::from(&[&[1, 2], &[3, 4]]);
        u.scl(2);
        assert_eq!(Vec::from([2, 4]), u.positions[0]);
        assert_eq!(Vec::from([6, 8]), u.positions[1]);

        let mut u = Matrix::from(&[&[21., 21.], &[21., 21.]]);
        u.scl(0.5);
        assert_eq!(Vec::from([10.5, 10.5]), u.positions[0]);
        assert_eq!(Vec::from([10.5, 10.5]), u.positions[1]);
    }

    #[test]
    fn matrix_mul_vec() {
        let mut u = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        let v = Vector::from(&[4., 2.]);
        let result = u.mul_vec(v);
        assert_eq!(result.positions[0], 4.);
        assert_eq!(result.positions[1], 2.);

        let mut u = Matrix::from(&[&[2., 0.], &[0., 2.]]);
        let v = Vector::from(&[4., 2.]);
        let result = u.mul_vec(v);
        assert_eq!(result.positions[0], 8.);
        assert_eq!(result.positions[1], 4.);

        let mut u = Matrix::from(&[&[2., -2.], &[-2., 2.]]);
        let v = Vector::from(&[4., 2.]);
        let result = u.mul_vec(v);
        assert_eq!(result.positions[0], 4.);
        assert_eq!(result.positions[1], -4.);
    }

    #[test]
    fn matrix_mul_mat() {
        let mut u = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        let v = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        let result = u.mul_mat(v);
        assert_eq!(result.positions[0], Vec::from([1., 0.]));
        assert_eq!(result.positions[1], Vec::from([0., 1.]));

        let mut u = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        let v = Matrix::from(&[&[2., 1.], &[4., 2.]]);
        let result = u.mul_mat(v);
        assert_eq!(result.positions[0], Vec::from([2., 1.]));
        assert_eq!(result.positions[1], Vec::from([4., 2.]));

        let mut u = Matrix::from(&[&[3., -5.], &[6., 8.]]);
        let v = Matrix::from(&[&[2., 1.], &[4., 2.]]);
        let result = u.mul_mat(v);
        assert_eq!(result.positions[0], Vec::from([-14., -7.]));
        assert_eq!(result.positions[1], Vec::from([44., 22.]));

        let mut u = Matrix::from(&[&[0., 4., -2.], &[-4., -3., 0.]]);
        let v = Matrix::from(&[&[0., 1.], &[1., -1.], &[2., 3.]]);
        let result = u.mul_mat(v);
        assert_eq!(result.positions[0], Vec::from([0., -10.]));
        assert_eq!(result.positions[1], Vec::from([-3., -1.]));
    }

    #[test]
    fn matrix_trace_with_zero() {
        let mut u = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        assert_eq!(u.trace(), 2.0);
    }

    #[test]
    fn matrix_trace_positive() {
        let mut u = Matrix::from(&[&[2., -5., 0.], &[4., 3., 7.], &[-2., 3., 4.]]);
        assert_eq!(u.trace(), 9.0);
    }

    #[test]
    fn matrix_trace_negative() {
        let mut u = Matrix::from(&[&[-2., -8., 4.], &[1., -23., 4.], &[0., 6., 4.]]);
        assert_eq!(u.trace(), -21.0);
    }

    #[test]
    fn matrix_transpose_zero() {
        let mut u = Matrix::from(&[&[0., 0.], &[0., 0.]]);
        let result = u.transpose();
        assert_eq!(result.positions[0], Vec::from([0., 0.]));
        assert_eq!(result.positions[1], Vec::from([0., 0.]));
    }

    #[test]
    fn matrix_transpose_no_change() {
        let mut u = Matrix::from(&[&[1., 0.], &[0., 1.]]);
        let result = u.transpose();
        assert_eq!(result.positions[0], Vec::from([1., 0.]));
        assert_eq!(result.positions[1], Vec::from([0., 1.]));
    }

    #[test]
    fn matrix_transpose_reverse() {
        let mut u = Matrix::from(&[&[1., 2.], &[3., 4.]]);
        let result = u.transpose();
        assert_eq!(result.positions[0], Vec::from([1., 3.]));
        assert_eq!(result.positions[1], Vec::from([2., 4.]));
    }

    #[test]
    fn matrix_transpose_bin_no_change() {
        let mut u = Matrix::from(&[&[1., 0., 0.], &[0., 1., 0.], &[0., 0., 1.]]);
        let result = u.transpose();
        assert_eq!(result.positions[0], Vec::from([1., 0., 0.]));
        assert_eq!(result.positions[1], Vec::from([0., 1., 0.]));
        assert_eq!(result.positions[2], Vec::from([0., 0., 1.]));
    }

    #[test]
    fn matrix_transpose_positive() {
        let mut u = Matrix::from(&[&[1., 2., 3.], &[4., 5., 6.]]);
        let result = u.transpose();
        assert_eq!(result.positions[0], Vec::from([1., 4.]));
        assert_eq!(result.positions[1], Vec::from([2., 5.]));
        assert_eq!(result.positions[2], Vec::from([3., 6.]));
    }

    #[test]
    fn matrix_rref() {
        let mut u = Matrix::from(&[&[1, -1, 2], &[3, 2, 1], &[2, -3, -2]]);
        let result = u.row_echelon();
        assert_eq!(result.positions[0], Vec::from([1, 0, 0]));
        assert_eq!(result.positions[1], Vec::from([0, 1, -2]));
        assert_eq!(result.positions[2], Vec::from([0, 0, 1]));

        let mut u = Matrix::from(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]]);
        let result = u.row_echelon();
        assert_eq!(result.positions[0], Vec::from([1, 0, 0]));
        assert_eq!(result.positions[1], Vec::from([0, 1, 0]));
        assert_eq!(result.positions[2], Vec::from([0, 0, 1]));

        let mut u = Matrix::from(&[&[1, 2], &[3, 4]]);
        let result = u.row_echelon();
        assert_eq!(result.positions[0], Vec::from([1, 1]));
        assert_eq!(result.positions[1], Vec::from([0, 1]));

        let mut u = Matrix::from(&[&[1, 2], &[2, 4]]);
        let result = u.row_echelon();
        assert_eq!(result.positions[0], Vec::from([1, 2]));
        assert_eq!(result.positions[1], Vec::from([0, 0]));
    }

    #[test]
    fn matrix_determinant_2() {
        let mut u = Matrix::from(&[&[1., -1.], &[-1., 1.]]);
        assert_eq!(u.determinant(), 0.);

        let mut u = Matrix::from(&[&[0, 0], &[0, 0]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[1, 0], &[0, 1]]);
        assert_eq!(u.determinant(), 1);

        let mut u = Matrix::from(&[&[2, 0], &[0, 2]]);
        assert_eq!(u.determinant(), 4);

        let mut u = Matrix::from(&[&[1, 1], &[1, 1]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[0, 1], &[1, 0]]);
        assert_eq!(u.determinant(), -1);

        let mut u = Matrix::from(&[&[1, 2], &[3, 4]]);
        assert_eq!(u.determinant(), -2);

        let mut u = Matrix::from(&[&[-7, 5], &[4, 6]]);
        assert_eq!(u.determinant(), -62);
    }

    #[test]
    fn matrix_determinant_3() {
        let mut u = Matrix::from(&[&[2., 0., 0.], &[0., 2., 0.], &[0., 0., 2.]]);
        assert_eq!(u.determinant(), 8.);

        let mut u = Matrix::from(&[&[4, 2, 5], &[1, 8, 9], &[2, 7, 3]]);
        assert_eq!(u.determinant(), -171);

        let mut u = Matrix::from(&[&[1, 0, 0], &[0, 1, 0], &[0, 0, 1]]);
        assert_eq!(u.determinant(), 1);
    }

    #[test]
    fn matrix_determinant_4() {
        let mut u = Matrix::from(&[
            &[8., 5., -2., 4.],
            &[4., 2.5, 20., 4.],
            &[8., 5., 1., 4.],
            &[28., -4., 17., 1.],
        ]);

        assert_eq!(u.determinant(), 1032.);

        let mut u = Matrix::from(&[
            &[1, 1, 1, -1],
            &[1, 1, -1, 1],
            &[1, -1, 1, 1],
            &[-1, 1, 1, 1],
        ]);

        assert_eq!(u.determinant(), -16);
    }

    #[test]
    fn matrix_determinant_0() {
        let mut u = Matrix::from(&[&[1, 4, 2], &[1, 4, 2], &[3, 9, 5]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[1, 4, 2], &[0, 0, 0], &[3, 9, 5]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[1, 4, 2], &[3, 9, 5], &[3, 9, 5]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[1, 4, 2], &[2, 8, 4], &[3, 9, 5]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[4, 4, 2], &[2, 8, 1], &[6, 12, 3]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[4, 4], &[4, 4]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[1, 2], &[2, 4]]);
        assert_eq!(u.determinant(), 0);

        let mut u = Matrix::from(&[&[1, 2], &[0, 0]]);
        assert_eq!(u.determinant(), 0);
    }
}
