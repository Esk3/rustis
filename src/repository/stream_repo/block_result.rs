#[derive(Debug)]
pub enum BlockResult<T> {
    Found(T),
    NotFound,
    Err(anyhow::Error),
}

impl<T: Eq> Eq for BlockResult<T> {}

impl<T: PartialEq> PartialEq for BlockResult<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Found(l0), Self::Found(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<T> BlockResult<T> {
    pub fn is_not_found(&self) -> bool {
        matches!(self, Self::NotFound)
    }
}
