use crate::model::*;
use derive_more::{IsVariant, Unwrap};
use std::ops::Deref;
use std::rc::Rc;

/// A CQL Type
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html>
#[derive(Debug, Clone, PartialEq, IsVariant, Unwrap)]
pub enum CqlType<UdtType> {
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
    /// The frozen keyword is used to indicate that the type is immutable and can be used as a key in a map or set.
    FROZEN(Box<CqlType<UdtType>>),
    /// A map of keys to values.
    MAP(Box<(CqlType<UdtType>, CqlType<UdtType>)>),
    /// A set of values.
    SET(Box<CqlType<UdtType>>),
    /// A list of values.
    LIST(Box<CqlType<UdtType>>),
    /// The tuple type is used to indicate that the type is a tuple type.
    TUPLE(Vec<CqlType<UdtType>>),
    /// The user defined type is used to indicate that the type is a user defined type.
    UserDefined(UdtType),
}

impl<UdtTypeRef> CqlType<UdtTypeRef> {
    pub(crate) fn reference_types<I, Table, UdtType>(
        self,
        keyspace: Option<&CqlIdentifier<I>>,
        context: &Vec<CqlStatement<Table, Rc<UdtType>>>,
    ) -> Result<CqlType<Rc<UdtType>>, CqlQualifiedIdentifier<I>>
    where
        I: Deref<Target = str> + Clone,
        UdtTypeRef: Identifiable<I>,
        UdtType: Identifiable<I>,
    {
        match self {
            CqlType::ASCII => Ok(CqlType::ASCII),
            CqlType::BIGINT => Ok(CqlType::BIGINT),
            CqlType::BLOB => Ok(CqlType::BLOB),
            CqlType::BOOLEAN => Ok(CqlType::BOOLEAN),
            CqlType::COUNTER => Ok(CqlType::COUNTER),
            CqlType::DATE => Ok(CqlType::DATE),
            CqlType::DECIMAL => Ok(CqlType::DECIMAL),
            CqlType::DOUBLE => Ok(CqlType::DOUBLE),
            CqlType::DURATION => Ok(CqlType::DURATION),
            CqlType::FLOAT => Ok(CqlType::FLOAT),
            CqlType::INET => Ok(CqlType::INET),
            CqlType::INT => Ok(CqlType::INT),
            CqlType::SMALLINT => Ok(CqlType::SMALLINT),
            CqlType::TEXT => Ok(CqlType::TEXT),
            CqlType::TIME => Ok(CqlType::TIME),
            CqlType::TIMESTAMP => Ok(CqlType::TIMESTAMP),
            CqlType::TIMEUUID => Ok(CqlType::TIMEUUID),
            CqlType::TINYINT => Ok(CqlType::TINYINT),
            CqlType::UUID => Ok(CqlType::UUID),
            CqlType::VARCHAR => Ok(CqlType::VARCHAR),
            CqlType::VARINT => Ok(CqlType::VARINT),
            CqlType::FROZEN(udt) => Ok(CqlType::FROZEN(Box::new(
                udt.reference_types(keyspace, context)?,
            ))),
            CqlType::MAP(map) => {
                let (key, value) = *map;
                Ok(CqlType::MAP(Box::new((
                    key.reference_types(keyspace, context)?,
                    value.reference_types(keyspace, context)?,
                ))))
            }
            CqlType::SET(udt) => {
                let udt = *udt;
                Ok(CqlType::SET(Box::new(
                    udt.reference_types(keyspace, context)?,
                )))
            }
            CqlType::LIST(udt) => {
                let udt = *udt;
                Ok(CqlType::LIST(Box::new(
                    udt.reference_types(keyspace, context)?,
                )))
            }
            CqlType::TUPLE(udts) => Ok(CqlType::TUPLE(
                udts.into_iter()
                    .map(|udt| udt.reference_types(keyspace, context))
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            CqlType::UserDefined(udt) => context
                .iter()
                .find(|statement| {
                    statement
                        .create_user_defined_type()
                        .map(|udt_definition| {
                            udt_definition.contextualized_identifier(keyspace.clone())
                                == udt.contextualized_identifier(keyspace.clone())
                        })
                        .unwrap_or(false)
                })
                .map(|udt_definition| {
                    CqlType::UserDefined(udt_definition.create_user_defined_type().unwrap().clone())
                })
                .ok_or(udt.contextualized_identifier(keyspace)),
        }
    }
}
