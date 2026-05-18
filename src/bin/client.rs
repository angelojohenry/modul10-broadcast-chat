use futures_util::SinkExt;
use futures_util::stream::StreamExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:8080"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();


    loop {
        tokio::select! {
            input = stdin.next_line() => {
                match input {
                    Ok(Some(line)) => {
                        ws_stream.send(Message::text(line)).await?;
                    }

                    Ok(None) => {
                        break;
                    }

                    Err(e) => {
                        eprintln!("Error reading stdin: {e}");
                        break;
                    }
                }
            }

            msg = ws_stream.next() => {
                match msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        println!("{}", msg.as_text().unwrap());
                    }

                    Some(Ok(_)) => {}

                    Some(Err(e)) => {
                        eprintln!("Error receiving message: {e}");
                        return Err(e);
                    }

                    None => {
                        println!("Server disconnected");
                        break;
                    }
                }
            }
        }
    }

    Ok(())

}