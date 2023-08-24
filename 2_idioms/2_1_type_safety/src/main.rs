use step_2_1::post::{
    typestate_boilerplate::*,
    types,
};




fn main() {
    let post = Post::<New>::new(
        types::Id(72),
        types::UserId(83),
        types::Title("sometitle".to_owned()),
        types::Body("This is body".to_owned()),
    );

    let unmoderated = post.publish();
    let published = unmoderated.allow();

    published.print_info();

    let _deleted = published.delete();
    // deleted.print_info(); // Compile error


    let _post = Post::<New>::new(
        types::Id(72),
        types::UserId(83),
        types::Title("sometitle".to_owned()),
        types::Body("This is body".to_owned()),
    )
        .publish()
        .deny();

    // Compile error
    // post.set_title(types::Title(String::from("NEW TITLE"));
    // post.set_body(types::Body(String::from("NEW BODY"));
    // post.allow();
    // post.publish();
    // post.delete();
}
