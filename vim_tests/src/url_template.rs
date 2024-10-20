use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

/// A substitution group name and value.
pub type Substitutions<'a> = Vec<(&'a str, &'a str)>;

/// Set that should be safe for usage in VIM API i.e. hostname and path string
/// Reduced the NON_ALPHANUMERIC from percent_encoding
pub const RESERVED: &AsciiSet = &CONTROLS
    // Reserved RFC 3986 - ! 	# 	$ 	& 	' 	( 	) 	* 	+ 	, 	/ 	: 	; 	= 	? 	@ 	[ 	]
    .add(b'!')
    .add(b'#')
    .add(b'$')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b',')
    .add(b'/')
    .add(b':')
    .add(b';')
    .add(b'=')
    .add(b'?')
    .add(b'@')
    .add(b'[')
    .add(b']');

/// Rough URI template friendly substitution utility of placeholders with values. The placeholders
/// are enclosed in curly braces, e.g. `{name}`. The substitutions are a list
/// of tuples with the placeholder name and the value to substitute.
///
/// Encode all non-visible ASCII characters and the reserved characters in the
/// URL template. The reserved characters are defined in RFC 3986.
///
/// The URI template is defined in RFC 6570.
///
pub fn substitute(s: &str, substitutions: &Substitutions) -> String {
    substitute_ascii_set(s, substitutions, RESERVED)
}
pub fn substitute_ascii_set(
    s: &str,
    substitutions: &Substitutions,
    ascii_set: &'static AsciiSet,
) -> String {
    let mut result = s.to_string();
    for (name, value) in substitutions {
        result = result.replace(
            &format!("{{{}}}", name),
            &utf8_percent_encode(value, ascii_set).to_string(),
        );
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn substitute_test() {
        assert_eq!(
            substitute(
                "https://{server}/vim/{release}/PC/{moId}/RP",
                &vec![
                    ("server", "www.vim.org"),
                    ("release", "8.0.2.0"),
                    ("moId", "vm-42")
                ]
            ),
            "https://www.vim.org/vim/8.0.2.0/PC/vm-42/RP"
        );
    }
}
