extern crate todo_rs;

use todo_rs::TaskList;
use std::io::{self, Write};

enum Action {
    Add(String),
    Remove(usize),
    Toggle(usize),
    Quit,
}

fn main() {
    let mut task_list = TaskList::new(ask_list_name());

    loop {
        display_list(&task_list);

        match ask_action() {
            Action::Add(name) => {
                task_list.add(name);
            },
            Action::Remove(num) => {
                task_list.remove(num - 1);
            },
            Action::Toggle(num) => {
                task_list.toggle(num - 1);
            },
            Action::Quit => {
                break;
            }
        }
    }
}

fn ask_list_name() -> String {
    print!("Entrez le nom de la liste : ");
    io::stdout().flush().unwrap();
    let mut list_name = String::new();
    io::stdin().read_line(&mut list_name).ok().expect("read error");

    list_name.trim().to_string()
}

fn ask_action() -> Action {
    use std::io;
    println!("\tA) Ajouter une tâche");
    println!("\tD) Supprimer une tâche");
    println!("\tX) Cocher/Décocher une tâche");
    println!("\tQ) Quitter");
    print!("Que souhaitez-vous faire : ");
    io::stdout().flush().unwrap();

    let mut action = String::new();
    io::stdin().read_line(&mut action).ok().expect("read error");
    match action.trim() {
        "A" => {    // Ajout
            print!("Entrez le nom de la nouvelle tâche : ");
            io::stdout().flush().unwrap();
            let mut task = String::new();

            io::stdin().read_line(&mut task).ok().expect("read error");

            Action::Add(task.trim().to_string())
        },
        "D" => {
            print!("Entrez le numéro de la tâche à supprimer : ");
            io::stdout().flush().unwrap();
            let mut num = String::new();

            io::stdin().read_line(&mut num).ok().expect("read error");

            Action::Remove(num.trim().parse().ok().expect("parse error"))
        },
        "X" => {
            print!("Entrez le numéro de la tâche à cocher/décocher : ");
            io::stdout().flush().unwrap();
            let mut num = String::new();

            io::stdin().read_line(&mut num).ok().expect("read error");

            Action::Toggle(num.trim().parse().ok().expect("parse error"))
        },
        "Q" => {
            println!("à bientôt !");
            Action::Quit
        },
        _ => {
            println!("Action inconnue : {}", action);
            ask_action()
        }
    }
}

fn display_list(list: &TaskList) {
    use todo_rs::TaskStatus;
    println!("\t{}", list.name);

    for (i, task) in list.tasks.iter().enumerate() {
        println!("{}\t{}. {}", match task.status {
            TaskStatus::DONE => "X",
            TaskStatus::TODO => ""
        }, i + 1, task.name);
    }

    println!("\t----------");
}