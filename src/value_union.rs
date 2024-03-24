#[derive(Clone, Debug)]
pub struct ValueUnion<T> {
    values: Vec<T>,
}

impl<T> ValueUnion<T> {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn push_value(&mut self, a: T) {
        self.values.push(a);
    }

    pub fn as_vec(self) -> Vec<T> {
        self.into()
    }
}

impl<T> Into<Vec<T>> for ValueUnion<T> {
    fn into(self) -> Vec<T> {
        self.values
    }
}

impl<T> From<Vec<T>> for ValueUnion<T> {
    fn from(value: Vec<T>) -> Self {
        Self { values: value }
    }
}

impl<T: Clone> From<&[T]> for ValueUnion<T> {
    fn from(value: &[T]) -> Self {
        Self { values: value.to_vec() }
    }
}

impl<T: Clone, const N: usize> From<[T; N]> for ValueUnion<T> {
    fn from(value: [T; N]) -> Self {
        Self { values: value.to_vec() }
    }
}

impl<T: PartialEq> PartialEq for ValueUnion<T> {
    fn eq(&self, other: &Self) -> bool {
        self.values.iter().any(|v| other.values.contains(v))
    }
}

impl<T: PartialEq> PartialEq<T> for ValueUnion<T> {
    fn eq(&self, other: &T) -> bool {
         self.values.contains(other)
    }
}
impl<T: Eq> Eq for ValueUnion<T> {}
