fn main() {
	// items
	let o:f64 = 450_000.00;
	let a:f64 = 1_500_000.00;
	let p:f64 = 750_000.00;
	let e:f64 = 2_850_000.00;
	let c:f64 = 250_000.00;

	// quantity
	let t:f64 = 2.00;
	let m:f64 = 1.00;
	let h:f64 = 3.00;
	let d:f64 = 3.00;
	let c:f64 = 1.00;

	// total sum 
	let ts = (o * t) + (a * m) + (e * d) + (c * c) + (p * h);
	println!("Total sum of sales is{}",ts);

	// total quantity
	let tq = t + m + h + d + c;
	println!("Total quantity{}",tq);

	// average
	let g = ts / tq;
	println!("Average of sales{}",g);
}