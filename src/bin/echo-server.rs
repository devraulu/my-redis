// use tokio::{
//     io::{self, AsyncReadExt, AsyncWriteExt},
//     net::{TcpListener, TcpStream},
// };

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     // let listener = TcpListener::bind("127.0.0.1:6379").await?;
//     let socket = TcpStream::connect("127.0.0.1:6142").await?;
//     // let (mut rd, mut wr) = io::split(socket);
//
//     // let (socket, _) = listener.accept().await?;
//
//     tokio::spawn(async move {
//         wr.write_all(b"hello\r\n");
//         wr.write_all(b"world\r\n");
//
//         Ok::<_, io::Error>(())
//     });
//
//     let mut buf = vec![0; 128];
//
//     loop {
//         let n = rd.read(&mut buf).await?;
//
//         if n == 0 {
//             break;
//         }
//
//         println!("GOT {:?}", &buf[..n]);
//
//         // tokio::spawn(async move {
//         //     // This fails to compile
//         // });
//     }
//
//     Ok(())
// }
//

use tokio::{
    io::{self, AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return,
                    Ok(n) => {
                        if socket.write_all(&buf[..n]).await.is_err() {
                            return;
                        }
                    }
                    Err(_) => return,
                }
            }
        });
    }
}
