use crate::constants::SECONDS_IN_DAY;
use arrow::array::{
    ArrayBuilder, BooleanBuilder, Date32Builder, Date64Builder, Float32Builder, Float64Builder,
    Int32Builder, Int64Builder, LargeBinaryBuilder, StringBuilder, Time64NanosecondBuilder,
    UInt32Builder, UInt64Builder,
};
use arrow::datatypes::Field;
use arrow::datatypes::{DataType as ArrowDataType, TimeUnit};
use chrono::{Date, DateTime, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Utc};

/// Associate arrow builder with native type
pub trait ArrowAssoc {
    type Builder: ArrayBuilder + Send;

    fn builder(nrows: usize) -> Self::Builder;
    fn append(builder: &mut Self::Builder, value: Self);
    fn field(header: &str) -> Field;
}

macro_rules! impl_arrow_assoc {
    ($T:ty, $AT:expr, $B:ty) => {
        impl ArrowAssoc for $T {
            type Builder = $B;

            fn builder(nrows: usize) -> Self::Builder {
                Self::Builder::with_capacity(nrows)
            }

            fn append(builder: &mut Self::Builder, value: Self) {
                builder.append_value(value);
            }

            fn field(header: &str) -> Field {
                Field::new(header, $AT, false)
            }
        }

        impl ArrowAssoc for Option<$T> {
            type Builder = $B;

            fn builder(nrows: usize) -> Self::Builder {
                Self::Builder::with_capacity(nrows)
            }

            fn append(builder: &mut Self::Builder, value: Self) {
                builder.append_option(value);
            }

            fn field(header: &str) -> Field {
                Field::new(header, $AT, true)
            }
        }
    };
}

impl_arrow_assoc!(u32, ArrowDataType::UInt32, UInt32Builder);
impl_arrow_assoc!(u64, ArrowDataType::UInt64, UInt64Builder);
impl_arrow_assoc!(i32, ArrowDataType::Int32, Int32Builder);
impl_arrow_assoc!(i64, ArrowDataType::Int64, Int64Builder);
impl_arrow_assoc!(f32, ArrowDataType::Float32, Float32Builder);
impl_arrow_assoc!(f64, ArrowDataType::Float64, Float64Builder);
impl_arrow_assoc!(bool, ArrowDataType::Boolean, BooleanBuilder);

impl ArrowAssoc for &str {
    type Builder = StringBuilder;

    fn builder(_nrows: usize) -> Self::Builder {
        StringBuilder::new()
    }

    fn append(builder: &mut Self::Builder, value: Self) {
        builder.append_value(value);
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}

impl ArrowAssoc for Option<&str> {
    type Builder = StringBuilder;

    fn builder(_nrows: usize) -> Self::Builder {
        StringBuilder::new()
    }

    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s),
            None => builder.append_null(),
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, true)
    }
}

impl ArrowAssoc for String {
    type Builder = StringBuilder;

    fn builder(_nrows: usize) -> Self::Builder {
        StringBuilder::new()
    }

    fn append(builder: &mut Self::Builder, value: String) {
        builder.append_value(value.as_str());
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, false)
    }
}

impl ArrowAssoc for Option<String> {
    type Builder = StringBuilder;

    fn builder(_nrows: usize) -> Self::Builder {
        StringBuilder::new()
    }

    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(s) => builder.append_value(s.as_str()),
            None => builder.append_null(),
        }
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Utf8, true)
    }
}

impl ArrowAssoc for DateTime<Utc> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: DateTime<Utc>) {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Option<DateTime<Utc>> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Option<DateTime<Utc>>) {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Date<Utc> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Date<Utc>) {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

impl ArrowAssoc for Option<Date<Utc>> {
    type Builder = Float64Builder;

    fn builder(_nrows: usize) -> Float64Builder {
        unimplemented!()
    }

    fn append(_builder: &mut Self::Builder, _value: Option<Date<Utc>>) {
        unimplemented!()
    }

    fn field(_header: &str) -> Field {
        unimplemented!()
    }
}

fn naive_date_to_arrow(nd: NaiveDate) -> i32 {
    (nd.and_hms(0, 0, 0).timestamp() / SECONDS_IN_DAY) as i32
}

fn naive_datetime_to_arrow(nd: NaiveDateTime) -> i64 {
    nd.timestamp_millis()
}

impl ArrowAssoc for Option<NaiveDate> {
    type Builder = Date32Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date32Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveDate>) {
        builder.append_option(value.map(naive_date_to_arrow));
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date32, true)
    }
}

impl ArrowAssoc for NaiveDate {
    type Builder = Date32Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date32Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveDate) {
        builder.append_value(naive_date_to_arrow(value));
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date32, false)
    }
}

impl ArrowAssoc for Option<NaiveDateTime> {
    type Builder = Date64Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date64Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveDateTime>) {
        builder.append_option(value.map(naive_datetime_to_arrow));
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date64, true)
    }
}

impl ArrowAssoc for NaiveDateTime {
    type Builder = Date64Builder;

    fn builder(nrows: usize) -> Self::Builder {
        Date64Builder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveDateTime) {
        builder.append_value(naive_datetime_to_arrow(value));
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Date64, false)
    }
}

impl ArrowAssoc for Option<NaiveTime> {
    type Builder = Time64NanosecondBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        Time64NanosecondBuilder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: Option<NaiveTime>) {
        builder.append_option(
            value.map(|t| {
                t.num_seconds_from_midnight() as i64 * 1_000_000_000 + t.nanosecond() as i64
            }),
        );
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Time64(TimeUnit::Nanosecond), true)
    }
}

impl ArrowAssoc for NaiveTime {
    type Builder = Time64NanosecondBuilder;

    fn builder(nrows: usize) -> Self::Builder {
        Time64NanosecondBuilder::with_capacity(nrows)
    }

    fn append(builder: &mut Self::Builder, value: NaiveTime) {
        builder.append_value(
            value.num_seconds_from_midnight() as i64 * 1_000_000_000 + value.nanosecond() as i64,
        );
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::Time64(TimeUnit::Nanosecond), false)
    }
}

impl ArrowAssoc for Option<Vec<u8>> {
    type Builder = LargeBinaryBuilder;

    fn builder(_nrows: usize) -> Self::Builder {
        LargeBinaryBuilder::new()
    }

    fn append(builder: &mut Self::Builder, value: Self) {
        match value {
            Some(v) => builder.append_value(v),
            None => builder.append_null(),
        };
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeBinary, true)
    }
}

impl ArrowAssoc for Vec<u8> {
    type Builder = LargeBinaryBuilder;

    fn builder(_nrows: usize) -> Self::Builder {
        LargeBinaryBuilder::new()
    }

    fn append(builder: &mut Self::Builder, value: Self) {
        builder.append_value(value);
    }

    fn field(header: &str) -> Field {
        Field::new(header, ArrowDataType::LargeBinary, false)
    }
}
