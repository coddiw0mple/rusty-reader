use std::fs;
use toml::Value;
use dialoguer::{
    FuzzySelect,
    theme::ColorfulTheme
};

struct Repo {

    name: String,
    url: String,
    description: String,
}
#[derive(Debug, Clone)]
struct Book {
    name: String,
    author: String,
    publish_date: String,
    pages: String,
    description: String,
    url: String
}
fn main() {
    let repos = get_repos("repos.toml");
    let mut books_names_vec: Vec<String> = Vec::new();
    let mut books_vec: Vec<Book> = Vec::new();
    for repo in repos {
        let toml: String = get_books_toml(repo.url).unwrap_or("".to_owned());
        let books = get_books(toml);
        for book in books {
            books_vec.push(book.clone());
            books_names_vec.push(book.name);

        }
    }
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("What do you want to read?")
        .default(0)
        .items(&books_names_vec[..])
        .interact()
        .unwrap();
    let selected_index = books_names_vec.iter().position(|s| s == &books_names_vec[selection]).unwrap();
    println!("{}", &reqwest::blocking::get(&books_vec[selected_index].url).unwrap().text().unwrap());
}

fn get_books_toml(repo: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!("https://pastebin.com/raw/QWwB72SH"))?;
    let body = resp.text().unwrap();
    Ok(body)
}

fn get_books(toml: String) -> Vec<Book> {
    let value: toml::Value = toml::from_str(&toml).expect("Failed to parse TOML");

    // Extract the array of books from the Value
    let books = match value.get("book") {
        Some(Value::Array(books)) => books,
        _ => panic!("No 'book' array found"),
    };
    let mut book_struct: Vec<Book> = Vec::new();

    // Print information about each book
    for book in books {
        let name = book.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let author = book.get("author").and_then(|v| v.as_str()).unwrap_or("");
        let publish_date = book.get("publish_date").and_then(|v| v.as_str()).unwrap_or("");
        let pages = book.get("pages").and_then(|v| v.as_integer()).unwrap_or(0);
        let description = book.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let url = book.get("url").and_then(|v| v.as_str()).unwrap_or("");

        let book_lol = Book {
            name: name.to_owned(),
            author: author.to_owned(),
            publish_date: publish_date.to_owned(),
            pages: pages.to_string(),
            description: description.to_owned(),
            url: url.to_owned(),
        };
        book_struct.push(book_lol);
    }
    book_struct
}

fn get_repos(path: &str) -> Vec<Repo> {
    let contents = fs::read_to_string(path).expect("Error reading config file");
    let value = contents.parse::<Value>().expect("Error parsing config file");
    let repos = value
        .get("repo")
        .expect("Missing 'repo' section in config file")
        .as_array()
        .expect("'repo' section in config file is not an array");

    let mut result = Vec::new();

    for repo in repos {
        let name = repo
            .get("name")
            .expect("Missing 'name' field in repository")
            .as_str()
            .expect("'name' field in repository is not a string")
            .to_string();

        let url = repo
            .get("url")
            .expect("Missing 'url' field in repository")
            .as_str()
            .expect("'url' field in repository is not a string")
            .to_string();

        let description = repo
            .get("description")
            .expect("Missing 'description' field in repository")
            .as_str()
            .expect("'description' field in repository is not a string")
            .to_string();

        let new_repo = Repo {
            name,
            url,
            description,
        };

        result.push(new_repo);
    }

    result
}