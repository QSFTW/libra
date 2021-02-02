use crate::{client_proxy::ClientProxy, commands::*};
use std::thread;
use std::time::{Duration, SystemTime};

pub struct SequenceQueryCommand {}

impl Command for SequenceQueryCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["sequence-query", "sq"]
    }
    fn get_description(&self) -> &'static str {
        "Query the sequence number of an account periodically in a loop"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        loop {
            match client.get_sequence_number(&params) {
                Ok(sn) => println!("Timestamp is : {:?} Sequence number is: {}", 
                                    SystemTime::now().duration_since(SystemTime::UNIX_EPOCH),
                                     sn),
                Err(e) => report_error("Error getting sequence number", e),
            }
            thread::sleep(Duration::from_millis(1000));
        }
        return;
    }
}
