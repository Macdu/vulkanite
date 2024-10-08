use std::{
    borrow::Cow,
    cell::{Cell, RefCell},
    collections::HashMap,
};

use anyhow::{anyhow, Result};
use proc_macro2::{Span, TokenStream};
use syn::Ident;

use crate::{
    helpers::{camel_case_to_snake_case, longuest_common_prefix, screaming_snake_to_pascal_case},
    xml,
};

pub enum Api {
    Vulkan,
    //VulkanSc
}

pub struct Enum<'a> {
    pub name: String,
    pub bitflag: bool,
    pub width_is_64: bool,
    pub values: RefCell<Vec<(&'a str, EnumValue<'a>)>>,
    pub aliases: RefCell<Vec<(&'a str, String)>>,
}

impl<'a> Enum<'a> {
    pub fn parse_name(name: &str) -> String {
        // remove the Vk prefix
        // and replace FlagBits by Flags
        // VkMemoryAllocateFlagBitsKHR -> MemoryAllocateFlagsKHR
        name[2..].replace("FlagBits", "Flags")
    }
}

impl<'a> Enum<'a> {
    pub fn try_from(value: &'a xml::Enums, ext_names: &[&str]) -> Result<Self> {
        let values = value
            .enums
            .iter()
            .map(|val| {
                Ok((
                    val.name.as_str(),
                    EnumValue::try_from(val, &value.name, ext_names)?,
                ))
            })
            .collect::<Result<_>>()?;

        let bitflag: bool = matches!(value.ty, Some(xml::EnumsType::Bitmask));

        let name = Self::parse_name(&value.name);
        Ok(Enum {
            name,
            bitflag,
            width_is_64: value.bitwidth_is_64.is_some(),
            values: RefCell::new(values),
            aliases: RefCell::new(Vec::new()),
        })
    }
}

pub enum EnumValue<'a> {
    Aliased(EnumAliased<'a>),
    Variant(EnumVariant<'a>),
    Flag(EnumFlag),
}

impl<'a> EnumValue<'a> {
    pub fn try_from(
        value: &'a xml::Enum,
        container_name: &'a str,
        ext_names: &[&str],
    ) -> Result<Self> {
        match &value {
            xml::Enum {
                name,
                alias: Some(alias),
                ..
            } => {
                let name = convert_field_to_snake_case(&container_name, name, ext_names)?;
                Ok(EnumValue::Aliased(EnumAliased { name, alias }))
            }
            xml::Enum {
                name,
                value: Some(value),
                ..
            } => {
                let name = convert_field_to_snake_case(&container_name, name, ext_names)?;
                Ok(EnumValue::Variant(EnumVariant {
                    name,
                    value: Cow::Borrowed(value),
                }))
            }
            xml::Enum {
                name,
                bitpos: Some(bitpos),
                ..
            } => {
                let name = convert_field_to_snake_case(&container_name, name, ext_names)?;
                Ok(EnumValue::Flag(EnumFlag {
                    name,
                    bitpos: *bitpos,
                }))
            }
            _ => Err(anyhow!("XML enum {:?} is ill-formed for an enum", value)),
        }
    }
}

pub struct EnumAliased<'a> {
    pub name: String,
    pub alias: &'a str,
}

pub struct EnumVariant<'a> {
    pub name: String,
    pub value: Cow<'a, str>,
}

pub struct EnumFlag {
    pub name: String,
    pub bitpos: u8,
}

pub enum Constant<'a> {
    Aliased {
        name: String,
        alias: &'a str,
    },
    Field {
        name: String,
        ty: CType,
        value: &'a str,
    },
}

impl<'a> TryFrom<&'a xml::Enum> for Constant<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Enum) -> Result<Self, Self::Error> {
        match &value {
            xml::Enum {
                name,
                alias: Some(alias),
                ..
            } => {
                // same as below
                let name = name[("VK_".len())..].to_owned();
                Ok(Constant::Aliased { name, alias })
            }
            xml::Enum {
                name,
                ty: Some(ty),
                value: Some(value),
                ..
            } => {
                // this is a real constant, use the appropriate rust case
                let name = name[("VK_".len())..].to_owned();
                let ty: CType = (ty as &str).try_into()?;
                Ok(Constant::Field { name, ty, value })
            }
            _ => Err(anyhow!("XML enum {:?} is ill-formed for a constant", value)),
        }
    }
}

