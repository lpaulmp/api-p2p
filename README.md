# Rust Simple API
Using Rocket create endpoints to consult informations about messages/events

It requires to install diesel-cli https://diesel.rs/guides/getting-started.html
And install nightly rust version due to rocket framework uses new features of rust.

### Installation 
```
cargo build
cargo run
```

## Endpoints
🛰  Mounting /api/v1/:
    => POST /api/v1/username application/json (get_all)
    => POST /api/v1/newMessage application/json (new_message)
    => POST /api/v1/getUser application/json (find_username)

## TODO
Add tests 
Add CI/CD
Implement the delete fn and path.
Add test data.
Reply to another API interacction.