use std::error::Error;
use std::io::{stdin, stdout, Write};

use midir::{Ignore, MidiInput, MidiInputPort};

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err),
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    // Get an input port (read from console if multiple are available)
    let in_ports = midi_in.ports();

    for port in &in_ports {
        println!("Input port: {}", midi_in.port_name(port).unwrap());
    }

    // let in_port = match in_ports.len() {
    //     0 => return Err("no input port found".into()),
    //     1 => {
    //         println!(
    //             "Choosing the only available input port: {}",
    //             midi_in.port_name(&in_ports[0]).unwrap()
    //         );
    //         &in_ports[0]
    //     }
    //     _ => {
    //         println!("\nAvailable input ports:");
    //         for (i, p) in in_ports.iter().enumerate() {
    //             println!("{}: {}", i, midi_in.port_name(p).unwrap());
    //         }
    //         print!("Please select input port: ");
    //         stdout().flush()?;
    //         let mut input = String::new();
    //         stdin().read_line(&mut input)?;
    //         in_ports
    //             .get(input.trim().parse::<usize>()?)
    //             .ok_or("invalid input port selected")?
    //     }
    // };

    // let in_port_name = midi_in.port_name(&in_port)?;

    // println!("\nOpening connection: in_port_name: {:#?}", in_port_name);

    let in_port_name = "32:0".to_string();

    let load_midi_port = midi_in.find_port_by_id(in_port_name.clone());

    if let Some(in_port) = load_midi_port {
        println!("Found port");

        // _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
        let _conn_in = midi_in.connect(
            &in_port,
            "midir-read-input",
            move |stamp, message, _| {
                println!("{}: {:?} (len = {})", stamp, message, message.len());
            },
            (),
        )?;

        println!(
            "Connection open, reading input from '{}' (press enter to exit) ...",
            in_port_name
        );

        input.clear();
        stdin().read_line(&mut input)?; // wait for next enter key press

        println!("Closing connection");
    } else {
        println!("Port not found");
    }

    Ok(())
}
