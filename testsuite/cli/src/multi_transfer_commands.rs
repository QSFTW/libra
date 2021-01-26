use crate::{client_proxy::ClientProxy, commands::*};

pub struct MultiTransferCommand {}

impl Command for MultiTransferCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["multi-transfer","multi-transferb", "mt", "mtb"]
    }
    fn get_description(&self) -> &'static str {
        "execute multiple number of transaction with one command"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        let is_blocking = blocking_cmd(&params[0]);
        let num_iter: i32 = params[4].parse().unwrap();
        for i in 1..num_iter{
            client.transfer_coins(&["transfer", &params[1], &params[2], "1", &params[3]], is_blocking);
        }
        return;
    }
}

