use std::io::{self, Write};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("todolist.txt")?;

    println!("Veuillez entrer une tâche que vous souhaitez accomplir :");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let todo = input.trim();

    if !todo.is_empty() {
        println!("À effectuer : - {}", todo);
        writeln!(file, "{}", todo)?;
        println!("Tâche ajoutée à todolist.txt");
    } else {
        println!("Aucune tâche n'a été ajoutée.");
    }

    Ok(())
}