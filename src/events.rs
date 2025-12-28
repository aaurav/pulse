/*
 * @src/events.rs
 */

#[derive(Debug)]
#[allow(dead_code)]
pub enum PulseEvent {
    ProcessRunning {
        name : String,
        is_running : bool,
    },
}
