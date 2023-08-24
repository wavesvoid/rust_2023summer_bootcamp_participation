use step_3_5::prelude::*;
use im::HashMap;


#[test]
fn test_user_repository() {
    let mut repo = URepo::new(&[
        (1, "casper"),
        (2, "banan"),
        (3, "curve"),
        (4, "banderlog")
    ]);

    let user1 = User::new(2, "banan");
    let user2 = &User::new(3, "curve");
    let sample2 = vec![&user1, user2];

    let mut ids_by_nick = repo.search_ids_by_nick("ban");
    ids_by_nick.sort();

    assert_eq!(Some(&User::new(3, "curve")), repo.get_user_by_id(3));
    assert!(sample2.cmp(&repo.get_users_by_ids(&[2, 3])).is_eq());
    assert!(vec![&Id(2), &Id(4)].cmp(&ids_by_nick).is_eq());
}


#[test]
fn test_hashmap_crate() {
    let mut original = HashMap::new();
    original.insert("new", 5);
    
    let mut copy = original.clone();
    
    let mut copy_mod = original.clone();
    copy_mod.insert("new", 5);

    assert!(original.ptr_eq(&copy));
    assert!(! original.ptr_eq(&copy_mod));
}