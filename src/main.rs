use dialoguer::{
    FuzzySelect,
    theme::ColorfulTheme
};


//use crate::reader::Cli;
use crate::repo::{Repo, Book, get_repos, get_books, get_books_toml};

pub mod reader;
pub mod repo;

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