# H2o KV

A Toy KV Database (Server).

## Project Current Status

A merely working POC. See To Do List Below for details.

## Build & Run Server

A simple `cargo run` in root of the repo should do the job:
```
$ cargo run
H2o KV started at 127.0.0.1:30160
```

## Testing with Client

We will use telnet for client.
```
$ telnet 127.0.0.1 30160
> get foo
None
> set foo bar
true
> get foo
bar
```

Hint: You can use `nc` instead if you don't have telnet on head.

## Stability

Since for now it's just a toy, so we used a lot of `expect()`, `unwrap()` in
code. This should be solved in future.

## To Do List

- Store Tree Implementation
- DB Data Persistence in disk (refer Redis)
- Refine API param types
- Thread Safe
- Perf tuning
- Remove all `unwrap()`, `expect()`.
- and more ...
