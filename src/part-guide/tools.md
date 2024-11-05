# Tools for async programming

- Why we need specialist tools for async
- Are there other tools to cover
  - loom

## Monitoring

- [Tokio console](https://github.com/tokio-rs/console)

## Tracing and logging

- issues with async tracing
- tracing crate (https://github.com/tokio-rs/tracing)

## Debugging

- Understanding async backtraces (RUST_BACKTRACE and in a debugger)
- Techniques for debugging async code
- Using Tokio console for debugging
- Debugger support (WinDbg?)

## Profiling

- How async messes up flamegraphs
- How to profile async IO
- Getting insight into the runtime
  - Tokio metrics
