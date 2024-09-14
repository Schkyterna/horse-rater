use std::io;
//i haven't written jack shit in at least a year this is gonna be messy
fn main() {
    //intro text
    println!("Welcome to Horse Rater, a program intended to more objectively evaluate");
    println!("the statistics of horses in Minecraft.");
    println!("Please enter the values for your horse as follows:");
    println!("speed in b/s, health in half-hearts, and jump height in blocks. No limit on decimals.");
    println!("Please enter following the example: 9.7, 23, 1.8");
    println!("Please note there is minimal error handling in this program because I'm lazy.");

    loop
    {
        let mut horseInput = String::new();
        //take input and make it's valid text
        io::stdin()
            .read_line(&mut horseInput)
            .expect("How'd you fuck this up?");

        let horseList: Vec<f64> = horseInput
            .trim()//removes whitespace
            .split(',')//separates at each comma
            .map(|s| s.trim().parse().unwrap_or(0.0)) //string to number
            .collect();
        //call calculator
        horseEquation(horseList)
    }
}

fn horseEquation(horseList: Vec<f64>){
    let mut speed= &horseList[0];
    let mut health = &horseList[1];
    let mut height = &horseList[2];
    println!("Your speed is {}, health is {}, and height is {}. If this was not what you meant, please revise your input.", speed, health, height);
    let rating = speed/2.0 + health/5.5 + height/3.0;
    println!("Your rating for this horse is {}", rating);
}