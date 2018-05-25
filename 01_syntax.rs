fn main() {

    //definizione implicita
    let decimal_impl = 16; 
    //definizione esmplicita 
    let decimal_espl : i32 = 16;
    //notazione esadecimale
    let decimal_hex_form = 0x10;
    //ci sono ovviamente molti altri tipi e notazioni...
    println!("\ndecimal_impl {} \ndecimal_espl {} \ndecimal_hex_form {}\n", decimal_impl, decimal_espl, decimal_hex_form);
    
    //tupla
    let tup: (i32, f64, u8, char) = (500, 6.4, 1, 'a');
    println!("Tuple: tup.0 {}, tup.1 {}, tup.2 {}, tup.3 {}\n", tup.0, tup.1, tup.2, tup.3);

    //array 
    let a = [1, 2, 3, 4, 5];
    println!("array's first value: {}\n", a[0]);


    let text = String::from("I'm a text!");
    println!("text: {}\n", text);

}