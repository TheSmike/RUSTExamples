fn main() {
    let s1 = String::from("A");  
    let s2 = String::from("perfect"); 
    let s3 = String::from("Circle");  
    let result = concat3(s1, s2, s3);

    println!("result is '{}'", result.0);

    let s1 = result.1; 
    let s2 = result.2;
    let s3 = result.3;
    println!("input values are '{}','{}','{}'", s1, s2, s3);
    
    
} 

fn concat3(s1: String, s2: String, s3: String) -> (String, String, String, String) {
    let mut result_str = String::new();
    result_str.push_str(&s1);
    result_str.push_str(" ");
    result_str.push_str(&s2);
    result_str.push_str(" ");
    result_str.push_str(&s3);
    (result_str, s1,s2,s3) 
}  