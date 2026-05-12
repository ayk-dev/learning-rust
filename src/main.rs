#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast (u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => format!("Book: '{}' by {}", title, author),
            Media::Movie { title, director } => format!("Movie: '{}' directed by {}", title, director),
            Media::Audiobook { title } => format!("Audiobook: '{}'", title),
            Media::Podcast (episode_num) => format!("Podcast: {} episode", episode_num),
            Media::Placeholder => format!("Placeholder"),
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add_media(&mut self, media: Media) {
        self.items.push(media);
    }

    fn list_media(&self) {
        for media in &self.items {
            println!("{}", media.description());
        }
    }
}

fn print_media(media: &Media) {
    println!("{}", media.description());
}

fn main() {
    let audiobook = Media::Audiobook {
        title: String::from("The Rust Programming Language"),
    };
    print_media(&audiobook);

    let good_movie = Media::Movie {
        title: String::from("Inception"),
        director: String::from("Christopher Nolan"),
    };
    print_media(&good_movie);

    let good_book = Media::Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik and Carol Nichols"),
    };
    print_media(&good_book);

    audiobook.description();
    good_movie.description();
    good_book.description();

    let mut catalog = Catalog::new();
    catalog.add_media(audiobook);
    catalog.add_media(good_movie);
    catalog.add_media(good_book);
    
    println!("{:#?}", catalog);
    
    match catalog.items.get(0) {
        Some(value) => println!("Item: {:#?}", value),
        None => println!("Nothing at that index"),
    }


}