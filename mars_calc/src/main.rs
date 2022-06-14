use std::io;


fn main() {
    println!("Enter your weight (kg):");
    let mut input = String::new();
    // let s1 = &input;
    // let s2 = &input;
    // println!("{} {}", s1,s2);
     
    // some_fn(&mut input);
    io::stdin().read_line(&mut input).unwrap();

    let weight: f32 = input.trim().parse().unwrap();

    dbg!(weight);


    // borrow_string(&input);
    // own_string(input);


    let mars_weight: f32 = calculate_weight_on_mars(weight);
    //mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars: {}kg", mars_weight ); 
}

// fn borrow_string(s: &String){
//     println!("{}",s)
// }

// fn own_string(s: String){
//     println("{}",s)
// }

fn calculate_weight_on_mars(weight: f32) -> f32{
    return (weight / 9.81) * 3.711;
}

// fn some_fn(s: &mut String){

// }