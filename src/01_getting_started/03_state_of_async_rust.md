## The State of Asynchronous Rust

The asynchronous Rust ecosystem has undergone a lot of evolution over time,
so it can be hard to know what tools to use, what libraries to invest in,
or what documentation to read. However, the `Future` trait inside the standard
library has recently been stabilized, and the `async`/`await` feature will
follow shortly.

The ecosystem is currently in a state of migration from the 0.1 `futures` crate
to the 0.3 `futures` crate. While this is ongoing, you may find that you will
have to reach for the `compat` functionality of the 0.3 crate. You may also
find that the `async`/`await` language feature is still new, so examples
may take some time to pop up. There's still some language oddities, like
`async fn` functions not working on traits and error messages are under constant
improvement.

We shipping a small product that we're comfortable with. We have chosen to ship
the current asynchronous programming features like this to make sure that people
can use it and we can improve on learning from their usage. We aim to
have some of the most performant and ergonomic support for asynchronous
programming around, and if you're not afraid of doing some spelunking,
enjoy your dive into the world of asynchronous programming in Rust!

Asynchronous Rust has always been used in production with great success and you
should not hesitate to jump on it now.
