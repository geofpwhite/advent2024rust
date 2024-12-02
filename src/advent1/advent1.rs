use std::fs::File;
use std::io::Read;
use std::io::Result;
use std::collections::LinkedList;





pub(crate) fn advent1(){

    let x:String = read_file_to_string("src/input.txt").unwrap();
    let parts: Vec<&str> = x.split('\r').collect();

    let mut ray =  vec![];
    let mut rays = vec![];
    for part in parts {
        for p in part.split(',').into_iter() {
            if p == "\n"{
                continue;
            }
            ray.push(p);
        }
        if ray.len() > 0 {
            rays.push(ray);
            ray = vec![];
        }
    }
    println!("{:#?}",rays);

    let mut paths: [LinkedList<[i32;2]>;2] = [LinkedList::new(),LinkedList::new()];
    let mut cur_coords:[i32;2] = [0,0];
    let mut index = 0;
    for ray in rays{
        cur_coords = [0,0];
        
        for step in &ray {
            let num: i32;
            let substr = step[1..].parse::<i32>();
            match substr {
                Ok(num2) => num = num2,
                Err(e) => {
                    continue
                },
            }

            match step.chars().next().unwrap(){
                'U'=> {
                    cur_coords[0] = cur_coords[0] + num
                },
                'D'=>{
                    cur_coords[0] = cur_coords[0] - num
                } ,
                'L'=>{
                    cur_coords[1] = cur_coords[1] + num
                } ,
                'R'=>{
                    cur_coords[1] = cur_coords[1] - num
                } ,
                _=>{
                    continue;
                }
            }
            paths[index].push_back(cur_coords.clone());
        }
        index = index + 1;
    }

    println!("{:#?}",paths);
    
}


fn read_file_to_string(filename: &str) -> Result<String> {
    let mut file: std::fs::File = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
