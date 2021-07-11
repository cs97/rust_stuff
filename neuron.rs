fn multvec(v1: &Vec<f32>, v2: &Vec<f32>) -> Vec<f32> {
	let laenge = v1.len();
	let mut nv = Vec::new();
	for n in 0..laenge {
		nv.push(v1[n] * v2[n]);
	}
	return nv;
}


fn vecsum(v: Vec<f32>) -> f32 {
	let mut sum = 0.0;
	for n in v.iter() {
		sum += n;
	}
	return sum;
}

fn sigmoid32(mut x: f32) -> f32 {
	x = -x;
	return 1.0/(1.0 + x.exp());
}

fn neuron(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
	return sigmoid32(vecsum(multvec(&v1, &v2)))
}

fn main() {

//	let zehner = vec![10; 4];
    let v1 = vec![0.9,0.3,0.4];
    let v2 = vec![0.2,0.8,0.2];
	let v3 = vec![0.1,0.5,0.6];
	
	let vx = vec![0.9,0.1,0.8];
	
	println!("{}", "");
	println!("{}", "X hidden:");

	println!("{:?}", vecsum(multvec(&v1, &vx)));
	println!("{:?}", vecsum(multvec(&v2, &vx)));
	println!("{:?}", vecsum(multvec(&v3, &vx)));
	
	println!("{}", "");
	println!("{}", "O hidden:");

	println!("{:?}", sigmoid32(vecsum(multvec(&v1, &vx))));
	println!("{:?}", sigmoid32(vecsum(multvec(&v2, &vx))));
	println!("{:?}", sigmoid32(vecsum(multvec(&v3, &vx))));
	
	println!("{}", "");
	println!("{}", "neuron:");
	println!("{:?}", neuron(&v1, &vx));
	println!("{:?}", neuron(&v2, &vx)); 
	println!("{:?}", neuron(&v3, &vx)); 

}
