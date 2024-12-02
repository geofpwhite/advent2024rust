use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::collections::HashMap;






pub(crate) fn advent1(){

    let x:String = read_file_to_string("./inputs/advent1.txt").unwrap();
    let parts: Vec<&str> = x.split('\n').collect();

    let mut l1:Vec<i32>  =  vec![];
    let mut l2:Vec<i32> = vec![];
    for part in parts {
        let nums:Vec<&str> = part.split("   ").collect();

        let num: i32;
        let Ok(num) = nums[0].parse::<i32>() else {continue};
        l1.push(num);
        let Ok(num) = nums[1].parse::<i32>() else {continue};
        l2.push(num);
    }
    l1.sort();
    l2.sort();
    
    let mut index: usize = 0;
    let mut sum:i32 = 0;

    sum = l1.iter().fold(0,|mut sum,x: &i32| {
        let y:&i32= &l2[index];
        index += 1;
        sum = match x > y{
            true=>{sum + x - y}
            false=> {sum + y - x}
        };
        sum
    });
    println!("{:?}",sum);
    sum = 0;
    // let mut counts = HashMap::new();
    




    
}


fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file: std::fs::File = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
