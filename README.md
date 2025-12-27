# Pulse

A flexible CLI tool to monitor process status in real-time.

## What is Pulse?

Pulse tracks the running status of any specified process on your system. Built with Rust and designed to be extensible.

**Example use cases:**
- Monitor if your text editor (like Fresh) is running
- Track development servers
- Watch system processes
- Extend to any application you want to monitor

## Architecture

Pulse is built with three core components:

### 1. `events.rs` - The Message Format

**Purpose:** Defines what kind of messages can be sent.
```rust
PulseEvent::ProcessRunning {
    name: "fresh",
    is_running: true
}
```

**Why separate?**
- Both `main.rs` AND `spy_worker.rs` need to know about `PulseEvent`
- Keeps the data structure isolated and reusable

**Real-world analogy:** Like a company's "message template" - everyone agrees on the format.

---

### 2. `spy_worker.rs` - The Scout

**Purpose:** Does the actual work of checking if a process is running.

**Why separate?**
- Keeps the "checking logic" isolated
- `main.rs` doesn't need to know HOW we check processes
- Easy to modify or extend without touching other code

**Real-world analogy:** Like a security guard patrolling - you don't need to know their route, just get their reports.

**What it does:**
1. Receives: sender (`tx`) + process name (`"fresh"`)
2. Checks: Is the process running?
3. Sends: Report through `tx`

---

### 3. `main.rs` - The Commander

**Purpose:** Coordinates everything and handles results.

**Why it's the entry point:**
- Every Rust program needs a `main()` function
- This is where the program starts

**What it does:**
1. Creates the communication channel (`tx`, `rx`)
2. Starts the worker (gives it `tx`)
3. Listens for reports (receives on `rx`)
4. Takes action (prints the report)

**Real-world analogy:** Mission control - sets up communication, deploys scouts, receives intel, makes decisions.


## How They Work Together
```
┌─────────────────────────────────────────────────────┐
│  main.rs (Commander)                                │
│                                                      │
│  1. Creates channel: (tx, rx)                       │
│  2. Spawns worker with tx                           │
│  3. Waits on rx for reports                         │
└──────────────────┬──────────────────────────────────┘
                   │
                   │ gives tx + "fresh"
                   ↓
┌─────────────────────────────────────────────────────┐
│  spy_worker.rs (Scout)                              │
│                                                      │
│  1. Receives tx + process name                      │
│  2. Checks if process running                       │
│  3. Creates PulseEvent ←─────────┐                  │
│  4. Sends via tx                 │                  │
└──────────────────┬───────────────┼──────────────────┘
                   │               │
                   │               │ uses format from
                   │               │
                   │          ┌────┴──────────────────┐
                   │          │  events.rs            │
                   │          │  (Message Format)     │
                   │          │                       │
                   │          │  Defines:             │
                   │          │  PulseEvent enum      │
                   │          └───────────────────────┘
                   │
                   │ message travels through channel
                   ↓
┌─────────────────────────────────────────────────────┐
│  main.rs (receives on rx)                           │
│                                                      │
│  Prints: ProcessRunning { name: "fresh", ... }      │
└─────────────────────────────────────────────────────┘
```

## Project Structure
```
pulse/
├── Cargo.toml           # Project configuration
├── README.md            # This file
└── src/
    ├── events.rs        # Message format definitions
    ├── spy_worker.rs    # Process monitoring logic
    └── main.rs          # Entry point and coordinator
```

## License

MIT

---

**Built with Rust 🦀**
