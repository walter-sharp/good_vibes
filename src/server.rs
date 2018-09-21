#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod ./services/make_satellite;

let Username: String = "";
let Password: String = "";

/**Will attempt to start the good vibes by communicating with 
 * the make satellite. If successful will post a message to slack
 * informing makers that good vibes are on the way
 */
#[post("initialiseGoodVibes")]
fn initialiseGoodVibes()
{
	make_satellite::initialiseGoodVibes(Username, Password);
}

/**Starts the HTTP server that will
 * begin listening for requests from the Alexa Skill
 */
fn startServer(username: String, password: String)
{
	Username = username;
	Password = password;

	//fire up the rocket HTTP server
	rocket::ignite().mount("/", routes![initialiseGoodVibes]).launch();;
}