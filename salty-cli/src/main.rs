use salty::{vault_client::VaultClient, CommandRequest};
use salty_utils::vault::commands::{Cmd, CmdErrorCode, CmdResponse, Parser};
use serde_json::{self};
pub mod salty {
    tonic::include_proto!("salty");
}

#[tokio::main]
async fn main() {
    let args = Cmd::parse();
    let result = VaultClient::connect("http://[::1]:50051").await;
    let mut client = match result {
        Ok(client) => client,
        Err(_) => {
            eprintln!("Error: Failed connecting to server!");
            return;
        }
    };

    let request = tonic::Request::new(CommandRequest {
        command: serde_json::to_string(&args).unwrap(),
    });

    let result = client.process_cmd(request).await;
    let response = match result {
        Ok(response) => response,
        Err(e) => {
            eprintln!("Error: {}, message: {}", e.code(), e.message().to_string());
            return;
        }
    };
    let rp: CmdResponse = serde_json::from_str(&response.get_ref().message).unwrap();

    match rp.result {
        CmdErrorCode::Ok => println!("{}", rp.message),
        _ => {
            eprintln!("Error: {}", rp.message);
            return;
        }
    };
}
