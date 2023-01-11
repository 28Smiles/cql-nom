//! # Cql-Nom, a parser for the Cassandra Query Language
//!
//! This crate provides a parser for the Cassandra Query Language (CQL).
//! It is based on the [nom](https://github.com/rust-bakery/nom) parser combinator library.
//!
//! ## Example
//! ```rust
//! use cql_nom::parse_cql;
//!
//! let input = r#"CREATE TABLE IF NOT EXISTS my_keyspace.my_table (
//!     my_field1 int,
//!     my_field2 text,
//!     PRIMARY KEY (my_field1)
//! ) WITH CLUSTERING ORDER BY (my_field2 DESC);"#;
//!
//! let result = parse_cql(input).unwrap();
//! ```
//!
//! The code is available on [GitHub](https://github.com/28Smiles/cql-nom).

use nom::error::{ErrorKind, ParseError};
use nom::Err::Failure;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum CqlError<I> {
    UnknownType(String),
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for CqlError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CqlError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}

pub type IResult<I, O, E = CqlError<I>> = nom::IResult<I, O, E>;

pub fn parse_cql(input: &str) -> IResult<&str, CqlStatements> {
    use nom::character::complete::multispace0;

    let (input, _) = multispace0(input)?;
    let (input, statements) = CqlStatements::parse(input)?;
    let (input, _) = multispace0(input)?;

    Ok((input, statements))
}

/// A CQL Type
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html>
#[derive(Debug, Clone, PartialEq)]
pub enum CqlType {
    /// The frozen keyword is used to indicate that the type is immutable and can be used as a key in a map or set.
    Frozen(Box<CqlType>),
    /// The native type is used to indicate that the type is a native type.
    Native(CqlNativeType),
    /// The collection type is used to indicate that the type is a collection type.
    Collection(Box<CqlCollectionType>),
    /// The user defined type is used to indicate that the type is a user defined type.
    /// It is wrapped in an Rc and shared across the AST.
    UserDefined(Rc<CqlUserDefinedType>),
    /// The tuple type is used to indicate that the type is a tuple type.
    Tuple(CqlTupleType),
}

impl CqlType {
    /// Parse a CQL type.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlType> {
        use nom::branch::alt;
        use nom::bytes::complete::tag_no_case;
        use nom::combinator::{map, opt};

        let (input, cql_type) = alt((
            |input| Self::parse_frozen(input, udts),
            map(CqlNativeType::parse, CqlType::Native),
            map(
                |input| CqlCollectionType::parse(input, udts),
                |c| CqlType::Collection(Box::new(c)),
            ),
            map(|input| CqlTupleType::parse(input, udts), CqlType::Tuple),
            map(|input| Self::parse_udt(input, udts), CqlType::UserDefined),
        ))(input)?;
        let (input, _) = opt(tag_no_case("frozen"))(input)?;
        Ok((input, cql_type))
    }

    /// Parse a udt type identifier.
    fn parse_udt<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, Rc<CqlUserDefinedType>> {
        let (input, identifier) = CqlIdentifier::parse(input)?;
        let udt = udts
            .iter()
            .find(|udt| udt.name == identifier)
            .map(|udt| *udt);

        if let Some(udt) = udt {
            Ok((input, udt.clone()))
        } else {
            Err(Failure(CqlError::UnknownType(identifier.into())))
        }
    }

    /// Parse frozen CQL type.
    fn parse_frozen<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlType> {
        use nom::bytes::complete::{tag, tag_no_case};
        use nom::character::complete::multispace0;

        let (input, _) = tag_no_case("frozen")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("<")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, cql_type) = Self::parse(input, udts)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(">")(input)?;
        Ok((input, CqlType::Frozen(Box::new(cql_type))))
    }
}

/// The cql native types.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#native-types>
///
/// Grammar:
/// ```bnf
/// native_type::= ASCII | BIGINT | BLOB | BOOLEAN | COUNTER | DATE
///     | DECIMAL | DOUBLE | DURATION | FLOAT | INET | INT
///     | SMALLINT | TEXT | TIME | TIMESTAMP | TIMEUUID | TINYINT
///     | UUID | VARCHAR | VARINT
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CqlNativeType {
    /// ASCII character string.
    ASCII,
    /// 64-bit signed integer.
    BIGINT,
    /// A variable-length byte array.
    BLOB,
    /// Boolean value.
    BOOLEAN,
    /// 64-bit counter. More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#counters>
    COUNTER,
    /// Date without a time zone. More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#dates>
    DATE,
    /// Arbitrary-precision decimal number.
    DECIMAL,
    /// 64-bit IEEE 754 floating point number.
    DOUBLE,
    /// A duration of time. More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#durations>
    DURATION,
    /// 32-bit IEEE 754 floating point number.
    FLOAT,
    /// IPv4 or IPv6 address.
    INET,
    /// 32-bit signed integer.
    INT,
    /// 16-bit signed integer.
    SMALLINT,
    /// UTF-8 character string.
    TEXT,
    /// Time without a time zone. More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#times>
    TIME,
    /// Timestamp without a time zone. More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#timestamps>
    TIMESTAMP,
    /// Time-based UUID. More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#timeuuid>
    TIMEUUID,
    /// 8-bit signed integer.
    TINYINT,
    /// A UUID of any version.
    UUID,
    /// UTF-8 character string.
    VARCHAR,
    /// Arbitrary-precision integer.
    VARINT,
}

