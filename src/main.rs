use std::process;
use local_ip_address::local_ip;
use warp::{Filter, Reply};


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
async fn main() -> Result<(), &'static str>{

    
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



    let server = tokio::spawn(async move {
        warp::serve(send_file)
        .run((SERVER_IP, SERVER_PORT))
        .await;
    });


    server.await.unwrap();


    return Ok(());
}
