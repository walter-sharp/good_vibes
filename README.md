# Good Vibes Sender

A Simple command line application written in [Rust](https://www.rust-lang.org) that creates an HTTP server to listen for incoming Alexa Skill requests for toggling the Maker Satellite good vibes beam and communicating with the Makers via Slack once activated.

## Getting up and running

Running the application requires several steps:

1. Grab and install ngrok [here](https://dashboard.ngrok.com/get-started). ngrok will expose a port running on your local machine to the outside world without you having to worry about setting up port fowarding. We need this to expose the HTTP server this application creates to the interet.

2. Start ngrok up using the following command:

```
ngrok port 8000
```

This application uses the [Rocket library](https://rocket.rs/) to create an HTTP server in order to listen to incoming requests. Rocket defaults to using port 8000.

3. Grab and install the Rust build tools for your OS [here](https://www.rust-lang.org/en-US/install.html).

4. Clone this repo via:

```
git clone https://github.com/walter-sharp/good_vibes
```

4. Build the application via the Rust's [Cargo](https://doc.rust-lang.org/cargo/index.html) tool by executing the build command on the application folder:

```
cargo build
```

