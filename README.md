# Good Vibes Sender

A Simple command line application written in [Rust](https://www.rust-lang.org) that creates an HTTP server to listen for incoming Alexa Skill requests for toggling the Maker Satellite good vibes beam and communicating with the Makers via Slack once activated.

Getting this application up and running requires several steps:

 - [Downloading and installing ngrok](#1-downloading-and-installing-ngrok)
 - [Building and running the application](#2-building-and-running-the-application)
 - [Setting up the Alexa Skill](#3-setting-up-the-alexa-skill)
 - [Invoking the Alexa Skill](#4-invoking-the-alexa-skill)
 
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

## 3. Setting up the Alexa Skill

The good vibes application is meant to be invoked by issuing a voice command through an Alexa Skill. For our purposes though, we can just use the test system on the Amazon developer console to invoke the skill manually. Before we can invoke the skill though, we need to tell Alexa where to send the request once the skill is invoked.

1. Browse to the Amazon developer console [here](https://developer.amazon.com/alexa) and sign in using the OfferZen Make credentials.

2. Once logged in, you should be redirected to the Alexa Skills Kit Console. Click the 'good vibes sender' skill in the skills list.

3. On the left hand sidebar, click on the 'Endpoint' item to go to the skill endpoint configuration page.

4. If the 'HTTPS' option is not already selected, go ahead and select it, additional fields will appear allowing you to enter the address Alexa will send the request to once the skill is invoked.

5. In the 'Default Region' field, enter in the HTTPS address that ngrok provided for you when you started up the ngrok service. It should look similar to the following:
```
Forwarding                    https://d8sjf7sd.ngrok.io -> localhost:8000
```

> Note that your address will likely be different from the example address above.

6. Click the 'Save Endpoints' button in the top left to save the endpoints for the skill.

7. Each time you change an Alexa skill, it will need to be rebuilt before being used again. To rebuild the skill, click the 'Invocation' item in the left hand sidebar then click the 'Build Model' button situated in the top left hand corner to begin the build process.
> The build process is actually queued and will not be completed immediately. After a few seconds a notification will popup telling you that the build is complete.

## 4. Invoking the Alexa Skill

Now that the skill is setup to use our exposed endpoint when invoked, we can go ahead and actually invoke the skill.

1. Click on the 'Test' tab to navigate to the skills test page.

2. Type in any one of the following commands to invoke the skill:

> fire good vibes at makers

> go for good vibes

> fire good vibes cannon

> initiate good vibes sender

> send good vibes

3. If the skill is invoked successfully, it should send a request to your local web server.