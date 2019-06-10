fn main() {
	// The Tupe Type 
	// have fixed length
	let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x,y,z) = tup;
	println!("The y value is : {}", y );
	let five_hundred = tup.0;
	let six_point_four = tup.1;
	let one = tup.2;
	
	// The Array Type
	// have fixed length
	let a = [1,2,3,4,5];
	let months = ["January", "February", "March", "April", "May", "June", 
	              "July", "August", "September", "October", "November", "December"];
	let a:[i32;5] = [1,2,3,4,5];
	let first = a[0];
	let second = a[1];
}
