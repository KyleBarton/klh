## KLH-Core

This is the core crate for KLH, containing the public API and client code to start an instance of KLH.

## KlhClient

### Communicating with a running Klh

UIs and other clients of a runnint Klh instance should use KlhClient to send Requests to the running Klh:

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

### Adding Plugins

All major functionality in Klh is written in plugins, save only for the code necessary to help those plugins communicate.

This includes core functionality. `Buffers`, `Display`, `LanguageParser` (Name TBD) are all implemented as plugins.

Adding a plugin to an instance of Klh must be done before the instance has `start`ed:

_Note: See [#11](https://github.com/KyleBarton/klh/issues/11) for more work needed here_


```rs
#[tokio::main]
async fn main() {
  let mut klh = Klh::new();
  
  let test_plugin = TestPlugin::new();

  klh.add_plugin(Box::new(test_plugin));

  klh.start().await;

}
```
