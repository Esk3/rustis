pub struct Echo;
impl super::Command for Echo {
    fn info(&self) -> super::CommandInfo {
        todo!()
    }

    fn handle(&self, input: super::Request) -> anyhow::Result<super::Response> {
        todo!()
    }
}
