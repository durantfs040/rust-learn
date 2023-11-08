struct Check {
    name: String,
    description: String,
    done: bool,
}

struct Progress {
    name: String,
    description: String,
    progress: u8,
}

enum TodoList {
    Progress(Progress),
    Check(Check),
}

trait Todo {
    fn new(name: String, description: String) -> Self;
    fn show(&self);
    fn edit(&mut self);
}

impl Todo for Check {
    fn new(name: String, description: String) -> Self {
        Check {
            name: name,
            description: description,
            done: false,
        }
    }
    fn show(&self) {
        println!("Check:");
        let done = if self.done { "X" } else { " " };
        println!("[{}] {}:", done, self.name);
        println!("    {}", self.description);
    }
    fn edit(&mut self) {
        self.done = !self.done;
    }
}

impl Todo for Progress {
    fn new(name: String, description: String) -> Self {
        Progress {
            name: name,
            description: description,
            progress: 0,
        }
    }
    fn show(&self) {
        println!("Progress:");
        let done = if self.progress == 100 { "X" } else { " " };
        println!("[{}] {}:", done, self.name);
        println!("    {}", self.description);
        println!("    Progress: {}", self.progress);
    }
    fn edit(&mut self) {
        println!("Set status progress");
        let mut ans = String::new();
        std::io::stdin().read_line(&mut ans).unwrap();
        match ans.trim().parse::<usize>() {
            Ok(n) => {
                if !(n <= 100) {
                    println!("Invalid input");
                    return;
                }
            },
            Err(_) => {
                println!("Invalid input");
                return;
            },
        }
        self.progress = ans.trim().parse::<u8>().unwrap();
    }
}

fn add_new_todo(todos: &mut Vec<TodoList>) {
    let mut ans = String::new();
    let mut name = String::new();
    let mut description = String::new();

    println!("What kind of todo do you want to add?");
    println!("1. Check");
    println!("2. Progress");

    std::io::stdin().read_line(&mut ans).unwrap();

    match ans.trim().parse::<i32>() {
        Ok(1 | 2) => {},
        Err(_) => {
            println!("Invalid input");
            return;
        },
        _ => {
            println!("Unexpected input");
            return;
        },  
    }

    println!("Name: ");
    std::io::stdin().read_line(&mut name).unwrap();
    name = String::from(name.trim());

    println!("Description: ");
    std::io::stdin().read_line(&mut description).unwrap();
    description = String::from(description.trim());

    match ans.trim().parse::<i32>() {
        Ok(1) => {
            let new_todo = Check::new(name, description);
            todos.push(TodoList::Check(new_todo));
        },
        Ok(2) => {
            let new_todo = Progress::new(name, description);
            todos.push(TodoList::Progress(new_todo));
        },
        _ => {},    
    }

}


fn edit_todo(todos: &mut Vec<TodoList>) {
    let mut ans = String::new();
    let mut index = 1;

    if todos.len() == 0 {
        println!("No todo to edit");
        return;
    }
    if todos.len() > 1 {
        println!("Which todo do you want to edit?[1-{}]", todos.len());
        std::io::stdin().read_line(&mut ans).unwrap();
        match ans.trim().parse::<usize>() {
            Ok(n) => {
                index = n;
                if !(index > 0 && index <= todos.len()) {
                    println!("Invalid input");
                    return;
                }
            },
            Err(_) => {
                println!("Invalid input");
                return;
            },
        }
    }
    let todo = &mut todos[index-1];

    match todo {
        TodoList::Check(check) => check.edit(),
        TodoList::Progress(progress) => progress.edit(),
    }
}

fn show_todo(todos: &mut Vec<TodoList>) {
    if todos.len() == 0 {
        println!("No todo to show");
    }
    for i in todos {
        match i {
            TodoList::Check(check) => check.show(),
            TodoList::Progress(progress) => progress.show(),
        };
    }
}

fn main() {
    let mut todos : Vec<TodoList> = Vec::new();

    loop {
        let mut ans = String::new();
        println!("What do you want to do");
        println!("1. Add new todo");
        println!("2. Edit todo");
        println!("3. Show todo");
        println!("4. Exit");

        std::io::stdin().read_line(&mut ans).unwrap();

        match ans.trim().parse::<i32>() {
            Ok(1) => add_new_todo(&mut todos),
            Ok(2) => edit_todo(&mut todos),
            Ok(3) => show_todo(&mut todos),
            Ok(4) => break,
            Err(_) => println!("Invalid input"),
            _ => println!("Unexpected input"),    
        }

       
    }
}