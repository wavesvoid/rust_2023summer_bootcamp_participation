
pub mod ken {
    pub use std::borrow::Borrow;

    #[derive(Debug, PartialEq)]
    pub enum EmailConversionError {
        InvalidEmailFormat
    }


    #[derive(Debug)]
    pub struct EmailString(String);

    impl EmailString {
        pub fn new<S: AsRef<str>>(value: S) -> Result<Self, EmailConversionError> {
            Self::check_convert(value)
        }

        /// Checks if a string is a valid email and returns EmailString or error.
        fn check_convert<T: AsRef<str>>(value: T) -> Result<Self, EmailConversionError> {
            let value = value.as_ref();
            if value.find('@').is_some() && value.find('.').is_some() {
                return Ok(Self(value.to_owned()));
            }
            Err(EmailConversionError::InvalidEmailFormat)
        }
    }


    impl Borrow<str> for EmailString {
        fn borrow(&self) -> &str {
            &self.0
        }
    }

    impl From<EmailString> for String {
        fn from(value: EmailString) -> Self {
            value.0
        }
    }

    impl TryFrom<&str> for EmailString {
        type Error = EmailConversionError;
        fn try_from(value: &str) -> Result<EmailString, Self::Error> {
            EmailString::check_convert(value)
        }
    }

    impl TryFrom<String> for EmailString {
        type Error = EmailConversionError;
        fn try_from(value: String) -> Result<EmailString, Self::Error> {
            EmailString::check_convert(value)
        }
    }
}



#[cfg(test)]
mod tests {
    use super::ken::*;


    #[test]
    fn test_fallible_conversions() {
        // we can do this as well
        let st = String::from("kdjksjdkf");
        let _:  Result<EmailString, _> = (&*st).try_into(); // Help?

        // Valid conversions
        let valid_email = "some@email.dot.com";
        let valid_email_string: Result<EmailString, _> = valid_email.try_into();
        let from_string = EmailString::try_from("some@email.com".to_owned());
        let into_email_string: Result<EmailString, _> = "some@mail.org".try_into();
        
        // // Errorneous conversions
        let from_string_err = EmailString::try_from("somel.com".to_owned());
        let wrong_email = "invalid_email";
        let invalid_email_string: Result<EmailString, _> = wrong_email.try_into();
        let into_email_string_err: Result<EmailString, _> = "somestring".try_into();


        // Results are Ok()
        assert!(valid_email_string.is_ok());
        assert!(from_string.is_ok());
        assert!(into_email_string.is_ok());

        // Results are Err()
        assert!(invalid_email_string.is_err());
        assert!(from_string_err.is_err());
        assert!(into_email_string_err.is_err());
        

        // Verify underlying stored data is correct
        assert_eq!(invalid_email_string.unwrap_err(), EmailConversionError::InvalidEmailFormat);
        assert_eq!(valid_email_string.unwrap().borrow(), "some@email.dot.com".to_owned());


        // assert_eq!(valid_email_string.unwrap().borrow(), "some@email.dot.com"); // ??
    }

    
    #[test]
    fn test_initialization() {
        let s ="valid@email.com";
        let valid_email_string = EmailString::new(s);
        let decanned = valid_email_string.unwrap();

        assert_eq!(decanned.borrow() as &str, s);
    }


}