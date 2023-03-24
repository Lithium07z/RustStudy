/*
Topic : Rust Implementing Iterator
*/
#[derive(Debug)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>
}

#[derive(Debug)]
enum LibraryType {
    City,
    Country
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            books: Vec::new()
        }
    }
}

impl Iterator for Library {
    type Item = String; // iterator 타입 지정, Associated Types이므로 직접 Iterator를 구현할 때 반드시 구현해줘야 함

    fn next(&mut self) -> Option<String> { // next는 Iterator의 Required Methods임 
        match self.books.pop() {
            Some(book_title) => Some(book_title + " is found"), // String + &str
            None => None
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the Darksword"); // add some books
    my_library.add_book("Demian - die Geschichte einer Jugend");
    my_library.add_book("구운몽");
    my_library.add_book("吾輩は猫である");

    println!("{:?}", my_library.books);

    for item in my_library { // Iterator를 직접 구현해줬기에 출력 가능함
        println!("{item}")
    }
}