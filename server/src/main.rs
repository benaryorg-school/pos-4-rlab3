use std::net::{TcpListener,TcpStream};
use std::io::{BufReader,BufRead,Write,Result as IoResult};
use std::fs::File;
use std::collections::HashMap;

fn handle_connection(map: &HashMap<String,String>,mut stream: &mut TcpStream) -> IoResult<()>
{
	let mut line = String::new();
	let mut reader = BufReader::new(try!(stream.try_clone()));
	while let Ok(_) = reader.read_line(&mut line)
	{
		line.pop();
		if line == "quit"
		{
			break;
		}
		if let Some(data) = map.get(&line)
		{
			try!(stream.write(format!("{}\n",data).as_bytes()));
		}
		else
		{
			try!(stream.write(b"not found\n"));
		}
		line.clear();
	}
	Ok(())
}

fn main()
{
	let mut map = HashMap::new();

	for line in BufReader::new(File::open("input.csv").unwrap()).lines()
	{
		if let Ok(line) = line
		{
			map.insert(String::from(line.split(";").nth(1).unwrap()),line);
		}
	}

	for stream in TcpListener::bind("0.0.0.0:1337").unwrap().incoming()
	{
		if let Ok(mut stream) = stream
		{
			let _ = handle_connection(&map,&mut stream);
		}
	}
}

