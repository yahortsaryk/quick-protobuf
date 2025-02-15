use std::borrow::Cow;

use quick_protobuf::*;

use super::basic::*;
use crate::rust_protobuf::hex::{decode_hex, encode_hex};

#[test]
fn test1() {
    let mut test1 = Test1::default();
    test1.a = 150;
    test_serialize_deserialize!("08 96 01", &test1, Test1);
}

#[test]
fn test2() {
    let mut test2 = Test2::default();
    test2.b = "testing".into();
    test_serialize_deserialize!("12 07 74 65 73 74 69 6e 67", &test2, Test2);
}

#[test]
fn test3() {
    let mut test1 = Test1::default();
    test1.a = 150;
    let mut test3 = Test3::default();
    test3.c = test1;
    test_serialize_deserialize!("1a 03 08 96 01", &test3, Test3);
}

#[test]
fn test4() {
    let mut test4 = Test4::default();
    test4.d = vec![3i32, 270, 86942];
    test_serialize_deserialize!("22 06 03 8E 02 9E A7 05", &test4, Test4);
}

#[test]
fn test_read_unpacked_expect_packed() {
    let mut test_packed_unpacked = TestPackedUnpacked::default();
    test_packed_unpacked.packed = Vec::new();
    test_packed_unpacked.unpacked = vec![17i32, 1000];
    test_deserialize!("20 11 20 e8 07", &test_packed_unpacked, TestPackedUnpacked);
}

#[test]
fn test_read_packed_expect_unpacked() {
    let mut test_packed_unpacked = TestPackedUnpacked::default();
    test_packed_unpacked.packed = vec![17i32, 1000];
    test_packed_unpacked.unpacked = Vec::new();
    test_deserialize!("2a 03 11 e8 07", &test_packed_unpacked, TestPackedUnpacked);
    test_serialize_deserialize_length_delimited!(&test_packed_unpacked, TestPackedUnpacked);
}

#[test]
fn test_empty() {
    test_serialize_deserialize!("", &TestEmpty::default(), TestEmpty);
}

#[test]
#[should_panic]
fn test_read_junk() {
    let bytes = decode_hex("00");
    let mut reader = BytesReader::from_bytes(&bytes);
    Test1::from_reader(&mut reader, &bytes).unwrap();
}

// #[test]
// fn test_unknown_fields_length_delimited() {
//     let mut message = TestUnknownFields::new();
//     message.set_a(150);
//     message.mut_unknown_fields().add_length_delimited(4, [0x10u8, 0x20, 0x30].to_vec());
//     test_serialize_deserialize("08 96 01 22 03 10 20 30", &message);
// }

// #[test]
// fn test_unknown_fields_fixed32() {
//     let mut message = TestUnknownFields::new();
//     message.set_a(150);
//     message.mut_unknown_fields().add_fixed32(4, 0x01020304);
//     message.mut_unknown_fields().add_fixed32(4, 0xA1A2A3A4);
//     test_serialize_deserialize("08 96 01 25 04 03 02 01 25 A4 A3 A2 A1", &message);
// }

#[test]
fn test_types_singular() {
    let mut message = TestTypesSingular::default();
    message.double_field = 19f64;
    message.float_field = 20f32;
    message.int32_field = 21;
    message.int64_field = -22;
    message.uint32_field = 23;
    message.uint64_field = 24;
    message.sint32_field = -25;
    message.sint64_field = 26;
    message.fixed32_field = 27;
    message.fixed64_field = 28;
    message.sfixed32_field = -29;
    message.sfixed64_field = 30;
    message.bool_field = true;
    message.string_field = "thirty two".into();
    message.bytes_field = vec![33u8, 34].into();
    message.enum_field = TestEnumDescriptor::BLUE;
    test_serialize_deserialize_length_delimited!(&message, TestTypesSingular);
}

