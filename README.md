Example app for Update Client
=============================

This is a simple Rust app to exercise the [Application Update client](https://github.com/rhelmer/update-client).

Getting started
===============

# Build and run.
`cargo run`

The example app will attempt to spawn the update client as a child process.
The update client is expected to be at: `/tmp/update_client`

If updates are found, the update client will download them and notify the
example app, which will print the result on stdout.
