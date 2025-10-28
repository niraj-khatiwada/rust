struct File<'a>(&'a str);

impl<'a> File<'a> {
    fn read(&self) {
        println!("Reading file...")
    }
    fn delete(self) {
        // Take notice of self here instead of &self
        println!("Deleting file {}...", self.0);
        println!("File deleted...")
    }
}

fn main() {
    let file = File("./fizzbuzz.rs");

    file.read();
    file.delete();
    // file.read(); // Compilation error here since we cannot read file after delete.
}
