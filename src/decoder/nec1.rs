use irp::{Irp, InfraredData};
use super::{IRResultData, IRResult};

pub fn nec1_decoder(str: &str) -> IRResult {
    let (tolerance, relative_tolerance, max_gap) = (200,30,11000);
    
    let rawir = InfraredData::from_rawir(&str.replace("# timeout ", "-")).unwrap();
    
    if check_repeat(&rawir, tolerance) {
        return IRResult::Repeat;
    }



    //IRP notation for NEC protocol
    let nec_irp = Irp::parse(r#"{38.4k,564}<1,-1|1,-3>(16,-8,D:8,S:8,F:8,~F:8,1,^108m,(16,-4,1,^108m)*)[D:0..255,S:0..255=255-D,F:0..255]"#)
        .expect("Error: Parsing IRP Notation")
        .compile()
        .expect("Error: Compiling NFA");
    

    let mut decoder = nec_irp.decoder(tolerance, relative_tolerance, max_gap);

    for ir in rawir{
        decoder.input(ir);
    }
    
    if let Some((_event,res)) = decoder.get() {
        return IRResult::Data(IRResultData {
            function: res["F"] as usize,
            device: res["D"] as usize,
        })
    
    } 
    IRResult::Unknown
}

fn check_repeat(data: &Vec<InfraredData>, tolerance: u32) -> bool {
    if 
        deviation(data[0], 9000) <  tolerance &&
        deviation(data[1], 2250) < tolerance &&
        deviation(data[2], 560) < tolerance
    {
        return true;
    }
    false
}

fn deviation(data: InfraredData, num2: u32) -> u32 {
    let num1 = match data {
        InfraredData::Gap(d) => d,
        InfraredData::Flash(d) => d,
        _ => 0
    };
    if num1 > num2 {
        return num1-num2
    }
    num2-num1
}


#[test]
fn decode_single() {
    let data = "+8994 -4439 +608 -1655 +586 -501 +612 -1655 +585 -502 +614 -500 +611 -502 +611 -1657 +583 -501 +611 -503 +612 -1656 +582 -503 +612 -1656 +583 -1656 +582 -1657 +584 -502 +613 -1657 +583 -1653 +609 -478 +613 -501 +612 -503 +611 -503 +612 -1657 +583 -503 +610 -503 +612 -504 +611 -1655 +583 -1658 +582 -1655 +584 -1657 +582 -503 +612 -1656 +583 -1655 +584 # timeout 20434";

    assert_eq!(
        nec1_decoder(data),
        IRResult::Data(IRResultData {
            function: 33,
            device: 69,
        })
    );
}

#[test]
fn decode_repeat() {
    let data = "+8999 -2225 +584 # timeout 16754";
    assert_eq!(
        nec1_decoder(data),
        IRResult::Repeat
    )
}
