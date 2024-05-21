mod protocol;

use std::{collections::HashMap, sync::Arc, time::Duration};

use protocol::ProtocolStream;
use tokio::sync::Mutex;
use tokio::{net::TcpListener, spawn, time::timeout};

use crate::protocol::Protocol;

#[derive(Default)]
struct UserAuth {
    users: HashMap<String, String>,
}

impl UserAuth {
    pub async fn is_valid_or_insert(&mut self, username: String, password: String) -> bool {
        *self.users.entry(username).or_insert(password.clone()) == password
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:1312").await?;
    let user_auth = Arc::new(Mutex::new(UserAuth::default()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let mut stream = ProtocolStream::new(stream);

        println!("New connection {addr}");

        let users = user_auth.clone();

        spawn(async move {
            let task = async { handshake(users, &mut stream).await };
            let elapsed = timeout(Duration::from_secs(3), task).await;

            let err = match elapsed {
                Ok(Err(e)) => stream.write(&Protocol::Error(e)).await,
                Err(_) => stream.write(&Protocol::Error(protocol::Error::Timeout)).await,
                _ => return,
            }
            .unwrap_err();

            eprintln!("Could not write to stream: {err}. Killing connection");
        });
    }
}

async fn handshake(users: Arc<Mutex<UserAuth>>, stream: &mut ProtocolStream) -> Result<(), protocol::Error> {
    proto_expect!(stream, Protocol::Login(username, password));
    println!("{username} has password {password}");

    if !users.lock().await.is_valid_or_insert(username, password).await {
        return Err(protocol::Error::InvalidCredentials);
    }

    stream.write(&Protocol::Motd("Hey".into())).await?;
    Ok(())
}
