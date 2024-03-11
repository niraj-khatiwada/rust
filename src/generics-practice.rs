#[derive(Debug)]
enum ServicePriority {
    High,
    Standard,
}

trait Priority {
    fn get_priority(&self) -> ServicePriority;
}

#[derive(Debug)]
struct ImportantGuest;
impl Priority for ImportantGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::High
    }
}

#[derive(Debug)]
struct NormalGuest;
impl Priority for NormalGuest {
    fn get_priority(&self) -> ServicePriority {
        ServicePriority::Standard
    }
}

fn main() {
    print_guest_priority(&ImportantGuest {});
    print_guest_priority(&NormalGuest {});
}

// Here, our function utilizes debug tokens while printing guest but the function has no idea if the guest allows debug or not (ImportantGuest and NormalGuest even after having Debug trait, it's not clear here that the guest parameter will have Debug trait or not ). So we need to manually add std::fmt::Debug trait
fn print_guest_priority<T: Priority + std::fmt::Debug>(guest: &T) {
    println!("{:?} has {:?} priority", guest, guest.get_priority());
}
