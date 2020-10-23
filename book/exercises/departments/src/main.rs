use std::collections::HashMap;
use std::io;

fn main() {
    let mut dir: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        println!("\n[A]dd person to department,\n[D]epartment list,\n[L]ist all people,\n[Q]uit\n");

        let mut select = String::new();
        io::stdin().read_line(&mut select).expect("Failure");
        select = select.to_lowercase();

        if select.starts_with('a') {
            println!("Issue command of form: 'Add [name] to [department]'.");
            let mut command = String::new();
            io::stdin().read_line(&mut command).expect("Failure");

            let mut split = command.split_whitespace();
            let name = match split.nth(1) {
                Some(x) => x.to_string(),
                None => {println!("Malformed command.");
                         continue},
            };
            let dept = match split.nth(1) {
                Some(x) => x.to_string(),
                None => {println!("Malformed command.");
                         continue},
            };

            if dir.contains_key(&dept) {
                dir.get_mut(&dept).unwrap().push(name);
            } else {
                dir.insert(dept, {
                    let mut vec = Vec::new();
                    vec.push(name);
                    vec
                });
            }
        } else if select.starts_with('d') {
            println!("Select a department by name.");
            let mut dept = String::new();
            io::stdin().read_line(&mut dept).expect("Failure");
            dept = dept.split_whitespace().collect();

            match dir.get_mut(&dept) {
                Some(x) => list_dept(x),
                None => println!("Department does not exist."),
            };
        } else if select.starts_with('l') {
            let mut depts = Vec::new();
            dir.keys().for_each(|x| depts.push(x.clone()));
            depts.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

            for dept in depts {
                println!("\n{}", dept);
                list_dept(dir.get_mut(&dept).unwrap());
            }
        } else if select.starts_with('q') {
            break;
        } else {
            println!("Incorrect option.");
        }
    }
}

fn list_dept(list: &mut Vec<String>) {
    println!("-----");

    list.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    for s in list {
        println!("{}", s);
    }
}
