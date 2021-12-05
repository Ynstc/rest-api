//cargo watch -q -c -x run
// -q, --quiet              Suppress output from cargo-watch itself
// -c, --clear              Clear the screen before each run
// -x, --exec <cmd>...      Cargo command(s) to execute on changes [default: check]
//cargo watch -q -c -x run


//1 cargo run
//open connection in terminal to this tcp server E.G by telnet -> `telnet localhost 8080` in two different terminals
// ready to char between terminals
// ref. https://www.youtube.com/watch?v=4DqP57BHaXI

use tokio::{
    io::{ AsyncWriteExt, BufReader, AsyncBufReadExt},
    net::TcpListener,
    sync::broadcast
};

#[tokio::main]
async fn main() {

    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, mut _rx) = broadcast::channel(10);

    loop {
    let ( mut socket, addr) = listener.accept().await.unwrap();


    let tx = tx.clone();
    let mut rx = tx.subscribe();

        tokio::spawn( async move{
            let (reader, mut write) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }
                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv()=> {
                        let (msg, other_addr) = result.unwrap();

                        if addr != other_addr {
                            write.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }

                // let bytes_read = reader.read_line(&mut line).await.unwrap();
                // tx.send(line.clone()).unwrap();

                // let msg = rx.recv().await.unwrap();

                // write.write_all(msg.as_bytes()).await.unwrap();
                // line.clear();
            }
            // loop {
            //     let mut buffer = [0u8; 1024];
            //     let bytes_read = socket.read(&mut buffer).await.unwrap();

            //     socket.write_all(&buffer[..bytes_read]).await.unwrap();
            // }
        });
    }

}
