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

## H2oKV Protocols

### Queries

The query is formed as follow:

    +--------+-----+------+--------+----------+
    | Header | CMD | Flag | Length | Content  |
    +--------+-----+------+--------+----------+
    | '\x0c' | 1   | 1    | 2      | Variable |
    +--------+-----+------+--------+----------+

**Where**:

- Header
    - protocol header: always be `\x0c` for now.
- CMD
    - GET: `\x01`
    - PUT: `\x02` *see next protocol table*
    - DEL: `\x03`
    - SCAN: `\x04`
- Flag
    - Plain Text: `\x00`
    - GZIP Text: `\x01`
    - Ciphered Text: `\x02` (details TBD)
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

**GET**

    +--------+------+------+-----+---------+
    | Header | Flag | LLen | Len | Content |
    +--------+------+------+-----+---------+
    | '\x0c' | 1    | 1    | Var | Var     |
    +--------+------+------+-----+---------+

**SCAN**

    +--------+------+-------+-----+-----+-----+-----+-----+
    | Header | Flag | Count | Len | Key | Len | Key | ... |
    +--------+------+-------+-----+-----+-----+-----+-----+
    | '\x0c' | 1    | 4     | 2   | Var | 2   | Var | ... |
    +--------+------+-------+-----+-----+-----+-----+-----+

**PUT, DEL, All**

    +--------+------+
    | Header | Flag |
    +--------+------+
    | '\x0c' | 1    |
    +--------+------+

- Flag
    - OK: `\x00`
    - Failed: `\x01`
    - No such Key: `\x02` (for `GET`)
    - Unknown command: `\xFF`

