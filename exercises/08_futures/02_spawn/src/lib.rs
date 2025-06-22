use tokio::net::TcpListener;

// TODO: write an echo server that accepts TCP connections on two listeners, concurrently.
//  Multiple connections (on the same listeners) should be processed concurrently.
//  The received data should be echoed back to the client.
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    let first = std::sync::Arc::new(first);
    let second = std::sync::Arc::new(second);

    let first_listener = first.clone();
    let second_listener = second.clone();

    let first_task = tokio::spawn(async move {
        loop {
            let listener = first_listener.clone();
            match listener.accept().await {
                Ok((socket, _)) => {
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.into_split();
                        if let Err(e) = tokio::io::copy(&mut reader, &mut writer).await {
                            eprintln!("Error while echoing data: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection on first listener: {}", e);
                }
            }
        }
    });

    let second_task = tokio::spawn(async move {
        loop {
            let listener = second_listener.clone();
            match listener.accept().await {
                Ok((socket, _)) => {
                    tokio::spawn(async move {
                        let (mut reader, mut writer) = socket.into_split();
                        if let Err(e) = tokio::io::copy(&mut reader, &mut writer).await {
                            eprintln!("Error while echoing data: {}", e);
                        }
                    });
                }
                Err(e) => {
                    eprintln!("Error accepting connection on second listener: {}", e);
                }
            }
        }
    });

    // Wait for both tasks to complete (which will be never, unless they panic)
    let _ = tokio::try_join!(first_task, second_task);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();

                    // Send the request
                    writer.write_all(request.as_bytes()).await.unwrap();
                    // Close the write side of the socket
                    writer.shutdown().await.unwrap();

                    // Read the response
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
