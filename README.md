# H2o KV

A Toy KV Database (Server).

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
(None)
> put foo bar
true
> get foo
bar
```

Hint: You can use `nc` instead if you don't have telnet on head.

## C/S Protocols

### Queries

The query is formed as follows:

    +--------+-----+------+--------+----------+
    | Header | CMD | Flag | Length | Content  |
    +--------+-----+------+--------+----------+
    | '\x0c' | 1   | 1    | 2      | Variable |
    +--------+-----+------+--------+----------+

**Where**:

- Header
    - protocol header: alwasy be `\x0c` for now.
- CMD
    - GET: `\x01`
    - PUT: `\x02` *see next protocal table*
    - DEL: `\x03`
    - SCAN: `\x04`
- Flag
    - Plain Text: `\x01`
    - GZIP Text: `\x02`
    - Ciphered Text: `\x03` (TBD)
- Length
    - Two bytes indicating how many bytes the Content part are. LittleEndian.
- Content
    - The KEY bytes for `GET`, `PUT`, and `DEL`
    - The key search pattern for `SCAN`

### Protocol for PUT

    +--------+--------+------+------+-----+-------+------+-------+
    | Header | CMD    | Flag | KLen | KEY | VLLen | VLen | VALUE |
    +--------+--------+------+------+-----+-------+------+-------+
    | '\x0c' | '\x02' | 1    | 2    | Var | 1     | Var  | Var   |
    +--------+--------+------+------+-----+-------+------+-------+

- KLen
    - Two bytes indicating how many bytes the KEY part are
- KEY
    - The KEY bytes
- VLLen
    - 1 byte indicating how many bytes the VLen part are
- VLen
    - bytes indicating how many bytes the VALUE part are, LittleEndian.
- VALUE
    - The Value bytes

### Responses

**GET, SCAN, DEL**

    +--------+------+------+-----+---------+
    | Header | Flag | LLen | Len | Content |
    +--------+------+------+-----+---------+
    | '\x0c' | 1    | 1    | Var | Var     |
    +--------+------+------+-----+---------+

**PUT**

    +--------+------+
    | Header | Flag |
    +--------+------+
    | '\x0c' | 1    |
    +--------+------+

- Flag
    - OK: `\x00`
    - Failed: `\x01`

## To Do List

- doc all pub functions
- use a more mature protocol (with header, content len bytes)
- h2okv-cli
