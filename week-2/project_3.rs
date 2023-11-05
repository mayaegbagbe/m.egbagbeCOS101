fn main() {
	let pt:f64 = 210_000.0;
	let rt:f64 = 5.00;
	let nt:f64 = 3.0;

	// depreciation
	let r = 1.0 - (rt / 100.0);
	let n = f64:: powf(r,nt);
	let p = pt * nt;
	println!("Depreciation is{}",p);
}