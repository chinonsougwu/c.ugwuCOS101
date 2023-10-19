fn main(){
	
let p=520_000_000;
let r=10;
let t=5;
/*
p=principal
r=rate
t=time
*/
 // amount, A = P[1 + (R/100)]n
let b=r/100;
let c=1+b;
let d=c^t;
let e=d*p;
let f=e-p;
//this is to find the compound intrest
println!("your compound intrest is {}",f);

}