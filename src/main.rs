mod services;
mod server;

fn main()
{
	//lets get the username and password for our satellite server from the command line arguments
	//the command format we're expecting here is: good_vibes username password
	let args: Vec<String> = std::env::args().collect();

	//arguments 2 & 3 should corresond to the username and password, if not, exit
	if(args.len() >= 3)
	{
		let username: String = args[1];
		let password: String = args[2];

		server::startServer(username, password);
	}
	else	
	{
		println!("Expected username and password as arguments e.g: good_vibes.exe username password");
	}
}