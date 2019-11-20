// ANCHOR: example
//!
//! Get the results of a collection of futures.
//!
//! Using async await syntax and futures 0.3.
//!
use failure::{format_err, Error};
use futures::{
    future::join_all,
    executor::block_on,
};

enum Outcome {
    Good,
    Bad,
}

// Function to model a future that can fail given either good or
// bad input.
async fn get_single_future(outcome: Outcome) -> Result<String, Error> {
    match outcome {
        Outcome::Good => Ok("Success!".to_string()),
        Outcome::Bad => Err(format_err!("Failure")),
    }
}

async fn get_joined_future() -> Vec<Result<String, Error>> {
    // Let outcomes model the success of the futures we're going to get.
    let outcomes = vec![Outcome::Good, Outcome::Bad, Outcome::Good];

    let packed_futures = outcomes
        .into_iter()
        .map(|outcome| async {
            // Pack the result of each future into an Ok which we'll unwrap
            // after joining.
            match get_single_future(outcome).await {
                // We need to tell the compiler the Err type of the Result
                // we're wrapping our futures' results in.  As we'll never use
                // it, say it's ().
                Ok(message) => Ok::<Result<String, Error>, ()>(Ok(message)),
                Err(whoopsie) => Ok(Err(whoopsie)),
            }
        })
        .collect::<Vec<_>>();

    // join_all with return a Vec of results which we know we can unwrap
    // as the match above never creates an Err variant.
    join_all(packed_futures).await.into_iter().map(|x| x.unwrap()).collect()
}

pub fn get_results() -> Vec<Result<String, Error>> {
    block_on(get_joined_future())
}
// ANCHOR_END: example
