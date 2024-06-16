/// Simple macro that allows defining a const array without specifying the length.
macro_rules! arr {
    ($id:ident $name:ident: [$ty:ty; _] = $value:expr) => {
        $id $name: [$ty; $value.len()] = $value;
    };
    (pub $id:ident $name:ident: [$ty:ty; _] = $value:expr) => {
        pub $id $name: [$ty; $value.len()] = $value;
    }
}