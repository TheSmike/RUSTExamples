use std::intrinsics::type_name;

pub fn print_type<T>(_: T) {
    println!("type is {:?}", unsafe { type_name::<T>() });
}

