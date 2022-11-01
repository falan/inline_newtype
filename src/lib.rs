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
