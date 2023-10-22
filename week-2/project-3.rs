fn main() {
	let p:f64 = 210000.0;
	let r:f64 = 5.0;
	let t:f64 = 3.0;

	//to find the depreciation
	let a = p * ((1.0 - r / 100.)).powf(t);

	println!("The amunt is {}", a);

	let d = p - a;

	println!("The depreciation is {}", d);

}