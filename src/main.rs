/*
 * @src/main.rs
 */
mod events;
mod spy_worker;

use tokio::sync::mpsc;
use std::env; 

#[tokio::main]
async fn main(){
    let args:Vec<String> = env::args().collect();

      if args.len() < 2 {
        eprintln!("Usage: pulse <process_name>");
        eprintln!("Example: pulse fresh");
        return;
    }

      let process_name = &args[1];

   // 1. Create the channel (walkie-talkie system)
   let (tx, mut rx) = mpsc::channel(10);

   // 2. Start the worker (give it the sender and process name)
   tokio::spawn(spy_worker::start(tx, process_name.to_string()));

   // 3. Wait for a message from the worker
   if let Some(event) = rx.recv().await {
       println!("Received: {:?}", event);
   }
}
