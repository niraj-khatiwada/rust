// TypeState Pattern: Convert the type into another type and destroy itself.
// Eg: When a file is deleted, you can no longer access the file owner.
struct File<'a> {
    path: &'a str,
}

impl File<'_> {
    fn read(&self) {
        // Read file content
    }

    fn delete(self) {
        // Delete the file
    }
}

fn main() {
    let file = File { path: "./abc.txt" };
    file.read();
    file.read();
    file.read();
    file.delete();
    file.read(); // Cannot read now
}
