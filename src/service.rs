pub trait Service<Req> {
    type Response;

    fn call(&mut self, request: Req) -> anyhow::Result<Self::Response>;
}
