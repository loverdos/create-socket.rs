use std::env;
use std::os::unix::net::UnixListener;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} SOCKET_FILENAME", args[0]);
        process::exit(1);
    }


    let socket_name = &args[1];

    let listener = UnixListener::bind(socket_name);
    match listener {
        Ok(socket) => { 
            eprintln!("Info: bound {:?}", socket);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
            process::exit(2);
        }
    }
}
