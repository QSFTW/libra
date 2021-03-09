use crate::{client_proxy::ClientProxy, commands::*};

use std::io::prelude::*;
use std::net::TcpStream;

pub struct DiabloCommand {}

impl Command for DiabloCommand {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["diablo", "d"]
    }
    fn get_description(&self) -> &'static str {
        "Commands for diablo benchmark tests"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        let commands: Vec<Box<dyn Command>> = vec![
            Box::new(DiabloCommandConnect {}),
            Box::new(DiabloCommandCreateLocal {}),
            Box::new(DiabloCommandGetTxnByAccountSeq {}),
            Box::new(DiabloCommandMakeTransaction {}),
	    Box::new(DiabloCommandExecuteTransaction{}),
        ];
        subcommand_execute(&params[0], commands, client, &params[1..]);
    }
}

pub struct DiabloCommandConnect {}
impl Command for DiabloCommandConnect {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["connect"]
    }

    fn get_params_help(&self) -> &'static str {
        "<url_to_connect_to>"
    }

    fn get_description(&self) -> &'static str {
        "connect to diablo, set up the channel to transfer result back"
    }

    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        let stream = TcpStream::connect(&params[1]);
        let st = match stream{
            Ok(s) => s,
            Err(e) => panic!("Problem connecting: {:?}",e),
        };
        client.diablo = Some(st);
    }
}

/// Sub command to create a random local keypair and account index. This does not have any on-chain effect.
pub struct DiabloCommandCreateLocal {}

impl Command for DiabloCommandCreateLocal {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["create", "c"]
    }
    fn get_description(&self) -> &'static str {
        "Create a local account--no on-chain effect. Returns reference ID to use in other operations"
    }
    fn execute(&self, client: &mut ClientProxy, _params: &[&str]) {
        println!(">> Creating/retrieving next local account from wallet");
        match client.create_next_account(true) {
            Ok(account_data) => {println!(
                "Created/retrieved local account #{} address {}",
                account_data.index,
                hex::encode(account_data.address)
            );
            let result = format!("{:#?}", account_data);
            client.diablo.as_ref().unwrap().write(result.as_bytes());
        },
            Err(e) => report_error("Error creating local account", e),
        }
    }
}

pub struct DiabloCommandGetTxnByAccountSeq {}
impl Command for DiabloCommandGetTxnByAccountSeq {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["get-txn", "gt"]
    }
    fn get_params_help(&self) -> &'static str {
        "<account_ref_id>|<account_address> <sequence_number> <fetch_events=true|false>"
    }
    fn get_description(&self) -> &'static str {
        "Get the committed transaction by account and sequence number.  \
         Optionally also fetch events emitted by this transaction."
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        println!(">> Getting committed transaction by account and sequence number");
        match client.get_committed_txn_by_acc_seq(&params) {
            Ok(txn_view) => {
                match txn_view {
                    Some(txn_view) => {
                        let result = format!("{:#?}", txn_view);
                        client.diablo.as_ref().unwrap().write(result.as_bytes());
                    }
		            None => (),
                };
            }
            Err(e) => report_error(
                "Error getting committed transaction by account and sequence number",
                e,
            ),
        }
    }
}

pub struct DiabloCommandMakeTransaction {}
impl Command for DiabloCommandMakeTransaction {
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["make-txn", "mt"]
    }
    fn get_params_help(&self) -> &'static str {
        "<account_ref_id>|<account_address> <sequence_number> <path_to_script> [parameters]"
    }
    fn get_description(&self) -> &'static str {
        "Generate signed transaction and store it for execution later"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        client.create_signed_txn_with_sequence_number(params);
    }
}


pub struct DiabloCommandExecuteTransaction{}
impl Command for DiabloCommandExecuteTransaction{
    fn get_aliases(&self) -> Vec<&'static str> {
        vec!["execute-txn", "et"]
    }
    fn get_params_help(&self) -> &'static str {
        ""
    }
    fn get_description(&self) -> &'static str {
        "execute a transaction in client.transaction_pool"
    }
    fn execute(&self, client: &mut ClientProxy, params: &[&str]) {
        let txn =  client.transaction_pool.pop().unwrap();
        //println!("{:#?}", txn);
        let sender_ref_id = match client.get_account_ref_id(&txn.sender()){
            Ok(result) => result,
            Err(e) => return,
        };
        // TODO get the starting time 
        match client.client.submit_transaction(client.accounts.get_mut(sender_ref_id), txn){
            Ok(result) => {
                println!("Result {:#?}", result);
                client.diablo.as_ref().unwrap().write("OK".as_bytes());
            },
            Err(e) => {
                report_error("Err", e,);
                client.diablo.as_ref().unwrap().write("FAIL".as_bytes());
            },
        }
    }
}