impl CqlNativeType {
    /// Parse a CQL native type.
    pub fn parse(input: &str) -> IResult<&str, CqlNativeType> {
        use nom::branch::alt;
        use nom::bytes::complete::tag_no_case;
        use nom::combinator::map;

        alt((
            map(tag_no_case("ASCII"), |_| CqlNativeType::ASCII),
            map(tag_no_case("BIGINT"), |_| CqlNativeType::BIGINT),
            map(tag_no_case("BLOB"), |_| CqlNativeType::BLOB),
            map(tag_no_case("BOOLEAN"), |_| CqlNativeType::BOOLEAN),
            map(tag_no_case("COUNTER"), |_| CqlNativeType::COUNTER),
            map(tag_no_case("DATE"), |_| CqlNativeType::DATE),
            map(tag_no_case("DECIMAL"), |_| CqlNativeType::DECIMAL),
            map(tag_no_case("DOUBLE"), |_| CqlNativeType::DOUBLE),
            map(tag_no_case("DURATION"), |_| CqlNativeType::DURATION),
            map(tag_no_case("FLOAT"), |_| CqlNativeType::FLOAT),
            map(tag_no_case("INET"), |_| CqlNativeType::INET),
            map(tag_no_case("INT"), |_| CqlNativeType::INT),
            map(tag_no_case("SMALLINT"), |_| CqlNativeType::SMALLINT),
            map(tag_no_case("TEXT"), |_| CqlNativeType::TEXT),
            map(tag_no_case("TIME"), |_| CqlNativeType::TIME),
            map(tag_no_case("TIMESTAMP"), |_| CqlNativeType::TIMESTAMP),
            map(tag_no_case("TIMEUUID"), |_| CqlNativeType::TIMEUUID),
            map(tag_no_case("TINYINT"), |_| CqlNativeType::TINYINT),
            map(tag_no_case("UUID"), |_| CqlNativeType::UUID),
            map(tag_no_case("VARCHAR"), |_| CqlNativeType::VARCHAR),
            map(tag_no_case("VARINT"), |_| CqlNativeType::VARINT),
        ))(input)
    }
}

/// The cql collection types.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#collections>
///
/// Grammar:
/// ```bnf
/// collection_type::= MAP '<' cql_type',' cql_type'>'
///     | SET '<' cql_type '>'
/// 	| LIST '<' cql_type'>'
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum CqlCollectionType {
    /// A map of keys to values.
    MAP(CqlType, CqlType),
    /// A set of values.
    SET(CqlType),
    /// A list of values.
    LIST(CqlType),
}

impl CqlCollectionType {
    /// Parse a CQL collection type.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlCollectionType> {
        use nom::branch::alt;

        alt((
            |input| Self::parse_map(input, udts),
            |input| Self::parse_set(input, udts),
            |input| Self::parse_list(input, udts),
        ))(input)
    }

    fn parse_map<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlCollectionType> {
        use nom::bytes::complete::{tag, tag_no_case};
        use nom::character::complete::multispace0;

        let (input, _) = tag_no_case("MAP")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("<")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, key) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, value) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(">")(input)?;

        Ok((input, CqlCollectionType::MAP(key, value)))
    }

    fn parse_set<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlCollectionType> {
        use nom::bytes::complete::{tag, tag_no_case};
        use nom::character::complete::multispace0;

        let (input, _) = tag_no_case("SET")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("<")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, value) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(">")(input)?;

        Ok((input, CqlCollectionType::SET(value)))
    }

    fn parse_list<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlCollectionType> {
        use nom::bytes::complete::{tag, tag_no_case};
        use nom::character::complete::multispace0;

        let (input, _) = tag_no_case("LIST")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("<")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, value) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag(">")(input)?;

        Ok((input, CqlCollectionType::LIST(value)))
    }
}

