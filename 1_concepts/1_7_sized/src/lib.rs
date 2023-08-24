mod repo {
    use std::borrow::Cow;
    use std::convert::Infallible;
    use std::cell::Ref;



    // Dumb storage
    pub trait Storage<K, V> {
        fn set(&mut self, key: K, val: V);
        fn get(&self, key: &K) -> Option<&V>;
        fn remove(&mut self, key: &K) -> Option<V>;
    }

    // Dumb user
    #[derive(Debug, PartialEq, Clone)]
    pub struct User {
        id: u64,
        email: Cow<'static, str>,
        activated: bool,
    }

    impl User {
        pub fn new(id: u64, email: Cow<'static, str>, activated: bool) -> Self
        {
            Self { id, email, activated }
        }
    }

    pub trait UserRepository<K, V> {
        fn set(&self, key: K, val: V);
        fn get(&self, key: &K) -> Ref<'_, V>;
        fn remove(&self, key: &K) -> Option<V>;
    }
    pub trait Command {}

    pub struct CreateUser {
        pub id: u64,
        pub email: String,
        pub activated: bool
    }
    impl Command for CreateUser {}

    pub trait CommandHandler<C: Command> {
        type Context: ?Sized;
        type Result;
    
        fn handle_command(&self, cmd: &C, ctx: &Self::Context) -> Self::Result;
    }

    impl CommandHandler<CreateUser> for User {
        type Context = dyn UserRepository<String, User>;
        type Result = Result<(), Infallible>;
        
        fn handle_command(&self,
                          cmd: &CreateUser,
                          ctx: &Self::Context) -> Self::Result
        {
            // Here we operate with UserRepository
            // via its trait object: &dyn UserRepository
            let u = User::new(
                cmd.id,
                Cow::Owned(cmd.email.clone()),
                cmd.activated);

            ctx.set(cmd.email.clone(), u);

            Ok(())
        }
    }
}


#[cfg(test)]
mod tests {
    use super::repo::*;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::cell::{RefCell, Ref};


    /* CUSTOM REPOSITORY
     *******************/
    #[derive(Debug)]
    struct CustomUserRepo<K, V>(RefCell<HashMap<K, V>>);
    impl CustomUserRepo<String, User> {
        pub fn new() -> Self {
            Self (RefCell::new(HashMap::new()))
        }
    }

    /* IMPLEMENT REPOSITORY
     **********************/
    impl<K: Hash+Eq+ std::fmt::Debug, V: std::fmt::Debug> UserRepository<K, V> for CustomUserRepo<K, V> {
        fn set(&self, key: K, val: V) {
            self.0.borrow_mut().insert(key, val);
        }
        fn get<'a>(&'a self, key: &K) -> Ref<'_, V> {
            let k = key.clone();
            let b: Ref<'_, HashMap<K, V>> = self.0.borrow();

            // HOW DO I GET BORROWED ITEM VALUE UNDER RefCell<HashMap>???
            Ref::filter_map(b, |borrow| borrow.get(k)).unwrap()
        }
        fn remove(&self, key: &K) -> Option<V> {
            self.0.borrow_mut().remove(key)
        }
    }

    #[test]
    fn test_static_dispatch() {
        // Instantiate custom Repository
        let custom_repo = CustomUserRepo::new();
        let user1 = User::new(
            54,
            "username@email.com".into(),
            true,
        );
        
        // Create new User using CommandHandler
        let key = "custom_user@email.com";
        let test_user = User::new(22, key.to_owned().into(), true);
        let com = CreateUser {
            id: 22,
            email: key.to_owned(),
            activated: true
        };
        let handled = user1.handle_command(&com, &custom_repo);

        assert_eq!(Ok(()), handled);
        assert_eq!(*custom_repo.get(&key.to_owned()), test_user);
        assert_eq!(custom_repo.remove(&key.to_owned()), Some(test_user));
    }
}