use std::io::{Read, Write};
use std::str;
use std::net::{TcpListener, TcpStream};
use std::collections::VecDeque;

fn get_array_size(resp_command: &String) -> i16{
    let len = resp_command.len();
    let array_size: i16 = (&resp_command[1..len]).parse().unwrap();
    return array_size;
}

fn handle_keyword(mut stream: &TcpStream, command: &str) {
    match command {
        "ping" => {
            stream.write("+PONG\r\n".as_bytes());
        }
        _ => {
            // Do nothing
        }
    }

}

fn handle_commands(stream: &TcpStream, commands:&mut VecDeque<&str>) {

    let initial_command = commands[0].to_string();
    let ch = initial_command.chars().nth(0).unwrap();
    match ch {
        '*' => {
            let array_size = get_array_size(&initial_command);
            commands.pop_front();
            for _num in 0..array_size {
                handle_commands(&stream, commands)
            }
        }
        '$' => {
            let _array_size = get_array_size(&initial_command);
            commands.pop_front();
            let command = commands[0];
            handle_keyword(&stream, &command);
            commands.pop_front();
        }
        _ => {
            println!("Unknown command");
        }
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let mut buffer= [0; 128];
    stream.read(&mut buffer);
    let buffer_str = str::from_utf8(&buffer).unwrap();
    println!("We got - {}", &buffer_str);
    let commands_vector: Vec<&str> = buffer_str.split("\r\n").collect();
    let mut commands = VecDeque::from(commands_vector);
    handle_commands(&stream, &mut commands);

}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    println!("Starting a mock redis server");

    for stream in listener.incoming() {
        loop {
            match stream {
                Ok(ref stream) => {
                    handle_connection(&stream);
                }
                Err(ref e) => {
                    println!("error: {}", &e);
                }
            }
        }
    }
}