pub struct Handle<'a> {
    pub name: &'a str,
    pub is_dispatchable: bool,
    #[allow(dead_code)]
    pub parent: Option<&'a str>,
    pub object_type: &'a str,
    pub aliases: RefCell<Vec<&'a str>>,
}

impl<'a> TryFrom<&'a xml::Type> for Handle<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Type) -> Result<Self> {
        match &value {
            xml::Type {
                category: Some(cat),
                parent,
                alias: None,
                content,
                object_type_enum: Some(object_type),
                ..
            } if cat == "handle" => {
                match &content[..] {
                    // content should be
                    // <type>VK_DEFINE_HANDLE</type>(<name>VkInstance</name>)
                    // or
                    // <type>VK_DEFINE_NON_DISPATCHABLE_HANDLE</type>(<name>VkBuffer</name>)
                    [xml::TypeContent::Type(ty), xml::TypeContent::Text(_), xml::TypeContent::Name(name), xml::TypeContent::Text(_)] =>
                    {
                        let is_dispatchable = match ty.as_str() {
                            "VK_DEFINE_HANDLE" => true,
                            "VK_DEFINE_NON_DISPATCHABLE_HANDLE" => false,
                            _ => return Err(anyhow!("Unknown type {ty} for {:?}", value)),
                        };
                        // remove the Vk Prefix
                        let name = &name[2..];
                        Ok(Handle {
                            name,
                            is_dispatchable,
                            object_type: object_type.as_str(),
                            parent: parent.as_ref().map(|p| p.as_str()),
                            aliases: RefCell::new(Vec::new()),
                        })
                    }
                    _ => Err(anyhow!(
                        "Failed to parse content as handle from {:?}",
                        value
                    )),
                }
            }
            _ => Err(anyhow!("Failed to extract handle from {:?}", value)),
        }
    }
}

pub enum Struct<'a> {
    BaseType(StructBasetype<'a>),
    Standard(StructStandard<'a>),
}

impl<'a> Struct<'a> {
    pub fn lifetime(&self) -> Option<bool> {
        match self {
            Struct::BaseType(StructBasetype { has_lifetime, .. })
            | Struct::Standard(StructStandard { has_lifetime, .. }) => has_lifetime.get(),
        }
    }
}

impl<'a> TryFrom<&'a xml::Type> for Struct<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Type) -> Result<Self> {
        let content = value
            .content
            .iter()
            .filter(|cnt| !matches!(cnt, xml::TypeContent::Comment(_)))
            .collect::<Vec<_>>();

        let s_type = content
            .get(0)
            .map(|ty| match ty {
                xml::TypeContent::Member(xml::Member {
                    values: Some(values),
                    ..
                }) if values.starts_with("VK_STRUCTURE_TYPE_") => Some(values.as_str()),
                _ => None,
            })
            .unwrap_or(None);

        let correct_name = |mut name: &str| {
            if name.starts_with("Vk") {
                name = &name[2..];
            }
            if name.ends_with("_id") {
                format!("{}Id", &name[..(name.len() - "_id".len())])
            } else {
                name.to_string()
            }
        };
        let has_lifetime = Cell::new(None);

