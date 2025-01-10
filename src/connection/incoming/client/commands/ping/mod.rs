pub struct Ping;
impl super::Command for Ping {
    fn info(&self) -> super::CommandInfo {
        todo!()
    }

    fn handle(&self, input: super::Request) -> anyhow::Result<super::Response> {
        todo!()
    }
}
