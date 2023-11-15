// struct BusTicket;
//
// struct BoardedBusTicket;
//
// impl BusTicket {
//     fn board(self) -> BoardedBusTicket {
//         return BoardedBusTicket;
//     }
// }
//
// fn main() {
//     let ticket = BusTicket;
//     ticket.board(); // The bust ticket as moved, so it won't be available after here.
//     ticket.board(); // Compile error
// }
//
//


/*
    Employee Onboarding System
    The type will make sure that after each transition, the previous state will be lost.
*/


#[derive(Debug)]
enum State {
    Agreement,
    Signature,
    Training,
    Passed(u8),
    Failed(u8),
}

// Here CurrentState is generic
struct Employee<CurrentState> {
    name: String,
    current_state: CurrentState,
}

impl<T> Employee<T> {
    fn transition<NextState>(self, next_state: NextState) -> Employee<NextState> {
        return Employee { name: self.name, current_state: next_state };
    }
}

impl Employee<State> {
    fn new(name: &str) -> Self {
        return Self { name: String::from(name), current_state: State::Agreement };
    }

    fn read_agreement(self) -> Employee<State> {
        return self.transition(State::Signature);
    }

    fn sign(self) -> Employee<State> {
        return self.transition(State::Training);
    }

    fn train(self, score: u8) -> Result<Employee<State>, Employee<crate::State>> {
        if score <= 7 {
            return Err(self.transition(State::Passed(score)));
        }
        return Ok(self.transition(State::Failed(score)));
    }
}

fn main() {
    let employee = Employee::new("Niraj");
    let status = employee.read_agreement().sign().train(10);

    match status {
        Ok(emp) => println!("Accepted with score {:?}", emp.current_state),
        Err(emp) => println!("Rejected with score {:?}", emp.current_state)
    }
}