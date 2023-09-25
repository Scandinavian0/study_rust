use std::collections::{BTreeSet, HashMap};
use std::io::stdin;

pub fn home_work_3(){
    let mut company:HashMap<String,BTreeSet<String>>=HashMap::new();
    //它用于创建一个无限循环,这意味着一旦程序执行到循环的末尾，它将返回循环的开头并继续执行，以便等待用户输入并处理命令。
    loop {
        println!("Enter a command (e.g., 'Add Sally to Engineering' or 'List Engineering'): ");

        let mut input=String::new();

        stdin().read_line(&mut input).expect("Failed to read input");
        // 保存输入的数据
        let parts:Vec<&str> =input.trim().split_whitespace().collect();
        // 判断值是否符合条件
        if parts.len()<4{
            break;
        }

        let action=parts[0];
        let name=parts[1];
        let department=parts[3];

        match action {
            "Add" => {
                let deparment_employees=company
                    .entry(department.to_string())
                    .or_insert(BTreeSet::new());
                deparment_employees.insert(name.to_string());
                println!("Added {} to {} department",name,department);
            }

            "List"=>{
                if let  Some(employees)=company.get(department) {
                    println!("  Employees in {} department: {:?}", department, employees);
                } else {
                    println!("Department {} not found.", department);
                }
            }
            _=>{
                println!("Invalid command. Try again.");
            }
        }

    }
}