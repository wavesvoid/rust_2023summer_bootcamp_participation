use step_3_2::{decl_btm, proc_btm};


fn main() {
    let dbtm = decl_btm![2 => "qua", 3 => "wha?"];
    let pbtm = proc_btm![2 => "qua", 3 => "wha?"];

    println!("DECL MAP: {:?}\nPROC MAP: {:?}", dbtm, pbtm);
    assert_eq!(pbtm, dbtm);
}