        match &value {
            xml::Type {
                category: Some(cat),
                ..
            } if cat == "basetype" => match &content[..] {
                [xml::TypeContent::Text(decl), xml::TypeContent::Name(name), xml::TypeContent::Text(_)] =>
                {
                    let ty = if decl == "struct" || decl.ends_with("typedef void") {
                        Type::Void
                    } else if decl.ends_with("typedef void*") {
                        Type::VoidPtr
                    } else if decl.starts_with("typedef struct") && decl.ends_with("*") {
                        Type::VoidPtr
                    } else {
                        return Err(anyhow!("Failed to parse type for basetype {:?}", value));
                    };
                    let name = correct_name(name);
                    Ok(Struct::BaseType(StructBasetype {
                        name,
                        ty,
                        has_lifetime,
                    }))
                }
                [xml::TypeContent::Text(typedef), xml::TypeContent::Type(ty), xml::TypeContent::Name(name), xml::TypeContent::Text(_)]
                    if typedef == "typedef" =>
                {
                    let name = correct_name(name);
                    Ok(Struct::BaseType(StructBasetype {
                        name,
                        ty: Type::Path(ty),
                        has_lifetime,
                    }))
                }
                [xml::TypeContent::Text(typedef), xml::TypeContent::Type(ty), xml::TypeContent::Text(star), xml::TypeContent::Name(name), xml::TypeContent::Text(_)]
                    if typedef == "typedef" && star == "*" =>
                {
                    let name = correct_name(name);
                    let ty = if ty == "void" {
                        Type::VoidPtr
                    } else {
                        Type::Ptr(ty)
                    };
                    Ok(Struct::BaseType(StructBasetype {
                        name,
                        ty,
                        has_lifetime,
                    }))
                }
                _ => Err(anyhow!(
                    "XML value {:?} is ill-formed for a basetype",
                    value
                )),
            },
            xml::Type {
                category: Some(cat),
                name_attr: Some(name),
                ..
            } if cat == "struct" || cat == "union" => {
                let members = content
                    .iter()
                    .filter_map(|cnt| match cnt {
                        xml::TypeContent::Member(mem) => {
                            if mem.api == Some(xml::Api::Vulkansc) {
                                None
                            } else {
                                Some(StructField::try_from(mem, &name))
                            }
                        }
                        _ => Some(Err(anyhow!("Unexpected content in {:?}", value))),
                    })
                    .collect::<Result<Vec<_>>>()?;

                // don't use the usual stype / pnext tools on these structures
                let s_type =
                    s_type.filter(|_| name != "VkBaseInStructure" && name != "VkBaseOutStructure");

                let name = correct_name(name);
                Ok(Struct::Standard(StructStandard {
                    name,
                    is_union: cat == "union",
                    return_only: value.returned_only.is_some(),
                    s_type,
                    fields: members,
                    has_lifetime,
                    extends: &value.struct_extends,
                    aliases: RefCell::new(Vec::new()),
                }))
            }
            _ => Err(anyhow!("XML value {:?} is ill-formed for a struct", value)),
        }
    }
}
pub struct StructBasetype<'a> {
    pub name: String,
    pub ty: Type<'a>,
    pub has_lifetime: Cell<Option<bool>>,
}

pub struct StructStandard<'a> {
    pub name: String,
    pub s_type: Option<&'a str>,
    pub is_union: bool,
    pub return_only: bool,
    pub fields: Vec<StructField<'a>>,
    pub has_lifetime: Cell<Option<bool>>,
    pub extends: &'a Vec<String>,
    pub aliases: RefCell<Vec<&'a str>>,
}

pub struct StructField<'a> {
    pub name: String,
    pub vk_name: &'a str,
    pub ty: Type<'a>,
    pub advanced_ty: Cell<Option<AdvancedType<'a>>>,
    pub optional: bool,
    pub xml: &'a xml::Member,
}

