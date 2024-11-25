pub struct Follower;

impl Follower {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn handle_set<N>(&self, key: String, value: String, node: N) -> Result<(), ()> {
        todo!()
    }

    fn handle_wait<N>(&self, node: N) {
        todo!()
    }
}