/// User-defined type.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#user-defined-types>
///
/// Type Grammar:
/// ```bnf
/// user_defined_type::= udt_name
/// udt_name::= [ keyspace_name '.' ] identifier
/// ```
///
/// Definition Grammar:
/// ```bnf
/// create_type_statement::= CREATE TYPE [ IF NOT EXISTS ] udt_name
///         '(' field_definition ( ',' field_definition)* ')'
/// field_definition::= identifier cql_type
/// ```
///
/// Example Definition:
/// ```cql
/// CREATE TYPE IF NOT EXISTS user (
///    id uuid,
///    name text,
///    age int
/// );
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CqlUserDefinedType {
    /// The keyspace name.
    keyspace: Option<CqlIdentifier>,
    /// The name of the user-defined type.
    name: CqlIdentifier,
    /// The fields of the user-defined type.
    fields: Vec<(CqlIdentifier, CqlType)>,
}

impl CqlUserDefinedType {
    /// Parse user defined type.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlUserDefinedType> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{multispace0, multispace1};
        use nom::combinator::opt;
        use nom::multi::separated_list0;
        use nom::sequence::delimited;

        let (input, _) = Self::parse_create_type(input)?;
        let (input, _) = opt(Self::parse_if_not_exists)(input)?;

        let (input, _) = multispace1(input)?;

        let (input, (keyspace, name)) = Self::parse_identifier(input)?;
        let udts = &udts
            .iter()
            .filter(|udt| &udt.keyspace == &keyspace)
            .map(|udt| *udt)
            .collect::<Vec<_>>();
        let (input, _) = multispace0(input)?;
        let (input, fields) = delimited(
            tag("("),
            separated_list0(tag(","), |input| Self::parse_field(input, udts)),
            tag(")"),
        )(input)?;

        Ok((
            input,
            CqlUserDefinedType {
                keyspace,
                name,
                fields,
            },
        ))
    }

    fn parse_create_type(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace1;

        let (input, _) = tag_no_case("CREATE")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("TYPE")(input)?;

        Ok((input, ()))
    }

    fn parse_if_not_exists(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace1;

        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("IF")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("NOT")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("EXISTS")(input)?;

        Ok((input, ()))
    }

    fn parse_identifier(input: &str) -> IResult<&str, (Option<CqlIdentifier>, CqlIdentifier)> {
        use nom::bytes::complete::tag;
        use nom::character::complete::multispace0;
        use nom::combinator::opt;

        let (input, name_or_keyspace) = CqlIdentifier::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, dot) = opt(tag("."))(input)?;

        if dot.is_some() {
            let (input, _) = multispace0(input)?;
            let (input, name) = CqlIdentifier::parse(input)?;
            Ok((input, (Some(name_or_keyspace), name)))
        } else {
            Ok((input, (None, name_or_keyspace)))
        }
    }

    fn parse_field<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, (CqlIdentifier, CqlType)> {
        use nom::character::complete::{multispace0, multispace1};

        let (input, _) = multispace0(input)?;
        let (input, name) = CqlIdentifier::parse(input)?;
        let (input, _) = multispace1(input)?;
        let (input, ty) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;

        Ok((input, (name, ty)))
    }
}

/// Tuple type.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#tuples>
///
/// Grammar:
/// ```bnf
/// tuple_type::= TUPLE '<' cql_type( ',' cql_type)* '>'
/// tuple_literal::= '(' term( ',' term )* ')'
/// ```
///
/// Example:
/// ```cql
/// CREATE TABLE durations (
///   event text,
///   duration tuple<int, text>,
/// );
///
/// INSERT INTO durations (event, duration) VALUES ('ev1', (3, 'hours'));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CqlTupleType {
    /// The types in the tuple.
    types: Vec<CqlType>,
}

impl CqlTupleType {
    /// Parse tuple type.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlTupleType> {
        use nom::bytes::complete::tag;
        use nom::character::complete::multispace0;
        use nom::multi::separated_list0;

        let (input, _) = tag("TUPLE")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag("<")(input)?;
        let (input, types) =
            separated_list0(tag(","), |input| Self::parse_entry(input, udts))(input)?;
        let (input, _) = tag(">")(input)?;

        Ok((input, CqlTupleType { types }))
    }

    pub fn parse_entry<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlType> {
        use nom::character::complete::multispace0;

        let (input, _) = multispace0(input)?;
        let (input, ty) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;

        Ok((input, ty))
    }
}

