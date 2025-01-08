use std::collections::HashMap;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Stream {
    indexes: HashMap<String, usize>,
    values: Vec<String>,
    i: usize,
}

impl Stream {
    #[must_use]
    pub fn new() -> Self {
        Self {
            indexes: HashMap::new(),
            values: Vec::new(),
            i: 0,
        }
    }
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.indexes.is_empty()
    }
    pub fn add_default_key(&mut self, key: impl ToString, value: impl ToString) -> String {
        self.i += 1;
        self.indexes.insert(self.i.to_string(), self.values.len());
        self.values.push(value.to_string());
        format!("{}", self.i)
    }

    pub fn read(&self, key: impl ToString, count: usize) -> Vec<String> {
        let start = self.get_index_or_closests_later(&key.to_string());
        self.values[start..].to_vec()
    }

    #[must_use]
    pub fn read_last(&self) -> Option<String> {
        self.values.last().cloned()
    }

    pub fn range(&self, start: impl ToString, end: impl ToString) -> Vec<String> {
        dbg!(start.to_string(), end.to_string());
        let start = self.get_index_or_closests_later(&start.to_string());
        dbg!(start,);
        let end = self.get_index_or_closest_earlier(&end.to_string());
        dbg!(end);
        dbg!(&self.indexes);
        self.values[start..end].to_vec()
    }

    fn get_index_or_closests_later(&self, key: &str) -> usize {
        if let Some(i) = self.indexes.get(key) {
            return *i;
        }
        let mut best = None::<usize>;
        let key = key.parse::<u64>().unwrap();
        for n in self.indexes.keys() {
            let n = n.parse::<u64>().unwrap();
            if key < n {
                best = Some(best.map_or(n as usize, |best| best.min(n as usize)));
            }
        }
        best.map_or(usize::MAX.min(self.values.len()), |n| n.saturating_sub(1))
    }
    fn get_index_or_closest_earlier(&self, key: &str) -> usize {
        if let Some(i) = self.indexes.get(key) {
            return *i + 1;
        }
        let mut best = None::<usize>;
        let key = key.parse::<u64>().unwrap();
        for n in self.indexes.keys() {
            let n = n.parse::<u64>().unwrap();
            if key > n {
                best = best.map(|best| best.max(n as usize));
            }
        }
        best.unwrap_or(usize::MAX.min(self.values.len()))
    }
}
