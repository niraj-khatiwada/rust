use tracing::{debug, info, trace};

fn main() {
    print_message("Hello World");
    trace!("Printing message now");
}

#[tracing::instrument]
fn print_message(msg: &str) {
    info!("Printing message now");
    debug!("Message is: {}", msg);
}