/// The cql table.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
///
/// Grammar:
/// ```bnf
/// create_table_statement::= CREATE TABLE [ IF NOT EXISTS ] table_name '('
/// 	column_definition  ( ',' column_definition )*
/// 	[ ',' PRIMARY KEY '(' primary_key ')' ]
/// 	 ')' [ WITH table_options ]
/// column_definition::= column_name cql_type [ STATIC ] [ PRIMARY KEY]
/// primary_key::= partition_key [ ',' clustering_columns ]
/// partition_key::= column_name  | '(' column_name ( ',' column_name )* ')'
/// clustering_columns::= column_name ( ',' column_name )*
/// table_options:=: COMPACT STORAGE [ AND table_options ]
/// 	| CLUSTERING ORDER BY '(' clustering_order ')'
/// 	[ AND table_options ]  | options
/// clustering_order::= column_name (ASC | DESC) ( ',' column_name (ASC | DESC) )*
/// ```
///
/// Example:
/// ```cql
/// CREATE TABLE monkey_species (
///     species text PRIMARY KEY,
///     common_name text,
///     population varint,
///     average_size int
/// ) WITH comment='Important biological records';
///
/// CREATE TABLE timeline (
///     userid uuid,
///     posted_month int,
///     posted_time uuid,
///     body text,
///     posted_by text,
///     PRIMARY KEY (userid, posted_month, posted_time)
/// ) WITH compaction = { 'class' : 'LeveledCompactionStrategy' };
///
/// CREATE TABLE loads (
///     machine inet,
///     cpu int,
///     mtime timeuuid,
///     load float,
///     PRIMARY KEY ((machine, cpu), mtime)
/// ) WITH CLUSTERING ORDER BY (mtime DESC);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct CqlTable {
    /// The name of the table.
    keyspace: Option<CqlIdentifier>,
    /// The name of the table.
    name: CqlIdentifier,
    /// The columns of the table.
    columns: Vec<Rc<CqlColumn>>,
    /// The primary key of the table.
    primary_key: Option<CqlPrimaryKey>,
    /// The table options.
    options: Option<CqlTableOptions>,
}

impl CqlTable {
    /// Parse table.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlTable> {
        use nom::bytes::complete::tag;
        use nom::character::complete::{multispace0, multispace1};
        use nom::combinator::opt;
        use nom::multi::separated_list0;

        let (input, _) = Self::parse_create_table(input)?;
        let (input, _) = opt(Self::parse_if_not_exists)(input)?;
        let (input, _) = multispace1(input)?;
        let (input, (keyspace, name)) = Self::parse_identifier(input)?;

        let udts = &udts
            .iter()
            .filter(|udt| &udt.keyspace == &keyspace)
            .map(|udt| *udt)
            .collect::<Vec<_>>();

        let (input, _) = multispace0(input)?;
        let (input, _) = tag("(")(input)?;
        let (input, columns) =
            separated_list0(tag(","), |input| Self::parse_column(input, udts))(input)?;
        let columns = columns.into_iter().map(Rc::new).collect();
        let (input, primary_key) = opt(|input| Self::parse_primary_key(input, &columns))(input)?;
        let (input, _) = tag(")")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, options) = opt(|input| Self::parse_table_options(input, &columns))(input)?;

        Ok((
            input,
            CqlTable {
                keyspace,
                name,
                columns,
                primary_key,
                options,
            },
        ))
    }

    fn parse_column<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlColumn> {
        use nom::character::complete::multispace0;

        let (input, _) = multispace0(input)?;
        let (input, column) = CqlColumn::parse(input, udts)?;
        let (input, _) = multispace0(input)?;

        Ok((input, column))
    }

    fn parse_primary_key<'de>(
        input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, CqlPrimaryKey> {
        use nom::bytes::complete::{tag, tag_no_case};
        use nom::character::complete::{multispace0, multispace1};

        let (input, _) = tag(",")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, _) = tag_no_case("PRIMARY")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("KEY")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, primary_key) = CqlPrimaryKey::parse(input, columns)?;
        let (input, _) = multispace0(input)?;

        Ok((input, primary_key))
    }

    fn parse_table_options<'de>(
        input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, CqlTableOptions> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace1;

        let (input, _) = tag_no_case("WITH")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, table_options) = CqlTableOptions::parse(input, columns)?;

        Ok((input, table_options))
    }

    fn parse_create_table(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace1;

        let (input, _) = tag_no_case("CREATE")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("TABLE")(input)?;

        Ok((input, ()))
    }

    fn parse_if_not_exists(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace1;

        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("IF")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("NOT")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("EXISTS")(input)?;

        Ok((input, ()))
    }

    fn parse_identifier(input: &str) -> IResult<&str, (Option<CqlIdentifier>, CqlIdentifier)> {
        use nom::bytes::complete::tag;
        use nom::character::complete::multispace0;
        use nom::combinator::opt;

        let (input, name_or_keyspace) = CqlIdentifier::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, dot) = opt(tag("."))(input)?;

        if dot.is_some() {
            let (input, _) = multispace0(input)?;
            let (input, name) = CqlIdentifier::parse(input)?;
            Ok((input, (Some(name_or_keyspace), name)))
        } else {
            Ok((input, (None, name_or_keyspace)))
        }
    }
}

