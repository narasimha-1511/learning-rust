
// Iterators
// Generally iterators are slow , but not Into_Iterator

// there are again  1.mutabel  2.immutabel_iterators

// last type of iterator is 3.into_iterator
// this takes the ownership of the collections
// so it is faster than others haha

fn _main(){
    
    // let v1 = vec![1,2,4];

    // let v1_iter = v1.iter();

    // for val in v1_iter {
    //     print!("{} " , val)
    // }

    
    let mut v2 = vec![1,2,4];

    let mut v2_iter_mut = v2.iter_mut();

    while let Some(val) = v2_iter_mut.next() {
        *val  = *val + 1;
    }
    // for val in v2_iter_mut {
    //     *val = *val + 1;
    // }

    println!("{:?}" , v2);


    //into iterator 

    let vec = vec![1,2,3];

    let into_iter = vec.into_iter();

    for val in into_iter {
        println!("{} " , val)
    }

}

fn main(){
println!("learing Iterators ---------------- \n\n");
    _main();
print!("\n\ndone Iterators ----------------");
}

