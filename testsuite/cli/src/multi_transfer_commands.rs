use std::thread;
use std::time::{Duration, Instant, SystemTime};
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
        if is_blocking {
            for _i in 1..num_iter{
                println!("Timestamp: {:?}", SystemTime::now().duration_since(SystemTime::UNIX_EPOCH));
                let start = Instant::now();
                match client.transfer_coins(&["transfer", &params[1], &params[2], "1", &params[3]], is_blocking){
                    Ok(_k) => (),
                    Err(_e) => println!("Error submitting blocking transaction"),
                }
                let duration = start.elapsed();
                println!("Time elapsed is: {:?}", duration);
            }
        } else{
            let start = Instant::now();
            for _i in 1..num_iter{
                match client.transfer_coins(&["transfer", &params[1], &params[2], "1", &params[3]], is_blocking){
                    Ok(_k) => (),
                    Err(_e) => println!("Error submitting non-blocking transaction"),
                }
                thread::sleep(Duration::from_millis(10));
            }
            let duration = start.elapsed();
            println!("Time elapsed is: {:?}", duration);
        }
        return;
    }
}

