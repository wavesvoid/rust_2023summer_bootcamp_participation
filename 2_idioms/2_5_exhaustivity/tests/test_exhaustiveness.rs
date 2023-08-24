use step_2_5::*;
use std::time::SystemTime;

    
#[test]
fn test_event() {
    let u = user::Event::Created(event::UserCreated {
        user_id: user::Id(4),
        at: user::CreationDateTime(SystemTime::now())
    });


    match u {
        user::Event::Created(event::UserCreated { user_id, at }) => {
            println!("{:?} {:?}", user_id, at);
        }
        user::Event::NameUpdated(_uname_updated) => {}
        user::Event::Online(_became_online) => {}
        user::Event::Offline(_became_offline) => {}
        user::Event::Deleted(_user_deleted) => {}
    }
}


#[test]
fn test_user() {
    // You cannot construct the User outside a lib crate (where it is defined)
    let u = user::User::new();

    // If we are to extend User in some way
    // we'll get compiler error and, thus we'll be saved from missing something
    // for example when implementing Display trait...
    #[allow(unused_variables)]
    let user::User {
        id,
        name,
        online_since,
        created_at,
        last_activity_at,
        deleted_at,
    } = u;
}