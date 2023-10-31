fn main(){
	
let p = 520_000_000.00;
let r = 10.00;
let t = 5.00;
/*
p=principal
r=rate
t=time
*/
 // amount, A = P[1 + (R/100)]
let b = r / 100.00;
let c:f64 = 1.00 + b;
let d:f64 = c.powf(t);
let e = d * p;
let f = e - p;
//this is to find the compound intrest
println!("your compound intrest is {}",f);

}