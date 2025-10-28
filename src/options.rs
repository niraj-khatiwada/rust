fn options() {
    // let some_num: Option<u8> = None;
    let some_num: Option<u8> = Some(100);

    let add_1_if_num = some_num.map(|num| num + 1); // only adds 1 if there's some value.

    println!("Final {:?}", add_1_if_num);

    // let rs: Result<u8, ()> = Err(());
    let rs: Result<u8, ()> = Ok(100);

    let add_1_if_num = rs.map(|num| num + 1); // only adds 1 if the result is OK

    println!("Final {:?}", add_1_if_num);

    // Option combinator
    // let val = None;
    let val = Some(100);

    println!("{}", val.is_some());
    println!("{}", val.is_none());

    let filtered = val.filter(|x| x % &10 == 0); // Filter out the some value. If the filter condition does not match, it'll return None.
    println!("{:?}", filtered);

    let default_option = val.or_else(|| Some(99)); // Some(99) will be selected when val is None
    println!("{:?}", default_option);

    let default_val = val.unwrap_or_else(|| 99); // 99 will be selected when val is None. Final value will be unwrapped
    println!("{:?}", default_val);
}
