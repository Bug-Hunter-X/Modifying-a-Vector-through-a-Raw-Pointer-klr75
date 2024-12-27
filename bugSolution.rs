fn main() {
    let mut v = vec![1, 2, 3];
    let index = 0;
    unsafe {
        //This is unsafe code. Only modify the vector using safe operations
        *v.get_unchecked_mut(index) = 10; 
    }
    println!("Value at index {}: {}", index, v[index]);
    println!("Vector v: {:?}", v);
}