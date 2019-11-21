use std::io;

fn main() {
       
    println!("Enter your Height in Cm :");
     let mut height = String::new();
    io::stdin().read_line(&mut height).expect("Failed");
    let height:f32=height.trim().parse().expect("Please enter an  integer");
     println!("Enter your Weight in Kg :");
     let mut weight = String::new();
    io::stdin().read_line(&mut weight).expect("Failed");
    let weight:f32=weight.trim().parse().expect("Please enter an  integer");
    let bmi = (weight/(height*height))*10000.0;
    println!("Your BMI is : {}", bmi );
    if bmi < 18.5{
        println!("You are underweight !!!");
    }
    else if bmi > 18.5 &&  bmi < 25.0{
        println!("You have normal weight !!!");
    } 
    else if bmi > 25.0{
        println!("You are overweight !!!");
    }
    else{
        println!("Error in calculating your BMI.")
    }

}
