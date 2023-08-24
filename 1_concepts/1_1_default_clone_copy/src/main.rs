use step_1_1::copied;
use step_1_1::cloned;



fn main() {
    play_with_copy_default();
    play_with_clone();
}


/// Demonstrates a Copy and Default traits behaviour
fn play_with_copy_default() {
    let pretty_delimiter = get_pretty_line();

    println!(
        "\n// Derived default behaviour with Copy semantics {}",
        pretty_delimiter
    );

    let p1 = copied::default::Point::default();
    let p2 = p1;

    println!("P1: {p1:?} | P2 (copied): {p2:?}"); /* Both are printed
    * cause none of the "move semantic" rules are applied here
    * and, thus a copying is performed.
    * 
    * It is cheap though for this case,
    *   but for other cases, 
    *   i.e. when the struct has too much fields,
    *   your would be better off with the borrowing shenanigans...*/


    println!(
        "\n\n// Custom default behaviour with Copy semantics {}",
        pretty_delimiter
    );

    // ShadowFight
    let p1: copied::default::CustomDefaultPoint = Default::default();
    let p2 = p1;
    let p3 = p2;

    println!("P1 with custom defaults: {p1:?}");
    println!(
        "Derived point: {1:?},\nCopied point: {0:?}",
        p3,
        p2
    );
    
    println!();



    let default_extend_p = copied::default::CustomDefaultPoint {
        x: 1,
        ..Default::default()
    };

    println!("Struct update syntax with Default:");
    println!(
        "\tPoint {0:?}",
        default_extend_p,
    );


    // SMART DEFAULT (but not too smart)
    use copied::default::GodSmartDefault as GSD;


    let mut smart_defaulted = GSD::default();

    if let Some(whats_there) = smart_defaulted.get_hidden() {
        println!("What's hidden?: {whats_there}");
        smart_defaulted.set_hidden(String::from("Feef"));
    }
}


/// Demonstrates Clone trait behaviour
fn play_with_clone() {
    use copied::default::Point;

    
    println!(
        "\n\n// Non-default polyline with Clone behaviour {}",
        get_pretty_line()
    );

    let (x, y) = (15, 41);
    let pline = cloned::Polyline::new(vec![
        Point::default(),
        Point::new(34, 91),
        Point { x: 5, y: 5 },
        Point { x, y },
    ]).unwrap();

    let mut pline2 = pline.clone();
    
    // expose_mut exposes internal private value as mutable reference
    if let Some(mut it) = pline2.expose_mut(2) {
        it.x += 128;
    }

    println!("1:{pl}\n2:Cloned {pl2}", pl=pline, pl2=pline2);



    println!(
        "\n\n// Non-default polyline with CUSTOM Clone behaviour {}",
        get_pretty_line()
    );

    let mut pline = cloned::ClonedCustomPolyline(vec![
        Point::default(),
        Point::new(34, 91),
        Point { x: 5, y: 5 },
        Point { x, y },
    ]);
    let mut cloned_pline = pline.clone();
    
    println!(
        "BEFORE cloning operations: \n{:?}\n{:?}",
        pline,
        cloned_pline);

    pline.0[0].x = 12;
    cloned_pline.0[0].x = 13;
    pline.clone_from(&cloned_pline);


    println!(
        "\nAFTER cloning-FROM operations: \n\t{:?}\n\t{:?}",
        pline,
        cloned_pline);
    
}



/// Return console output decoration string
fn get_pretty_line() -> String {
    "\n".to_owned() + &"-".repeat(80)
}
