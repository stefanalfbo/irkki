use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use irkki_core::IRCClient;

fn spawn_stub_server() -> (u16, mpsc::Receiver<Vec<String>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port();

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));

        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut line = String::new();
        let mut received = Vec::new();

        for _ in 0..3 {
            line.clear();
            if reader.read_line(&mut line).unwrap() == 0 {
                break;
            }
            received.push(line.trim_end().to_string());
        }

        let _ = stream.write_all(b"PING :stub\r\n");
        let _ = stream.flush();

        line.clear();
        let _ = reader.read_line(&mut line);
        received.push(line.trim_end().to_string());

        let _ = stream.write_all(b":server 001 nick :welcome\r\n");
        let _ = stream.flush();

        drop(stream);

        let _ = tx.send(received);
    });

    (port, rx)
}

#[test]
fn client_handles_ping_and_receives_message() {
    let (port, rx) = spawn_stub_server();

    let mut client = IRCClient::connect("nick", "127.0.0.1", port).unwrap();

    let mut seen = Vec::new();
    client
        .listen(|line| {
            seen.push(line);
            Ok(())
        })
        .unwrap();

    let received = rx.recv_timeout(Duration::from_secs(2)).unwrap();
    assert!(received[0].starts_with("NICK "));
    assert!(received[1].starts_with("USER "));
    assert!(received[2].starts_with("JOIN "));
    assert_eq!(received[3], "PONG :stub");

    assert_eq!(seen, vec![":server 001 nick :welcome"]);
}
