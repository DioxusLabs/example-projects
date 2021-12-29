# TodoApp with Rust + Dioxus


This repository holds example code for a variety of TodoMVCs built with different patterns.
- Hooks only
- Model
- Hooks + Shared State
- Model + Shared State
- Borrowed

There isn't a real "best pattern" - but the goal with showcasing these patterns is to highlight the tradeoffs of each.

In general:
- use_ref is more performant than use_state, but _can_ panic at runtime.
- Shared State is an automatic memoization barrier, but falls apart for collections (Recoil supports collections natively)
- Borrowing tends to be quite convenient but never benefits from memoization
- Models tend to be easier to understand but less composable than hooks

With Dioxus, you can really write whatever you want.

It's important to remember that our "budget" is 16ms - and that's a massive amount of time compared to your Rust code's performance. For perspective, Dioxus takes less than 100us to diff and update the TodoMVC app, regardless of approach. These strategies are only relevant when your app gets huge or on platforms with constrained resources.


![Live Example](./example.png)

