use pyo3::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
enum Lang {
    Python,
    Rust,
    C,
    Java,
}

#[pyclass]
#[derive(Debug, Clone, Copy)]
enum Language {
    Python,
    Rust,
    C,
    Go,
    Cpp,
    Java,
    Typescript,
    Tsx,
    Javascript,
    Jsx,
    Scala,
    CSharp,
    Ruby,
}

impl Lang {
    fn get_meaningful_line_indices(&self, src: &str) -> Vec<usize> {
        use dracula::count::*;
        use dracula::langs::*;
        match self {
            Lang::Python => get_meaningful_line_indices::<Python>(src)
                .flatten()
                .collect(),
            Lang::Rust => get_meaningful_line_indices::<Rust>(src).flatten().collect(),
            Lang::C => get_meaningful_line_indices::<C>(src).flatten().collect(),
            Lang::Java => get_meaningful_line_indices::<Java>(src).flatten().collect(),
        }
    }
    #[rustfmt::skip]
    fn get_cleaned_source_code(&self, src: &str) -> String {
        use dracula::count::*;
        use dracula::langs::*;
        match self {
            Lang::Python => {
                get_cleaned_source_code::<Python>(src)
                    .unwrap_or_else(|| src.to_string())
            }
            Lang::Rust => {
                get_cleaned_source_code::<Rust>(src)
                    .unwrap_or_else(|| src.to_string())
            },
            Lang::C => {
                get_cleaned_source_code::<C>(src)
                    .unwrap_or_else(|| src.to_string())
            },
            Lang::Java => {
                get_cleaned_source_code::<Java>(src)
                    .unwrap_or_else(|| src.to_string())
            },
        }
    }
    fn get_count_of_meaningful_lines(&self, src: &str) -> usize {
        use dracula::count::*;
        use dracula::langs::*;
        match self {
            Lang::Python => get_count_of_meaningful_lines::<Python>(src),
            Lang::Rust => get_count_of_meaningful_lines::<Rust>(src),
            Lang::C => get_count_of_meaningful_lines::<C>(src),
            Lang::Java => get_count_of_meaningful_lines::<Java>(src),
        }
    }
}

#[pyfunction]
fn get_meaningful_line_indices(lang: Lang, src: &str) -> Vec<usize> {
    lang.get_meaningful_line_indices(src)
}

#[pyfunction]
/// This function gets the list of lines that can be assumed to be meaningful/executable
/// from a test-coverage or similar standpoint.
///
/// Further returns the list of line indexes starting from index 1.
///
/// If the parsing fails, then this returns None
fn get_lines_with_executable_code(lang: Language, src: &str) -> Option<Vec<usize>> {
    use dracula::parse::v2::*;
    let treesitter_lang = match lang {
        Language::Python => TreeSitterLanguage::Python,
        Language::Rust => TreeSitterLanguage::Rust,
        Language::C => TreeSitterLanguage::C,
        Language::Cpp => TreeSitterLanguage::Cpp,
        Language::Java => TreeSitterLanguage::Java,
        Language::Typescript => TreeSitterLanguage::Typescript,
        Language::Tsx => TreeSitterLanguage::TSX,
        Language::Javascript | Language::Jsx => TreeSitterLanguage::Javascript,
        // currently untested consider waiting for proper tests to be merged before using
        Language::Scala => TreeSitterLanguage::Scala,
        Language::CSharp => TreeSitterLanguage::CSharp,
        Language::Ruby => TreeSitterLanguage::Ruby,
        Language::Go => TreeSitterLanguage::Go,
    };
    Parser::new(treesitter_lang)
        .and_then(|mut parser| parser.non_executable_src_spans(src))
        .map(|spans| get_lines_without_ranges(src, spans))
}

#[pyfunction]
fn get_cleaned_source_code(lang: Lang, src: &str) -> String {
    lang.get_cleaned_source_code(src)
}

#[pyfunction]
fn get_count_of_meaningful_lines(lang: Lang, src: &str) -> usize {
    lang.get_count_of_meaningful_lines(src)
}

/// the python module definition
#[pymodule]
fn pydracula(_py: Python<'_>, m: &types::PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_meaningful_line_indices, m)?)?;
    m.add_function(wrap_pyfunction!(get_cleaned_source_code, m)?)?;
    m.add_function(wrap_pyfunction!(get_count_of_meaningful_lines, m)?)?;
    m.add_function(wrap_pyfunction!(get_lines_with_executable_code, m)?)?;
    m.add_class::<Language>()?;
    m.add_class::<Lang>()?;
    Ok(())
}
