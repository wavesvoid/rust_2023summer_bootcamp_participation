use std::net::{ ToSocketAddrs, SocketAddr};
use std::borrow::Cow;


fn main() {
    println!("Refactor me!");

    let mut err = Error::new("NO_USER");
    err.status(404).message("User not found");
}

#[derive(Debug)]
pub struct Error<'a> {
    code: Cow<'a, str>,
    status: u16,
    message: Cow<'a, str>,
}

impl<'a> Default for Error<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            code: "UNKNOWN".into(),
            status: 500,
            message: "Unknown error has happened.".into(),
        }
    }
}

impl<'a> Error<'a> {
    pub fn new(code: &'a str) -> Self {
        let mut err = Self::default();
        err.code = code.into();
        err
    }

    pub fn status(&mut self, s: u16) -> &mut Self {
        self.status = s.into();
        self
    }

    pub fn message(&mut self, m: &'a str) -> &mut Self {
        self.message = m.into();
        self
    }
}


#[derive(Debug, Default)]
pub struct Server(Option<SocketAddr>);
impl Server {
    pub fn bind<S: ToSocketAddrs>(&mut self, sa: S) {
        if let Ok(mut sit) = sa.to_socket_addrs() {
            self.0 = sit.next();
        }
    }
}


#[cfg(test)]
mod server_spec {
    use super::*;

    mod bind {
        use super::*;

        #[test]
        fn sets_provided_address_to_server() {
            let mut server = Server::default();

            server.bind("127.0.0.1:8080");
            assert_eq!(format!("{}", server.0.unwrap()), "127.0.0.1:8080");

            server.bind(("::1", 9911));
            assert_eq!(format!("{}", server.0.unwrap()), "[::1]:9911");
        }
    }
}
