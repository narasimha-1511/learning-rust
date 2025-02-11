
// Multi Threading
// fearless concurrency -> rust book

//why multi threading ?
// to run multiple threads concurrently
// to run multiple threads in parallel

// example webserver
// 1. handle requests concurrently
// 2. handle requests in parallel

// also we are learning message passing in this 



use std::thread;
use std::time::Duration;

//channels
use std::sync::mpsc; // mpsc => multiple producers , single consumer

//for calculating the powf 
use std::f64;

fn _multi_threading() {

     //  this can also happen that , the thread is spawned and 
    // before the thread runs , the program finished
    
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi from spwan thread {i}");
            thread::sleep(Duration::from_millis(1));
        }
    });

    //waiting for the spawn thread to finish before running the main thread
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number from main thread {}",i);
        thread::sleep(Duration::from_millis(1));
    }
    
    //trying something  crrazt

    let vec = vec![1,2,3];

    // so what do we do here , we are using the move keyword
    // so that need the values inside the spawned thread
    // the ownership will be taken haha
    thread::spawn(move || {
        println!("hehe this is a variable trying to print {:?} " , vec);
    }).join().unwrap();

    // .join.wnwrap makes sure the spwaned thread finishes  its work

}

/*
 * two ways are there
 * - sharing memory
 * - sharing memory by communication (Recommended)
 * 
 * use the channels inside the rust std library
 */
fn _message_passing(){
    
    //what is a channel

    // a channel has
    // - transmitter
    // - receiver

    //channel is closed if one of them is out of scope 

    let (transmitter , receiver) = mpsc::channel();

    // we used the channel got the message from the other thread to main thread 
    thread::spawn(move || {
        let val = String::from("hi");
        transmitter.send(val).unwrap();
    });

    let received = receiver.recv().unwrap();

    println!("Got haha {}" , received);


}


fn _calculate_10_8() {

    let (tx , rc) =   mpsc::channel();

    let mut i =  1;
    // i have 8 corres
    while i < 9 {
        let producer = tx.clone();
        thread::spawn(move || {
            let mut sum: u64 = 0;

            // println!("calculation for {} , {}" , i , i+2);

            let start = 10_f64.powf(i as f64) as u64;
            let end = 10_f64.powf((i + 2) as f64) as u64;

            for j in start..(end - 1) {
                sum += j;
            }

            producer.send(sum).unwrap();
        });

        i+=2;
        if i == 9{ break;}
    }

    let mut sum: u64 = 0;
    

    // so why did we drop the transmitter here ??
    // beacause if this does not go out of scope it will keep running the infinite forloop
    // the receiver will be waiting for this until this gets out of scope hahaha
    drop(tx);


    // this runs untill all the transmitter go , out of the scope 
    // while first  coding this , the main transitter was left , so thats the issue
    for val in rc {
        // println!(" receiving {}" , val);
        sum += val;
    }

    print!("final sum {}" , sum);

}


fn _main(){

    // first part spawning and sending 
    // owner ship to the threads
    // _multi_threading();

    //message Passing between Threads
    // _message_passing();

    //calculating  (1 + 2 + 3  ...10^8) in multi thread
    _calculate_10_8();
   
}


fn main(){
println!("Learing MultiThreading ---------------- \n\n");
    _main();
print!("\n\ndone MultiThreading  ----------------");
}