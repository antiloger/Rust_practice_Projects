
use std::io;

struct Todocol{
    id:i32,
    time_set: String,
    reminder: String,
    level: String,
}


fn main() {
    let mut id_count: i32 = 0;

    let mut listoftodo: Vec<Todocol> = Vec::new();

    loop {
        let mut choice= String::new();

        println!("=== Welcome to Rust TODO App ===\n");
        println!("1. Add TODO");
        println!("2. Show ToDo List");
        println!("3.Remove Todo");
        println!("4.Alarm Settings");
        println!("5.Exit\n");

        println!("-> Enter Your Choice:");

        io::stdin().read_line(&mut choice).expect("error ! try again!!");

        if choice.trim() == "1"{
            
            let mut y_n = String::new();

            let mut mark = input(&id_count);

            println!("entering to list?[Y/N]");

            io::stdin().read_line(&mut y_n).expect("error ! try again!!");

            if y_n.trim() == "Y"{
                
                mark.id = id_count;
                listoftodo.push(mark);
            }else {
                break;
            }

            id_count += 1;
            println!("Added!!");
        }
        else if choice.trim() == "2" {
            
            if listoftodo.is_empty(){
                println!("ToDo list is empty");
                continue;
            }

            for i in &listoftodo{
                println!("---{}---", i.id);
                println!("Timeset: {}", i.time_set);
                println!("Reminder: {}", i.reminder);
                println!("level: {}", i.level);
                println!("--------");   
            }

        }
        else if choice == "3"{

        }
        else if choice == "4"{

        }
        else if choice == "5"{

        }
        else{
            println!("Invalid input!!! try again...");
            continue;
        }

    }
}


fn input(id_c: &i32) -> Todocol{



    let mut todolist = Todocol{
        id: *id_c,
        time_set: String::new(),
        reminder: String::new(),
        level: String::new(),
    };

    println!("1. Set the Time (eg:18:00) :");
    io::stdin().read_line(&mut todolist.time_set).expect("Put time, or check it!!!");

    println!("2. Set the Reminder :");
    io::stdin().read_line(&mut todolist.reminder).expect("Put reminder, or check it!!!");

    println!("3. Set the Level (eg: normal: N / importent: I / minor: M) :");
    io::stdin().read_line(&mut todolist.level).expect("Put level, or check it!!!  (eg: normal: N / importent: I / minor: M)");


    todolist
}

