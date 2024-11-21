#[derive(Debug)]
struct Library {
    name: String,
    books: BookCollection,
}

#[derive(Debug, Clone)]
struct BookCollection(Vec<String>);

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.0.push(book.to_string());
    }

    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            books: BookCollection(Vec::new()),
        }
    }

    fn get_books(&self) -> BookCollection {
        self.books.clone()
    }
}

impl Iterator for BookCollection {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.0.pop() {
            Some(book) => {
                println!("Accessing book: {}", book);
                Some(book)
            }
            None => {
                println!("Out of books at the library!");
                None
            }
        }
    }
}

fn main() {
    let mut my_lib = Library::new("Calgary");
    my_lib.add_book("Learn Rust in a Month of Lunches");
    my_lib.add_book("Rust in Action");
    my_lib.add_book("Rust Web Development");
    my_lib.add_book("Rust Apps, Services and Servers");
    my_lib.add_book("Code Like a Pro in Rust");
    my_lib.add_book("Idiomatic Rust");
    my_lib.add_book("Advanced Algorithms and Data Structures");

    for item in my_lib.get_books() {
        println!("{}\n", item);
    }
}
