fn main(){
	
let p=210_000;
let r=5;
let t=3;
/*
p= principal
r=rate
t=time
*/
// formula for depriciation is A = P[1 - (R/100)]^t
let a=r/100;
let b=1-a;
let c=b^t;
let d=c*p;
println!("the total depriciation of the tv is {}",d);
// this would print the answer






}