fn main() {
    //Tupple
    let emp_info:(&str,u8)=("anant",20);

    let emp_name =emp_info.0;
    let emp_age =emp_info.1;

    let(employee_name,employee_age)=emp_info;

    println!("Employee Name: {}, Employee Age: {}", emp_name,emp_age);
    println!("Employee Name: {}, Employee Age: {}", employee_name,employee_age);
}
