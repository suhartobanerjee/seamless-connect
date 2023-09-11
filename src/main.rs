use std::io;
use std::process;
use local_ip_address::local_ip;
use warp::{Filter, Reply};
use tokio::sync::oneshot::channel;


// the IP address and port values for the server
const SERVER_IP: [u8; 4] = [0; 4];
const SERVER_PORT: u16 = 2702;


fn parse_args() -> Result<String, ()>  {

    let args: Vec<String> = std::env::args().collect();


    // If file dir not provided, show error
    if args.len() < 2 {
        eprintln!("ERROR : Not enough arguments!");
        process::exit(1);

    } else {
        
        // cloning the file dir args so that main gets ownership
        // of the variable
        return Ok(args[1].clone());
    }
}




#[tokio::main]
async fn main() {

    // reading in the file dir from cmd args
    let file_dir = parse_args().unwrap();

    
    // logic to serve files with filename
    let send_file = warp::any()
        .and(warp::fs::file(file_dir.clone()))
        .map(move |reply: warp::filters::fs::File| {

            let file_name = file_dir.split("/").last().unwrap();
            warp::reply::with_header(
                reply,
                "Content-Disposition",
                format!("attachment; filename={}", file_name)).into_response()
        });


    // getting and showing the ip address of the machine
    // to connect from any device in the local network
    let my_ip = local_ip().unwrap();
    println!("\nPlease connect to http://{}:{}", my_ip, SERVER_PORT);


    let (tx, rx)= channel::<String>();


    // setting up the server with option to gracefully shutdown
    // works with "" as well
    let (_, server) = warp::serve(send_file)
        .bind_with_graceful_shutdown((SERVER_IP, SERVER_PORT), async {
        match rx.await {
            Ok(_) => {
                println!("Stopping the server...");
            },
            Err(err) => {
                println!("Error in gracefully stopping the server. Check the error below\n{}", err)
            }
        }
    });


    // Spawn the server into a runtime
    tokio::task::spawn(server);


    // Reading in user input and stalling the program
    println!("\nPress any key to quit the server...");
    let mut end_signal = String::new();
    io::stdin().read_line(&mut end_signal).expect("failed to readline");

    
    // Later, start the shutdown
    // service rendered. You are welcome! :D
    let _ = tx.send(end_signal);

}
