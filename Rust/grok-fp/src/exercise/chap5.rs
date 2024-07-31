#[derive(Debug)]
struct Book {
    title:String,
    authors: Vec<String>
}

impl Book {
    fn new(title:String,authors:Vec<String>) -> Book {
        Book {
            title,
            authors
        }
    }
}

#[derive(Debug)]
struct Movie {
    title:String
}

pub fn book_adaptations(author: &str) -> Vec<Movie> {
    if author == "Tolkien" {
        return vec![
            Movie { title: "An Unexpected Journey".to_string() },
            Movie { title: "The Desolation of Smaug".to_string() },
        ];
    }
    vec![]
}

pub fn recommended_books(friend:String) -> Vec<Book> {
    let scala = vec![
        Book::new("FP in Scala".to_string(), vec!["Chiusano".to_string(),"Bjarnason".to_string()]),
        Book::new("Get Programing with Scala".to_string(), vec!["Sfregola".to_string()])
    ];

    let fiction = vec![
        Book::new("Harry Potter".to_string(), vec!["Rowling".to_string()]),
        Book::new("The Lord of the Rings".to_string(), vec!["Tolkien".to_string()])
    ];

    if friend == "Alice" {
        return scala;
    } else if friend == "Bob" {
        return  fiction;
    }
    vec![]
}

pub fn recommendation_feed(books: Vec<Book>) -> Vec<String> {
    books.iter()
        .flat_map(|book| {
            book.authors.iter().flat_map(move |author| {
                book_adaptations(author).into_iter().map(move |movie| {
                    format!("You may like {} because you like {}'s {}", movie.title, author, book.title)
                })
            })
        })
        .collect()
}

pub fn run() {
    println!("chapter5");

    let books = vec![Book::new("FP in Scala".to_string(), vec!["Chiusano".to_string(),"Bjarnason".to_string()]),
        Book::new("The Hobbit".to_string(), vec!["Tolkien".to_string()])];

    println!("{:?}",recommendation_feed(books));
}