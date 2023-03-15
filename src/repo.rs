use std::fs;
use toml::Value;



pub fn get_books_toml(repo: String) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::blocking::get(format!("https://pastebin.com/raw/QWwB72SH"))?;
    let body = resp.text().unwrap();
    Ok(body)
}

pub fn get_books(toml: String) -> Vec<Book> {
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

pub fn get_repos(path: &str) -> Vec<Repo> {
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

pub struct Repo {

    pub name: String,
    pub url: String,
    pub description: String,
}
#[derive(Debug, Clone)]
pub struct Book {
    pub name: String,
    pub author: String,
    pub publish_date: String,
    pub pages: String,
    pub description: String,
    pub url: String
}