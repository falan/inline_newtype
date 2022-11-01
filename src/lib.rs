///
/// A rust newtype macro inspired by kotlin's inline class.
///
///
/// When we use
/// ```rust newtype!(NewTypeOne, u32);```
/// It generate the struct
///```
/// #[derive(Debug, Clone)]
///     struct UserHomeDirectory {
///         pub v: u32,
///     }
///```
/// for you.
/// The v is the default public field.
///
/// ```rust
/// use inline_newtype::newtype;
/// use std::path::PathBuf;
/// newtype!(UserHomeDirectory, PathBuf);
/// newtype!(UserRuntimeDirectory, PathBuf);
/// let user_home_directory = UserHomeDirectory { v: PathBuf::from("hello") };
/// let user_runtime_directory= UserRuntimeDirectory {v: PathBuf::from("hello")};
/// fn test_newtype_type_func(user_home_directory: UserHomeDirectory) -> UserHomeDirectory{
///          user_home_directory
/// }
/// ```
/// ```compile_fail
/// test_newtype_type_func(user_runtime_directory);  // mismatch type
/// ```
/// You can aslo make the newtype public just adding the pub.
/// ```rust
/// use inline_newtype::newtype;
/// use std::path::PathBuf;
/// newtype!(UserHomeDirectory, PathBuf, pub);
/// ```
///
/// You also can change the field name if you want.
/// ```rust
/// use inline_newtype::newtype;
/// use std::path::PathBuf;
/// newtype!(UserHomeDirectory, PathBuf, path_buf);
/// let user_home_directory = UserHomeDirectory { path_buf: PathBuf::from("hello")};
/// assert_eq!(user_home_directory.path_buf, PathBuf::from("hello"));
/// ```
///
/// You can also using curly brackets to declare the newtype.
/// ```rust
/// use inline_newtype::newtype;
/// use std::path::PathBuf;
/// newtype! {UserHomeDirectory, PathBuf, path_buf}
/// let user_home_directory = UserHomeDirectory { path_buf: PathBuf::from("hello") };
/// assert_eq!(user_home_directory.path_buf, PathBuf::from("hello"))
/// ```
///
#[macro_export]
macro_rules! newtype {
    ($type_name: ident, $type: ty) => {
        #[derive(Debug, Clone)]
        struct $type_name {
            pub v: $type,
        }
    };
    ($type_name: ident, $type: ty, $is_pub:vis) => {
        #[derive(Debug, Clone)]
        pub struct $type_name {
            pub v: $type,
        }
    };
    ($type_name: ident, $type: ty, $access_name: ident) => {
        #[derive(Debug, Clone)]
        struct $type_name {
            pub $access_name: $type,
        }
    };
    ($type_name: ident, $type: ty, $access_name: ident, $is_pub:vis) => {
        #[derive(Debug, Clone)]
        $is_pub struct $type_name {
            pub $access_name: $type,
        }
    };
    {$type_name: ident, $type: ty} => {
        #[derive(Debug, Clone)]
        struct $type_name {
            pub v: $type,
        }
    };
    {$type_name: ident, $type: ty, $is_pub:vis} => {
        #[derive(Debug, Clone)]
        pub struct $type_name {
            pub v: $type,
        }
    };
    {$type_name: ident, $type: ty, $access_name: ident} => {
        #[derive(Debug, Clone)]
        struct $type_name {
            pub $access_name: $type,
        }
    };
    {$type_name: ident, $type: ty, $access_name: ident, $is_pub:vis} => {
        #[derive(Debug, Clone)]
        $is_pub struct $type_name {
            pub $access_name: $type,
        }
    };
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn test_new_type() {
        newtype!(UserHomeDirectory, PathBuf);
        let user_home_directory = UserHomeDirectory { v: PathBuf::from("hello") };
        assert_eq!(user_home_directory.v, PathBuf::from("hello"));
    }


    #[test]
    fn test_type_not_equal() {
        newtype!(UserHomeDirectory, PathBuf);
        newtype!(UserRuntimeDirectory, PathBuf);
        let user_home_directory = UserHomeDirectory { v: PathBuf::from("hello") };
        let user_runtime_directory = UserRuntimeDirectory { v: PathBuf::from("hello") };
        fn test_newtype_type_func(user_home_directory: UserHomeDirectory) -> UserHomeDirectory {
            user_home_directory
        }
        // test_newtype_type_func(user_runtime_directory); mismatch type
    }

    #[test]
    fn test_field_name() {
        newtype!(UserHomeDirectory, PathBuf, path_buf);
        let user_home_directory = UserHomeDirectory { path_buf: PathBuf::from("hello") };
        assert_eq!(user_home_directory.path_buf, PathBuf::from("hello"))
    }

    #[test]
    fn test_curly_brackets_name() {
        newtype! {UserHomeDirectory, PathBuf, path_buf}
        let user_home_directory = UserHomeDirectory { path_buf: PathBuf::from("hello") };
        assert_eq!(user_home_directory.path_buf, PathBuf::from("hello"))
    }
}
//create new type of UserHomeDirectory  as PathBuf

//create new type of UserHomeDirectory  as PathBuf
// newtype!(UserRuntimeDirectory, PathBuf)

// let user_runtime_directory = UserRuntimeDicr
