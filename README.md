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
> put foo bar
true
> get foo
bar
```

Hint: You can use `nc` instead if you don't have telnet on head.

## To Do List

- UT
- use a more mature protocol
