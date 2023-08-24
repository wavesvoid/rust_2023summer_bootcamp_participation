pub use btm as decl_btm;
pub use proc_btm::btm as proc_btm;

#[macro_export]
macro_rules! btm {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut hm = ::std::collections::BTreeMap::new();
            $(
                hm.insert($key, $value);
            )*
            hm
        }
    }
}

#[cfg(test)]
mod test_macro {
    use super::{decl_btm, proc_btm};
    use ::std::collections::BTreeMap;

    #[test]
    fn test_btreemap_declarative() {
        let btm = decl_btm! (
            "one"   => 1,
            "two"   => 2,
            "three" => 3,
        );

        let mut hm = BTreeMap::new();
                hm.insert("one", 1);
                hm.insert("two", 2);
                hm.insert("three", 3);

        assert_eq!(btm, hm);
    }

    #[test]
    fn test_btreemap_procedural() {
        let mut hm = BTreeMap::new();
        hm.insert("carg", 3);
        hm.insert("darg", 4);
        hm.insert("park", 5);
        hm.insert("sarg", 6);
        hm.insert("nark", 7);

        let btm = proc_btm![
            "carg"     =>   3,
            "darg"     =>   4,
            "park"     =>   5,
            "sarg"     =>   6,
            "nark"     =>   7,
        ];

        assert_eq!(hm, btm);
    }

    #[test]
    fn test_empty_map() {
        let btm: ::std::collections::BTreeMap<&str, i32> = proc_btm!();
        let hm = ::std::collections::BTreeMap::<&str, i32>::new();

        assert_eq!(btm, hm);
    }
}
