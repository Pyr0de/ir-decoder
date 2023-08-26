mod nec1;

#[derive(Debug)]
pub struct IRResultData {
    pub function: usize,
    pub device: usize,
    pub subdevice: usize,
}

pub enum IRResult {
    Data(IRResultData),
    Repeat,
    Unknown,
}

pub struct IRDecoder {
    last_message: Option<IRResultData>,
}

impl IRDecoder {
    pub fn new() -> Self {
        IRDecoder {
            last_message: None
        }
    }

    pub fn decode(&mut self, raw_ir: &str) -> &Option<IRResultData> {
        let ir_decoded = nec1::nec_decoder(raw_ir);
        match ir_decoded {
            IRResult::Data(data) => {
                self.last_message = Some(data);
                &self.last_message
            },
            IRResult::Repeat => &self.last_message,
            IRResult::Unknown => &None
        }
    
    }
}
