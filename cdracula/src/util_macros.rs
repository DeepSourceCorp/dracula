#[macro_export]
macro_rules! languages_supported {
    ($(const $name:ident = $num:literal;)+) => {
        $(
            const $name: std::ffi::c_uint = $num;
        )+
        pub fn get_meaningful_line_indices_as_u64(idx: std::ffi::c_uint, src: &str) -> Option<Vec<u64>> {
            $(
                if idx == $num {
                    return Some(
                        dracula::count::get_meaningful_line_indices::<dracula::langs::$name>(src)
                            .flatten()
                            .map(|x| x as u64)
                            .collect()
                    );
                }
            )+
            None
        }
        pub fn get_cleaned_source_code(idx: std::ffi::c_uint, src: &str) -> Option<String> {
            $(
                if idx == $num {
                    return Some(
                        dracula::count::get_cleaned_source_code::<dracula::langs::$name>(src)
                    );
                }
            )+
            None
        }
        pub fn get_count_of_meaningful_lines_as_u64(idx: std::ffi::c_uint, src: &str) -> Option<u64> {
            $(
                if idx == $num {
                    return Some(
                        dracula::count::get_count_of_meaningful_lines::<dracula::langs::$name>(src) as _
                    );
                }
            )+
            None
        }
    }
}
