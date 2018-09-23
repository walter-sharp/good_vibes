# Good Vibes Sender

A Simple command line application written in [Rust](https://www.rust-lang.org) that creates an HTTP server to listen for incoming Alexa Skill requests for toggling the Maker Satellite good vibes beam and communicating with the Makers via Slack once activated.

Getting this application up and running requires several steps:

 - [Downloading and installing ngrok](#1-downloading-and-installing-ngrok)
 - [Building and running the application](#2-building-and-running-the-application)
 
## 1. Downloading and installing ngrok

ngrok allows you to expose a port running on your local machine to the outside world without you having to worry about setting up port fowarding. We need this to expose the HTTP server this application creates to the internet.

1. Grab and install ngrok [here](https://dashboard.ngrok.com/get-started). 

2. Start ngrok up using the following command:
```
ngrok http 8000
```
This application uses the [Rocket library](https://rocket.rs/) to create an HTTP server in order to listen to incoming requests. Rocket defaults to using port 8000 (hence the use of port 8000 in the above command).

If the command executes successfully, ngrok should print the address used to access your local server as well as other information:
```
Session Expires               7 hours, 56 minutes
Version                       2.2.8
Region                        United States (us)
Web Interface                 http://127.0.0.1:4040
Forwarding                    http://your-address.ngrok.io -> localhost:8000
Forwarding                    https://your-address.ngrok.io -> localhost:8000
```
## 2. Building and running the application

1. To actually build the application, grab and install the Rust build tools for your operating system [here](https://www.rust-lang.org/en-US/install.html).

2. Clone this repo via:
```
git clone https://github.com/walter-sharp/good_vibes
```

3. Build the application via Rust's [Cargo](https://doc.rust-lang.org/cargo/index.html) tool by executing the build command on the application folder:
```
cargo build
```

4. If the application builds successfully, you should be able to run it using Cargo's run command:
```
cargo run username password
```
In order to communicate with the Make Satellite HTTP server, this application requires the username and password used to authenticate with the service. You provide these credentials as command line arguments when executing the application. Note that the order these arguments appear is important, the username must come before the password.

## 3. Setting up and invoking the Alexa Skill

