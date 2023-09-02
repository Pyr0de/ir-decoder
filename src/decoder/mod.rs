mod nec1;

#[derive(Debug, PartialEq)]
pub struct IRResultData {
    pub function: usize,
    pub device: usize,
}


#[derive(Debug, PartialEq)]
pub enum IRResult {
    Data(IRResultData),
    Repeat,
    Unknown,
}

pub struct IRDecoder {
    last_message: Option<IRResultData>,
    count: usize,
}

impl IRDecoder {
    pub fn new() -> Self {
        IRDecoder {
            last_message: None,
            count: 0
        }
    }

    pub fn decode(&mut self, raw_ir: &str) -> (&Option<IRResultData>,usize) {
        let ir_decoded = nec1::nec1_decoder(raw_ir);
        match ir_decoded {
            IRResult::Data(data) => {
                self.last_message = Some(data);
                self.count = 0;
                (&self.last_message,self.count)
            },
            IRResult::Repeat => {
                self.count += 1;
                (&self.last_message, self.count)
            },
            IRResult::Unknown => (&None,0)
        }
    
    }
}
