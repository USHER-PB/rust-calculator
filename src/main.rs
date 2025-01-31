use std::io;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
fn main()
{   let   answer: f64= 0.0;
    let _args: Vec<String> = env::args().collect();
    println!("ask the user to enter a value");
    let mut num1 = String::new() ;
   
    io::stdin().read_line(&mut num1).expect("Failed to read line");
    let mut num1:f64 = num1.trim().parse().expect("please enter back a value");

    
    println!("ask the user to enter the second value ");
    let mut num2 = String::new();
    io::stdin().read_line(&mut num2).expect("Failed to read line");
    let num2:f64 = num2.trim().parse().expect("please enter back a value");

    let mut operator = String::new();  
    println!("ask the user to enter the operator");
    io::stdin().read_line(&mut operator).expect("no value entered");
    let operator: char = operator.trim().parse().expect("please enter correct operator");

    
    let a = [ '+', '%', '-', '*', '/' , '$', '#'];
    if operator == a[0]
    {
     let answer:f64  = num1 + num2;
     println!("the summation of your values num1 and num2 give a result of {}", answer);
    }
    else if operator == a[1]
    {
     let answer: f64 = num1 % num2;
     println!("the modulus of your values num1 and num2 give a result of {}", answer);
    }
    else if operator == a[2]
    {
      let answer: f64 = num1 - num2 ;
      println!("the subtractio of the numbers are {}",answer);
    }
    else if operator == a[3]
    {
      let answer: f64 = num1 * num2;
      println!("the multiplication of num1 and num2 give a result of {}",answer);
    }
    else if operator == a[4]
    {
      let answer:f64 = num1 / num2;
      println!("the result of the division between the two number is {}", answer);
    }
    else if operator == a[5]     
    {
      let answer: f64 = num1.powf(num2); 
      println!("the exponential of the 02  value are {}", answer);
    }
    else if operator == a[6]
   {
       let mut answer: f64 = 1.0;
        if num1==1.0 || num1==0.0
        {
        println!("1");
        
      } 
       else if num1 < 0.0
      {
        println!("this is not valid")

      }
        else if num1>1.0 
        {
          while num1 >1.0
            {
              answer = num1 *(num1-1.0) * answer;
              num1 = num1 - 2.0;
             }
             println!("the factorial of num1 is {}", answer);
        }
         
      }
  else  {
    println!("this is wrong");
      
    }

  log(num1, operator, num2, answer);

}



fn log(num1: f64, operator:char ,num2: f64, answer: f64 ){
  let log_entry = format!("{} {} {} = {}\n", num1, operator, num2, answer);
  let mut file = OpenOptions::new()
      .append(true)
      .create(true)
      .open("history")
      .expect("Cannot open file");
  file.write_all(log_entry.as_bytes()).expect("Cannot write to file");
}


    //      {
    //       println("this is not valid")
    //      }
         
    //     }
      
//    else 
//     {
//       println!("this cannot be proceeded");
//     }

// }




