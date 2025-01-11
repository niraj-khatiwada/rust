#[derive(Debug)]
enum ResultType<T, U> {
    OkType(T),
    ErrorType(U),
}

impl<T, U> ResultType<T, U> {
    fn unwrap(&self) -> &T {
        match self {
            Self::OkType(val) => val,
            Self::ErrorType(_) => panic!("Error"),
        }
    }
}

fn main() {
    let rs1: ResultType<&str, &str> = ResultType::OkType("Rust");
    println!("{:?}", rs1);
    let rs2: ResultType<&str, &str> = ResultType::ErrorType("Rust");
    println!("{:?}", rs2);

    let rs = rs1.unwrap();
    dbg!(rs);
    let rs = rs2.unwrap();
    dbg!(rs);
}
