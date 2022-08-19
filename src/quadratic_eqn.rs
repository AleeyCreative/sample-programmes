pub fn quadratic_eqn_solver(a:i32, b:i32, c:i32) {
    let discriminant: i32 = ((4*a*c) as f64).sqrt() as i32;
    let root1 = (i32::pow(b, 2) - discriminant)/ (2*a);
    let root2 = (i32::pow(b,2) + discriminant)/ (2*a);


    if root1 == root2 {
        println!("Equal roots, {}", root1);
    } else {
        println!("The roots are {} and {}", root1, root2);
    } 
}