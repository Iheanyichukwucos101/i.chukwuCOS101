	fn main() {
		let p = 520_000_000;
		let r = 10;
		let n = 5;

		// compound interest 
		let a = p * ( 1 + (10/100))^5;
		println!("amount is {}", a);
		let ci = a -p;
		println!("compund intrest is{}", ci);

	}		
