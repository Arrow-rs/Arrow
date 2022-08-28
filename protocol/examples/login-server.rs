//! An example using [`arrow-protocol`] to accept login packets

use std::{error::Error, sync::Arc};

use arrow_protocol::{
    chat::{Chat, Color, Component, TextComponent},
    codec::Codec,
    login::{EncryptionRequest, Login, LoginDisconnect, VerifyToken},
    Bound, Protocol, State,
};

use futures_util::{SinkExt, StreamExt};

use num_bigint::BigInt;
use reqwest::{Client, StatusCode};
use rsa::{pkcs8::EncodePublicKey, RsaPrivateKey};
use serde::Deserialize;
use sha1::{Digest, Sha1};
use tokio::net::TcpListener;
use tokio_util::codec::Decoder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:25565").await?;

    let mut rng = rand::thread_rng();
    let rsa_private = RsaPrivateKey::new(&mut rng, 1028)?;
    let rsa_public = rsa_private.to_public_key();

    let client = Arc::new(Client::new());
    println!("Ready to accept connections");
    loop {
        let client = client.clone();
        let rsa_public = rsa_public.clone();
        let rsa_private = rsa_private.clone();
        let (connection, _) = listener.accept().await?;
        println!("Accepted new connection from {}", connection.peer_addr()?);
        tokio::spawn(async move {
            let codec = Codec::new(Bound::Serverbound, State::Handshake);
            let mut frame = codec.framed(connection);
            let handshake = match frame.next().await.ok_or(LoginError::NoHandshake)?? {
                Protocol::Handshake(handshake) => handshake,
                _ => Err(LoginError::NoHandshake)?,
            };
            println!("Recieved handshake packet from {}", handshake.address);

            frame.codec_mut().set_state(State::Login);

            let login_start = match frame.next().await.ok_or(LoginError::NoLoginStart)?? {
                Protocol::Login(Login::LoginStart(login)) => login,
                _ => Err(LoginError::NoLoginStart)?,
            };

            println!("Recieved login start packet from {}", login_start.name);

            let server_id = "foo";
            let verify_token = VerifyToken {
                verify_token: rand::random::<[u8; 4]>().into(),
            };
            let encryption_request = Protocol::Login(Login::EncryptionRequest(EncryptionRequest {
                public_key: rsa_public.clone(),
                server_id: server_id.to_string(),
                verify_token,
            }));
            println!("Sending encryption request");
            frame.send(encryption_request).await?;

            let encryption_response = match frame.next().await.ok_or(LoginError::NoPacket)?? {
                Protocol::Login(Login::EncryptionResponse(res)) => res,
                _ => Err(LoginError::NoPacket)?,
            };
            let secret = encryption_response.shared_secret.decrypt(rsa_private)?;

            let mut hasher = Sha1::new();
            hasher.update(server_id);
            hasher.update(secret);
            hasher.update(rsa_public.to_public_key_der()?);

            // cursed mojang representation of the hash output
            let server_id_hash = BigInt::from_signed_bytes_be(&hasher.finalize()).to_str_radix(16);
            println!("Trying to authenticate user with mojang session server");
            let res = client.get(format!("https://sessionserver.mojang.com/session/minecraft/hasJoined?username={}&serverId={}", login_start.name, server_id_hash)).send().await?;
            if res.status() != StatusCode::OK {
                return Err(LoginError::HttpError.into());
            }
            #[derive(Deserialize)]
            struct Player {
                name: String,
                id: String,
            }
            let player: Player = res.json().await?;

            println!("Authenticated player {}({})", player.name, player.id);

            frame.codec_mut().enable_encyption(secret);

            // let login_success = Protocol::Login(Login::LoginSuccess(LoginSuccess {
            //     username: player.name,
            //     uuid: Uuid::try_parse(&player.id)?,
            //     properties: Vec::new(),
            // }));

            // frame.send(login_success).await?;

            // frame.codec_mut().set_state(State::Play);

            let disconnect = Protocol::Login(Login::LoginDisconnect(LoginDisconnect {
                reason: Chat {
                    component: Component::String(TextComponent {
                        text: format!("Successfully authenticated as \n{}\nUUID: {}", player.name, player.id),
                    }),
                    ..Default::default()
                }.with_color(Color::WebColor(100, 49, 3))
            }));
            frame.send(disconnect).await?;

            Ok::<_, Box<dyn Error + Send + Sync + 'static>>(())
        })
        .await?.unwrap();
    }
}

#[derive(Debug, thiserror::Error)]
enum LoginError {
    /// No handshake occured
    #[error("expected handshake packet")]
    NoHandshake,
    #[error("expected login start packet")]
    NoLoginStart,
    #[error("expected a packet but the stream was empty")]
    NoPacket,
    #[error("got invalid response from session server")]
    HttpError,
}
