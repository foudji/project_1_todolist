use clap::Parser;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string},
    io::{self},
};

#[derive(Serialize, Deserialize)]
struct Todo {
    content: String,
}

#[derive(Parser)]
struct Flag {
    #[arg(long, short)]
    delete: Option<usize>,
}

fn main() {
    let flag: Flag = Flag::parse();

    let mut todos: Vec<Todo> = match read_to_string("todolist.json") {
        Ok(file_content) => {
            serde_json::from_str(&file_content).expect("Impossible de deserialiser JSON")
        }
        Err(_) => Vec::new(),
    };

    if let Some(index) = flag.delete {
        if index > 0 && index <= todos.len() {
            todos.remove(index - 1);
            let _ = fs::write(
                "todolist.json",
                serde_json::to_string(&todos).expect("Impossible de créer un fichier pour la todo"),
            );
        }
    } else {
        println!("Veuillez entrer une tâche que vous souhaitez accomplir :");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Impossible d'accéder à l'input de l'utilisateur");

        let input = input.trim();

        todos.push(Todo {
            content: input.to_string(),
        });

        let _ = fs::write(
            "todolist.json",
            serde_json::to_string(&todos).expect("Impossible de créer un fichier pour la todo"),
        );
    }
}
