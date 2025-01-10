pub struct Echo;
impl super::Command for Echo {
    fn info(&self) -> super::CommandInfo {
        super::CommandInfo::new_name("ECHO")
    }

    fn handle(&self, input: super::super::Request) -> anyhow::Result<super::super::Response> {
        todo!()
    }
}
