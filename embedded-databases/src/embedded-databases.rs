use std::error::Error;

fn main() -> Result<(), StructsyError> {
    eprintln!("##############################");
    eprintln!("## embedded DATABASE CRATES ##");
    eprintln!("##############################");

    structsy_spikes()
}

use structsy::{Structsy, StructsyError, StructsyTx};
use structsy_derive::{queries, Persistent};

#[derive(Persistent, Debug, PartialEq)]
struct MyData {
    #[index(mode = "cluster")]
    name: String,
    address: String,
}

#[queries(MyData)]
trait MyDataQuery {
    /// The parameters name have two match the field names and type
    /// like the `address` parameter match the `address` field of the struct.
    fn search(self, address: String) -> Self;
}

fn structsy_spikes() -> Result<(), StructsyError> {
    eprintln!();
    eprintln!("## 'structsy' crate:");
    eprintln!("####################");
    eprintln!("# can it do what it promises with complex types and good performance?");

    let db = Structsy::open("/tmp/example_basic_query.db")?;
    db.define::<MyData>()?;

    let my_data = MyData {
        name: "Structsy".to_string(),
        address: "https://gitlab.com/tglman/structsy".to_string(),
    };
    let mut tx = db.begin()?;
    let _id = tx.insert(&my_data)?;
    tx.commit()?;

    let num_elements = 100000;
    let mut tx = db.begin()?;
    for i in 0..num_elements {
        let my_data = MyData {
            name: i.to_string(),
            address: format!("https://gitlab.com/tglman/structsy for {}", i),
        };
        let _id = tx.insert(&my_data)?;
        if i % (num_elements/100) == 0 {
            println!("{}%, ", (i*100)/num_elements);
        }
    }
    println!("100% done. Committing...");
    tx.commit()?;
    println!("Committed. Searching for the first element on the unindexed field...");

    let to_find = "https://gitlab.com/tglman/structsy".to_string();
    let mut iter = db.query::<MyData>().search(to_find.clone()).into_iter();
    let (_id, data) = iter.next().unwrap();
    assert_eq!(data.address, to_find);

    println!("Found. Searching for the last element on the unindexed field...");
    let to_find = "https://gitlab.com/tglman/structsy for 99999".to_string();
    let mut iter = db.query::<MyData>().search(to_find.clone()).into_iter();
    let (_id, data) = iter.next().unwrap();
    assert_eq!(data.address, to_find);

    eprintln!("# CONCLUSION: structsy 0.3 is unreasonably slow to insert, even when using a single transaction, as measured");
    eprintln!("              by this 100.000 insertions test. Surprisingly, insertion is not O(1) -- looks like O(n²).");
    eprintln!("              Compiling in RELEASE mode improves times, but still don't take it to production-acceptable speeds.");

    Ok(())
}