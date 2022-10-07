mod base;
mod median_mode;
use base::run_base;


fn main() {
    run_base(); 
    let v= vec![1,2,3,4,5,6,7];
    println!("Vetor: {:?}",v.clone());
    println!("Mediana do vetor = {}", median_mode::median(v));
    let v2= vec![1,1,1,2,2,4,4,4,4,3,4,5,6,7,77,77,77,77,77,77,77,77,77,77,77,77,77,77,77,77,77,77,7];
    println!("Vetor: {:?}",v2.clone());
    println!("Moda do vetor = {}", median_mode::mode(v2));
}
