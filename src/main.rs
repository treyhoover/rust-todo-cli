use std::io::{self, Write};

struct Task {
    id: usize,
    description: String,
    done: bool,
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();
    let mut counter: usize = 0;
    
    loop {
        print!("(a)dd, (l)ist, (q)uit, (u)pdate: ");
        io::stdout().flush().unwrap(); // Flush stdout to see the prompt immediately
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "a" => {
                print!("Enter task: ");
                io::stdout().flush().unwrap();
                
                let mut description = String::new();
                io::stdin().read_line(&mut description).unwrap();
                
                counter += 1;
                tasks.push(Task {
                    id: counter,
                    description: description.trim().to_string(),
                    done: false,
                });
            }
            "l" => {
                for task in &tasks {
                    println!("[{}] {}: {}", task.id, task.description, if task.done { "done" } else { "pending" });
                }
            }
            "u" => {
                print!("Enter task ID to toggle: ");
                io::stdout().flush().unwrap();
                
                let mut id_input = String::new();
                io::stdin().read_line(&mut id_input).unwrap();
                
                match id_input.trim().parse::<usize>() {
                    Ok(id) => {
                        if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
                            task.done = !task.done;
                        } else {
                            println!("Task with ID {} not found.", id);
                        }
                    },
                    Err(_) => println!("Invalid ID."),
                }
            }
            "q" => break,
            _ => println!("Unknown command."),
        }
    }
}
