# inline_newtype
A rust newtype macro inspired by kotlin's inline class.
When we use
```rust 
newtype!(NewTypeOne, u32);
```
It generate the struct
```
#[derive(Debug, Clone)]
    struct NewTypeOne {
        pub v: u32,
    }
```
for you.
The ***v*** is the default public field.
```rust
use inline_newtype::newtype;
use std::path::PathBuf;
newtype!(UserHomeDirectory, PathBuf);
newtype!(UserRuntimeDirectory, PathBuf);
let user_home_directory = UserHomeDirectory { v: PathBuf::from("hello") };
let user_runtime_directory= UserRuntimeDirectory {v: PathBuf::from("hello")};
fn test_newtype_type_func(user_home_directory: UserHomeDirectory) -> UserHomeDirectory{
         user_home_directory
}
```
```compile_fail
test_newtype_type_func(user_runtime_directory);  // mismatch type
```
You can aslo make the newtype public just adding the pub.
```rust
use inline_newtype::newtype;
use std::path::PathBuf;
newtype!(UserHomeDirectory, PathBuf, pub);
```
You also can change the field name if you want.
```rust
use inline_newtype::newtype;
use std::path::PathBuf;
newtype!(UserHomeDirectory, PathBuf, path_buf);
let user_home_directory = UserHomeDirectory { path_buf: PathBuf::from("hello")};
assert_eq!(user_home_directory.path_buf, PathBuf::from("hello"));
```
Transform from one newtype to another
```rust
use inline_newtype::newtype;
use std::path::PathBuf;
newtype!(UserHomeDirectory, PathBuf);
newtype!(UserRuntimeDirectory, PathBuf);
let user_home_directory = UserHomeDirectory { v: PathBuf::from("hello") };
let user_runtime_directory = UserRuntimeDirectory { v: PathBuf::from("hello") };
fn transform_user_home_to_runtime_directory(
    mut user_home_directory: UserHomeDirectory,
) -> UserRuntimeDirectory {
    let mut runtime_dir = user_home_directory.v;
    runtime_dir.push("runtime_dir");
    UserRuntimeDirectory { v: runtime_dir }
}

```
You can also using braces to declare the newtype.
```rust
use inline_newtype::newtype;
use std::path::PathBuf;
newtype! {UserHomeDirectory, PathBuf, path_buf}
let user_home_directory = UserHomeDirectory { path_buf: PathBuf::from("hello") };
assert_eq!(user_home_directory.path_buf, PathBuf::from("hello"))
```
