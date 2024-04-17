struct Ticket;
struct BoardedTicket;

impl Ticket {
    // self instead of &self here since once the ticket is boarded, it'll no longer be available.
    fn board(self) -> BoardedTicket {
        BoardedTicket
    }
}

struct File<'a>(&'a str);

impl<'a> File<'a> {
    fn read(&self) -> &'a str {
        return "reading...";
    }

    fn delete(self) {
        println!("File deleted.")
    }
}

fn main() {
    let ticket = Ticket;
    let boarded_ticket = ticket.board();
    // Since the ticket is already boarded, you cannot board again,
    // ticket.board();

    let file = File("./abc.txt");
    file.read();

    file.delete();

    // You cannot read now since it's already deleted.
    // file.read();
}
