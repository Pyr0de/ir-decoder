use irp::{Irp, InfraredData};
use super::{IRResultData, IRResult};

pub fn nec_decoder(str: &str) -> IRResult {
    //IRP notation for NEC protocol
    let nec_irp = Irp::parse(r#"{38.4k,564}<1,-1|1,-3>(16,-8,D:8,S:8,F:8,~F:8,1,^108m,(16,-4,1,^108m)*)[D:0..255,S:0..255=255-D,F:0..255]"#)
        .expect("Error: Parsing IRP Notation")
        .compile()
        .expect("Error: Compiling NFA");
    
    let (tolerance, relative_tolerance, max_gap) = (200,30,11000);
    let mut decoder = nec_irp.decoder(tolerance, relative_tolerance, max_gap);

    for ir in InfraredData::from_rawir(&str.replace("# timeout ", "-")).unwrap() {
        decoder.input(ir);
    }
     
    if let Some((_event,res)) = decoder.get() {
        return IRResult::Data(IRResultData {
            function: res["F"] as usize,
            device: res["T"] as usize,
            subdevice: res["S"] as usize,
        })
    
    } 

    IRResult::Unknown
    //println!("{}, {:?}", event, res);

    //let ir = Message::parse("+9039 -2161 +648").expect("error");

    //println!("{}",ir.);
}
