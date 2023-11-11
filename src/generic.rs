trait Seat {
    fn print_type(&self);
}

#[derive(Debug, Clone, Copy)]
struct Ticket<T: Seat> {
    seat: T,
}


#[derive(Debug, Clone, Copy)]
enum ConcertSeat {
    FrontRow,
    MidSection(u32),
    BackSeat(u32),
}


impl Seat for ConcertSeat {
    fn print_type(&self) {
        println!("Concert Seat {:?}", self)
    }
}

#[derive(Debug, Clone, Copy)]
enum AirplaneSeat {
    BusinessClass,
    EconomyClass,
    FirstClass,
}

impl Seat for AirplaneSeat {
    fn print_type(&self) {
        println!("Airplane Seat {:?}", self)
    }
}


fn ticket_info<T: Seat>(ticket: Ticket<T>) {
    ticket.seat.print_type()
}

fn main() {
    let airplane_ticket = Ticket { seat: AirplaneSeat::BusinessClass };
    ticket_info(airplane_ticket);


    let concert_ticket = Ticket { seat: ConcertSeat::FrontRow };
    ticket_info(concert_ticket);
}





