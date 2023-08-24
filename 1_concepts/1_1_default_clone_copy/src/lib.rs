pub mod copied {
    /*! Contains an API
     * that can demonstrate a Default and Copy behaviours of objects
     */
    pub mod default {
        use smart_default::SmartDefault;


        /// Represents a point on a Cartesian plane.
        /// 
        /// Implements a derived default behaviour.
        /// Has its move semantic are overrided by "copy semantic"
        #[derive(Default, Copy, Clone, Debug)]
        pub struct Point {
            /* Many primitive types do have a predefined default behaviour
             * so we do not need to implement is by ourselves
             *  - this is the case for the i32 as well.
             */
            pub x: i32,
            pub y: i32,
        }


        /* We could also have implement Copy trait in somewhat manualy
         * as shown below.
         * The difference is:
         *  - derive will also try implement Copy for all fields,
         *  - while manual implementation is, well manual implementation
         */
        
        // impl Copy for Point { /* yes, empty implementation */ }


        impl Point {
            pub fn new(x: i32, y: i32) -> Self {
                Self { x, y }
            }
        }

        
        /// Represents a point on a Cartesian plane.
        ///
        /// Implements custom default behaviour.
        /// Has its move semantic are overrided by "copy semantic"
        
        // Invariants that do implement a Copy behaviour
        // are required to implement a Clone trait as well.
        // Copy trait has Clone specified as its supertrait...
        #[derive(Debug, Copy, Clone)]
        pub struct CustomDefaultPoint {
            pub x: i32,
            pub y: i32,
        }

        /* An example of a custom default implementation
         * in case when it is really needed:
         *  - for x to be 42
         *  - for y to be -12
         * by default..
         */
        impl Default for CustomDefaultPoint {
            fn default() -> Self {
                Self {
                    x: 42,
                    y: -12,
                }
            }
        }



        #[derive(SmartDefault, Debug)]
        pub struct GodSmartDefault {
            pub x: i32,
            pub y: i32,
    
            #[default(Some(String::from("casted")))]
            hidden_field: Option<String>,
        }

        impl GodSmartDefault {
            pub fn get_hidden(&mut self) -> Option<String> {
                self.hidden_field.take()
            }

            pub fn set_hidden(&mut self, s: String) {
                self.hidden_field = Some(s);
            }
        }

        impl std::fmt::Display for GodSmartDefault {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                writeln!(f,
                         "GodSmartDefault {{ x = {}, y = {}, hidden = {} }}",
                         self.x, 
                         self.y,
                         &self.hidden_field.as_deref().unwrap_or("EMPTY"))
                /* // As an alternative:
                    writeln!(f,
                         "GodSmartDefault {{ x = {}, y = {} }",
                         self.x, 
                         self.y)?;

                    if Some(ref hidden) = self.hidden_field.as_deref() {
                        write!(f, ", hidden = {hidden}")?;
                    }
                    write(f, " }")                    
                */
            }
        }
    }
}


pub mod cloned {
    use super::copied::default::Point;


    #[derive(Clone)]
    pub struct Polyline(Vec<Point>);

    #[derive(Debug)]
    pub enum PolyError {
        EmptyError(&'static str),
    }

    const POLYERROR_EMPTY: &str = "Input vector cannot be empty";


    impl Polyline {
        pub fn new(v: Vec<Point>) -> Result<Self, PolyError> {
            if v.is_empty() {
                return Err(PolyError::EmptyError(POLYERROR_EMPTY))
            }
            
            Ok(Self (v))
        }

        // due to requirement of non-empty requirement
        // We cannot have access to internal set of points
        // instead we can expose public API to mutate a specific point
        /// Exposes mutable reference to a specific point in the set.
        /// 
        /// An index must be provided which denotes the position of the `Point`
        /// inside of the set.
        pub fn expose_mut(&mut self, index: usize) -> Option<&mut Point> {
            if index >= self.0.len() {
                return None;
            }
            Some(&mut self.0[index])
        }

        // same goes here
        /// Exponses immutable reference to a specific point in the set.
        /// 
        /// An index must be provided which denotes the position of the `Point`
        /// inside of the set.
        pub fn expose_ref(&self, index: usize) -> Option<&Point> {
            if index >= self.0.len() {
                return None;
            }
            Some(&self.0[index])
        }

        pub fn insert_point(&mut self, p: Point) {
            self.0.push(p);
        }

        /// Remove point from set if the set won't be left empty
        /// 
        /// Otherwise return error string.
        pub fn remove_point(&mut self, index: usize) -> Result<(), String> {
            if self.0.len() - 1 == 0 {
                return Err("The set cannot be empty!!!!!!!!!!".to_owned());
            }
            self.0.remove(index);

            Ok(())
        }
    }
    
    impl std::fmt::Display for Polyline {
        fn fmt(
            &self,
            f: &mut std::fmt::Formatter
        ) -> std::fmt::Result
        {
            writeln!(f, "Polyline: ")?;

            for item in self.0.iter() {
                write!(f, "\t{p:?}", p=item)?;
            }

            Ok(())
        }
    }

    // This is just for demonstration point
    // This does not adhere stated invariants obviously
    // The point here was to show custom Clone implementation
    #[derive(Debug, Clone)]
    pub struct ClonedCustomPolyline(pub Vec<Point>);
}