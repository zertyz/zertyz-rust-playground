use std::future::Future;
use std::io::Write;
use std::time::Duration;
use futures::{stream,Stream,StreamExt};

#[tokio::main]
async fn main() {
    println!("##########################");
    println!("## ITERATORS vs STREAMS ##");
    println!("##########################");
    println!("In what situations Streams are really needed? Lets find out!");
    println!("");
    println!("The main difference between an Iterator and a Stream is that Iterators will");

    iterator_of_futures().await;
    println!();
    stream_concrete_types().await;
    println!();
    non_blocking_iterators().await;
    println!();
}

async fn non_blocking_iterators() {

    println!("non_blocking_iterators():");
    println!("    Can we have non-blocking Iterators?");
    println!("    ");

    struct MyIter {
        n: u32,
    }
    impl Iterator for MyIter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.n < 6 {
                self.n += 1;
                Some(self.n)
            } else {
                None
            }
        }
    }
    print!("    ");
    for n in (MyIter { n: 0 }) {
        print!("{}, ", n);
    }

    println!("");
    println!("Conclusion: Mystery solved: the main difference between an Iterator and a String is in their .next() function: Streams have them async, as Iterators don't!");

}

async fn stream_concrete_types() {

    println!("stream_concrete_types():");
    println!("    What does `futures::Stream` brings to the table?");
    println!("    ");

    // by using map, the pipeline will be processed without resolving the returned futures
    let mut futures = tokio_stream::iter([1, 2, 3, 4, 5, 6])
        .map(|number| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            number
        });
    print!("    ");
    futures.for_each(|number| async {
        print!("{}, ", number.await);
        std::io::stdout().flush().unwrap();
    }).await;
    // it could also be used in this form:
    // while let Some(number) = futures.next().await {
    //     print!("{}, ", number.await);
    //     std::io::stdout().flush().unwrap();
    // }
    println!();

    // by using .then(), the pipeline resolves the futures together with the stream items
    // (like using .map() and .buffered(1))
    let mut futures = tokio_stream::iter([1, 2, 3, 4, 5, 6])
        .then(|number| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            number
        })
        // at this point, we could simply collect them as vector
        //.collect::<Vec<u32>>().await
    ;
    print!("    ");
    futures.for_each(|number| async move {
        print!("{}, ", number);
        std::io::stdout().flush().unwrap();
    }).await;
    println!();

    // to resolve them all at once:
    // by using map, the pipeline will be processed without resolving the returned futures
    let mut futures = tokio_stream::iter([1, 2, 3, 4, 5, 6])
        .map(|number| async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            number
        })
        // resolves the futures in chunks of 7 at a time
        .buffer_unordered(7);
    // no longer futures
    print!("    ");
    futures.for_each(|number| async move {
        print!("{}, ", number);
        std::io::stdout().flush().unwrap();
    }).await;
    println!();

    println!("");
    println!("Conclusion: Streams seems just to have some facility methods for handling Futures -- like concurrently resolving several of their futures per take.");
    println!("            There is, although, one more difference to explore: Streams do wait for new elements without blocking. Can we do that with Iterators?");

}

async fn iterator_of_futures() {

    println!("iterator_of_futures():");
    println!("    Here I'm creating an Iterator<Item=Future<Output=u32>> that resolves after 1s... and see if that works ...");
    println!("    ");

    /// transforms the given `number` into a future that will resolve in 1 second -- yielding that same `number`
    async fn to_1s_future(number: u32) -> u32 {
        tokio::time::sleep(Duration::from_secs(1)).await;
        number
    }

    // trying to give it a type gives: error[E0277]: a value of type `Vec<dyn Iterator<Item = (dyn Future<Output = u32> + 'static)>>` cannot be built from an iterator over elements of type `impl Future<Output = &{integer}>`
    /*let futures = [1, 2, 3, 4, 5, 6].iter()
        .map(|number| to_1s_future(*number))
        .collect::<Vec<dyn Iterator<Item=dyn Future<Output=u32>>>>();
     */

    // but it works if we don't specify a type (instead of dyn they will be impl)
    let futures = [1, 2, 3, 4, 5, 6].iter()
        .map(|number| to_1s_future(*number))
        .collect::<Vec<_>>();

    /// demonstration how to receive an iterator of futures
    async fn show_futures(futures: impl Iterator<Item=impl Future<Output=u32>>) {
        print!("    Iterating: ");
        for number in futures {
            print!("{}, ", number.await);
            std::io::stdout().flush().unwrap();
        }
    }

    show_futures([1, 2, 3, 4, 5, 6].iter()
        .map(|number| to_1s_future(*number))
        // it is even possible to map, etc!
        .map(|future_number| async {
            future_number.await + 1
        })
    ).await;

    struct with_iterator_of_futures {

    }

    println!("\n");
    println!("Conclusion: yes! it is possible, but with some restrictions:");
    println!("    1. I couldn't declare a type for it, unless the type is a function parameter using `impl`");
    println!("    2. Using generic `let`s works fine for local variables");
}