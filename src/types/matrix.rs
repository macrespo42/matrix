use std::fmt;

pub struct Matrix<K> {
    pub positions: Vec<Vec<K>>,
}
impl<
        K: Copy
            + std::fmt::Display
            + std::ops::Add<Output = K>
            + std::ops::Sub<Output = K>
            + std::ops::Mul<Output = K>
            + std::fmt::Debug,
    > Matrix<K>
{
    fn column_size(&self) -> usize {
        self.positions.len()
    }

    fn row_size(&self) -> usize {
        self.positions[0].len()
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.column_size(), self.row_size())
    }

    pub fn is_square(&self) -> bool {
        self.positions.len() == self.positions[0].len()
    }

    pub fn reshape_to_vector(&self) -> Vec<&K> {
        let mut returned_vector: Vec<&K> = Vec::new();
        for row in self.positions.iter() {
            returned_vector.extend(row);
        }
        returned_vector
    }
}

impl<K: fmt::Display> fmt::Display for Matrix<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[\n").expect("can't write in stdout");
        for row in self.positions.iter() {
            write!(f, "[").expect("can't write in stdout");
            for point in row.iter() {
                write!(f, "{}", point).expect("can't write in stdout");
            }
            write!(f, "]\n").expect("can't write in stdout");
        }
        write!(f, "]")
    }
}
