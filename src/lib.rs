pub mod decoder;

use std::{process::{Command, Stdio}, io::{BufReader, BufRead}};

use decoder::{IRResultData, IRDecoder}; 

pub fn start(call: fn(&IRResultData, usize)) {
    let mut ir_ctl_child = Command::new("ir-ctl")
        .arg("-r")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error: Cannot run ir-ctl -r");

    let ir_ctl_output = BufReader::new(ir_ctl_child.stdout.take().unwrap());

    let mut ir_decoder = IRDecoder::new(); 

    ir_ctl_output.lines().for_each(|line| {
        if let (Some(decoded), count) = ir_decoder.decode(line.unwrap().as_str()) {
            call(decoded, count);
        }

    })
}
