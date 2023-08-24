pub mod user_repo;


use im::HashMap;
pub use user_repo::UsersRepository;


pub mod prelude {
    pub use super::{UsersRepository, User, user::Id, URepo};
}


#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct User {
    id: user::Id,
    username: user::Username,
}

impl User {
    pub fn new<U, I>(id: I, username: U) -> User
    where
        I: Into<user::Id>,
        U: Into<user::Username>,
    {
        User {
            id: id.into(),
            username: username.into(),
        }
    }
}


pub struct URepo {
    users: HashMap<user::Id, User>,
}


impl URepo {
    pub fn new(users: &[(usize, &str)]) -> Self
    {
        URepo {
            users: HashMap::from(users.into_iter()
                .map(|(id, uname)| (user::Id(*id), User::new(*id, *uname)))
                .collect::<HashMap<user::Id, User>>())
        }
    }

    pub fn get_mut(&mut self) -> &HashMap<user::Id, User> {
        &self.users
    }
}

impl UsersRepository for URepo {
    type User = User;
    type UserId = user::Id;

    fn get_user_by_id<I: Into<user::Id>>(&self, id: I) -> Option<&Self::User> {
        self.users.get(&id.into())
    }
    
    fn get_users_by_ids(
        &self,
        ids: &[usize],
    ) -> Vec<&Self::User>
    {
        ids.into_iter()
            .filter_map(|id| self.users.get(&(*id).into()))
            .collect()
    }
    
    fn search_ids_by_nick(
        &self,
        search_phrase: impl AsRef<str>
    ) -> Vec<&Self::UserId>
    {
        self.users.iter()
            .filter_map(|(id, user)| {
                if user.username.as_ref().contains(search_phrase.as_ref()) {
                    return Some(id);
                }
                None
            })
            .collect()
    }
}


pub mod user {
    #[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Copy, Debug)]
    pub struct Id(pub usize);
    impl AsRef<usize> for Id {
        fn as_ref(&self) -> &usize {
            &self.0
        }
    }
    impl From<usize> for Id {
        fn from(value: usize) -> Id {
            Id(value)
        }
    }

    #[derive(Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
    pub struct Username(String);
    impl AsRef<str> for Username {
        fn as_ref(&self) -> &str {
            &self.0[..]
        }
    }
    impl From<&str> for Username {
        fn from(value: &str) -> Username {
            Username(value.to_owned())
        }
    }
}