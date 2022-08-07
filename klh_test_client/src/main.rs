use klh_core::klh::{Klh, KlhClient};
pub(crate) use klh_core::event::{Query, Command};
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
	    let mut bad_query = Query::from_id("NoSuchId");
	    client.send(bad_query.get_event_message().unwrap()).await.unwrap();
	  },
	  "bad_command" => {
	    println!("Sending bogus command");
	    let mut bad_command = Command::from_id("NoSuchId", "nocontent".to_string());
	    client.send(bad_command.get_event_message().unwrap()).await.unwrap();
	  }
	  "dl" => {
	    println!("Sending a diagnostics log");
	    let mut diagnostics_log_command : Command = Command::from_id(
	      "diagnostics::log_event",
	      "This is some content".to_string(),
	    );
	    client.send(diagnostics_log_command.get_event_message().unwrap()).await.unwrap();
	  },
	  "db" => {
	    println!("Sending a slow bomb");
	    let mut diagnostics_log_command = Command::from_id(
	      "diagnostics::slow_bomb",
	      // An example of what content should be doing
	      "{wait_time: 10}".to_string(),
	    );
	    client.send(diagnostics_log_command.get_event_message().unwrap()).await.unwrap();
	  }
	  "bc" => {
	    println!("Creating a buffer");
	    let mut create_buffer_command = Command::from_id(
	      "buffers::create_buffer",
	      "specialbuffer".to_string(),
	    );
	    client.send(create_buffer_command.get_event_message().unwrap()).await.unwrap();
	  },
	  "bl" => {
	    println!("Asking for a buffers list");

	    let mut list_buffer_query = Query::from_id("buffers::list_buffers");

	    let mut list_buffer_handler = list_buffer_query.get_handler().unwrap();

	    client.send(list_buffer_query.get_event_message().unwrap()).await.unwrap();

	    match list_buffer_handler.handle_response().await {
	      Ok(response) => {
		println!("Buffer plugin responded");
		println!("Active buffers: {}", response.content);
	      },
	      Err(msg) => println!("Sender dropped probably: {}", &msg),
	    };
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
  let mut klh = Klh::new();

  klh.start().await;

  let client : KlhClient = klh.get_client().unwrap();

  prompt_and_read(
    client,
  ).await;
}




