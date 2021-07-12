// This file is generated by rust-protobuf 2.24.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `google/bigtable/admin/cluster/v1/bigtable_cluster_data.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_24_1;

#[derive(PartialEq,Clone,Default)]
pub struct Zone {
    // message fields
    pub name: ::std::string::String,
    pub display_name: ::std::string::String,
    pub status: Zone_Status,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Zone {
    fn default() -> &'a Zone {
        <Zone as ::protobuf::Message>::default_instance()
    }
}

impl Zone {
    pub fn new() -> Zone {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string display_name = 2;


    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    pub fn clear_display_name(&mut self) {
        self.display_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_display_name(&mut self, v: ::std::string::String) {
        self.display_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display_name(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // Take field
    pub fn take_display_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display_name, ::std::string::String::new())
    }

    // .google.bigtable.admin.cluster.v1.Zone.Status status = 3;


    pub fn get_status(&self) -> Zone_Status {
        self.status
    }
    pub fn clear_status(&mut self) {
        self.status = Zone_Status::UNKNOWN;
    }

    // Param is passed by value, moved
    pub fn set_status(&mut self, v: Zone_Status) {
        self.status = v;
    }
}

impl ::protobuf::Message for Zone {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display_name)?;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.status, 3, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.display_name);
        }
        if self.status != Zone_Status::UNKNOWN {
            my_size += ::protobuf::rt::enum_size(3, self.status);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.display_name.is_empty() {
            os.write_string(2, &self.display_name)?;
        }
        if self.status != Zone_Status::UNKNOWN {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.status))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Zone {
        Zone::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Zone| { &m.name },
                |m: &mut Zone| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "display_name",
                |m: &Zone| { &m.display_name },
                |m: &mut Zone| { &mut m.display_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Zone_Status>>(
                "status",
                |m: &Zone| { &m.status },
                |m: &mut Zone| { &mut m.status },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Zone>(
                "Zone",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Zone {
        static instance: ::protobuf::rt::LazyV2<Zone> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Zone::new)
    }
}

impl ::protobuf::Clear for Zone {
    fn clear(&mut self) {
        self.name.clear();
        self.display_name.clear();
        self.status = Zone_Status::UNKNOWN;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Zone {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Zone {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Zone_Status {
    UNKNOWN = 0,
    OK = 1,
    PLANNED_MAINTENANCE = 2,
    EMERGENCY_MAINENANCE = 3,
}

impl ::protobuf::ProtobufEnum for Zone_Status {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Zone_Status> {
        match value {
            0 => ::std::option::Option::Some(Zone_Status::UNKNOWN),
            1 => ::std::option::Option::Some(Zone_Status::OK),
            2 => ::std::option::Option::Some(Zone_Status::PLANNED_MAINTENANCE),
            3 => ::std::option::Option::Some(Zone_Status::EMERGENCY_MAINENANCE),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Zone_Status] = &[
            Zone_Status::UNKNOWN,
            Zone_Status::OK,
            Zone_Status::PLANNED_MAINTENANCE,
            Zone_Status::EMERGENCY_MAINENANCE,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<Zone_Status>("Zone.Status", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for Zone_Status {
}

impl ::std::default::Default for Zone_Status {
    fn default() -> Self {
        Zone_Status::UNKNOWN
    }
}

impl ::protobuf::reflect::ProtobufValue for Zone_Status {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Cluster {
    // message fields
    pub name: ::std::string::String,
    pub current_operation: ::protobuf::SingularPtrField<super::operations::Operation>,
    pub display_name: ::std::string::String,
    pub serve_nodes: i32,
    pub default_storage_type: StorageType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Cluster {
    fn default() -> &'a Cluster {
        <Cluster as ::protobuf::Message>::default_instance()
    }
}

impl Cluster {
    pub fn new() -> Cluster {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // .google.longrunning.Operation current_operation = 3;


    pub fn get_current_operation(&self) -> &super::operations::Operation {
        self.current_operation.as_ref().unwrap_or_else(|| <super::operations::Operation as ::protobuf::Message>::default_instance())
    }
    pub fn clear_current_operation(&mut self) {
        self.current_operation.clear();
    }

    pub fn has_current_operation(&self) -> bool {
        self.current_operation.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_operation(&mut self, v: super::operations::Operation) {
        self.current_operation = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_current_operation(&mut self) -> &mut super::operations::Operation {
        if self.current_operation.is_none() {
            self.current_operation.set_default();
        }
        self.current_operation.as_mut().unwrap()
    }

    // Take field
    pub fn take_current_operation(&mut self) -> super::operations::Operation {
        self.current_operation.take().unwrap_or_else(|| super::operations::Operation::new())
    }

    // string display_name = 4;


    pub fn get_display_name(&self) -> &str {
        &self.display_name
    }
    pub fn clear_display_name(&mut self) {
        self.display_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_display_name(&mut self, v: ::std::string::String) {
        self.display_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_display_name(&mut self) -> &mut ::std::string::String {
        &mut self.display_name
    }

    // Take field
    pub fn take_display_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.display_name, ::std::string::String::new())
    }

    // int32 serve_nodes = 5;


    pub fn get_serve_nodes(&self) -> i32 {
        self.serve_nodes
    }
    pub fn clear_serve_nodes(&mut self) {
        self.serve_nodes = 0;
    }

    // Param is passed by value, moved
    pub fn set_serve_nodes(&mut self, v: i32) {
        self.serve_nodes = v;
    }

    // .google.bigtable.admin.cluster.v1.StorageType default_storage_type = 8;


    pub fn get_default_storage_type(&self) -> StorageType {
        self.default_storage_type
    }
    pub fn clear_default_storage_type(&mut self) {
        self.default_storage_type = StorageType::STORAGE_UNSPECIFIED;
    }

    // Param is passed by value, moved
    pub fn set_default_storage_type(&mut self, v: StorageType) {
        self.default_storage_type = v;
    }
}

impl ::protobuf::Message for Cluster {
    fn is_initialized(&self) -> bool {
        for v in &self.current_operation {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.current_operation)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.display_name)?;
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_int32()?;
                    self.serve_nodes = tmp;
                },
                8 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.default_storage_type, 8, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if let Some(ref v) = self.current_operation.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.display_name.is_empty() {
            my_size += ::protobuf::rt::string_size(4, &self.display_name);
        }
        if self.serve_nodes != 0 {
            my_size += ::protobuf::rt::value_size(5, self.serve_nodes, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.default_storage_type != StorageType::STORAGE_UNSPECIFIED {
            my_size += ::protobuf::rt::enum_size(8, self.default_storage_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if let Some(ref v) = self.current_operation.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.display_name.is_empty() {
            os.write_string(4, &self.display_name)?;
        }
        if self.serve_nodes != 0 {
            os.write_int32(5, self.serve_nodes)?;
        }
        if self.default_storage_type != StorageType::STORAGE_UNSPECIFIED {
            os.write_enum(8, ::protobuf::ProtobufEnum::value(&self.default_storage_type))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Cluster {
        Cluster::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Cluster| { &m.name },
                |m: &mut Cluster| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::operations::Operation>>(
                "current_operation",
                |m: &Cluster| { &m.current_operation },
                |m: &mut Cluster| { &mut m.current_operation },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "display_name",
                |m: &Cluster| { &m.display_name },
                |m: &mut Cluster| { &mut m.display_name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeInt32>(
                "serve_nodes",
                |m: &Cluster| { &m.serve_nodes },
                |m: &mut Cluster| { &mut m.serve_nodes },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StorageType>>(
                "default_storage_type",
                |m: &Cluster| { &m.default_storage_type },
                |m: &mut Cluster| { &mut m.default_storage_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Cluster>(
                "Cluster",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Cluster {
        static instance: ::protobuf::rt::LazyV2<Cluster> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Cluster::new)
    }
}

impl ::protobuf::Clear for Cluster {
    fn clear(&mut self) {
        self.name.clear();
        self.current_operation.clear();
        self.display_name.clear();
        self.serve_nodes = 0;
        self.default_storage_type = StorageType::STORAGE_UNSPECIFIED;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Cluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Cluster {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StorageType {
    STORAGE_UNSPECIFIED = 0,
    STORAGE_SSD = 1,
    STORAGE_HDD = 2,
}

impl ::protobuf::ProtobufEnum for StorageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StorageType> {
        match value {
            0 => ::std::option::Option::Some(StorageType::STORAGE_UNSPECIFIED),
            1 => ::std::option::Option::Some(StorageType::STORAGE_SSD),
            2 => ::std::option::Option::Some(StorageType::STORAGE_HDD),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StorageType] = &[
            StorageType::STORAGE_UNSPECIFIED,
            StorageType::STORAGE_SSD,
            StorageType::STORAGE_HDD,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<StorageType>("StorageType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for StorageType {
}

impl ::std::default::Default for StorageType {
    fn default() -> Self {
        StorageType::STORAGE_UNSPECIFIED
    }
}

impl ::protobuf::reflect::ProtobufValue for StorageType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n<google/bigtable/admin/cluster/v1/bigtable_cluster_data.proto\x12\x20g\
    oogle.bigtable.admin.cluster.v1\x1a\x1cgoogle/api/annotations.proto\x1a#\
    google/longrunning/operations.proto\x1a\x1fgoogle/protobuf/timestamp.pro\
    to\"\xd6\x01\n\x04Zone\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\
    !\n\x0cdisplay_name\x18\x02\x20\x01(\tR\x0bdisplayName\x12E\n\x06status\
    \x18\x03\x20\x01(\x0e2-.google.bigtable.admin.cluster.v1.Zone.StatusR\
    \x06status\"P\n\x06Status\x12\x0b\n\x07UNKNOWN\x10\0\x12\x06\n\x02OK\x10\
    \x01\x12\x17\n\x13PLANNED_MAINTENANCE\x10\x02\x12\x18\n\x14EMERGENCY_MAI\
    NENANCE\x10\x03\"\x8e\x02\n\x07Cluster\x12\x12\n\x04name\x18\x01\x20\x01\
    (\tR\x04name\x12J\n\x11current_operation\x18\x03\x20\x01(\x0b2\x1d.googl\
    e.longrunning.OperationR\x10currentOperation\x12!\n\x0cdisplay_name\x18\
    \x04\x20\x01(\tR\x0bdisplayName\x12\x1f\n\x0bserve_nodes\x18\x05\x20\x01\
    (\x05R\nserveNodes\x12_\n\x14default_storage_type\x18\x08\x20\x01(\x0e2-\
    .google.bigtable.admin.cluster.v1.StorageTypeR\x12defaultStorageType*H\n\
    \x0bStorageType\x12\x17\n\x13STORAGE_UNSPECIFIED\x10\0\x12\x0f\n\x0bSTOR\
    AGE_SSD\x10\x01\x12\x0f\n\x0bSTORAGE_HDD\x10\x02B\x8b\x01\n$com.google.b\
    igtable.admin.cluster.v1B\x18BigtableClusterDataProtoP\x01ZGgoogle.golan\
    g.org/genproto/googleapis/bigtable/admin/cluster/v1;clusterJ\xfc\x18\n\
    \x06\x12\x04\x0e\0\\\x01\n\xbd\x04\n\x01\x0c\x12\x03\x0e\0\x122\xb2\x04\
    \x20Copyright\x202017\x20Google\x20Inc.\n\n\x20Licensed\x20under\x20the\
    \x20Apache\x20License,\x20Version\x202.0\x20(the\x20\"License\");\n\x20y\
    ou\x20may\x20not\x20use\x20this\x20file\x20except\x20in\x20compliance\
    \x20with\x20the\x20License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\
    \x20the\x20License\x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/li\
    censes/LICENSE-2.0\n\n\x20Unless\x20required\x20by\x20applicable\x20law\
    \x20or\x20agreed\x20to\x20in\x20writing,\x20software\n\x20distributed\
    \x20under\x20the\x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20\
    IS\"\x20BASIS,\n\x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20A\
    NY\x20KIND,\x20either\x20express\x20or\x20implied.\n\x20See\x20the\x20Li\
    cense\x20for\x20the\x20specific\x20language\x20governing\x20permissions\
    \x20and\n\x20limitations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\
    \x12\x03\x10\0)\n\t\n\x02\x03\0\x12\x03\x12\0&\n\t\n\x02\x03\x01\x12\x03\
    \x13\0-\n\t\n\x02\x03\x02\x12\x03\x14\0)\n\x08\n\x01\x08\x12\x03\x16\0^\
    \n\t\n\x02\x08\x0b\x12\x03\x16\0^\n\x08\n\x01\x08\x12\x03\x17\0\"\n\t\n\
    \x02\x08\n\x12\x03\x17\0\"\n\x08\n\x01\x08\x12\x03\x18\09\n\t\n\x02\x08\
    \x08\x12\x03\x18\09\n\x08\n\x01\x08\x12\x03\x19\0=\n\t\n\x02\x08\x01\x12\
    \x03\x19\0=\ng\n\x02\x04\0\x12\x04\x1d\06\x01\x1a[\x20A\x20physical\x20l\
    ocation\x20in\x20which\x20a\x20particular\x20project\x20can\x20allocate\
    \x20Cloud\x20BigTable\n\x20resources.\n\n\n\n\x03\x04\0\x01\x12\x03\x1d\
    \x08\x0c\n*\n\x04\x04\0\x04\0\x12\x04\x1f\x02+\x03\x1a\x1c\x20Possible\
    \x20states\x20of\x20a\x20zone.\n\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x1f\
    \x07\r\nA\n\x06\x04\0\x04\0\x02\0\x12\x03!\x04\x10\x1a2\x20The\x20state\
    \x20of\x20the\x20zone\x20is\x20unknown\x20or\x20unspecified.\n\n\x0e\n\
    \x07\x04\0\x04\0\x02\0\x01\x12\x03!\x04\x0b\n\x0e\n\x07\x04\0\x04\0\x02\
    \0\x02\x12\x03!\x0e\x0f\n-\n\x06\x04\0\x04\0\x02\x01\x12\x03$\x04\x0b\
    \x1a\x1e\x20The\x20zone\x20is\x20in\x20a\x20good\x20state.\n\n\x0e\n\x07\
    \x04\0\x04\0\x02\x01\x01\x12\x03$\x04\x06\n\x0e\n\x07\x04\0\x04\0\x02\
    \x01\x02\x12\x03$\t\n\n:\n\x06\x04\0\x04\0\x02\x02\x12\x03'\x04\x1c\x1a+\
    \x20The\x20zone\x20is\x20down\x20for\x20planned\x20maintenance.\n\n\x0e\
    \n\x07\x04\0\x04\0\x02\x02\x01\x12\x03'\x04\x17\n\x0e\n\x07\x04\0\x04\0\
    \x02\x02\x02\x12\x03'\x1a\x1b\nI\n\x06\x04\0\x04\0\x02\x03\x12\x03*\x04\
    \x1d\x1a:\x20The\x20zone\x20is\x20down\x20for\x20emergency\x20or\x20unpl\
    anned\x20maintenance.\n\n\x0e\n\x07\x04\0\x04\0\x02\x03\x01\x12\x03*\x04\
    \x18\n\x0e\n\x07\x04\0\x04\0\x02\x03\x02\x12\x03*\x1b\x1c\n{\n\x04\x04\0\
    \x02\0\x12\x03/\x02\x12\x1an\x20A\x20permanent\x20unique\x20identifier\
    \x20for\x20the\x20zone.\n\x20Values\x20are\x20of\x20the\x20form\x20proje\
    cts/<project>/zones/[a-z][-a-z0-9]*\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04/\
    \x02+\x03\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03/\x02\x08\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03/\t\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03/\x10\x11\n:\
    \n\x04\x04\0\x02\x01\x12\x032\x02\x1a\x1a-\x20The\x20name\x20of\x20this\
    \x20zone\x20as\x20it\x20appears\x20in\x20UIs.\n\n\r\n\x05\x04\0\x02\x01\
    \x04\x12\x042\x02/\x12\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x032\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x032\t\x15\n\x0c\n\x05\x04\0\x02\x01\
    \x03\x12\x032\x18\x19\n.\n\x04\x04\0\x02\x02\x12\x035\x02\x14\x1a!\x20Th\
    e\x20current\x20state\x20of\x20this\x20zone.\n\n\r\n\x05\x04\0\x02\x02\
    \x04\x12\x045\x022\x1a\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x035\x02\x08\n\
    \x0c\n\x05\x04\0\x02\x02\x01\x12\x035\t\x0f\n\x0c\n\x05\x04\0\x02\x02\
    \x03\x12\x035\x12\x13\nX\n\x02\x04\x01\x12\x049\0P\x01\x1aL\x20An\x20iso\
    lated\x20set\x20of\x20Cloud\x20BigTable\x20resources\x20on\x20which\x20t\
    ables\x20can\x20be\x20hosted.\n\n\n\n\x03\x04\x01\x01\x12\x039\x08\x0f\n\
    \xe0\x01\n\x04\x04\x01\x02\0\x12\x03>\x02\x12\x1a\xd2\x01\x20A\x20perman\
    ent\x20unique\x20identifier\x20for\x20the\x20cluster.\x20For\x20technica\
    l\x20reasons,\x20the\n\x20zone\x20in\x20which\x20the\x20cluster\x20resid\
    es\x20is\x20included\x20here.\n\x20Values\x20are\x20of\x20the\x20form\n\
    \x20projects/<project>/zones/<zone>/clusters/[a-z][-a-z0-9]*\n\n\r\n\x05\
    \x04\x01\x02\0\x04\x12\x04>\x029\x11\n\x0c\n\x05\x04\x01\x02\0\x05\x12\
    \x03>\x02\x08\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03>\t\r\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03>\x10\x11\n\xf5\x01\n\x04\x04\x01\x02\x01\x12\x03D\
    \x025\x1a\xe7\x01\x20The\x20operation\x20currently\x20running\x20on\x20t\
    he\x20cluster,\x20if\x20any.\n\x20This\x20cannot\x20be\x20set\x20directl\
    y,\x20only\x20through\x20CreateCluster,\x20UpdateCluster,\n\x20or\x20Und\
    eleteCluster.\x20Calls\x20to\x20these\x20methods\x20will\x20be\x20reject\
    ed\x20if\n\x20\"current_operation\"\x20is\x20already\x20set.\n\n\r\n\x05\
    \x04\x01\x02\x01\x04\x12\x04D\x02>\x12\n\x0c\n\x05\x04\x01\x02\x01\x06\
    \x12\x03D\x02\x1e\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03D\x1f0\n\x0c\n\
    \x05\x04\x01\x02\x01\x03\x12\x03D34\nd\n\x04\x04\x01\x02\x02\x12\x03H\
    \x02\x1a\x1aW\x20The\x20descriptive\x20name\x20for\x20this\x20cluster\
    \x20as\x20it\x20appears\x20in\x20UIs.\n\x20Must\x20be\x20unique\x20per\
    \x20zone.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\x04H\x02D5\n\x0c\n\x05\x04\
    \x01\x02\x02\x05\x12\x03H\x02\x08\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\
    \x03H\t\x15\n\x0c\n\x05\x04\x01\x02\x02\x03\x12\x03H\x18\x19\nC\n\x04\
    \x04\x01\x02\x03\x12\x03K\x02\x18\x1a6\x20The\x20number\x20of\x20serve\
    \x20nodes\x20allocated\x20to\x20this\x20cluster.\n\n\r\n\x05\x04\x01\x02\
    \x03\x04\x12\x04K\x02H\x1a\n\x0c\n\x05\x04\x01\x02\x03\x05\x12\x03K\x02\
    \x07\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03K\x08\x13\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03K\x16\x17\n\x9b\x01\n\x04\x04\x01\x02\x04\x12\
    \x03O\x02'\x1a\x8d\x01\x20What\x20storage\x20type\x20to\x20use\x20for\
    \x20tables\x20in\x20this\x20cluster.\x20Only\x20configurable\x20at\n\x20\
    cluster\x20creation\x20time.\x20If\x20unspecified,\x20STORAGE_SSD\x20wil\
    l\x20be\x20used.\n\n\r\n\x05\x04\x01\x02\x04\x04\x12\x04O\x02K\x18\n\x0c\
    \n\x05\x04\x01\x02\x04\x06\x12\x03O\x02\r\n\x0c\n\x05\x04\x01\x02\x04\
    \x01\x12\x03O\x0e\"\n\x0c\n\x05\x04\x01\x02\x04\x03\x12\x03O%&\n\n\n\x02\
    \x05\0\x12\x04R\0\\\x01\n\n\n\x03\x05\0\x01\x12\x03R\x05\x10\n4\n\x04\
    \x05\0\x02\0\x12\x03T\x02\x1a\x1a'\x20The\x20storage\x20type\x20used\x20\
    is\x20unspecified.\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03T\x02\x15\n\x0c\
    \n\x05\x05\0\x02\0\x02\x12\x03T\x18\x19\nR\n\x04\x05\0\x02\x01\x12\x03W\
    \x02\x12\x1aE\x20Data\x20will\x20be\x20stored\x20in\x20SSD,\x20providing\
    \x20low\x20and\x20consistent\x20latencies.\n\n\x0c\n\x05\x05\0\x02\x01\
    \x01\x12\x03W\x02\r\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03W\x10\x11\nZ\n\
    \x04\x05\0\x02\x02\x12\x03[\x02\x12\x1aM\x20Data\x20will\x20be\x20stored\
    \x20in\x20HDD,\x20providing\x20high\x20and\x20less\x20predictable\n\x20l\
    atencies.\n\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03[\x02\r\n\x0c\n\x05\x05\
    \0\x02\x02\x02\x12\x03[\x10\x11b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
