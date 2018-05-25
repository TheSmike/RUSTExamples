
fn main(){
    let mut x = 1;
    println!("The value of x is: {}", x);
    
	x = five();
    println!("The value of x is: {}", x);

    //const FIVE : i32 = five();
    

}

fn five() -> i32{
    5
}