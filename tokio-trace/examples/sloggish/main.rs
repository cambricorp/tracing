//! A simple example demonstrating how one might implement a custom
//! subscriber.
//!
//! This subscriber implements a tree-structured logger similar to
//! the "compact" formatter in [`slog-term`]. The demo mimicks the
//! example output in the screenshot in the [`slog` README].
//!
//! Note that this logger isn't ready for actual production use.
//! Several corners were cut to make the example simple.
//!
//! [`slog-term`]: https://docs.rs/slog-term/2.4.0/slog_term/
//! [`slog` README]: https://github.com/slog-rs/slog#terminal-output-example
#[macro_use]
extern crate tokio_trace;

use tokio_trace::{field, Level};

mod sloggish_subscriber;
use self::sloggish_subscriber::SloggishSubscriber;

fn main() {
    let subscriber = SloggishSubscriber::new(2);

    tokio_trace::Dispatch::to(subscriber).as_default(|| {
        span!("", version = &field::Value::display(5.0)).enter(|| {
            span!("server", host = "localhost", port = 8080u64).enter(|| {
                event!(Level::Info, {}, "starting");
                event!(Level::Info, {}, "listening");
                let mut peer1 = span!("conn", peer_addr = "82.9.9.9", port = 42381u64);
                peer1.enter(|| {
                    event!(Level::Debug, {}, "connected");
                    event!(Level::Debug, { length = 2u64 }, "message received");
                });
                let mut peer2 = span!("conn", peer_addr = "8.8.8.8", port = 18230u64);
                peer2.enter(|| {
                    event!(Level::Debug, {}, "connected");
                });
                peer1.enter(|| {
                    event!(Level::Warn, { algo = "xor" }, "weak encryption requested");
                    event!(Level::Debug, { length = 8u64 }, "response sent");
                    event!(Level::Debug, {}, "disconnected");
                });
                peer2.enter(|| {
                    event!(Level::Debug, { length = 5u64 }, "message received");
                    event!(Level::Debug, { length = 8u64 }, "response sent");
                    event!(Level::Debug, {}, "disconnected");
                });
                event!(Level::Error, {}, "internal error");
                event!(Level::Info, {}, "exit");
            })
        });
    });
}
