struct Commit {
    pub id: String,
    pub message: String,
    pub author: String,
    pub date: String,
}

impl Commit {
    pub fn new(id: String, message: String, author: String, date: String) -> Commit {
        Commit {
            id,
            message,
            author,
            date,
        }
    }

    pub fn print_commit(&self) {
        println!("commit {}", self.id);
        println!("Author: {}", self.author);
        println!("Date: {}", self.date);
        println!();
        println!("    {}", self.message);
    }
}