/// The cql column.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlColumn {
    /// The name of the column.
    name: CqlIdentifier,
    /// The type of the column.
    cql_type: CqlType,
    /// Whether the column is static.
    is_static: bool,
    /// Whether the column is part of the primary key.
    is_primary_key: bool,
}

impl CqlColumn {
    /// Parse column.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlColumn> {
        use nom::character::complete::multispace0;
        use nom::combinator::opt;

        let (input, name) = CqlIdentifier::parse(input)?;
        let (input, _) = multispace0(input)?;
        let (input, cql_type) = CqlType::parse(input, udts)?;
        let (input, _) = multispace0(input)?;
        let (input, is_static) = opt(Self::parse_static)(input)?;
        let (input, _) = multispace0(input)?;
        let (input, is_primary_key) = opt(Self::parse_primary_key)(input)?;

        Ok((
            input,
            CqlColumn {
                name,
                cql_type,
                is_static: is_static.is_some(),
                is_primary_key: is_primary_key.is_some(),
            },
        ))
    }

    fn parse_static(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace0;

        let (input, _) = multispace0(input)?;
        let (input, _) = tag_no_case("STATIC")(input)?;

        Ok((input, ()))
    }

    fn parse_primary_key(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::{multispace0, multispace1};

        let (input, _) = multispace0(input)?;
        let (input, _) = tag_no_case("PRIMARY")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("KEY")(input)?;

        Ok((input, ()))
    }
}

/// The cql primary key.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlPrimaryKey {
    /// The partition key.
    partition_key: Vec<Rc<CqlColumn>>,
    /// The clustering columns.
    clustering_columns: Vec<Rc<CqlColumn>>,
}

impl CqlPrimaryKey {
    /// Parse primary key.
    pub fn parse<'de>(
        input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, CqlPrimaryKey> {
        use nom::character::complete::multispace0;
        use nom::combinator::opt;

        let (input, partition_key) = Self::parse_partition_key(input, columns)?;
        let (input, _) = multispace0(input)?;
        let (input, clustering_columns) =
            opt(|input| Self::parse_clustering_columns(input, columns))(input)?;

        Ok((
            input,
            CqlPrimaryKey {
                partition_key,
                clustering_columns: clustering_columns.unwrap_or_default(),
            },
        ))
    }

    fn parse_partition_key<'de>(
        input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, Vec<Rc<CqlColumn>>> {
        use nom::branch::alt;
        use nom::bytes::complete::tag;
        use nom::character::complete::multispace0;
        use nom::combinator::map;
        use nom::multi::separated_list0;
        use nom::sequence::delimited;

        fn parse_partition_key_list(input: &str) -> IResult<&str, Vec<CqlIdentifier>> {
            fn parse_identifier(input: &str) -> IResult<&str, CqlIdentifier> {
                let (input, _) = multispace0(input)?;
                let (input, primary_key) = CqlIdentifier::parse(input)?;
                let (input, _) = multispace0(input)?;

                Ok((input, primary_key))
            }

            let (input, partition_key_list) = delimited(
                tag("("),
                separated_list0(tag(","), parse_identifier),
                tag(")"),
            )(input)?;

            Ok((input, partition_key_list))
        }

        let (input, names) = alt((
            map(CqlIdentifier::parse, |name| vec![name]),
            parse_partition_key_list,
        ))(input)?;

        Ok((
            input,
            names
                .into_iter()
                .map(|name| {
                    columns
                        .iter()
                        .find(|column| column.name == name)
                        .unwrap()
                        .clone()
                })
                .collect(),
        ))
    }

    fn parse_clustering_columns<'de>(
        input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, Vec<Rc<CqlColumn>>> {
        use nom::bytes::complete::tag;
        use nom::character::complete::multispace0;
        use nom::multi::separated_list1;

        fn parse_identifier(input: &str) -> IResult<&str, CqlIdentifier> {
            let (input, _) = multispace0(input)?;
            let (input, primary_key) = CqlIdentifier::parse(input)?;
            let (input, _) = multispace0(input)?;

            Ok((input, primary_key))
        }

        let (input, _) = multispace0(input)?;
        let (input, _) = tag(",")(input)?;
        let (input, clustering_column_names) = separated_list1(tag(","), parse_identifier)(input)?;

        Ok((
            input,
            clustering_column_names
                .into_iter()
                .map(|name| {
                    columns
                        .iter()
                        .find(|column| column.name == name)
                        .unwrap()
                        .clone()
                })
                .collect(),
        ))
    }
}

/// The cql table options.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlTableOptions {
    compact_storage: bool,
    /// The clustering order.
    clustering_order: Vec<(Rc<CqlColumn>, CqlOrder)>,
    /// The other options.
    options: Vec<(String, String)>,
}

