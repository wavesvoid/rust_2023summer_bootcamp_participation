pub mod types {
    #[derive(Clone, Debug, PartialEq)]
    pub struct Id(pub u64);
    
    #[derive(Clone, Debug, PartialEq)]
    pub struct Title(pub String);
    
    #[derive(Clone, Debug, PartialEq)]
    pub struct Body(pub String);
    
    #[derive(Clone, Debug, PartialEq, Default)]
    pub struct UserId(pub u64);
}



/** Boilerplate TypeState pattern
 */
pub mod typestate_boilerplate {
    use super::types::*;
    use std::marker::PhantomData;

    pub struct New;
    pub struct Unmoderated;
    pub struct Published;
    pub struct Deleted;


    #[derive(Clone)]
    pub struct Post<S> {
        pub id: Id,
        pub user_id: UserId,
        pub title: Title,
        pub body: Body,
        _s: PhantomData<S>,
    }

    impl<S> Post<S> {
        fn take_with_state<T>(self) -> Post<T> {
            Post {
                id: self.id,
                user_id: self.user_id,
                title: self.title,
                body: self.body,
                _s: PhantomData,
            }
        }
    }

    impl Post<New> {
        pub fn new(id: Id,
                   user_id: UserId,
                   title: Title, body: Body) -> Post<New>
        {
            Post {
                id,
                user_id,
                title,
                body,
                _s: PhantomData,
            }
        }

        pub fn publish(self) -> Post<Unmoderated> {
            self.take_with_state::<Unmoderated>()
        }
    }

    impl Post<Unmoderated> {
        pub fn allow(self) -> Post<Published> {
            self.take_with_state::<Published>()
        }

        pub fn deny(self) -> Post<Deleted> {
            self.take_with_state::<Deleted>()
        }
    }

    impl Post<Published> {
        pub fn delete(self) -> Post<Deleted> {
            self.take_with_state::<Deleted>()
        }

        pub fn print_info(&self) {
            println!("Post: \n\tUser: {:?}\n\tTitle: {:?}\n\tBody: {:?}",
                     self.user_id,
                     self.title,
                     self.body);
        }
    }
}
