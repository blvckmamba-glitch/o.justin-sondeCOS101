struct Employee {
    name:String,
    company:String,
    age:i32
}

fn main() {
    let emp1 = Employee {
        company:String::from("Enrst & Young "),
        name:String::from("Ebibong Jessica"),
        age:25,
    };
    println!("Name = {} \n", emp1.name);
    println!("Company = {} \n", emp1.company);
    println!("Age = {}", emp1.age);
}
