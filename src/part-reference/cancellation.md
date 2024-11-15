# Cancellation and cancellation safety

Internal vs external cancellation
Threads vs futures
  drop = cancel
  only at await points
  useful feature
  still somewhat abrubt and surprising
Other cancellation mechanisms
  abort
  cancellation tokens

## Cancellation safety

Not a memory safety issue or race condition
  Data loss or other logic errors
Different definitions/names
  tokio's definition
  general definition/halt safety
  applying a replicated future idea
Simple data loss
Resumption
Issue with select or similar in loops
Splitting state between the future and the context as a root cause


