pub struct DeserializeInfo<T> {
    pub value: T,
    pub bytes_read: usize,
}

impl<T> DeserializeInfo<T> {
    pub fn new(value: T, size: usize) -> Self {
        Self {
            value,
            bytes_read: size,
        }
    }
    pub fn map_value<V, F>(self, f: F) -> DeserializeInfo<V>
    where
        F: Fn(T) -> V,
    {
        DeserializeInfo {
            value: f(self.value),
            bytes_read: self.bytes_read,
        }
    }
}

impl<T> From<(T, usize)> for DeserializeInfo<T> {
    fn from((value, size): (T, usize)) -> Self {
        Self::new(value, size)
    }
}

impl<T> From<DeserializeInfo<T>> for (T, usize) {
    fn from(value: DeserializeInfo<T>) -> Self {
        (value.value, value.bytes_read)
    }
}

impl<T> std::ops::Add<usize> for DeserializeInfo<T> {
    type Output = Self;

    fn add(mut self, rhs: usize) -> Self::Output {
        self.bytes_read += rhs;
        self
    }
}

pub trait MapValue<T, E> {
    fn map_value<F, V>(self, f: F) -> Result<DeserializeInfo<V>, E>
    where
        F: Fn(T) -> V;
}

impl<T, E> MapValue<T, E> for std::result::Result<DeserializeInfo<T>, E> {
    fn map_value<F, V>(self, f: F) -> Result<DeserializeInfo<V>, E>
    where
        F: Fn(T) -> V,
    {
        self.map(|inner| inner.map_value(f))
    }
}
