#[macro_export]
macro_rules! languages_supported {
    ($(const $name:ident = $num:literal;)+) => {
        $(
            const $name: std::ffi::c_uint = $num;
        )+
        fn get_parser(lang: std::ffi::c_uint) -> Option<fn(&str) -> dracula::parse::Parser> {
            use dracula::parse::Language;
            $(
                if lang == $name {
                    return Some(dracula::langs::$name::get_parser());
                }
            )+
            None
        }
        fn is_meaningful(lang: std::ffi::c_uint) -> Option<fn(&dracula::parse::ParseOutput) -> bool> {
            use dracula::parse::Language;
            $(
                if lang == $name {
                    return Some(dracula::langs::$name::is_meaningful());
                }
            )+
            None
        }
        fn is_meaningful_src(lang: std::ffi::c_uint) -> Option<fn(&str) -> bool> {
            use dracula::parse::Language;
            $(
                if lang == $name {
                    return Some(dracula::langs::$name::is_meaningful_src);
                }
            )+
            None
        }
    };
}
