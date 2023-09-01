use ir_decoder::start;


fn main()  {
    start(|res, count| {
        println!("{:?}, {}", res, count);
    })
}

