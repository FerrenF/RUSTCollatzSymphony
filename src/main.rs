use win_beep::*;
use std::io::{stdin, stdout, Read, Write};
fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn collatx(lis : &mut Vec<i32>) -> Vec<i32>{
    let n = lis[0];
    if n == 1 { 
        return lis.to_vec();
    }
    else if (n % 2) == 1 {
        let mut result: Vec<i32> = vec![n*3 + 1];
        result.append(lis);
        return collatx(&mut result);
    }
    let mut result: Vec<i32> = vec![n/2];
    result.append(lis);
    return collatx(&mut result);
}
fn collatz(from_num : i32) -> Vec<i32>{
    let mut n: Vec<i32> = vec![from_num];
    return collatx(&mut n);
}
fn get_input() -> i32
{
    let mut line = String::new();
    println!("Enter a number:");
    std::io::stdin().read_line(&mut line).unwrap();
    let num = line.trim().parse::<i32>();
    if num.is_err()
    {
        return 62;
    }
    return num.unwrap();
}
fn main() {
    let num = get_input();
    println!("Collatz Symphony {}!",num);
    let steps: Vec<i32> = collatz(num);
    let mut owned: String = "Steps: ".to_owned();
    owned.push_str(steps.len().to_string().as_str());
    owned.push_str(" - [");
    let end = steps.len() - 1;
    let mut it = 0;
    let mut max = 0;
    let join = steps.clone().into_iter();
    for step in join {
        if step > max{
            max = step;
        }
        owned.push_str(&step.to_string());
        if it != end
        {
        owned.push_str(",");
        }       
        it+=1;
    }
    owned.push_str("]");
    println!("{}",owned);


    let sound_step: u32 = (20000-20) / end as u32;
    let join = steps.into_iter();
    for step in join {
       beep_with_hz_and_millis(sound_step * step as u32,500);
    }
    pause();
}