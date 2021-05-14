use std::{str, io, sync::Arc};
use clap::{App, Arg};
use std::net::{SocketAddrV4, Ipv4Addr};
use tokio::time::{Duration, sleep};
use tokio::net::UdpSocket;
use std::error::Error;
use rand::{Rng};


const DEFAULT_PORT: &str = "50692";
const DEFAULT_MULTICAST: &str = "239.255.42.98";
const IP_ALL: [u8; 4] = [0, 0, 0, 0];


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    let app = App::new("Udp Multicast Clipboard")
        .version("0.1.0")
        .author("Francisco Revilla <paconte@gmail.com>") //TODO: how to add second author
        .about("Async UDP multicast CLI app to share your clipboard content in a local network.")
        .arg(Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("PORT")
            .takes_value(true)
            .default_value(DEFAULT_PORT)
            .help("Sets UDP port number"))
        .arg(Arg::with_name("ip")
            .short("i")
            .long("ip")
            .value_name("IP")
            .takes_value(true)
            .default_value(DEFAULT_MULTICAST)
            .help("Sets multicast IP"))
        .get_matches();

    let port = app.value_of("port")
        .unwrap()
        .parse::<u16>()
        .expect("Invalid port number");

    let addr = SocketAddrV4::new(IP_ALL.into(), port);
    let multi_addr = SocketAddrV4::new(
        app.value_of("ip")
            .unwrap()
            .parse::<Ipv4Addr>()
            .expect("Invalid IP"),
        port,
    );

    println!("Starting server on: {}", addr);
    println!("Multicast address: {}\n", multi_addr);

    let sock = UdpSocket::bind(addr).await?;
    let r = Arc::new(sock);
    let s = r.clone();
    let mut buf = [0; 1024];

    tokio::spawn(async move {
        loop {
            let data = fake_clipboard_events().await.unwrap();
            let len = s.send_to(&String::as_bytes(&data), &addr).await.unwrap();
            println!("{:?} => {:?} bytes sent", data, len);
        }
    });

    loop {
        let (len, addr) = r.recv_from(&mut buf).await?;
        let data = str::from_utf8(&buf).unwrap();
        println!("{:?} => {:?} bytes received from {:?}", &data[0 .. len], len, addr);
        // escribir en el portapapeles
    }
}

async fn fake_clipboard_events() -> io::Result<String> {
    let seconds = rand::thread_rng().gen_range(1..10);
    sleep(Duration::from_secs(seconds)).await;
    Ok(get_random_text())
}

fn get_random_text() -> String {
    let texts = ["Hola mundo", "Hola Andrew", "Hola Paco"];
    let index = rand::thread_rng().gen_range(0..texts.len());
    String::from(texts[index])
}