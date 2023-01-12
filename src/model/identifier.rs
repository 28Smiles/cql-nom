use std::ops::Deref;

/// Cql Identifier.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#identifiers>
/// ```bnf
/// identifier::= unquoted_identifier | quoted_identifier
/// unquoted_identifier::= re('[a-zA-Z][link:[a-zA-Z0-9]]*')
/// quoted_identifier::= '"' (any character where " can appear if doubled)+
/// ```
#[derive(Debug, Clone)]
pub enum CqlIdentifier<I> {
    /// The unquoted identifier.
    Unquoted(I),
    /// The quoted identifier.
    Quoted(String),
}

impl<I: Deref<Target = str>> PartialEq for CqlIdentifier<I> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CqlIdentifier::Unquoted(s), CqlIdentifier::Unquoted(o)) => s.eq_ignore_ascii_case(o),
            (CqlIdentifier::Unquoted(s), CqlIdentifier::Quoted(o)) => s.eq_ignore_ascii_case(o),
            (CqlIdentifier::Quoted(s), CqlIdentifier::Unquoted(o)) => s.eq_ignore_ascii_case(o),
            (CqlIdentifier::Quoted(s), CqlIdentifier::Quoted(o)) => s == o,
        }
    }
}

impl<I: Deref<Target = str>> Deref for CqlIdentifier<I> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            CqlIdentifier::Unquoted(s) => s.deref(),
            CqlIdentifier::Quoted(s) => s,
        }
    }
}
