use hyper::{service::service_fn, Body};
use rustls::{Certificate, PrivateKey, ServerConfig};
use tokio::spawn;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use http::{Response};
use openssl::pkcs12::Pkcs12;
use openssl::pkcs12::ParsedPkcs12_2;
use futures_util::stream::StreamExt;
use serde_json::json;
use flume::unbounded;
use uuid::Uuid;
use chrono::prelude::*;
use std::{fs::File, io::Read, sync::Arc};
use std::{env, net::SocketAddr};

async fn handle_request(req: hyper::Request<Body>) -> Result<hyper::Response<Body>, hyper::Error> {
    let txid = Uuid::new_v4().to_string();
    env::set_var("txid", &txid);
    let readi: DateTime<Utc> = Utc::now();
    println!("[ {} INFO ] - {} - processing handle_request of client data...", readi, &txid);
    let (sender, receiver) = unbounded::<Vec<u8>>();
    spawn(async move {
        let mut body_stream = req.into_body();

        while let Some(chunk) = body_stream.next().await {
            match chunk {
                Ok(bytes) => {
                    if sender.send_async(bytes.to_vec()).await.is_err() {
                        break;
                    }
                },
                Err(e) => {
                    let errorout: DateTime<Utc> = Utc::now();
                    let jexid = env::var("txid");
                    eprintln!("[ {:?} INFO ] - {:?} - Error reading chunk: {:?}", errorout, jexid, e);
                    break;
                }
            }
        }
    });

    let mut collected_data = Vec::new();
    while let Ok(chunk) = receiver.recv_async().await {
        collected_data.extend(chunk);
    }
    let collected: DateTime<Utc> = Utc::now();
    let demodata = format!("{:?}", collected_data);
    let genid = env::var("txid");
    println!("[ {} INFO ] - {:?} - Data read from client, now sending response body...", collected, &genid);
    let response_body = json!({ "data": demodata }).to_string();
    Ok(Response::new(Body::from(response_body)))
}

fn load_tls_config() -> Result<ServerConfig, Box<dyn std::error::Error>> {
    let pkcs12_path = env::var("PKCSPATH")?;
    let pkcs12_password = env::var("PKCSPASSWORD")?;
    let mut file = File::open(pkcs12_path)?;
    let mut pkcs12_data = vec![];
    file.read_to_end(&mut pkcs12_data)?;
    let pkcs12 = Pkcs12::from_der(&pkcs12_data)?;
    let ParsedPkcs12_2 { cert, pkey, .. } = pkcs12.parse2(&pkcs12_password)?;
    let cert_chain = cert.into_iter().map(|cert| Certificate(cert.to_der().unwrap())).collect::<Vec<_>>();
    let key = PrivateKey(pkey.expect("NO PRIVATE KEY FOUND").private_key_to_der().unwrap());

    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)?;

    Ok(config)
}

#[tokio::main]
async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let port = env::args().nth(1).unwrap_or_else(|| "3459".to_string()).parse()?;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let server_config = load_tls_config()?;
    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));
    let tcp_listener = TcpListener::bind(&addr).await?;
    let startime: DateTime<Utc> = Utc::now();
    println!("[ {} INFO ] - INIT - Server running on https://{}", startime, addr);

    loop {
        let (tcp_stream, _) = tcp_listener.accept().await?;
        let tls_acceptor = tls_acceptor.clone();
        tokio::spawn(async move {
            let tls_stream = match tls_acceptor.accept(tcp_stream).await {
                Ok(stream) => stream,
                Err(e) => {
                    let dropdate: DateTime<Utc> = Utc::now();
                    let gxid = env::var("txid").unwrap();
                    eprintln!("[ {} ERRO ] - {:?} - TLS error: {e}", dropdate, gxid);
                    return;
                },
            };
            let service = service_fn(handle_request);
            if hyper::server::conn::Http::new().serve_connection(tls_stream, service).await.is_err() {
                let erdate: DateTime<Utc> = Utc::now();
                let gxid = env::var("txid").unwrap();
                println!("[ {} INFO ] - {:?} - End client transaction.", erdate, gxid);
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run_server()
}    
