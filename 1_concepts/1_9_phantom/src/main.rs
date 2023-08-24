use step_1_9::*;



fn main() {
    let vec_fact: Fact<Vec<i32>> = Fact::new();
    let str_fact: Fact<String> = Fact::new();
    println!("Fact about Vec: {}", vec_fact.fact());
    println!("Fact about String: {}", str_fact.fact());
}
