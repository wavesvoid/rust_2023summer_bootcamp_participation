mod repo {
    use std::borrow::Cow;
    use std::marker::PhantomData;


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
        pub fn new(id: u64, email: Cow<'static, str>, activated: bool) -> Self {
            Self {
                id,
                email,
                activated
            }
        }
    }

    // Dumb static repository
    pub struct UserRepositoryStatic<T, K, V> where T: Storage<K, V> {
        storage: T,
        // Generics are not allowed to be "alone"
        _k: PhantomData<K>,
        _v: PhantomData<V>
    }

    // Dumb static implementation
    // Reflects the Storage methods, but has a layer of accessing internal field
    impl<T: Storage<K, V>, K, V> UserRepositoryStatic<T, K, V> {
        pub fn new(storage: T) -> Self {
            Self { storage, _k: PhantomData, _v: PhantomData }
        }

        pub fn set(&mut self, key: K, val: V) {
            self.storage.set(key, val);
        }
        pub fn get(&self, key: &K) -> Option<&V> {
            self.storage.get(key)
        }
        pub fn remove(&mut self, key: &K) -> Option<V> {
            self.storage.remove(key)
        }
    }


    pub struct UserRepositoryDynamic<K, V> {
        storage: Box<dyn Storage<K, V>>
    }

    impl<K, V> UserRepositoryDynamic<K, V> {
        pub fn new(storage: Box<dyn Storage<K, V>>) -> Self {
            Self { storage, }
        }

        pub fn set(&mut self, key: K, val: V) {
            self.storage.set(key, val);
        }
        pub fn get(&self, key: &K) -> Option<&V> {
            self.storage.get(key)
        }
        pub fn remove(&mut self, key: &K) -> Option<V> {
            self.storage.remove(key)
        }
    }

}


#[cfg(test)]
mod tests {
    use super::repo::*;
    use std::collections::HashMap;
    use std::hash::Hash;
    use std::borrow::Cow;

    struct Store<K, V>(HashMap<K, V>);
    impl<K: Hash+Eq> Store<K, User> {
        fn new() -> Self {
            Self (HashMap::new())
        }
    }

    impl<K: Hash+Eq> Storage<K, User> for Store<K, User> {
        fn set(&mut self, key: K, val: User) {
            self.0.insert(key, val);
        }
        fn get(&self, key: &K) -> Option<&User> {
            self.0.get(key)
        }
        fn remove(&mut self, key: &K) -> Option<User> {
            self.0.remove(key)
        }
    }


    #[test]
    fn test_static_dispatch() {
        let user1 = User::new(
            54,
            Cow::<'_>::Owned("some@user.email".to_owned()),
            true,
        );
        let user2 = User::new(
            53,
            Cow::<'_>::Owned("some@user.email".to_owned()),
            false,
        );

        let test_user1 = Some(user1.clone());
        let test_user2 = Some(user2.clone());

        let store1: Store<String, User> = Store::new();
        let mut repo_static = UserRepositoryStatic::new(store1);

        let store1: Store<String, User> = Store::new();
        let mut repo_dyn = UserRepositoryDynamic::new(Box::new(store1));

        repo_static.set(String::from("key"), user1);
        repo_dyn.set(String::from("key"), user2);


        assert_eq!(repo_static.get(&"key".to_owned()), test_user1.as_ref());
        assert_eq!(repo_dyn.get(&"key".to_owned()), test_user2.as_ref());
        assert_eq!(repo_static.remove(&"key".to_owned()), test_user1);
        assert_eq!(repo_dyn.remove(&"key".to_owned()), test_user2);
    }
}