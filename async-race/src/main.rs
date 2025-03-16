
// use trpl::{Either, Html};
// use trpl::Duration;
use std::time::Duration;
use std::time::Instant;
use std::thread;

fn main() {




    trpl::run(async {
            
        let one_ns = Duration::from_nanos(1);
        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::sleep(one_ns).await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'sleep' version finished after {} seconds.",
            time.as_secs_f32()
        );

        let start = Instant::now();
        async {
            for _ in 1..1000 {
                trpl::yield_now().await;
            }
        }
        .await;
        let time = Instant::now() - start;
        println!(
            "'yield' version finished after {} seconds.",
            time.as_secs_f32()
        );
    });
    // let args: Vec<String> = std::env::args().collect();

    // trpl::run(async {
    //     let slow = async {
    //         println!("'slow' started.");
    //         trpl::sleep(Duration::from_millis(100)).await;
    //         println!("'slow' finished.");
    //     };

    //     let fast = async {
    //         println!("'fast' started.");
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         println!("'fast' finished.");
    //     };

    //     trpl::race(slow, fast).await;
    // })
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
