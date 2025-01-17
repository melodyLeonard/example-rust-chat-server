use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() {
    let listner = TcpListener::bind("localhost:8080").await.unwrap();

    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, addr) = listner.accept().await.unwrap();

        let tx = tx.clone();

        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();

            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) =>{
                        if result.unwrap() == 0 {
                            break;
                        }

                        tx.send((line.clone(), addr)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() =>{
                        let (message, other_addr) = result.unwrap();
                       if addr != other_addr{
                            writer.write_all(message.as_bytes()).await.unwrap();
                       }
                    }
                }
            }
        });
    }
}
