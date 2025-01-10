use anyhow::anyhow;

use crate::resp;

//#[cfg(test)]
//pub mod tests;

#[derive(Debug, PartialEq, Eq)]
pub struct OutgoingHandshake {
    advances: usize,
}

impl OutgoingHandshake {
    #[must_use]
    pub fn new() -> Self {
        Self { advances: 0 }
    }

    #[must_use]
    pub fn is_finished(&self) -> bool {
        self.advances >= 5
    }

    pub fn try_advance(
        &mut self,
        response: &Option<resp::Value>,
    ) -> anyhow::Result<Option<resp::Value>> {
        todo!()
        //let result = match (self.advances, response) {
        //    (0, None) => Ok(Some(Input::Ping)),
        //    (1, Some(Output::Pong)) => Ok(Some(Input::ReplConf(ReplConf::ListingPort(1)))),
        //    (2, Some(Output::ReplConf(ReplConf::Ok))) => {
        //        Ok(Some(ReplConf::Capa(String::new()).into()))
        //    }
        //    (3, Some(Output::ReplConf(ReplConf::Ok))) => Ok(Some(Input::Psync)),
        //    (4, Some(Output::Psync)) => Ok(None),
        //    _ => Err(anyhow!("unepexted handshake message {response:?}")),
        //};
        //if result.is_ok() {
        //    self.advances += 1;
        //} else {
        //    self.advances = 0;
        //}
        //result
    }
}
