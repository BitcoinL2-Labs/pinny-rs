#[macro_export]
macro_rules! function_path {
    () => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        fn f() {}
        let full = type_name_of(f);
        let full = &full[..full.len() - 3]; // remove trailing "::f"

        match full.find("::") {
            Some(pos) => &full[pos + 2..], // skip crate name past first "::"
            None => full,
        }
    }};
    ($func:path) => {{
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        let full = type_name_of($func);
        match full.find("::") {
            Some(pos) => &full[pos + 2..],
            None => full,
        }
    }};
}
