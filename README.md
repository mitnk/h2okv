# H2o KV

A Toy KV Database.

## Build & Run Server

**Required**: Rust 2018 (latest Rust Stable or later versions)

```
$ make
H2o KV started at 127.0.0.1:30160
```

## Build & Run Client

```
$ make cli
Connected to h2okv server 127.0.0.1:30160, Ctrl-D to exit
h2okv> get foo
(None)
h2okv> set foo bar
OK
h2okv> get foo
"bar"
h2okv> put first 135
OK
h2okv> scan f
1) "first"
2) "foo"
```

## DB Data Persistence

Each write query would make the whole DB saved into a file named `h2okv.data`
under *current working directory*. For more details on disk persistence,
please see comments in file `src/persistence.rs`.

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

    +--------+------+------+------+-----+---------+
    | Header | Stat | Flag | LLen | Len | Content |
    +--------+------+------+------+-----+---------+
    | '\x0c' | 1    | 1    | 1    | Var | Var     |
    +--------+------+------+------+-----+---------+

**SCAN**

    +--------+------+------+-------+-----+-----+-----+-----+-----+
    | Header | Stat | Flag | Count | Len | Key | Len | Key | ... |
    +--------+------+------+-------+-----+-----+-----+-----+-----+
    | '\x0c' | 1    | 1    | 4     | 2   | Var | 2   | Var | ... |
    +--------+------+------+-------+-----+-----+-----+-----+-----+

**PUT, DEL, All**

    +--------+------+
    | Header | Stat |
    +--------+------+
    | '\x0c' | 1    |
    +--------+------+

- Flag
    - Plain Text: `\x00`
    - GZIP Text: `\x01`
    - Ciphered Text: `\x02` (details TBD)
- Stat(us)
    - OK: `\x00`
    - Failed: `\x01`
    - No such Key: `\x02` (for `GET`)
    - Unknown command: `\xFF`

