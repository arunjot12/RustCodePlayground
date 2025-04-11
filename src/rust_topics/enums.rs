// Enums 

pub fn rust_enum() {
    enum Office {
        Employee(String),
    }
    let employee_data = Office::Employee("Alice".to_string());
    match employee_data{
        Office::Employee(employee) => print!("{}",employee)
    }
}