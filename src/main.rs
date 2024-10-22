use std::io::{self, BufRead, Write};
use std::fs::OpenOptions;

fn main() -> std::io::Result<()> {
    let stdin = io::stdin();
    let mut read = stdin.lock();
    let mut input = String::new();
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("todolist.txt")?;

    println!("Veuillez entrer une liste de taches que vous souhaitez accomplir. (OK pour terminer la liste)");

    while input.trim() != "OK" {
        input.clear();
        read.read_line(&mut input)?;

        if input.trim() != "OK" {
            println!("À effectuer  : - {}", input.trim());
            writeln!(file, "{}", input.trim())?;
        }
    }

    println!("Liste de tâches enregistrée dans todolist.txt");
    Ok(())
}