use bevy::prelude::*;
use midir::{Ignore, MidiInput};
use std::sync::{
    mpsc::{self, Receiver},
    Arc, Mutex,
};
use std::thread;

pub enum MidiEvent {
    NoteOn(u8, u8),  // (note, velocity)
    NoteOff(u8, u8), // (note, velocity)
}

#[derive(Resource, Clone)]
pub struct MidiReceiver(Arc<Mutex<Receiver<MidiEvent>>>);

pub fn midi_key_listener(mut commands: Commands, // mut ev_levelup: EventWriter<LevelUpEvent>,
) {
    let (tx, rx) = mpsc::channel();
    let rx = Arc::new(Mutex::new(rx));
    thread::spawn(move || {
        let mut midi_in = MidiInput::new("bevy-midi-input").expect("Failed to open MIDI input");
        midi_in.ignore(Ignore::None);
        let in_ports = midi_in.ports();
        if in_ports.is_empty() {
            println!("No MIDI input ports found.");
            return;
        }
        let in_port = &in_ports[1];
        println!("Opening MIDI port: {}", midi_in.port_name(in_port).unwrap());
        let _conn_in = midi_in.connect(
            in_port,
            "bevy-midi-read",
            move |_, message, _| {
                println!("Received MIDI message: {:?}", message);

                if message.len() >= 3 {
                    match message[0] & 0xF0 {
                        0x90 => {
                            // Note On
                            let note = message[1];
                            let velocity = message[2];
                            let _ = tx.send(MidiEvent::NoteOn(note, velocity));
                        }
                        0x80 => {
                            // Note Off
                            let note = message[1];
                            let velocity = message[2];
                            let _ = tx.send(MidiEvent::NoteOff(note, velocity));
                        }
                        _ => {}
                    }
                }
            },
            (),
        );
        // Keep thread alive
        loop {
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    commands.insert_resource(MidiReceiver(rx));
}
