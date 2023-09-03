use ir_decoder::start;

fn main()  {
    start(|res, count| {
        println!("Device: {},Function: {}, Count: {}", res.device, res.function, count);
    })
}

