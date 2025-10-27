fn match_expression() {
    let num = 100;

    match num {
        0..100 => println!("less than 100"),
        _ => println!("more than or equal to 100"),
    }
}
