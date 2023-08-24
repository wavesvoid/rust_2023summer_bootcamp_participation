use std::env::var;


pub mod config_utils {
    use super::*;
    use std::borrow::Cow;
    use core::ops::{Deref, DerefMut};

    #[derive(Debug)]
    pub struct ConfigPath<'a>(Cow<'a, str>);

    impl<'a> Default for ConfigPath<'a> {
        fn default() -> Self {
            Self(Cow::Borrowed("/etc/app/app.conf"))
        }
    }

    impl<'a, U: AsRef<str>> PartialEq<U> for ConfigPath<'a> {
        fn eq(&self, other: &U) -> bool {
            &*self.0 == other.as_ref()
        }
    }


    impl<'a> ConfigPath<'a> {
        /// Construct new ConfigPath with defaults
        pub fn new() -> Self {
            Self::construct_with_defaults("")
        }

        /// Construct new ConfigPath along with setting a new path instantly
        /// or fallback to defaults
        pub fn with_path(p: &'a str) -> Self {
            Self::construct_with_defaults(p)
        }
        
        /// Set custom path.
        /// 
        /// If set - Ok(()) returned. If path is empty Err<&str> is returned.
        pub fn set_path(&mut self, p: &'a str) -> Result<(), &'a str> {
            if p.is_empty() {
                return Err("String is empty");
            }

            self.0 = Cow::Borrowed(p);
            Ok(())
        }

        pub fn get_path(&'a self) -> &'a str {
            &self.0
        }

        /// Print to console
        pub fn print_path(&self) {
            println!("After some complex number crunching");
            println!("We made it to calculate a real path for you\n{}",
                     &self.0);
        }


        // Goes step by step trying to extract path string from either of:
        // - string argument (which can be empty)
        // - environment variable APP_CONF
        // - default path
        fn construct_with_defaults(p: &'a str) -> Self {
            if !p.is_empty() {
                return Self(Cow::Borrowed(p))
            }

            if let Some(cp) = Self::try_from_env() {
                return Self(Cow::Owned(cp));
            }

            Self::default()
        }

        // Try to construct ConfigPath from environment variable
        // named `APP_CONF`
        fn try_from_env() -> Option<String> {
            if let Ok(p) = var("APP_CONF") {
                if !p.is_empty() {
                    return Some(p);
                }
            }
            None
        }   
    }

    /// Same as ConfigPath::get_path
    impl<'a> Deref for ConfigPath<'a> {
        type Target = str;

        fn deref(&self) -> &str {
            self.0.as_ref()
        }
    }

    impl<'a> DerefMut for ConfigPath<'a> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            self.0.to_mut()
        }
    }
}


#[cfg(test)]
pub mod tests {
    use std::env::set_var;
    use crate::config_utils::ConfigPath;
    use super::*;


    #[test]
    fn test_cow() {
        let cf = config_utils::ConfigPath::new();
        set_var("APP_CONF", "");
        
        assert_eq!(cf, "/etc/app/app.conf");
    }


    #[test]
    fn test_cow_from_env() {
        set_var("APP_CONF", "/some/path");
        let conf = config_utils::ConfigPath::new();

        assert_eq!(conf, "/some/path");
    }

    #[test]
    fn test_cow_custom() {
        set_var("APP_CONF", "");
        let conf = ConfigPath::with_path("/my/path");
        let mut conf_1 = ConfigPath::new();

        let is_set = conf_1.set_path("/new/path");

        assert_eq!(conf, "/my/path");
        assert_eq!(is_set, Ok(()));
        assert_eq!(conf_1, "/new/path");
    }
}