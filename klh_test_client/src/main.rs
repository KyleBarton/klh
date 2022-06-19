use std::thread;
use klh_core::session::{Session, SessionClient, SessionOptions};
use klh_core::event::{Event, CommandData};
use tokio::runtime::Runtime;

/* WOOHOO, we have a first client here. I think we have a couple of learnings to take away:

- Remember that it doesn't matter if this client is messy. It's just a way to exercise klh_core. Use this to drive quality in the other crate
- We need to figure out the 'Command' module, or whatever you want to call it, sooner rather than later. SessionInput/DispatchInput is awful
- Let's add some interactivity to this client so we can send a bunch of messages at once.
 */

#[tokio::main]
async fn main() {
  let session_opts: SessionOptions = SessionOptions::new();
  let mut session: Session = Session::new(session_opts);
  println!("Created session");

  let mut client: SessionClient = session.get_client().unwrap();
  println!("I have my client");

  let t1 = thread::spawn(move || {
    println!("Started the server thread!");
    let rt = Runtime::new().unwrap();
    rt.spawn(async move {
      println!("Awaiting commands on the server thread runtime");
      session.run().await.unwrap();
    });
  });

  println!("Sending a message from the client");
  client.send(Event::command_from("This message has been sent")).await;

  // Try sending something to diagnostics
  // Not working because my event matching is not solid.
  client.send(Event::Command {
    id: String::from("diagnostics::log_event"),
    data: CommandData {
        docs: String::from("This is the details of my log event"),
    }
  }).await;

  t1.join().unwrap();

}
