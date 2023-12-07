use std::io;
use std::collections::HashMap;
use std::vec::Vec;


fn handle_list_request(text: &mut String, company:&mut HashMap<String, Vec<String>>) {
    let splits = text.split_whitespace();
    let mut count = 0;
    let mut department:String = String::new();
    for (i, value) in splits.enumerate() {
        if i == 1 {
            department = value.to_string();
        }
        count += 1;
    }
    if count != 2{
        println!("Wrong Input!Please enter 2 letters seperated by whitespace");
        return;
    }
    let names = company.get(&department).unwrap();
    println!("{} {:?}", department, names);

}
fn handle_add_request(text: &mut String, company:&mut HashMap<String, Vec<String>>) {
    
    let splits = text.split_whitespace();
    let mut count = 0;
    let mut name:String = String::new();
    let mut department:String = String::new();
    for (i, value) in splits.enumerate() {
        if i == 1 {
            name = value.to_string();
        }else if i == 3 {
            department = value.to_string();
        }
        count += 1;
    }
    if count !=4 {
        println!("Wrong Input!Please enter 4 letters seperated by whitespace");
        return;
    }
    let com_department = company.entry(department).or_insert(vec![]);
    com_department.push(name);    
}

fn list_all_company_department(company:& HashMap<String, Vec<String>>) {
    let my_company = company.clone();
    let mut keys:Vec<String> = my_company.into_keys().collect();
    keys.sort_unstable();
    for department in keys.iter(){
        println!("{}: {:?}", department, company.get(department).unwrap());
    }

}

pub fn company_department_management(){
    
    let mut company = HashMap::new();
    loop {
        println!("Please enter your request:");
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                if buffer.starts_with("Add") {
                    handle_add_request(&mut buffer, &mut company);
                }else if buffer.starts_with("List") {
                    handle_list_request(&mut buffer, &mut company);
                }else if buffer.starts_with("All") {
                    list_all_company_department(&company);
                }
                else {
                    println!("Unknown command.\nUse \"Add <Name> to <Department>\" to add somebody to department \nUse \"List <Department>\" to list department");
                }
            },
            Err(error) => {println!("error: {error}");},
        }
    }
}
