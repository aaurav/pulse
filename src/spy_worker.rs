//src/spy_worker.rs
  use crate::events::PulseEvent;
  use tokio::sync::mpsc;

  pub async fn start(tx: mpsc::Sender<PulseEvent>, process_name: String){
        // For now, just send one test message
        let _ = tx.send(PulseEvent::ProcessRunning {
                    name : process_name,
                    is_running : true,
        }).await;
  }
