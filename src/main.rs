use std::io;
use std::process;
use local_ip_address::local_ip;
use warp::{Filter, Reply};
use tokio::sync::oneshot::channel;


const SERVER_IP: [u8; 4] = [0; 4];
const SERVER_PORT: u16 = 2702;


fn parse_args() -> Result<String, ()>  {

    let args: Vec<String> = std::env::args().collect();


    if args.len() < 2 {
        eprintln!("ERROR : Not enough arguments!");
        process::exit(1);

    } else {
        
        return Ok(args[1].clone());
    }
}





#[tokio::main]
async fn main() {

    
    let file_dir = parse_args().unwrap();

    
    let send_file = warp::any()
        .and(warp::fs::file(file_dir.clone()))
        .map(move |reply: warp::filters::fs::File| {

            let file_name = file_dir.split("/").last().unwrap();
            warp::reply::with_header(
                reply,
                "Content-Disposition",
                format!("attachment; filename={}", file_name)).into_response()
        });


    let my_ip = local_ip().unwrap();
    println!("\nPlease connect to http://{}:{}", my_ip, SERVER_PORT);


    let (tx, rx)= channel::<String>();


    let (_, server) = warp::serve(send_file)
        .bind_with_graceful_shutdown((SERVER_IP, SERVER_PORT), async {
        match rx.await {
            Ok(v) => {
                println!("Stopping the server... {}", v);
            },
            Err(_) => {}
        }
    });


    // Spawn the server into a runtime
    tokio::task::spawn(server);

    println!("\nPlease type q and <Enter> to quit the server...");
    let mut end_signal = String::new();
    io::stdin().read_line(&mut end_signal).expect("failed to readline");

    // Later, start the shutdown...
    let _ = tx.send(end_signal);

    /*let server = tokio::spawn(async move {
        warp::serve(send_file)
        .run((SERVER_IP, SERVER_PORT))
        .await;
    });


    server.await.unwrap();*/


    //return Ok(());
}
