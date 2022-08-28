use klh_core::klh::{Klh, KlhClient};
use klh_core::messaging::Request;
use klh_core::plugins::buffers::ListBuffersResponse;
use klh_core::plugins::{diagnostics, buffers};
use std::io;

async fn prompt_and_read(
  mut client: KlhClient,
) {
  loop {

    let mut input: String = String::new();
    println!("Enter any of the following:
bl: List Buffers
bc: Create Buffer
dl: Send a log event to diagnostics
db: Send a slow bomb to diagnostics
bad_query: Send an unknown query through the client
bad_command: Send an unknown command through the client
e: exit
    ");

    match io::stdin().read_line(&mut input) {
      Ok(_n) => {
	match input.as_str().trim() {
	  "bad_query" => {
	    println!("Sending bogus query");
	    let mut bad_query = Request::from_id("NoSuchId");
	    client.send(bad_query.to_message().unwrap()).await.unwrap();
	  },
	  "bad_command" => {
	    println!("Sending bogus command");
	    let mut bad_command = Request::from_id("NoSuchId");
	    client.send(bad_command.to_message().unwrap()).await.unwrap();
	  }
	  "dl" => {
	    println!("Sending a diagnostics log");
	    let mut diagnostics_request = diagnostics::new_log_event();
	    client.send(diagnostics_request.to_message().unwrap()).await.unwrap();
	  },
	  "db" => {
	    println!("Sending a slow bomb");
	    let mut diagnostics_request = diagnostics::new_slow_bomb();
	    client.send(diagnostics_request.to_message().unwrap()).await.unwrap();
	  }
	  "bc" => {
	    println!("Creating a buffer");
	    let mut create_buffer_request = buffers::new_create_buffer_request("special_buffer");
	    client.send(create_buffer_request.to_message().unwrap()).await.unwrap();
	  },
	  "bl" => {
	    println!("Asking for a buffers list");

	    let mut list_buffer_request = buffers::new_list_buffers_request();
	    let mut list_buffer_handler = list_buffer_request.get_handler().unwrap();

	    client.send(list_buffer_request.to_message().unwrap()).await.unwrap();

	    match list_buffer_handler.handle_response().await {
	      Ok(mut response) => {
		println!("Buffer plugin responded");
		let list_buffers_response : ListBuffersResponse = response.deserialize()
		  .expect("Should have a list buffers response");
		println!("Active buffers: {}", list_buffers_response.list_as_string);
	      },
	      Err(msg) => println!("Sender dropped probably: {}", &msg),
	    };
	  },
	  "e" => {
	    println!("e for exit");
	    break;
	  }
	  _ => {
	    println!("read the instructions dummy");
	  }
	}
      },
      Err(err) => {
	println!("Error: {err}");
	break;
      },
    }
  };
}

// What if you wanted it to actually follow the public interface
#[tokio::main]
async fn main() {
  let mut klh = Klh::new();

  klh.start().await;

  let client : KlhClient = klh.get_client().unwrap();

  prompt_and_read(
    client,
  ).await;
}