impl<'a> StructField<'a> {
    fn try_from(value: &'a xml::Member, struct_name: &str) -> Result<Self> {
        use xml::MemberContent as Ty;

        // remove comments before matching
        let content = value
            .content
            .iter()
            .filter(|cnt| !matches!(cnt, Ty::Comment(_)))
            .filter(|cnt| !matches!(cnt, Ty::Text(spec) if spec.starts_with("const") || spec == "struct"))
            .collect::<Vec<_>>();

        let (ty, vk_name) = match &content[..] {
            // <type>int32_t</type>        <name>x</name>
            [Ty::Type(ty), Ty::Name(name)] => match (ty.as_str(), name.as_str()) {
                // we have some custom types for QoL improvements
                (_, "apiVersion") => (Type::Path("ApiVersion"), name),
                ("PFN_vkDebugUtilsMessengerCallbackEXT", _) => {
                    (Type::Path("DebugUtilsMessengerCallbackEXT"), name)
                }
                _ => (Type::Path(ty), name),
            },
            // <type>char</type>           <name>deviceName</name>[<enum>VK_MAX_PHYSICAL_DEVICE_NAME_SIZE</enum>]
            [Ty::Type(ty), Ty::Name(name), Ty::Text(op), Ty::Enum(size), Ty::Text(cl)]
                if op == "[" && cl == "]" =>
            {
                let ty = Type::ArrayEnum {
                    ty,
                    size: size.as_str(),
                };
                (ty, name)
            }
            // const <type>char</type>*     <name>pApplicationName</name>
            [Ty::Type(ty), Ty::Text(star), Ty::Name(name)] if star == "*" => {
                if ty == "void" {
                    (Type::VoidPtr, name)
                } else {
                    (Type::Ptr(ty), name)
                }
            }
            // const <type>char</type>* const*      <name>ppEnabledLayerNames</name>
            [Ty::Type(ch), Ty::Text(sconsts), Ty::Name(name)]
                if ch == "char" && (sconsts == "* const*" || sconsts == "* const *") =>
            {
                match (struct_name, name.as_str()) {
                    // we have some custom types for QoL improvements
                    ("VkInstanceCreateInfo", "ppEnabledExtensionNames") => {
                        (Type::Ptr("InstanceExtensionName"), name)
                    }
                    ("VkDeviceCreateInfo", "ppEnabledExtensionNames") => {
                        (Type::Ptr("DeviceExtensionName"), name)
                    }
                    _ => (Type::CStrArr, name),
                }
            }
            // <type>VkOffset3D</type>             <name>srcOffsets</name>[2]
            [Ty::Type(ty), Ty::Name(name), Ty::Text(size)]
                if size.starts_with('[') && size.ends_with(']') =>
            {
                if let Some(pos) = size.find("][") {
                    let size1 = size[1..pos].parse()?;
                    let size2 = size[(pos + 2)..(size.len() - 1)].parse()?;
                    (
                        Type::ArrayDoubleCst {
                            ty: &ty,
                            size1,
                            size2,
                        },
                        name,
                    )
                } else {
                    let size = size[1..(size.len() - 1)].parse()?;
                    (Type::ArrayCst { ty: &ty, size }, name)
                }
            }
            // const <type>uint32_t</type>* const*             <name>ppMaxPrimitiveCounts</name>
            [Ty::Type(ty), Ty::Text(sconsts), Ty::Name(name)]
                if sconsts == "* const*" || sconsts == "* const *" =>
            {
                (Type::DoublePtr(ty), name)
            }
            // <type>uint32_t</type>                                                <name>mask</name>:8
            [Ty::Type(ty), Ty::Name(name), Ty::Text(bit)] if bit.starts_with(':') => {
                let bitsize = bit[1..].parse()?;
                (Type::Bitfield { ty: &ty, bitsize }, name)
            }
            _ => return Err(anyhow!("Failed to parse member {:?}", value)),
        };
        let name = camel_case_to_snake_case(vk_name);
        let name = match name.as_str() {
            "type" => "ty",
            // VkPipelineExecutableStatisticValueKHR has nice type names
            "b32" => "bool32",
            "i64" => "int64",
            "u64" => "uint64",
            "f64" => "float64",
            _ => &name,
        }
        .to_string();
        let optional = value.optional.contains(&true) || value.no_auto_validity.is_some();
        Ok(StructField {
            name,
            vk_name,
            ty,
            advanced_ty: Cell::new(None),
            optional,
            xml: value,
        })
    }
}

pub struct Command<'a> {
    pub vk_name: &'a str,
    pub name: String,
    pub return_ty: ReturnType<'a>,
    pub params: Vec<CommandParam<'a>>,
    pub handle: Cell<Option<&'a str>>,
    pub aliases: RefCell<Vec<(&'a str, String)>>,
}