impl CqlTableOptions {
    /// Parse table options.
    pub fn parse<'de>(
        mut input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, CqlTableOptions> {
        use nom::branch::alt;
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::{multispace0, multispace1};
        use nom::combinator::{map, opt};

        let mut compact_storage = false;
        let mut clustering_order = None;
        let mut options = Vec::new();

        loop {
            let (i, _) = multispace0(input)?;
            let (i, option) = opt(|input| {
                alt((
                    map(Self::parse_compact_storage, |_| {
                        compact_storage = true;
                    }),
                    map(
                        |input| Self::parse_clustering(input, columns),
                        |order| {
                            clustering_order = Some(order);
                        },
                    ),
                    map(Self::parse_option, |option| {
                        options.push(option);
                    }),
                ))(input)
            })(i)?;

            if option.is_none() {
                input = i;
                break;
            }

            fn parse_and(input: &str) -> IResult<&str, ()> {
                let (input, _) = multispace1(input)?;
                let (input, _) = tag_no_case("AND")(input)?;

                Ok((input, ()))
            }
            let (i, option) = opt(parse_and)(i)?;

            if option.is_none() {
                input = i;
                break;
            }

            input = i;
        }

        Ok((
            input,
            CqlTableOptions {
                compact_storage,
                clustering_order: clustering_order.unwrap_or_else(|| vec![]),
                options,
            },
        ))
    }

    pub fn parse_compact_storage(input: &str) -> IResult<&str, ()> {
        use nom::bytes::complete::tag_no_case;
        use nom::character::complete::multispace1;

        let (input, _) = tag_no_case("COMPACT")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("STORAGE")(input)?;

        Ok((input, ()))
    }

    pub fn parse_clustering<'de>(
        input: &'de str,
        columns: &Vec<Rc<CqlColumn>>,
    ) -> IResult<&'de str, Vec<(Rc<CqlColumn>, CqlOrder)>> {
        use nom::branch::alt;
        use nom::bytes::complete::{tag, tag_no_case};
        use nom::character::complete::{multispace0, multispace1};
        use nom::combinator::map;
        use nom::multi::separated_list1;
        use nom::sequence::delimited;

        fn parse_order(input: &str) -> IResult<&str, (CqlIdentifier, CqlOrder)> {
            let (input, _) = multispace0(input)?;
            let (input, name) = CqlIdentifier::parse(input)?;
            let (input, _) = multispace1(input)?;
            let (input, order) = alt((
                map(tag_no_case("ASC"), |_| CqlOrder::Asc),
                map(tag_no_case("DESC"), |_| CqlOrder::Desc),
            ))(input)?;

            Ok((input, (name, order)))
        }

        let (input, _) = tag_no_case("CLUSTERING")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("ORDER")(input)?;
        let (input, _) = multispace1(input)?;
        let (input, _) = tag_no_case("BY")(input)?;
        let (input, _) = multispace0(input)?;
        let (input, fields) =
            delimited(tag("("), separated_list1(tag(","), parse_order), tag(")"))(input)?;

        Ok((
            input,
            fields
                .into_iter()
                .map(|(name, order)| {
                    (
                        columns
                            .iter()
                            .find(|column| column.name == name)
                            .unwrap()
                            .clone(),
                        order,
                    )
                })
                .collect(),
        ))
    }

    pub fn parse_option(_input: &str) -> IResult<&str, (String, String)> {
        // TODO: parse options.
        unimplemented!("parse_option")
    }
}

/// The cql compaction strategy.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Clone, PartialEq)]
pub struct CqlCompactionStrategy {
    /// The class of the compaction strategy.
    class: String,
    /// The options of the compaction strategy.
    options: Vec<(String, String)>,
}

/// The cql order.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/ddl.html#create-table-statement>
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CqlOrder {
    Asc,
    Desc,
}

/// Cql Statement.
#[derive(Debug, Clone, PartialEq)]
pub enum CqlStatement {
    UDType(Rc<CqlUserDefinedType>),
    Table(Rc<CqlTable>),
}

impl CqlStatement {
    /// Parse a cql statement.
    pub fn parse<'de>(
        input: &'de str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'de str, CqlStatement> {
        use nom::branch::alt;
        use nom::combinator::map;

        alt((
            map(
                |input| CqlUserDefinedType::parse(input, udts),
                |udt| CqlStatement::UDType(Rc::new(udt)),
            ),
            map(
                |input| CqlTable::parse(input, udts),
                |table| CqlStatement::Table(Rc::new(table)),
            ),
        ))(input)
    }

    pub fn as_table(&self) -> Option<&Rc<CqlTable>> {
        match self {
            CqlStatement::Table(table) => Some(table),
            _ => None,
        }
    }

