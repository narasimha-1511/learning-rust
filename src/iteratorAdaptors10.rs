
//  Consuming Adaptors
// iter_variable.sum() -> this takes the ownership and the  variabel is gone form now and returns a value

// Iterator  adapters
// 1.map iter_variable.map(|x| x + 1)
// 2.filters iter_variable.filter(|x| *x % 2 == 0)
// these also consume the iterator and again return an iterator

fn _main(){
    
    let vec = vec![1,2,3];

    let vec_iter = vec.iter(); 

    // after this is the sum function eats the iterator
    let total_sum : i32 = vec_iter.sum();

    println!("this is the sum {}" , total_sum);

    let vec_iter_2 = vec.iter();

    // let vec_map = vec_iter_2.map(|x| x + 1);

    
    // here we first filtered all the values and then doubles them
    let vec_filter = vec_iter_2.filter(|x| *x %  2 != 0).map(|x| *x * 2);

    // this converts an iterator to a vector again
    let new_vec: Vec<i32> = vec_filter.collect();

    for x in new_vec {
        print!("{} ", x);
    }
}

fn main(){
println!("learing Iterators Adapters ---------------- \n\n");
    _main();
print!("\n\ndone Iterators Adapters ----------------");
}

