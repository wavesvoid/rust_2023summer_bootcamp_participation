
pub trait UsersRepository {
    type User;
    type UserId;

    fn get_user_by_id<I: Into<Self::UserId>>(&self, id: I) -> Option<&Self::User>;
    fn get_users_by_ids(&self, ids: &[usize]) -> Vec<&Self::User>;
    fn search_ids_by_nick(&self, search_phrase: impl AsRef<str>) -> Vec<&Self::UserId>;
}
