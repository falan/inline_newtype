//!
//! A rust newtype macro inspired by kotlin's inline class.
//!
//!
//! When we use
//! ```rust newtype!(NewTypeOne, u32);```
//! It generate the struct
//!```rust
//! #[derive(Debug, Clone)]
//! struct NewTypeOne(u32);
//! impl NewTypeOne {
//!    pub fn v(&self) -> u32 {
//!        self.0.clone()
//!    }
//! }
//!```
//! for you.
//! The ***v*** is the default public field.
//!
//! ```rust
//! use inline_newtype::newtype;
//! use std::path::PathBuf;
//! newtype!(UserHomeDirectory, PathBuf);
//! newtype!(UserRuntimeDirectory, PathBuf);
//! let user_home_directory = UserHomeDirectory (PathBuf::from("hello"));
//! let user_runtime_directory= UserRuntimeDirectory (PathBuf::from("hello"));
//! fn test_newtype_type_func(user_home_directory: UserHomeDirectory) -> UserHomeDirectory{
//!          user_home_directory
//! }
//! ```
//! Transform from one newtype to another
//! ```rust
//! use inline_newtype::newtype;
//! use std::path::PathBuf;
//! newtype!(UserHomeDirectory, PathBuf);
//! newtype!(UserRuntimeDirectory, PathBuf);
//! let user_home_directory = UserHomeDirectory(PathBuf::from("hello") );
//! let user_runtime_directory = UserRuntimeDirectory (PathBuf::from("hello") );
//! fn transform_user_home_to_runtime_directory(
//!     mut user_home_directory: UserHomeDirectory,
//! ) -> UserRuntimeDirectory {
//!     let mut runtime_dir = user_home_directory.v();
//!     runtime_dir.push("runtime_dir");
//!     UserRuntimeDirectory(runtime_dir)
//! }
//! ```

#[macro_export]
macro_rules! newtype {
    ($type_name: ident, $type: ty) => {
        #[derive(Debug, Clone)]
        struct $type_name($type);

        impl $type_name {
            pub fn v(&self) -> $type {
                self.0.clone()
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_new_type() {
        newtype!(UserHomeDirectory, PathBuf);
        let user_home_directory = UserHomeDirectory(PathBuf::from("hello"));
        assert_eq!(user_home_directory.v(), PathBuf::from("hello"));
    }
}
