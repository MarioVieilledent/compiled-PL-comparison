use serde::Serialize;
use serde_json;
use std::io::Write;
use std::net::TcpListener;

#[derive(Debug, Serialize)]
struct Person {
    name: String,
    age: u32,
}

fn main() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };

    let listener = TcpListener::bind("127.0.0.1:1200").unwrap();
    println!("Serveur HTTP en écoute sur le port 1200");

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("Connexion établie !");

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{}\r\n",
                    serde_json::to_string(&person).unwrap()
                );

                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("Erreur lors de l'établissement de la connexion : {}", e);
            }
        }
    }
}