#[test]
fn test_types_repeated() {
    let mut message = TestTypesRepeated::default();
    message.double_field = vec![19f64, 20f64];
    message.float_field = vec![20f32];
    message.int32_field = vec![21i32, -22, 23];
    message.int64_field = vec![22i64];
    message.uint32_field = vec![23u32, 24];
    message.uint64_field = vec![24u64];
    message.sint32_field = vec![25i32];
    message.sint64_field = vec![26i64, -27];
    message.fixed32_field = vec![27u32];
    message.fixed64_field = vec![28u64];
    message.sfixed32_field = vec![29i32, -30];
    message.sfixed64_field = vec![30i64];
    message.bool_field = vec![true, true];
    message.string_field = vec!["thirty two".into(), "thirty three".into()];
    message.bytes_field = vec![vec![33u8, 34].into(), vec![35u8].into()];
    message.enum_field = vec![TestEnumDescriptor::BLUE, TestEnumDescriptor::GREEN];
    test_serialize_deserialize_length_delimited!(&message, TestTypesRepeated);
}

#[test]
fn test_types_repeated_packed() {
    let mut message = TestTypesRepeatedPacked::default();
    message.double_field = vec![19f64, 20f64].into();
    message.float_field = vec![20f32].into();
    message.int32_field = vec![21i32, -22, 23];
    message.int64_field = vec![22i64];
    message.uint32_field = vec![23u32, 24];
    message.uint64_field = vec![24u64];
    message.sint32_field = vec![25i32];
    message.sint64_field = vec![26i64, -27];
    message.fixed32_field = vec![27u32].into();
    message.fixed64_field = vec![28u64].into();
    message.sfixed32_field = vec![29i32, -30].into();
    message.sfixed64_field = vec![30i64].into();
    message.bool_field = vec![true, true];
    message.string_field = vec!["thirty two".into(), "thirty three".into()];
    message.bytes_field = vec![vec![33u8, 34].into(), vec![35u8].into()];
    message.enum_field = vec![TestEnumDescriptor::BLUE, TestEnumDescriptor::GREEN];
    test_serialize_deserialize_packed_fixed!(&message, TestTypesRepeatedPacked, parsed, {
        parsed.double_field.own();
        parsed.float_field.own();
        parsed.fixed32_field.own();
        parsed.fixed64_field.own();
        parsed.sfixed32_field.own();
        parsed.sfixed64_field.own();
    });
}

// #[test]
// fn test_file_descriptor_proto() {
//     let p: &'static descriptor::FileDescriptorProto = file_descriptor_proto();
//     assert!(p.has_name());
//     assert_eq!("test_basic_pb.proto", p.get_name());
// }

#[test]
fn test_default_instance() {
    let d = TestDefaultInstance::default();
    assert_eq!("", d.field.s);
}

// #[test]
// fn test_message_descriptor() {
//     assert_eq!("TestDescriptor", TestDescriptor::new().descriptor().name());
//
//     let d = reflect::MessageDescriptor::for_type::<TestDescriptor>();
//     assert_eq!("TestDescriptor", d.name());
//     assert_eq!("basic.TestDescriptor", d.full_name());
//
//     let mut t = TestDescriptor::new();
//     t.set_stuff(55);
//
//     let field = d.field_by_name("stuff");
//     assert_eq!(55, field.get_i32(&t));
// }
//
// #[test]
// fn test_enum_descriptor() {
//     let d = TestEnumDescriptor::RED.enum_descriptor();
//     assert_eq!("TestEnumDescriptor", d.name());
//     assert_eq!("TestEnumDescriptor", \
//                 reflect::EnumDescriptor::for_type::<TestEnumDescriptor>().name());
//     assert_eq!("GREEN", d.value_by_name("GREEN").name());
// }

// #[test]
// fn test_invalid_tag() {
//     // 01 is invalid tag, because field number for that tag would be 0
//     let bytes = decode_hex("01 02 03");
//     let mut reader = BytesReader::from_bytes(&bytes);
//     assert!(TestInvalidTag::from_reader(&mut reader, &bytes).is_err());
// }

#[test]
fn test_truncated_no_varint() {
    // 08 is valid tag that should be followed by varint
    let bytes = decode_hex("08");
    let mut reader = BytesReader::from_bytes(&bytes);
    assert!(TestTruncated::from_reader(&mut reader, &bytes).is_err());
}

