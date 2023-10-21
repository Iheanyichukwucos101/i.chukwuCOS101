	fn main() {
		let qt:f64=2.0;
		let t:f64=450_000.0;
        println!("The amount for TOSHIBA is {} and the price is {}",qt,t);

		let qm:f64=1.0;
		let m:f64=1_500_000.0;
	    println!("The amount for MAC is {} and the price is {}",qm,m);

	    let qh:f64=3.0;
		let h:f64=750_000.0;
	    println!("The amount for HP is {} and the price is {}",qh,h);

	    let qd:f64=3.0;
		let d:f64=2_850_000.0;
	    println!("The amount for DELL is {} and the price is {}",qd,d);

	    let qa:f64=1.0;
		let a:f64=250_000.0;
	    println!("The amount for ACER is {} and the price is {}",qa,a);
        
        //Sum

        let s=(qt*t)+(qm*m)+(qh*h)+(qd*d)+(qa*a);
        println!("The total sum is {}",s);
        
        //Average
        let n=qt+qm+qh+qd+qa;
        let avg=s/n;
        println!("The average sales is {}",avg)


    }



