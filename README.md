## KLH

An API-first integrated development environment written (probably badly) in rust.


### API-First

KLH is written to be invoked asynchronously from a client:

```rs
#[tokio::main]
async fn main() {
  let mut klh = Klh::new();

  klh.start().await;

  let client : KlhClient = klh.get_client();

  println!("Creating a buffer");
  let create_buffer_request = buffers::requests::new_create_buffer_request("special_buffer");
  client.send(create_buffer_request).await.unwrap();
}
```