    pub fn as_user_defined_type(&self) -> Option<&Rc<CqlUserDefinedType>> {
        match self {
            CqlStatement::UDType(ud_type) => Some(ud_type),
            _ => None,
        }
    }
}

/// Cql Statements
pub struct CqlStatements(Vec<CqlStatement>);

impl CqlStatements {
    /// Parse cql statements.
    pub fn parse(input: &str) -> IResult<&str, CqlStatements> {
        use nom::bytes::complete::tag;
        use nom::combinator::opt;

        let mut statements = Vec::new();
        let mut input = input;
        loop {
            let (i, statement) = opt(|input| {
                Self::parse_statement(
                    input,
                    &statements
                        .iter()
                        .flat_map(|s: &CqlStatement| s.as_user_defined_type())
                        .collect(),
                )
            })(input)?;
            input = i;
            if let Some(statement) = statement {
                statements.push(statement);
                let (i, option) = opt(tag(";"))(input)?;
                input = i;
                if option.is_none() {
                    break;
                }
            } else {
                break;
            }
        }

        Ok((input, CqlStatements(statements)))
    }

    fn parse_statement<'a>(
        input: &'a str,
        udts: &Vec<&Rc<CqlUserDefinedType>>,
    ) -> IResult<&'a str, CqlStatement> {
        use nom::character::complete::multispace0;

        let (input, _) = multispace0(input)?;
        let (input, statement) = CqlStatement::parse(input, udts)?;
        let (input, _) = multispace0(input)?;

        Ok((input, statement))
    }

    pub fn get_udts(&self) -> Vec<&Rc<CqlUserDefinedType>> {
        self.0
            .iter()
            .flat_map(|s| s.as_user_defined_type())
            .collect()
    }
}

impl Deref for CqlStatements {
    type Target = Vec<CqlStatement>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Cql Identifier.
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html#identifiers>
/// ```bnf
/// identifier::= unquoted_identifier | quoted_identifier
/// unquoted_identifier::= re('[a-zA-Z][link:[a-zA-Z0-9]]*')
/// quoted_identifier::= '"' (any character where " can appear if doubled)+
/// ```
#[derive(Debug, Clone)]
pub enum CqlIdentifier {
    /// The unquoted identifier.
    Unquoted(String),
    /// The quoted identifier.
    Quoted(String),
}

impl PartialEq for CqlIdentifier {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (CqlIdentifier::Unquoted(s), CqlIdentifier::Unquoted(o))
            | (CqlIdentifier::Unquoted(s), CqlIdentifier::Quoted(o))
            | (CqlIdentifier::Quoted(s), CqlIdentifier::Unquoted(o)) => s.eq_ignore_ascii_case(o),
            (CqlIdentifier::Quoted(s), CqlIdentifier::Quoted(o)) => s == o,
        }
    }
}

impl Into<String> for CqlIdentifier {
    fn into(self) -> String {
        match self {
            CqlIdentifier::Unquoted(s) => s,
            CqlIdentifier::Quoted(s) => s,
        }
    }
}

impl Deref for CqlIdentifier {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        match self {
            CqlIdentifier::Unquoted(s) => s,
            CqlIdentifier::Quoted(s) => s,
        }
    }
}

impl CqlIdentifier {
    fn parse_quoted(input: &str) -> IResult<&str, CqlIdentifier> {
        use nom::bytes::complete::{tag, take_until};

        let (input, _) = tag("\"")(input)?;
        let mut acc = String::new();
        let mut input = input;
        loop {
            let (i, s) = take_until("\"")(input)?;
            acc.push_str(s);
            let (i, _) = tag("\"")(i)?;
            input = i;
            if !i.starts_with("\"") {
                break;
            }
            acc.push('"');
        }

        Ok((input, CqlIdentifier::Quoted(acc)))
    }

    fn parse_unquoted(input: &str) -> IResult<&str, CqlIdentifier> {
        use nom::bytes::complete::take_while1;
        use nom::character::complete::alpha1;
        use nom::AsChar;

        let (input, first) = alpha1(input)?;
        let (input, rest) =
            take_while1(|c: char| c.is_alpha() || c.is_dec_digit() || c == '_')(input)?;
        Ok((input, CqlIdentifier::Unquoted(format!("{}{}", first, rest))))
    }

