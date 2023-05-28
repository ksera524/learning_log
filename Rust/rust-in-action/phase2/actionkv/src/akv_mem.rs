use libactionlv::ActionKV;

#[cfg(target_os = "windows")]
const USAGE : &str = "
Usage: 
    akv_mem.exe FILE get KEY
    akv_mem.exe FILE delete KEY VALUE
    akv_mem.exe FILE insert KEY VALUE
    akv_mem.exe FILE update KEY VALUE
";

#[cfg(not(target_os = "windows"))]
const USAGE : &str = "
Usage: 
    akv_mem FILE get KEY
    akv_mem FILE delete KEY VALUE
    akv_mem FILE insert KEY VALUE
    akv_mem FILE update KEY VALUE
";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let fname = args.get(1).expect(&USAGE);
    let action = args.get(2).expect(&USAGE).as_ref();
    let key = args.get(3).expect(&USAGE).as_ref();
    let maybe_value = args.get(4);

    let path = std::path::Path::new(&fname);
    let mut store = ActionKV::open(path).expect("unabled to open file");
    store.load().expect("unable to load file");

    match action {
        "get" => match store.get(key).unwarp() {
            Some(value) => println!("{:?}", value),
            None => println!("Key not found"),
        },
        "delete" => {
            let value = store.delete(key).expect("unable to delete key");
        },
        "insert" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.insert(key, value).expect("unable to insert key");
        },
        "update" => {
            let value = maybe_value.expect(&USAGE).as_ref();
            store.update(key, value).expect("unable to update key");
        },
        _ => println!("{}", &USAGE),
    }
}