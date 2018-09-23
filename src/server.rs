#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

use services;

struct Auth
{
	pub Username: String,
	pub Password: String,
}

static mut Credentials: Auth = Auth();

/**Will attempt to start the good vibes by communicating with 
 * the make satellite. If successful will post a message to slack
 * informing makers that good vibes are on the way
 */
#[post("/")]
fn initialiseGoodVibes()
{
	make_satellite::initialiseGoodVibes(Credentials.Username, Credentials.Password);
}

/**Starts the HTTP server that will
 * begin listening for requests from the Alexa Skill
 */
pub fn startServer(username: String, password: String)
{
	Credentials = Auth { username, password  };
	
	//fire up the rocket HTTP server
	rocket::ignite().mount("/api", routes![initialiseGoodVibes]).launch();
}