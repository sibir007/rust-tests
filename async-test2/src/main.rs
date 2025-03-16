// Filename: src/main.rs

use std::time::Duration;
// use trpl
// use trpl::Future;
use std::{future::Future, pin::pin};
use std::pin::Pin;
// use std::time::Duration;

fn main() {
    trpl::run(async { // start Runtime and pass it main_task
    
        let (tx, mut rx) = trpl::channel(); // an async version of the multiple-producer, single-consumer channel API we used with threads.

        let tx1 = tx.clone();
        let tx1_fut = async move { // move keyword works with async blocks just as it does with closures.  We move tx1 into that async block, it would be dropped once that block ends

            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap(); // we don’t await the send call, because it doesn’t block. It doesn’t need to, because the channel we’re sending it into is unbounded.

                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {

            // The loop will continue executing as long as the pattern it specifies continues to match the value.
            // trpl::Receiver::recv method call produces a future, which we await. The runtime will pause the future until it is ready. Once a message arrives, the future will resolve to Some(message) as many times as a message arrives. When the channel closes, regardless of whether any messages have arrived, the future will instead resolve to None to indicate that there are no more values and thus we should stop polling—that is, stop awaiting.

            while let Some(value) =  rx.recv().await { // When all owners of tx will be dropped rx.recv().await returns None and loop ends
                println!("received '{value}'");
            }
        };

        let tx_fut = async move { // move keyword works with async blocks just as it does with closures.  We move tx into that async block, it would be dropped once that block ends
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        
        // let futures =
        let futures: Vec<Pin<Box<dyn Future<Output = ()>>>> =
        vec![Box::pin(tx1_fut), Box::pin(rx_fut), Box::pin(tx_fut)];


        trpl::join_all(futures).await;

        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;
    });
}


// fn main() {
//     trpl::run(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async {
//             for i in 1..5 {
//                 println!("hi number {i} from the second task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         trpl::join(fut1, fut2).await;

//     });
// }

// fn main() {
//     trpl::run(async {
//         let handle = trpl::spawn_task(async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         });

//         for i in 1..5 {
//             println!("hi number {i} from the second task!");
//             trpl::sleep(Duration::from_millis(500)).await;
//         }

//         handle.await.unwrap();
//     });
// }
// fn main() {
//     trpl::run(async {
//         let fut1 = async {
//             for i in 1..10 {
//                 println!("hi number {i} from the first task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         let fut2 = async {
//             for i in 1..5 {
//                 println!("hi number {i} from the second task!");
//                 trpl::sleep(Duration::from_millis(500)).await;
//             }
//         };

//         trpl::join(fut1, fut2).await;
//     });
// }
