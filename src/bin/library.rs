struct Library {
    books: Vec<Book>,
}

struct Book {
    title: String,
    year: u16,
}

impl Book {
    // This is a constructor, used below.
    fn new(title: &str, year: u16) -> Book {
        Book {
            title: String::from(title),
            year,
        }
    }
}

// Implement the methods below. Update the `self` parameter to
// indicate the method's required level of ownership over the object:
//
// - `&self` for shared read-only access,
// - `&mut self` for unique and mutable access,
// - `self` for unique access by value.
impl Library {
    fn new() -> Library {
        Library { books: Vec::new()}
        // todo!("Initialize and return a `Library` value")
    }

    fn len(&self) -> usize {
        self.books.len()
    }

    fn is_empty(&self) -> bool {
       self.books.len() == 0
    }

    fn add_book(&mut self, book: Book) {
       self.books.push(book);
    }

    fn print_books(&self) {
       for book in self.books.iter() {
        println!("name: {}, year: {}",book.title,book.year);       }
    }

    fn oldest_book(&self) -> Option<&Book> {
        let mut oldest: Option<&Book> = None;
        for book in self.books.iter() {
            if oldest.is_none() || oldest.unwrap().year < book.year {
                oldest = Some(book);
            }
        }
        oldest
    }
}


// This shows the desired behavior. Uncomment the code below and
// implement the missing methods. You will need to update the
// method signatures, including the "self" parameter! You may
// also need to update the variable bindings within main.
fn main() {
    let mut library = Library::new();
    println!("The library is empty: library.is_empty() -> {}", library.is_empty());
    //
    library.add_book(Book::new("Lord of the Rings", 1954));
    library.add_book(Book::new("Alice's Adventures in Wonderland", 1865));
    //
    println!("The library is no longer empty: library.is_empty() -> {}", library.is_empty());
    //
    //
    library.print_books();
    //
    match library.oldest_book() {
       Some(book) => println!("The oldest book is {}", book.title),
       None => println!("The library is empty!"),
    }
    
    println!("The library has {} books", library.len());
    //library.print_books();
}