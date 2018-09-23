extern crate reqwest;
use reqwest;

/**Attempts to initialise the good vibes satellite via a call to the Make satellite REST endpoint
 * username: The username used to authenticate with the satellite server
 * password: The password used to authenticate with the satellite server
 */
pub fn initialiseGoodVibes(username: String, password: String)
{
	let client = Client::new();
	let credentials = header::Basic { username: username, password: password };

	let request = client.post("https://make.offerzen.com/satellite/");
	request.header(header::Authorization(credentials));
	request.body("");
	
	let response = request.send();
}