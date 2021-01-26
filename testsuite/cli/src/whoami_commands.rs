use crate::{client_proxy::ClientProxy, commands::*};

pub struct WhoamiCommand {}

impl Command for WhoamiCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["whoami", "i"]
    }
    fn get_description(&self) -> &'static str {
        "Who am i"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        println!("IT\'S MEEE");
        return;
    }
}
