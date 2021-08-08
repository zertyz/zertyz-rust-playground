fn main() {
    eprintln!("#####################");
    eprintln!("## MMAP CONTAINERS ##");
    eprintln!("#####################");

 //   mmap_storage_spikes();
    mmap_rkyv_spikes()
 //   rustbrake_mmap_spikes();
}

use mmap_storage::file;
use std::collections::HashMap;
use mmap_storage::serializer::{Bincode, FileView};
use bincode;

fn mmap_storage_spikes() {
    eprintln!();
    eprintln!("## mmap_storage crate:");
    eprintln!("######################");
    eprintln!("# can we have an mmapped HashSet with this one?");

    let mmapped_file = "/tmp/mmap_storage.bin";

    eprintln!("Will open take a long time?");
    //Just open
    let mut storage = FileView::<HashMap<String, String>, Bincode>::open_or_default(mmapped_file).expect("To create a FileView");
    let mut data = storage.load_owned().expect("To load data from mmapped file");


    eprintln!("just opened mmapped hashmap has {} entries. Contents are:", data.len());
    eprintln!("{:?}", data);
    //eprintln!("bincode default options are {:?}", bincode::options());

    data.insert(1.to_string(), "".to_string());
    data.insert(2.to_string(), "two".to_string());

    data.insert(3.to_string(), "".to_string());
    data.insert(4.to_string(), "".to_string());

    data.insert(5.to_string(), "".to_string());
    data.insert(4.to_string(), "four".to_string());

    storage.save_sync(&data).expect("To save (or just sync?) modified data");

    eprintln!("mmapped hashmap has {} entries", data.len());

    data.insert(6.to_string(), "six".to_string());
    data.insert(7.to_string(), "seven".to_string());
    data.insert("5".to_string(), "changed".to_string());

    eprintln!("Inserting several things... memory should stay low...");
    for i in 100..1024*1024*2 {
        data.insert(i.to_string(), "please, save without trouble".to_string());
    }
    eprintln!("Did we insert with a low memory profile?");
    storage.save_sync(&data);
    eprintln!("Did saving took a long time?");

    // storage.modify(|mut _data| {
    //     _data.insert(6.to_string(), "six".to_string());
    //     _data.insert(7.to_string(), "seven".to_string());
    //     _data
    // }).expect("To modify inplace?");

    eprintln!("mmapped hashmap has now {} entries -- and I won't sync nor close it before quitting", data.len());
    data.insert("5".to_string(), "changed again".to_string());

    eprintln!("--> Conclusion: this crate may be cool for storing json, toml or even binary files... but it does not provide on-demand mmap load/save (std) containers");
    eprintln!("    (other crates to try are 'rkyv', 'flatdata', 'Flexbuffers', 'fst', 'vmap', 'rustbrake', 'tetsy-db', 'parity-db')")

}

use rkyv::{
    archived_root, archived_root_mut,
    ser::{serializers::AllocSerializer, Serializer},
    Archive, Deserialize, Infallible, Serialize,
};
use std::fs::OpenOptions;
use std::path::PathBuf;
use memmap::MmapMut;
use std::pin::Pin;

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
struct Test {
    int: u8,
    string: String,
    option: Option<Vec<i32>>,
    hashmap:  HashMap<String, String>,
}

