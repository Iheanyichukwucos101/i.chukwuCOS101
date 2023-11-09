fn main() {
    let a:i32 = 10;
    let b:i32 = 20;

    println!("Value of a:{}",a);
    println!("Value of b:{}",b);

    let mut res =a>b ;
    println!("a greater than b: {}",res);

    res = a>b;
    println!("a lesser thna b: {}",res);

    res = a>=b;
    println!("a breater than or equal to b:{}",res);

    res =a <=b;
    println!("a Lesser than or equals to B:{}",res);

    res =a==b;
    println!("a is equal to b:{}",res);

    res =a!=b;
    println!("a is not equal to b:{}",res);
    
    
    
}
