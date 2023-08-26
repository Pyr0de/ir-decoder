mod ir_ctl;
mod ir_decoder;

fn main()  {
    ir_ctl::start(|res| {
        println!("{:?}", res);
    })
}