#[test]
fn test_truncated_middle_of_varint() {
    // 08 is field 1, wire type varint
    // 96 is non-final byte of varint
    let bytes = decode_hex("08 96");
    let mut reader = BytesReader::from_bytes(&bytes);
    assert!(TestTruncated::from_reader(&mut reader, &bytes).is_err());
}

#[test]
fn test_truncated_middle_of_length_delimited() {
    // 0a is field 1, wire type length delimited
    // 03 is length 3
    let bytes = decode_hex("0a 03 10");
    let mut reader = BytesReader::from_bytes(&bytes);
    assert!(TestTruncated::from_reader(&mut reader, &bytes).is_err());
}

#[test]
fn test_truncated_repeated_packed() {
    // 12 is field 2, wire type length delimited
    // 04 is length 4
    let bytes = decode_hex("12 04 10 20");
    let mut reader = BytesReader::from_bytes(&bytes);
    assert!(TestTruncated::from_reader(&mut reader, &bytes).is_err());
}

#[test]
fn test_bug_sint() {
    {
        let mut x = TestBugSint::default();
        x.s32 = -1;
        test_serialize_deserialize!("08 01", &x, TestBugSint);
    }
    {
        let mut x = TestBugSint::default();
        x.s64 = -2;
        test_serialize_deserialize!("10 03", &x, TestBugSint);
    }
}

#[test]
fn test_defaults() {
    let a = TestTypesSingular::default();
    assert_eq!(
        a,
        TestTypesSingular {
            double_field: 0f64,
            float_field: 0f32,
            int32_field: 0i32,
            int64_field: 0i64,
            uint32_field: 0u32,
            uint64_field: 0u64,
            sint32_field: 0i32,
            sint64_field: 0i64,
            fixed32_field: 0u32,
            fixed64_field: 0u64,
            sfixed32_field: 0i32,
            sfixed64_field: 0i64,
            bool_field: false,
            string_field: Cow::Borrowed(""),
            bytes_field: Cow::Borrowed(b""),
            enum_field: TestEnumDescriptor::UNKNOWN,
        }
    );

    let b = TestTypesSingularOptional::default();
    assert_eq!(
        b,
        TestTypesSingularOptional {
            double_field: None,
            float_field: None,
            int32_field: None,
            int64_field: None,
            uint32_field: None,
            uint64_field: None,
            sint32_field: None,
            sint64_field: None,
            fixed32_field: None,
            fixed64_field: None,
            sfixed32_field: None,
            sfixed64_field: None,
            bool_field: None,
            string_field: None,
            bytes_field: None,
            enum_field: None,
        }
    );
    
    assert_eq!(a.double_field, b.double_field.unwrap_or_default());
    assert_eq!(a.float_field, b.float_field.unwrap_or_default());
    assert_eq!(a.int32_field, b.int32_field.unwrap_or_default());
    assert_eq!(a.int64_field, b.int64_field.unwrap_or_default());
    assert_eq!(a.uint32_field, b.uint32_field.unwrap_or_default());
    assert_eq!(a.uint64_field, b.uint64_field.unwrap_or_default());
    assert_eq!(a.sint32_field, b.sint32_field.unwrap_or_default());
    assert_eq!(a.sint64_field, b.sint64_field.unwrap_or_default());
    assert_eq!(a.fixed32_field, b.fixed32_field.unwrap_or_default());
    assert_eq!(a.fixed64_field, b.fixed64_field.unwrap_or_default());
    assert_eq!(a.sfixed32_field, b.sfixed32_field.unwrap_or_default());
    assert_eq!(a.sfixed64_field, b.sfixed64_field.unwrap_or_default());
    assert_eq!(a.bool_field, b.bool_field.unwrap_or_default());
    assert_eq!(a.string_field, b.string_field.unwrap_or_default());
    assert_eq!(a.bytes_field, b.bytes_field.unwrap_or_default());
    assert_eq!(a.enum_field, b.enum_field.unwrap_or_default());
}
