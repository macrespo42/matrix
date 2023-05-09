use std::fmt;

pub struct Vector<K> {
    pub positions: Vec<K>,
}

impl<K: fmt::Display> fmt::Display for Vector<K> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[").expect("can't write in stdout");
        for (i, pos) in self.positions.iter().enumerate() {
            if i != 0 {
                write!(f, ", ",).expect("can't write in stdout");
            }
            write!(f, "{}", pos).expect("can't write in stdout");
        }
        write!(f, "]")
    }
}

impl<K> Vector<K> {
    pub fn size(&self) -> usize {
        self.positions.len()
    }
}