impl<'a> TryFrom<&'a xml::Command> for Command<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Command) -> Result<Self> {
        let proto = value
            .proto
            .as_ref()
            .ok_or_else(|| anyhow!("Command {value:?} does not have a proto"))?;
        let return_ty = match proto.ty.as_str() {
            "void" => ReturnType::Void,
            "VkResult" => {
                let nb_errors = value.error_codes.len() as u32;
                let has_incomplete = value.success_codes.contains(&String::from("VK_INCOMPLETE"));
                let nb_successes = value.success_codes.len() as u32 - (has_incomplete as u32);
                ReturnType::Result {
                    nb_successes,
                    nb_errors,
                    has_incomplete,
                }
            }
            _ => ReturnType::BaseType(&proto.ty),
        };
        assert!(proto.name.starts_with("vk"));
        let name = camel_case_to_snake_case(&proto.name[("vk".len())..]);

        let params = value
            .param
            .iter()
            .filter(|param| param.api != Some(xml::Api::Vulkansc))
            .map(|param| CommandParam::try_from(param))
            .collect::<Result<_>>()?;

        Ok(Command {
            vk_name: &proto.name,
            name,
            return_ty,
            params,
            handle: Cell::new(None),
            aliases: RefCell::new(Vec::new()),
        })
    }
}

#[derive(PartialEq, Eq)]
pub enum ReturnType<'a> {
    Void,
    Result {
        nb_successes: u32,
        nb_errors: u32,
        has_incomplete: bool,
    },
    BaseType(&'a str),
}

pub struct CommandParam<'a> {
    pub name: String,
    pub vk_name: &'a str,
    pub ty: Type<'a>,
    pub advanced_ty: Cell<Option<AdvancedType<'a>>>,
    pub optional: bool,
    pub is_const: bool,
    pub xml: &'a xml::Param,
}

impl<'a> TryFrom<&'a xml::Param> for CommandParam<'a> {
    type Error = anyhow::Error;

    fn try_from(value: &'a xml::Param) -> Result<Self> {
        use xml::ParamContent as Ty;

        let const_check =
            |cnt: &xml::ParamContent| matches!(cnt, Ty::Text(modif) if modif.starts_with("const"));

        let is_const = value.content.first().is_some_and(const_check);
        let content: Vec<_> = value
            .content
            .iter()
            .filter(|cnt| !const_check(cnt))
            .filter(|cnt| !matches!(cnt, Ty::Text(t_struct) if t_struct == "struct"))
            .collect();
        let optional = value.optional.as_slice() == &[true];
        let advanced_ty = Cell::new(None);

        let (ty, vk_name): (Type, &'a str) = match content.as_slice() {
            // void* pData
            [Ty::Type(void), Ty::Text(star), Ty::Name(name)] if void == "void" && star == "*" => {
                (Type::VoidPtr, &name)
            }
            // VkInstanceCreateInfo* pCreateInfo
            [Ty::Type(ty), Ty::Text(star), Ty::Name(name)] if star == "*" => {
                (Type::Ptr(&ty), &name)
            }
            // VkInstance instance
            [Ty::Type(ty), Ty::Name(name)] => (Type::Path(ty), &name),
            // void** ppData
            [Ty::Type(ty), Ty::Text(starstar), Ty::Name(name)]
                if starstar == "**" || starstar == "* const*" =>
            {
                (Type::DoublePtr(ty), &name)
            }
            // float blendConstants[4]
            [Ty::Type(ty), Ty::Name(name), Ty::Text(array)] if array.starts_with('[') => {
                assert!(array.ends_with(']'));
                let size: u16 = array[1..(array.len() - 1)].parse()?;
                (Type::ArrayCst { ty, size }, &name)
            }

            _ => return Err(anyhow!("Unexpected command content {content:?}")),
        };
        let mut name = camel_case_to_snake_case(vk_name);
        if name == "type" {
            name = "ty".to_string();
        }
        Ok(CommandParam {
            name,
            vk_name,
            ty,
            advanced_ty,
            optional,
            is_const,
            xml: &value,
        })
    }
}

