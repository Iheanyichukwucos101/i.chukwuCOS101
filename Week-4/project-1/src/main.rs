fn main() {
    //chunk of code dedicated to getting distance 

let mut unc_distance = String::new();
println!("Input the distance in miles");
io::stdin().read_line(&mut unc_distance).expect("You screwed up lad. Abeg re-enter your string");
let unc_distance:i32 = unc_distance.trim().parse().expect("Not right type of value blud. rerun program and enter again");
let conv_distance = unc_distance as f64*1.609344;
println!("Your converted distance is {}km",conv_distance );
//chunk of the code dedicated to getting time in hours. That is time in hours unconverted. Since the time unconverted inputed by the user is in hours
let mut time = String::new();
println!("Enter the amount on time the trip took in hours");
io::stdin().read_line(&mut time).expect("Wrong datatype inputed ");
let time:f64 = time.trim().parse().expect("Wrong datatype");
println!("the time you inputed is {}",time);
//chunk of code dedicated to getting speed in km/h
let speed = conv_distance/time;
println!("Your vehicular speed is {} km/h",speed );
}
