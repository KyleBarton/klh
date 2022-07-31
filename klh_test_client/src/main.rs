use klh_core::klh::{Klh, KlhClient};
use klh_core::event::{Event, CommandData};
use std::io;

/* WOOHOO, we have a first client here. I think we have a couple of learnings to take away:

- Remember that it doesn't matter if this client is messy. It's just a way to exercise klh_core. Use this to drive quality in the other crate
- We need to figure out the 'Command' module, or whatever you want to call it, sooner rather than later. SessionInput/DispatchInput is awful
- Let's add some interactivity to this client so we can send a bunch of messages at once.
 */

async fn prompt_and_read(
  mut client: KlhClient,
  known_event: Event,
  bad_event: Event,
  expensive_event: Event,
) {
  // Let's see if the readline helps the race condition.


  loop {

    let mut input: String = String::new();
    println!("Enter a 1 or a 2 or a 3");

    match io::stdin().read_line(&mut input) {
      Ok(_n) => {
	match input.as_str().trim() {
	  "1" => {
	    println!("Sending known message");
	    client.send(known_event.clone()).await.unwrap();
	  },
	  "2" => {
	    println!("Sending bogus message");
	    client.send(bad_event.clone()).await.unwrap();
	  },
	  "3" => {
	    println!("Sending a slow-bomb");
	    client.send(expensive_event.clone()).await.unwrap();
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


  let diagnostics_command = Event::Command {
    id: String::from("diagnostics::log_event"),
    data: CommandData {
        docs: String::from("This is the details of my log event"),
    }
  };

  let expensive_command = Event::Command {
    id: String::from("diagnostics::slow_bomb"),
    data: CommandData {
      // TODO This means we should change "docs" to "json" and make
      // docs invariant with Id
      docs: String::from("{time_seconds: 10}"),
    }
  };

  let unknown_event = Event::Command {
    id: String::from("unknown world"),
    data: CommandData { docs: "No docs".to_string() }
  };

  prompt_and_read(
    client,
    diagnostics_command,
    unknown_event,
    expensive_command).await;
}




