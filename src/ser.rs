use serde::ser::{self, Serialize};
use byteorder::{LittleEndian, WriteBytesExt};
use error::{Error, Result};

const B_BEGIN:u8            = 0x40;
const B_END:u8              = 0x41;
const B_BEGIN_ARRAY:u8      = 0x42;
const B_END_ARRAY:u8        = 0x43;
const B_TRUE:u8             = 0x44;
const B_FALSE:u8            = 0x45;
const B_STRING_LEN:u8       = 0x14;
const B_STRING_LEN_16:u8    = 0x15;
const B_STRING_LEN_32:u8    = 0x16;
const B_INT8:u8             = 0x10;
const B_INT16:u8            = 0x11;
const B_INT32:u8            = 0x12;
const B_INT64:u8            = 0x13;
const B_DOUBLE:u8            = 0x46;

pub struct Serializer {
    output: Vec<u8>,
}

pub fn to_binson<T>(value: &T) -> Result<Vec<u8>>
    where T: Serialize
{
    let mut serializer = Serializer { output: Vec::new() };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output.push(if v { B_TRUE } else { B_FALSE });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.output.push(B_INT8);
        self.output.write_i8(v)?;
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.output.push(B_INT16);
        self.output.write_i16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.output.push(B_INT32);
        self.output.write_i32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output.push(B_INT64);
        self.output.write_i64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.output.push(B_INT8);
        self.output.write_u8(v)?;
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.output.push(B_INT16);
        self.output.write_u16::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.output.push(B_INT32);
        self.output.write_u32::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output.push(B_INT64);
        self.output.write_u64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(v as f64)
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output.push(B_DOUBLE);
        self.output.write_f64::<LittleEndian>(v)?;
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        let bytes = v.as_bytes();
        if bytes.len() <= 127 {
            self.output.push(B_STRING_LEN);
            self.output.push(bytes.len() as u8);
        } else if bytes.len() > 127 && bytes.len() <= 32767 {
            self.output.push(B_STRING_LEN_16);
            self.output.write_i16::<LittleEndian>(bytes.len() as i16).unwrap();
        } else {
            self.output.push(B_STRING_LEN_32);
            self.output.write_i32::<LittleEndian>(bytes.len() as i32).unwrap();
        }

        self.output.extend(v.as_bytes());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output.extend(v);
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_some<T>(self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.output.push(0x0);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str
    ) -> Result<()> {
        self.serialize_str(format!("{}:{}", _name, variant).as_str())
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T
    ) -> Result<()>
        where T: ?Sized + Serialize
    {
        variant.serialize(&mut *self)?;
        value.serialize(&mut *self)?;
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output.push(B_BEGIN_ARRAY);
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeTupleVariant> {
        // self.output.push(B_BEGIN);
        variant.serialize(&mut *self)?;
        self.output.push(B_BEGIN_ARRAY);
        Ok(self)
    }

    // Maps are represented as `{ K: V, K: V, ... }`.
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output.push(B_BEGIN);
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize
    ) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize
    ) -> Result<Self::SerializeStructVariant> {
        self.output.push(B_BEGIN);
        variant.serialize(&mut *self)?;
        self.output.push(B_BEGIN);
        Ok(self)
    }
}

// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END_ARRAY);
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END_ARRAY);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END_ARRAY);
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END_ARRAY);
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        key.serialize(&mut **self)
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END);
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END);
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing the seq opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
        where T: ?Sized + Serialize
    {
        key.serialize(&mut **self)?;
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        self.output.push(B_END);
        self.output.push(B_END);
        Ok(())
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn test_struct() {
    #[derive(Serialize)]
    struct Test {
        a: u8,
        b: bool,
    }

    let test = Test { b: false, a:2 };
    let expected = vec![B_BEGIN, B_STRING_LEN, 0x1, 0x61, B_INT8, 0x2, B_STRING_LEN, 0x1, 0x62, B_FALSE, B_END]; 
    assert_eq!(to_binson(&test).unwrap(), expected);
}
#[test]
fn test_struct_u32_and_seq_of_str() {
    #[derive(Serialize)]
    struct Test {
        int: u32,
        seq: Vec<&'static str>,
    }

    let test = Test { int: 1, seq: vec!["a", "b"] };
    let expected = vec![B_BEGIN, B_STRING_LEN, 0x3, 0x69, 0x6e, 0x74, B_INT32, 0x1, 0, 0, 0, B_STRING_LEN, 0x3, 0x73, 0x65, 0x71, B_BEGIN_ARRAY, B_STRING_LEN, 0x1, 0x61, B_STRING_LEN, 0x1, 0x62, B_END_ARRAY, B_END];
    assert_eq!(to_binson(&test).unwrap(), expected);
}

// fn print_bytes(s: &Vec<u8>) {
//     for byte in s {
//         print!("{:02X}", byte);
//     }
//     print!("\n");
// }

#[test]
fn test_enum() {
    #[derive(Serialize)]
    enum E {
        Newtype(u32),
        Unit,
        Tuple(u32, u32),
        Struct { a: u32 },
    }

    let u = E::Unit;
    let expected = vec![B_STRING_LEN, 0x6, 0x45, 0x3A, 0x55, 0x6E, 0x69, 0x74];
    assert_eq!(to_binson(&u).unwrap(), expected);

    let n = E::Newtype(3);
    let expected = vec![B_STRING_LEN, 0x7, 78, 101, 119, 116, 121, 112, 101, B_INT32, 3, 0, 0, 0];
    assert_eq!(to_binson(&n).unwrap(), expected);

    let t = E::Tuple(4, 5);
    let expected = vec![B_STRING_LEN, 0x5, 84, 117, 112, 108, 101, B_BEGIN_ARRAY, B_INT32, 4, 0 , 0, 0, B_INT32, 5, 0, 0, 0, B_END_ARRAY];
    assert_eq!(to_binson(&t).unwrap(), expected);

    let s = E::Struct { a: 1 };
    let expected = vec![B_BEGIN, B_STRING_LEN, 0x6, 83, 116, 114, 117, 99, 116, B_BEGIN, B_STRING_LEN, 0x1, 97, B_INT32, 1, 0, 0, 0, B_END, B_END];
    assert_eq!(to_binson(&s).unwrap(), expected);
}