#[derive(Clone, Copy)]
pub enum AdvancedType<'a> {
    Void,
    VoidPtr,
    Bool32,
    Handle(&'a str),
    HandlePtr(&'a str),
    HandleArray(&'a str, &'a str),
    Struct(&'a str),
    Enum(&'a str),
    Func(&'a str),
    Other(&'a str),
    OtherPtr(&'a str),
    OtherDoublePtr(&'a str),
    OtherArrayWithEnum(&'a str, &'a str),
    OtherArrayWithCst(&'a str, u16),
    OtherDoubleArray(&'a str, u8, u8),
    CharArray(&'a str),
    CString,
    CStringPtr,
    Bitfield(&'a str, u8),
}

pub enum Type<'a> {
    // void
    Void,
    // void*
    VoidPtr,
    // VkOffset3D
    Path(&'a str),
    // VkOffset3D*
    Ptr(&'a str),
    // uint32_t* const*
    DoublePtr(&'a str),
    // const char* const*
    CStrArr,
    // char[VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]
    ArrayEnum { ty: &'a str, size: &'a str },
    // VkExtent[2]
    ArrayCst { ty: &'a str, size: u16 },
    // float[3][4]
    ArrayDoubleCst { ty: &'a str, size1: u8, size2: u8 },
    // uint32_t:8
    Bitfield { ty: &'a str, bitsize: u8 },
}

#[derive(Clone, Copy)]
pub enum CType {
    Uint32,
    Int32,
    Uint64,
    Int64,
    Float,
}

impl TryFrom<&str> for CType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "uint32_t" => Ok(CType::Uint32),
            "int32_t" => Ok(CType::Int32),
            "uint64_t" => Ok(CType::Uint64),
            "int64_t" => Ok(CType::Int64),
            "float" => Ok(CType::Float),
            _ => Err(anyhow!("Unknown c type {value}")),
        }
    }
}

impl Into<Ident> for &CType {
    fn into(self) -> Ident {
        let ident_str = match self {
            CType::Uint32 => "u32",
            CType::Int32 => "i32",
            CType::Uint64 => "u64",
            CType::Int64 => "i64",
            CType::Float => "f32",
        };
        Ident::new(ident_str, Span::call_site())
    }
}

#[derive(Clone)]
pub struct MappingEntry<'a> {
    pub name: String,
    pub ty: MappingType<'a>,
}

#[derive(Clone, Copy)]
pub enum MappingType<'a> {
    Constant,
    Enum,
    AliasedEnum(&'a str),
    EnumValue,
    Handle,
    HandleAlias(&'a str),
    BaseType,
    Struct,
    AliasedStruct(&'a str),
    Command,
    CommandAlias(#[allow(dead_code)] &'a str),
    FunctionPtr,
}

pub struct CommandParamsParsed<'a, 'b> {
    pub handle: &'a str,
    pub output_length: Option<&'b CommandParam<'a>>,
    pub output_fields: Vec<(&'b str, &'b CommandParam<'a>)>,
    pub simple_fields: Vec<(&'b str, &'b CommandParam<'a>)>,
    pub vec_fields: Vec<(&'b str, &'b CommandParam<'a>)>,
    pub length_mappings: HashMap<&'b str, &'b CommandParam<'a>>,
    pub parsed_arg_templates: Vec<TokenStream>,
    pub parsed_args_in: Vec<(TokenStream, TokenStream)>,
    pub command: &'b Command<'a>,
}

pub struct SliceType {
    /// V: Alias<raw::Fence>
    pub template_param: Option<TokenStream>,
    /// p_offset: impl AsSlice<'a, V>
    pub input_ty: TokenStream,
    /// self.p_offset = p_offset.as_slice().as_ptr()
    pub affectation: TokenStream,
    /// &'b [DeviceSize]
    pub output_ty: TokenStream,
    /// unsafe { slices::from_raw_parts(..) }
    pub access: TokenStream,
}

/// Performs screaming snake case to pascal case conversion
/// We only keep the part of the field not already in the container:
/// VK_PRIMITIVE_TOPOLOGY_POINT_LIST => PointList
///
/// If the container name has an extension name, it is stripped of the field name:
/// VK_BLEND_OVERLAP_UNCORRELATED_EXT => Uncorrelated (container name is VkBlendOverlapEXT)
/// VK_PIPELINE_BIND_POINT_RAY_TRACING_KHR => RayTracingKHR (KHR is kept because the container name VkPipelineBindPoint has no KHR)
///
/// For bitflags, the _BIT part is removed
/// VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT => SampledImage
///
/// Because rust enums can't start with a number, it will be removed and instead some prefix based
/// on the container name will be added:
/// VK_SHADING_RATE_PALETTE_ENTRY_16_INVOCATIONS_PER_PIXEL_NV => Rate16InvocationsPerPixel
/// VK_SAMPLE_COUNT_4_BIT => Count4
pub fn convert_field_to_snake_case(
    container_name: &str,
    field_name: &str,
    ext_names: &[&str],
) -> Result<String> {
    let post_extension = ext_names
        .iter()
        .map(|ext| *ext)
        .find(|ext| container_name.ends_with(ext))
        .unwrap_or_default();
    let field_name = if !post_extension.is_empty() && field_name.ends_with(post_extension) {
        // No extension has one letter
        // also removing the underscore before the extension
        &field_name[..(field_name.len() - post_extension.len() - 1)]
    } else {
        field_name
    };
    let result = screaming_snake_to_pascal_case(field_name);

    let container_simplified = if let Some(pos) = container_name.find("FlagBits") {
        &container_name[..pos]
    } else if !post_extension.is_empty() {
        &container_name[..(container_name.len() - post_extension.len())]
    } else {
        &container_name[..]
    };
    let prefix = longuest_common_prefix(container_simplified, &result);

    // remove the prefix
    let mut result = result[prefix.len()..].to_string();
    let result_extension = ext_names
        .iter()
        .map(|ext| *ext)
        .find(|ext| {
            result
                .to_ascii_lowercase()
                .ends_with(&ext.to_ascii_lowercase())
        })
        .unwrap_or_default();
    if !result_extension.is_empty() && result_extension != post_extension {
        // keep the extension as uppercase
        result = format!(
            "{}{}",
            &result[..(result.len() - result_extension.len())],
            result_extension
        );
    }
    if let Some(pos) = container_name.find("FlagBits") {
        // remove the 'Bit' part
        if let Some(pos) = result.rfind("Bit") {
            result.replace_range(pos..(pos + "Bit".len()), "");
        }

        // the enum number is a bit all over the place (VkBufferUsageFlagBits2KHR -> VK_BUFFER_USAGE_2_TRANSFER_DST_BIT_KHR)
        // handle it here
        let nb_between = container_name.len() - (pos + "FlagBits".len() + post_extension.len());
        if nb_between == 1
            && container_name
                .chars()
                .nth_back(post_extension.len())
                .unwrap()
                .is_numeric()
        {
            //go from 2TransferDst to TransferDst
            result = result[1..].to_string();
        } else if nb_between >= 2 {
            return Err(anyhow!(
                "Unexpected name for {field_name} and {container_name}"
            ));
        }
    }

    let prefix = if result
        .chars()
        .next()
        .ok_or_else(|| anyhow!("The resulting field from {field_name} is an empty string"))?
        .is_numeric()
    {
        // we can't let it start with a number
        // add something relevant in front
        [
            "Type", "Count", "Depth", "Size", "Rate", "Format", "Result", "Controls", "Chroma",
            "Image",
        ]
        .into_iter()
        .find(|kw| container_name.contains(kw))
        .ok_or_else(|| {
            anyhow!(
                "Failed to find a relevant keyword to add in front of {result} for {field_name}"
            )
        })?
    } else {
        ""
    };

    Ok(format!("{prefix}{result}"))
}