fn mmap_rkyv_spikes() {
    eprintln!();
    eprintln!("## 'rkyv' crate with mmap:");
    eprintln!("##########################");
    eprintln!("# can we have an mmapped HashSet with this one?");


    let mut hashmap: HashMap<String, String> = HashMap::new();
    hashmap.insert("key1".to_string(), "value1".to_string());
    hashmap.insert("key2".to_string(), "value2".to_string());
    hashmap.insert("key3".to_string(), "value3".to_string());
    let value = Test {
        int: 42,
        string: "hello world".to_string(),
        option: Some(vec![1, 2, 3, 4]),
        hashmap:  hashmap,
    };

    let mut serializer = AllocSerializer::<256>::default();
    serializer.serialize_value(&value).unwrap();
    let mut serialized_bytes = serializer.into_serializer().into_inner();

    let path: PathBuf = PathBuf::from("/tmp/mmap_rkyv.bin");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path).unwrap();
    file.set_len(serialized_bytes.len() as u64).unwrap();
    let mut mmap = unsafe { MmapMut::map_mut(&file).unwrap() };

    // on the first time, copy the contents from serialized struct to the file
    mmap.copy_from_slice(&serialized_bytes);
    // on the second time, use the mmaped struct directly
    //let bytes = mmap.as_ptr();

    fn zerocopy_bytes_to_Archived<'a>(pointer: *mut u8, length: usize) -> &'a mut ArchivedTest {
        //let bytes = pointer as &[u8];
        let bytes = unsafe { core::slice::from_raw_parts_mut(pointer, length) };
        let archived = unsafe { archived_root_mut::<Test>(Pin::new_unchecked(&mut bytes[..])) };
        unsafe { Pin::into_inner_unchecked(archived) } //was: archive.deserialize(&mut Infallible).unwrap()
    }

    let deserialized = zerocopy_bytes_to_Archived(mmap.as_mut_ptr(), mmap.len());

    println!("archived version is, originally,  {{int={}, string='{}', option={:?}, hashmap={:?}}}", deserialized.int, deserialized.string, deserialized.option, deserialized.hashmap);
    mmap[53] = b'Y';
    println!("after a buffer byte change:       {{int={}, string='{}', option={:?}, hashmap={:?}}}", deserialized.int, deserialized.string, deserialized.option, deserialized.hashmap);
    deserialized.int = deserialized.int + 1;
    //mmap[108] = mmap[108] + 1;
    println!("after changing archived data:     {{int={}, string='{}', option={:?}, hashmap={:?}}}", deserialized.int, deserialized.string, deserialized.option, deserialized.hashmap);
    mmap[10] = mmap[10] + 1;
    println!("after another buffer byte change: {{int={}, string='{}', option={:?}, hashmap={:?}}}", deserialized.int, deserialized.string, deserialized.option, deserialized.hashmap);
    //deserialized.string[3] = b'Y';
    //println!("after inserting a key in the hash {{int={}, string='{}', option={:?}, hashmap={:?}}}", deserialized.int, deserialized.string, deserialized.option, deserialized.hashmap);

    eprintln!("## CONCLUSION: although I could use some tricks to change rkyv's Archived types, it is good for read-only");
    eprintln!("##             & bigger than RAM data. Cool enough.");
    eprintln!("## We may build a cool std containers database with this crate in the following manner:");
    eprintln!("  1) A wrapping generic module, with CRUD functions, register changes in RAM");
    eprintln!("  2) After a 'save' or 'optimize' operation, those as saved in the Archived form");
    eprintln!("  3) The CRUD functions go for the Archived data, then for the RAM data");
    eprintln!("  4) A little optimization, demonstrated here, may include a byte field to denote if the Archived");
    eprintln!("     record is the last version or if the RAM version is that one");
}


extern crate rustbreak;
use rustbreak::{MmapDatabase, deser::Bincode as RustBrakeBincodeDeser};
use rustbreak::backend::MmapStorage;

fn rustbrake_mmap_spikes() -> rustbreak::Result<()> {
    eprintln!();
    eprintln!("## 'rustbrake' crate with mmap:");
    eprintln!("###############################");
    eprintln!("# can we have an mmapped database with this one?");

    let db = MmapDatabase::<HashMap<u32, String>, RustBrakeBincodeDeser>::mmap(HashMap::new())?;

    println!("Writing to Database");
    db.write(|db| {
        db.insert(0, String::from("world"));
        db.insert(1, String::from("bar"));
    });

    db.read(|db| {
        // db.insert("foo".into(), String::from("bar"));
        // The above line will not compile since we are only reading
        println!("Hello: {:?}", db.get(&0));
    })?;

    eprintln!("# CONCLUSION: currently we can't. Rustbrake does not implement zero-copy... therefore, only anonymous mmaps");
    eprintln!("#             are used, pretty similar to MemoryDatabase, but requiring no additional copies when realloc'ing");

    Ok(())
}