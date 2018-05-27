fn main(){
	
}

fn bigger(a: &i64, b: &i64) -> &i64{
	if *a > *b {
		a
	}else{
		b
	}
} //compile-time error 

/*
error[E0106]: missing lifetime specifier
 --> ..\18_lifetimes_A0.rs:5:32
  |
5 | fn bigger(a: &i64, b: &i64) -> &i64{
  |                                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `a` or `b`
  */