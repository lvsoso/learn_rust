fn is_copy<T:Copy>(){}



fn types_impl_copy_trait(){
    is_copy::<bool>();
    is_copy::<char>();

    // all integer type are "copy"
    is_copy::<i8>();
    is_copy::<u64>();
    is_copy::<i64>();
    is_copy::<usize>();

    // function pointer is "copy"
    is_copy::<fn()>();

    is_copy::<*const String>();
    is_copy::<*mut String>();


    // Immutable reference is "copy"
    is_copy::<&[Vec<u8>]>(); 
    is_copy::<&String>();


    // To the array/tuple, if internal types are "copy", they are also "copy"
    is_copy::<[u8;4]>();
    is_copy::<(&str, &str)>();

}



fn types_not_impl_copy_trait() {
    // DST 类型不是 copy
    // is_copy::<str>();
    // is_copy::<[u8]>();

    // 有堆内存的类型不是 copy
    // is_copy::<Vec<u8>>();
    // is_copy::<String>();

    // 可变引用不是 copy
    // is_copy::<&mut String>();

    // 对于数组/元组/，如果其内部类型是不是 copy，那么它们也不是 copy
    // is_copy::<[Vec<u8>; 4]>();
    // is_copy::<(String, u32)>();
}



fn main(){
    types_impl_copy_trait();
    types_not_impl_copy_trait();
}
