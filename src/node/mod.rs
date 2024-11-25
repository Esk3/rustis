pub struct Node;
impl Node {
    #[must_use]
    pub fn get<'a>(
        key: String,
        repo: &'a mut super::repository::Repository,
        timestamp: &std::time::SystemTime,
    ) -> Option<&'a String> {
        repo.get(key, timestamp)
    }

    pub fn set(
        key: String,
        value: String,
        repo: &mut super::repository::Repository,
        expiry: Option<std::time::Duration>,
        timestamp: &std::time::SystemTime,
    ) -> Option<String> {
        repo.set(key, value, expiry, timestamp)
    }
}