    /// Parses the identifier.
    pub fn parse(input: &str) -> IResult<&str, CqlIdentifier> {
        use nom::branch::alt;

        alt((CqlIdentifier::parse_quoted, CqlIdentifier::parse_unquoted))(input)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_udt() {
        let input = r#"CREATE TYPE IF NOT EXISTS my_keyspace.my_type (
            my_field1 int,
            my_field2 text
        );"#;
        let (input, statement) = CqlUserDefinedType::parse(input, &vec![]).unwrap();
        assert_eq!(input, ";");
        assert_eq!(
            statement,
            CqlUserDefinedType {
                keyspace: Some(CqlIdentifier::Unquoted("my_keyspace".to_string())),
                name: CqlIdentifier::Unquoted("my_type".to_string()),
                fields: vec![
                    (
                        CqlIdentifier::Unquoted("my_field1".to_string()),
                        CqlType::Native(CqlNativeType::INT)
                    ),
                    (
                        CqlIdentifier::Unquoted("my_field2".to_string()),
                        CqlType::Native(CqlNativeType::TEXT)
                    ),
                ],
            }
        );
    }

    #[test]
    fn test_parse_table() {
        let input = r#"CREATE TABLE IF NOT EXISTS my_keyspace.my_table (
            my_field1 int,
            my_field2 text,
            PRIMARY KEY (my_field1)
        ) WITH CLUSTERING ORDER BY (my_field2 DESC);"#;
        let (input, statement) = CqlTable::parse(input, &vec![]).unwrap();
        assert_eq!(input, ";");
        let my_field1 = Rc::new(CqlColumn {
            name: CqlIdentifier::Unquoted("my_field1".to_string()),
            cql_type: CqlType::Native(CqlNativeType::INT),
            is_primary_key: false,
            is_static: false,
        });
        let my_field2 = Rc::new(CqlColumn {
            name: CqlIdentifier::Unquoted("my_field2".to_string()),
            cql_type: CqlType::Native(CqlNativeType::TEXT),
            is_primary_key: false,
            is_static: false,
        });
        assert_eq!(
            statement,
            CqlTable {
                keyspace: Some(CqlIdentifier::Unquoted("my_keyspace".to_string())),
                name: CqlIdentifier::Unquoted("my_table".to_string()),
                columns: vec![my_field1.clone(), my_field2.clone(),],
                primary_key: Some(CqlPrimaryKey {
                    partition_key: vec![my_field1.clone()],
                    clustering_columns: vec![],
                }),
                options: Some(CqlTableOptions {
                    clustering_order: vec![(my_field2.clone(), CqlOrder::Desc)],
                    compact_storage: false,
                    options: vec![],
                }),
            }
        )
    }

    #[test]
    fn test_parse_statements() {
        let input = r#"
        CREATE TYPE IF NOT EXISTS my_keyspace."my_type" (
            my_field1 int,
            my_field2 text
        );

        CREATE TABLE IF NOT EXISTS my_keyspace.my_table (
            my_field1 int,
            my_field2 my_type,
            PRIMARY KEY (my_field1)
        ) WITH CLUSTERING ORDER BY (my_field2 DESC);"#;

        let (input, statements) = CqlStatements::parse(input).unwrap();
        assert_eq!(input, "");
        let udt = statements
            .get(0)
            .unwrap()
            .as_user_defined_type()
            .unwrap()
            .clone();
        assert_eq!(
            *udt,
            CqlUserDefinedType {
                keyspace: Some(CqlIdentifier::Unquoted("my_keyspace".to_string())),
                name: CqlIdentifier::Quoted("my_type".to_string()),
                fields: vec![
                    (
                        CqlIdentifier::Unquoted("my_field1".to_string()),
                        CqlType::Native(CqlNativeType::INT)
                    ),
                    (
                        CqlIdentifier::Unquoted("my_field2".to_string()),
                        CqlType::Native(CqlNativeType::TEXT)
                    ),
                ],
            }
        );
        let table = statements.get(1).unwrap().as_table().unwrap().clone();
        let my_field1 = Rc::new(CqlColumn {
            name: CqlIdentifier::Unquoted("my_field1".to_string()),
            cql_type: CqlType::Native(CqlNativeType::INT),
            is_primary_key: false,
            is_static: false,
        });
        let my_field2 = Rc::new(CqlColumn {
            name: CqlIdentifier::Unquoted("my_field2".to_string()),
            cql_type: CqlType::UserDefined(udt.clone()),
            is_primary_key: false,
            is_static: false,
        });
        assert_eq!(
            *table,
            CqlTable {
                keyspace: Some(CqlIdentifier::Unquoted("my_keyspace".to_string())),
                name: CqlIdentifier::Unquoted("my_table".to_string()),
                columns: vec![my_field1.clone(), my_field2.clone(),],
                primary_key: Some(CqlPrimaryKey {
                    partition_key: vec![my_field1.clone()],
                    clustering_columns: vec![],
                }),
                options: Some(CqlTableOptions {
                    clustering_order: vec![(my_field2.clone(), CqlOrder::Desc)],
                    compact_storage: false,
                    options: vec![],
                }),
            }
        )
    }
}
