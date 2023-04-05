use std::{
    any,
    io::Read,
    net::{Shutdown, TcpListener, TcpStream}, fs::read,
};

use capnp::serialize_packed;
use serde::{Deserialize, Serialize};
use win32_synthetic_pointer::{
    PointerType::{self, Mouse},
    SyntheticPointer, TouchInput, TouchProperties,
};

use crate::touch_data_capnp::touch_data;

fn main() -> anyhow::Result<()> {
    let addr = "0.0.0.0:1337";
    println!("Starting server...");
    let listener = TcpListener::bind(addr)?;
    println!("Listening on {}", addr);
    for stream in listener.incoming() {
        handle_client(&mut stream?)?;
    }
    Ok(())
}

fn handle_client(stream: &mut TcpStream) -> Result<(), anyhow::Error> {
    println!("Connected to {}", stream.peer_addr()?);

    let mut buf = [0 as u8; 512];
    /*let properties =
        TouchProperties::Pressure | TouchProperties::Orientation | TouchProperties::ContactArea;
    let mut pointer = SyntheticPointer::new(PointerType::Touch(properties), 10)?;*/

    Ok(
        while match stream.read(&mut buf) {
            Ok(0) => {
                println!("Client Closed connection");
                false
            }
            Ok(size) => {
                let data: TouchData = touch_data_from_slice(&buf)?;
                // this creates errors randomly: fix them lol
                let mut input: [Option<TouchInput>; 10] = [None; 10];
                dbg!(&data);

                /*for (i, finger_or_none) in data.fingers.iter().enumerate() {
                    if i < input.len() {
                        if let Some(finger) = finger_or_none {
                            // change these to display resolution
                            let w = 1024f32;
                            let h = 768f32;
                            input[i] = Some(TouchInput {
                                x: map(finger.x, 0f32, data.width as f32, 0f32, w) as i32,
                                y: map(finger.y, 0f32, data.height as f32, 0f32, h) as i32,
                                pressure: (finger.pressure * 1024f32).round() as u32,
                                orientation: Some(finger.orientation as u32),
                                contact_area: None,
                                bind_active: true,
                            })
                        } else {
                            input[i] = None;
                        }

                    }
                }
                pointer.touch_input(&input)?;
                pointer.inject()?;*/
                true
            }
            Err(e) => {
                stream.shutdown(Shutdown::Both).unwrap();
                false
            }
        } {},
    )
}
fn map(value: f32, istart: f32, istop: f32, ostart: f32, ostop: f32) -> f32 {
    return ostart + (ostop - ostart) * ((value - istart) / (istop - istart));
}

fn touch_data_from_slice(buf: &[u8]) -> Result<TouchData, anyhow::Error> {
    let message_reader =
        serialize_packed::read_message(buf, ::capnp::message::ReaderOptions::new())?;
    
    let reader = message_reader.get_root::<touch_data::Reader>()?;

    let mut data = TouchData::default();

    data.width = reader.get_width();
    data.height = reader.get_height();
    data.fingers = [None; 10]; // init all the fields to none
    

    for finger_reader in reader.get_fingers().iter().enumerate() {
        let mut finger = FingerData::default();
        for a in finger_reader.1.iter().enumerate() {
            let index = a.1.get_id() as usize;
            finger.x = a.1.get_x();
            finger.y = a.1.get_y();
            finger.pressure = a.1.get_pressure();
            finger.size = a.1.get_size();
            finger.orientation = a.1.get_orientation();
            finger.touch_major = a.1.get_touch_major();
            finger.touch_minor = a.1.get_touch_minor();
            if (a.1.get_is_present()) {
                data.fingers[index] = Some(finger);
            }
        }
    }
    Ok(data)
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
struct TouchData {
    pub width: i32,
    pub height: i32,
    pub fingers: [Option<FingerData>; 10],
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy)]
struct FingerData {
    pub orientation: f32,
    pub pressure: f32,
    pub size: f32,
    pub touch_major: f32,
    pub touch_minor: f32,
    pub x: f32,
    pub y: f32,
}

mod touch_data_capnp;
