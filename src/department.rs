use std::io;
use std::collections::HashMap;

fn handle_request(text: &mut String, company:&mut HashMap<String, Vec<String>>) {
    
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

pub fn company_department_management(){
    
    let mut company = HashMap::new();
    loop {
        println!("Please enter your request:");
        let mut buffer = String::new();
        match io::stdin().read_line(&mut buffer) {
            Ok(_) => {
                handle_request(&mut buffer, &mut company)
            },
            Err(error) => {println!("error: {error}");},
        }
        println!("{:?}", company);
    }
}
