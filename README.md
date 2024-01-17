# Async Recursion with Monotonically Growing Shared List

This repo is created as a toy/contrived example to explore a particularly finicky use case of Rust's borrow checker. The task is as follows: a recursive async task with arguments `async fn recur(depth: i32, max_depth: i32, aux: Vec<Arc<Placeholder>>)` constructs a `Placeholder` and spawns several child tasks, passing each its own `aux` with the newly constructed `Placeholder` appended.

The issue I am running into is that there should in theory be no need for `Arc<Placeholder>`, a simple reference `&Placeholder` should do, as the referenced object is owned by the parent `recur` task, and hence will always outlive the child `recur` tasks.

Usually, this would be no cause for concern; however, in settings with high fanout, lock contention over `Placeholders` created earlier can slow down the program significantly.

Is there any way to dispense with the `Arc` and use a plain reference? Doing so lead to a compiler error involving `aux` not living long enough.

