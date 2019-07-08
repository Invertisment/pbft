## PBPBFT probably broken PBFT

Let's learn Rust

The application doesn't implement views and master-node.
The `Network` object acts as a master-node object for the current view.

#### The app has two modes of running:
1. non-interactive smoke-test mode
1. interactive console UI mode

##### Running in interactive mode:
`cargo run -- --ui`

App workflow: 
- Nodes report to a single channel 
- Messages are taken from the channel and added to queue so they could be seen before sending.

##### Running in non-interactive smoke-test mode:
`cargo run`

##### Run tests:
`cargo test`

