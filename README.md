# talk

A small data exchange program for communication that uses UNIX signals. Just a project I made to learn how to use unsafe Rust features and integrating C APIs to perform system calls.

It constists of two executables: a server and a client. The client sends the message passed as argument and the server receives it and prints it to the standard ouput. To run each of them execute the following commands:

```
cargo run --bin server
```
```
cargo run --bin client {server pid} {custom message}
```

The server is currently unstable, it can only print up to 65 characters at once before the behavior becomes unpredictable. I'll try to find a fix for it at some point in the future.
