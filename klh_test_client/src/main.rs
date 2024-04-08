use klh_core::klh::{Klh, KlhClient};
use klh_core::messaging::{Request, MessageType};
use klh_core::plugins::buffers::models::ListBuffersResponse;
use klh_core::plugins::display::models::ListWindowsResponse;
use klh_core::plugins::{diagnostics, buffers, display};
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
	    let bad_query = Request::from_message_type(
	      MessageType::query_from_str("NoSuchId").unwrap()
	    );
	    client.send(bad_query).await.unwrap();
	  },
	  "bad_command" => {
	    println!("Sending bogus command");
	    let bad_command = Request::from_message_type(
	      MessageType::command_from_str("NoSuchId").unwrap()
	    );
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
		let buffer_list = list_buffers_response.buffer_names
		  .iter()
		  .fold("".to_string(), |acc, name| {
		    acc + name + " "
		  });
		println!("Active buffers: {}", buffer_list);
	      },
	      Err(msg) => println!("Sender dropped probably: {:?}", &msg),
	    };
	  },
	  "wc" => {
	    println!("Creating a window buffer");
	    let create_window_request = display::requests::new_create_window_request("window_name".to_string());
	    client.send(create_window_request).await.unwrap()
	  },
	  "wl" => {
	    println!("Listing windows");
	    let mut list_window_request = display::requests::new_list_windows_request();
	    let mut list_window_handler = list_window_request.get_handler().unwrap();

	    client.send(list_window_request).await.unwrap();

	    match list_window_handler.handle_response().await {
	      Ok(mut response) => {
		println!("Display plugin responded");
		let list_window_response : ListWindowsResponse = response.deserialize()
		  .expect("Should have a list windows response");
		let windows_as_string = list_window_response.window_names
		  .iter()
		  .fold("".to_string(), |acc, name| {
		    acc + name + " "
		  });
		println!("Active windows: {}", windows_as_string);
	      },
	      Err(msg) => println!("Sender dropped probably: {:?}", &msg),
	    }
	  }
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




