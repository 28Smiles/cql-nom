/// A CQL Type
/// More Information: <https://cassandra.apache.org/doc/latest/cassandra/cql/types.html>
#[derive(Debug, Clone, PartialEq)]
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
