use crate::{client_proxy::ClientProxy, commands::*};

pub struct WhoamiCommand {}

impl Command for WhoamiCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["whoami", "i"]
    }
    fn get_description(&self) -> &'static str {
        "Who am i"
    }
    fn execute(&self, _client: &mut ClientProxy, _params: &[&str]) {
        println!("IT\'S MEEE");
        return;
    }
}
