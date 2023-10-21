fn main() {
	// amounts will be referred to as (x)a
	let ta:f64 = 450_000.0;
	let ma:f64 = 1_500_000.0;
	let ha:f64 = 750_000.0;
	let da:f64 = 2_850_000.0;
	let aa:f64 = 250_000.0;

	// quatities will be reffered to as (x)q

	let tq:f64 = 2.0;
	let mq:f64 = 1.0;
	let hq:f64 = 3.0;
	let dq:f64 = 3.0;
	let aq:f64 = 1.0;


	let	s = (ta * tq) + (ma * mq) + (ha * hq) + (da * dq) + (aa * aq);
println!("The sum of all sales is {} ", s);

 	let t = tq + mq + hq + dq + aq;

	let avg = s / t;

    println!("The average of all sales is {} ", avg);
}