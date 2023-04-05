use std::{
    any,
    io::Read,
    net::{Shutdown, TcpListener, TcpStream},
};

use serde::{Deserialize, Serialize};
use win32_synthetic_pointer::{
    PointerType::{self, Mouse},
    SyntheticPointer, TouchInput, TouchProperties,
};

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

fn touch_data_from_slice(buf: &[u8; 512]) -> Result<TouchData, anyhow::Error> {
    let mut data = TouchData::default();
    let mut count = 0;
    data.width = rmp::decode::read_u16(&mut &buf[count..]).unwrap() as i32;
    count += 2;
    data.height = rmp::decode::read_u16(&mut &buf[count..]).unwrap() as i32;
    count += 3;
    let size = rmp::decode::read_array_len(&mut &buf[count..]).unwrap();
    dbg!(size);
    count += 3;
    data.fingers = [None; 10]; // init all the fields to none

    // if there are more than 10 fingers, return None for all of them, as we can't handle that
    // windows doesnt support it and we risk trying to read past the buffer bounds
    if size > 10 {
        return Ok(data);
    }

    for _ in 0..size {
        let mut finger = FingerData::default();
        let index: usize = rmp::decode::read_u16(&mut &buf[count..]).unwrap() as usize;
        count += 3;
        let is_gone = rmp::decode::read_bool(&mut &buf[count..]).unwrap();
        count += 1;
        finger.x = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        count += 5;
        finger.y = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        count += 5;
        finger.pressure = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        count += 5;
        finger.orientation = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        count += 5;
        finger.size = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        count += 5;
        finger.touch_major = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        count += 5;
        finger.touch_minor = rmp::decode::read_f32(&mut &buf[count..]).unwrap();
        if !is_gone {
            data.fingers[index] = Some(finger);
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
