# `dgi-log`

`dgi-log` is structured logging system

## At a glance

```rust
use std::fs::File;
use dgi_log::impls::{Console, Json};
use dgi_log::prelude::{Verbosity, builder};

fn main() {
    let logger = builder()
        .writer(Console::new().max_verbosity(Verbosity::Debug))
        .writer(Json::new(File::create("./this.log").unwrap()))
        .run()
        .unwrap();

    let name = "log";
    info!("hello, this is {name}", extra = "with some extra");

    logger.stop();
}
```

> [!NOTE]
> `stop()` flushes remaining events and gracefully shuts down the formatting thread.

## Core Concepts

### Event

Event is composed of:

- Timestamp (`DateTime<Utc>`)
- Verbosity
- Thread ID (`usize`)
- Module (result of `module_name!()`)
- Message
- Fields (see Fields section)

### Writer

Writer is responsible for formatting and output.

Writer is owned and used only by the formatting thread,
So it should be `Send` (not have to be `Sync`).

```rust
pub trait Writer {
    fn enabled_for(&self, verbosity: Verbosity) -> bool;
    fn write(&self, event: &Event);
}
```

You can create your own writer by implementing above trait,
Or, there are some **builtin writers** (`Alert`, `Console`, `Json`).

- `dgi_log::impls::Alert` : Show message box

  > [!WARNING]
  > `Alert` will block formatter thread, effectively pausing all logging.
  > It's encouraged that using only for Fatal (default).

- `dgi_log::impls::Console` : Write in console

  Format as:

  ```
  <timestamp> <verbosity> <module>[<thread id>]: <message>
    <fields...>
  ```

  For example:

  ```
  2026-04-01T00:45:14.327619603+00:00 INF test[00]: hello, this is test
    some = "and this is field"
  ```

- `dgi_log::impls::Json` : Write in arbitrary file with JSON

  > [!NOTE]
  > Example shown below is prettied.
  > Event will be written line by line,
  > Thus **only each line is valid JSON, not whole log file**.
  > See *NDJSON*.

  ```json
  {
    "timestamp": "2026-04-01T00:47:59.883504862Z",
    "verbosity": 3,
    "thread_id": 0,
    "module": "dgi_shell::app",
    "message": "hello, this is test",
    "fields": [
      {
        "name": "some",
        "value": "and this is field"
      }
    ]
  }
  ```

### Verbosity

There are 5 levels of verbosity:

- Fatal
- Error
- Warn
- Info
- Debug

> [!NOTE]
> Fatal level of logging also causes panic in caller thread.
> It's ensured that the event is processed before the program aborts.
> 
> ```rust
> fn example() {
>     fatal!("error!!!") // <-- panic here
> }
> ```

### Ignorable vs. Important

```
warn!(important "this is very important!!")
```

The formatting is delayed to do formatting job in single background thread,
and thus, bounded channel is used to send event to this formatting thread from other thread.

It implies that event channel can be flooded.

`ignorable` and `important` are for this problem.

*ignorable* event is ignored (dropped) when event channel is full,
so that doesn't block control flow.

*important* event is **always** sent to formatters,
so it waits for available slot when if event channel is full.

> [!WARNING]
> and this also implies that important event
> MUST NOT be used for implementing formatters.
> It can cause deadlock.

Each verbosity level has its default value:

| Verbosity | Default     |
|----------:|:------------|
|     Fatal | `important` |
|     Error | `important` |
|      Warn | `ignorable` |
|      Info | `ignorable` |
|     Debug | `ignorable` |

### Fields

```rust
#[derive(Serialize)]
struct Foo {}

fn example() {
    let var = Foo;
    info!("this uses field", some = Foo, another = var, other = "hello");
}
```

It's encouraged that using fields over direct formatting,
so logs remain structured and machine-readable.

To use fields, struct must implement one of traits:
`Serialize`, `Display`, `Debug`
and are used by this order of priority.

Field serialization occurs in caller thread
(thread that calls log macro such as `info!`).
