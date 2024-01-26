use std::io::Read;
fn main(){
    println!("what is your user in the compnay");
    println!("if administrator(press a)
              if project manager(press p)
              if employee(press e)
              if customer(press c)
              if vendor(press v)");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("unable to read_line");
    let user = input.trim();

    if user == "a" {
    let mut file = std::fs::File::open("globacom_dbase.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    let mut file = std::fs::File::open("projects_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    let mut file = std::fs::File::open("staff_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    let mut file = std::fs::File::open("customer_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    print!("{}", contents);

    if user is an administrator
    {
        println!("the application returns the database structure");
    }
    else if user is not an administrator
    {
        println!("the application does not return the database structure");
    }

    if user is a project manager
    {
        println!("the application returns the structure of the project table");
    }
    else if user is not a project manager
    {
        println!("the application does not return the structure of the project table")
    }

    if user is an employee
    {
    println!("the application returns the structure of the project table");
    }
