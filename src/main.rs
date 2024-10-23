use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string}, io::{self, Read}, path, sync::TryLockResult
};

#[derive(Serialize, Deserialize)]
struct Todo {
    content: String,
}

fn main() {
    println!("Veuillez entrer une tâche que vous souhaitez accomplir :");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Impossible d'accéder à l'input de l'utilisateur");

    let mut todos: Vec<Todo> = match read_to_string("todolist.json") {
        Ok(file_content) => {
            serde_json::from_str(&file_content).expect("Impossible de deserialiser JSON")
        }
        Err(_) => Vec::new(),
    };

    let input = input.trim();
    
    if input.contains("--delete") {
        let condition = input.split(" ").last().expect("Cannot find the task");
        let todo_number: usize = condition.parse().expect("Cannot convert to number");
        todos.remove(todo_number - 1);
    };

    todos.push(Todo {
        content: input.to_string(),
    });

    let _ = fs::write(
        "todolist.json",
        serde_json::to_string(&todos).expect("Impossible de créer un fichier pour la todo"),
    );



}
