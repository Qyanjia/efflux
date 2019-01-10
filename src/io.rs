//! IO binding module for the `efflux` crate.
//!
//! Provides lifecycles for Hadoop Streaming IO, to allow the rest
//! of this crate to be a little more ignorant of how inputs flow.
use bytelines::*;
use std::io::{self, BufReader};

use crate::context::Context;

/// Lifecycle trait to allow hooking into IO streams.
///
/// This will be implemented by all stages of MapReduce (e.g. to
/// appropriately handle buffering for the reduction stage). All
/// trait methods default to noop, as they're all optional.
pub trait Lifecycle {
    /// Startup hook for the IO stream.
    fn on_start(&mut self, _ctx: &mut Context) {}

    /// Entry hook for the IO stream to handle input values.
    fn on_entry(&mut self, _input: String, _ctx: &mut Context) {}

    /// Finalization hook for the IO stream.
    fn on_end(&mut self, _ctx: &mut Context) {}
}

/// Executes an IO `Lifecycle` against `io::stdin`.
pub fn run_lifecycle<L>(mut lifecycle: L)
where
    L: Lifecycle,
{
    // lock stdin for perf
    let stdin = io::stdin();
    let stdin_lock = stdin.lock();

    // create a job context
    let mut ctx = Context::new();

    // fire the startup hooks
    lifecycle.on_start(&mut ctx);

    // read all inputs from stdin, and fire the entry hooks
    for input in BufReader::new(stdin_lock).byte_lines().into_iter() {
        // verify that the input line is valid
        if let Ok(input) = input {
            // parse a string value out of the incoming line
            if let Ok(input) = String::from_utf8(input) {
                // consume the input by passing to the lifecycle
                lifecycle.on_entry(input, &mut ctx);
            }
        }
    }

    // fire the finalization hooks
    lifecycle.on_end(&mut ctx);
}
