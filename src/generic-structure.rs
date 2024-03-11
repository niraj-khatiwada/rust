// Structure having generics

#[derive(Debug)]
struct Person<T, U> {
    name: T,
    age: U,
}

trait Seat {
    fn show(&self);
}

struct Ticket<T: Seat> {
    seat: T,
}

enum AirlineTicket {
    BusinessClass,
    EconomyClass,
}
impl Seat for AirlineTicket {
    fn show(&self) {
        match self {
            AirlineTicket::BusinessClass => println!("Business class ticket"),
            AirlineTicket::EconomyClass => println!("Economy class ticket"),
        }
    }
}

enum ConcertTicket {
    FrontRow,
    BackRow,
}

impl Seat for ConcertTicket {
    fn show(&self) {
        match self {
            ConcertTicket::FrontRow => println!("Front row ticket"),
            ConcertTicket::BackRow => println!("Back row ticket"),
        }
    }
}

fn main() {
    let person: Person<String, u16> = Person {
        name: String::from("Niraj"),
        age: 26,
    };

    println!("{:?}", person.name);
    println!("{:?}", person.age);

    // Tickets
    let airline_ticket: Ticket<AirlineTicket> = Ticket {
        seat: AirlineTicket::BusinessClass,
    };
    airline_ticket.seat.show();

    let concert_ticket: Ticket<ConcertTicket> = Ticket {
        seat: ConcertTicket::BackRow,
    };
    concert_ticket.seat.show();

    //
    print_ticket_seat(&airline_ticket);
    print_ticket_seat(&concert_ticket);
}

fn print_ticket_seat<T: Seat>(ticket: &Ticket<T>) {
    ticket.seat.show()
}
