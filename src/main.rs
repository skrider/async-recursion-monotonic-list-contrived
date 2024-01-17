use anyhow::Result;
use async_recursion::async_recursion;
use futures::stream::{FuturesUnordered, StreamExt};
use std::sync::Arc;
use std::time::Duration;

const MAX_DEPTH: i32 = 8;

struct Placeholder {
    foo: i32,
}

#[async_recursion]
async fn recur(depth: i32, max_depth: i32, aux: Vec<Arc<Placeholder>>) -> Result<Vec<Arc<Placeholder>>> {
    // base case
    if depth == max_depth {
        return Ok(vec![Arc::new(Placeholder {
            foo: 1
        })]);
    }
    // sleep for no seconds just to suspend the task and kick it to end of queue
    tokio::time::sleep(Duration::from_millis(0)).await;
    
    // construct a new Placeholder object on the heap
    let to_append = Arc::new(Placeholder {
        foo: depth,
    });
    
    // fanout arbitrary amount
    let mut tasks = FuturesUnordered::new();
    for _ in 0..(max_depth - depth) {
        let mut new_aux = aux.clone();
        new_aux.push(to_append.clone());
        tasks.push(recur(depth + 1, max_depth, new_aux));
    }
    
    // await completion of all tasks and return
    let mut ret = Vec::new();
    while let Some(Ok(res)) = tasks.next().await {
        for r in res.iter() {
            ret.push(r.clone());
        }
    }
    Ok(ret)
}

#[tokio::main]
async fn main() {
    let mut args = std::env::args();
    let depth: i32 = args.nth(1).unwrap().parse().unwrap();

    if depth > MAX_DEPTH {
        std::process::exit(1);
    }

    let res = recur(0, depth, Vec::new()).await.unwrap();
    let sum = res.iter().fold(0, |acc, p| acc + p.foo);

    println!("{}", sum);
}
