use klh_core::klh::{Klh, KlhClient};
use klh_core::messaging::{Request, MessageType};
use klh_core::plugins::buffers::models::ListBuffersResponse;
use klh_core::plugins::{diagnostics, buffers};
use std::{io, fs};

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
	    let bad_query = Request::from_message_type(MessageType::query_from_str("NoSuchId"));
	    client.send(bad_query).await.unwrap();
	  },
	  "bad_command" => {
	    println!("Sending bogus command");
	    let bad_command = Request::from_message_type(MessageType::command_from_str("NoSuchId"));
	    client.send(bad_command).await.unwrap();
	  }
	  "dl" => {
	    println!("Sending a diagnostics log");
	    let diagnostics_request = diagnostics::requests::new_log_event();
	    client.send(diagnostics_request).await.unwrap();
	  },
	  "db" => {
	    let mut thread_client = client.clone();
	    tokio::spawn(async move {
	      println!("Sending a slow bomb");
	      let mut diagnostics_request = diagnostics::requests::new_slow_bomb(10);
	      let mut slow_bomb_handler = diagnostics_request.get_handler().unwrap();
	      thread_client.send(diagnostics_request).await.unwrap();
	      match slow_bomb_handler.handle_response().await {
		Err(msg) => println!("Problem handling slow bomb response: {:?}", &msg),
		Ok(_) => {
		  println!("Slow bomb responded!")
		}
	      };
	    });
	  }
	  "bc" => {
	    println!("Creating a buffer");
	    let create_buffer_request = buffers::requests::new_create_buffer_request("special_buffer");
	    client.send(create_buffer_request).await.unwrap();
	  },
	  "bl" => {
	    println!("Asking for a buffers list");

	    let mut list_buffer_request = buffers::requests::new_list_buffers_request();
	    let mut list_buffer_handler = list_buffer_request.get_handler().unwrap();

	    client.send(list_buffer_request).await.unwrap();

	    match list_buffer_handler.handle_response().await {
	      Ok(mut response) => {
		println!("Buffer plugin responded");
		let list_buffers_response : ListBuffersResponse = response.deserialize()
		  .expect("Should have a list buffers response");
		println!("Active buffers: {}", list_buffers_response.list_as_string);
	      },
	      Err(msg) => println!("Sender dropped probably: {:?}", &msg),
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
  // Set up some logging.
  simplelog::WriteLogger::init(
    simplelog::LevelFilter::Debug,
    simplelog::Config::default(),
    fs::File::create("klh.log").unwrap(),
  ).unwrap();
  
  let mut klh = Klh::new();

  klh.start().await;

  let client : KlhClient = klh.get_client();

  prompt_and_read(
    client,
  ).await;
}




