#![allow(dead_code)]

use std::ops::Deref;

use serde::{Deserialize, Deserializer};

use crate::{deserialize_multiple, deserialize_skip, helpers::comma_separated};

#[derive(Debug, Deserialize)]
pub struct Registry {
    #[serde(rename = "$value", deserialize_with = "deserialize_registry_content")]
    content: RegistryContent,
}

impl Deref for Registry {
    type Target = RegistryContent;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

#[derive(Debug)]
pub struct RegistryContent {
    pub platforms: Vec<Platforms>,
    pub tags: Vec<Tags>,
    pub types: Vec<Types>,
    pub enums: Vec<Enums>,
    pub commands: Vec<Commands>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extensions>,
    pub formats: Vec<Formats>,
    pub spirvextensions: Vec<Spirvextensions>,
    pub spirvcapabilities: Vec<Spirvcapabilities>,
    pub syncs: Vec<Sync>,
    pub comments: Vec<Comment>,
}

deserialize_multiple! {deserialize_registry_content, RegistryContent,
    platforms => Platforms,
    tags => Tags,
    types => Types,
    enums => Enums,
    commands => Commands,
    features => Feature,
    extensions => Extensions,
    formats => Formats,
    spirvextensions => Spirvextensions,
    spirvcapabilities => Spirvcapabilities,
    syncs => Sync,
    comments => Comment,
}

pub type Comment = String;

#[derive(Debug, Deserialize)]
pub struct Platforms {
    #[serde(rename = "@comment")]
    pub comment: String,
    pub platform: Vec<Platform>,
}

#[derive(Debug, Deserialize)]
pub struct Platform {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@protect")]
    pub protect: String,
    #[serde(rename = "@comment")]
    pub comment: String,
}

#[derive(Debug, Deserialize)]
pub struct Tags {
    #[serde(rename = "@comment")]
    pub comment: String,
    pub tag: Vec<Tag>,
}

#[derive(Debug, Deserialize)]
pub struct Tag {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@author")]
    pub author: String,
    #[serde(rename = "@contact")]
    pub contact: String,
}

#[derive(Debug, Deserialize)]
pub struct Types {
    #[serde(rename = "@comment")]
    pub comment: String,
    #[serde(rename = "$value", deserialize_with = "deserialize_types")]
    pub types: Vec<Type>,
}

deserialize_skip! {deserialize_types, Type, "comment" => Comment }

#[derive(Debug, Deserialize)]
pub struct Type {
    #[serde(rename = "@category")]
    pub category: Option<String>,
    #[serde(rename = "@name")]
    pub name_attr: Option<String>,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
    #[serde(rename = "@parent")]
    pub parent: Option<String>,
    #[serde(rename = "@objtypeenum")]
    pub object_type_enum: Option<String>,
    #[serde(rename = "@requires")]
    pub requires: Option<String>,
    #[serde(rename = "@api")]
    pub api: Option<Api>,
    #[serde(rename = "@deprecated")]
    pub deprecated: Option<Deprecated>,
    #[serde(rename = "@allowduplicate")]
    pub allow_duplicate: Option<bool>,
    #[serde(rename = "@returnedonly")]
    pub returned_only: Option<()>,
    #[serde(default, rename = "@structextends", deserialize_with = "comma_separated")]
    pub struct_extends: Vec<String>,
    #[serde(default, rename = "$value")]
    pub content: Vec<TypeContent>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TypeContent {
    Name(String),
    Enum(String),
    Type(String),
    Member(Member),
    Comment(String),
    #[serde(rename = "$text")]
    Text(String),
}

#[derive(Deserialize, Debug)]
pub struct Member {
    #[serde(rename = "@altlen")]
    pub alt_len: Option<String>,
    #[serde(rename = "@api")]
    pub api: Option<Api>,
    #[serde(rename = "@deprecated")]
    pub deprecated: Option<Deprecated>,
    #[serde(rename = "@externsync")]
    pub extern_sync: Option<()>,
    #[serde(rename = "@len")]
    pub len: Option<String>,
    #[serde(default, rename = "@limittype", deserialize_with = "comma_separated")]
    pub limit_type: Vec<LimitType>,
    #[serde(rename = "@noautovalidity")]
    pub no_auto_validity: Option<()>,
    #[serde(rename = "@objecttype")]
    pub object_type: Option<()>,
    #[serde(default, rename = "@optional", deserialize_with = "comma_separated")]
    pub optional: Vec<bool>,
    #[serde(rename = "@selection")]
    pub selection: Option<String>,
    #[serde(rename = "@selector")]
    pub selector: Option<String>,
    #[serde(rename = "@values")]
    pub values: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<MemberContent>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MemberContent {
    Name(String),
    Type(String),
    Comment(String),
    Enum(String),
    #[serde(rename = "$text")]
    Text(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Deprecated {
    True,
    Ignored,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Api {
    Vulkan,
    Vulkansc,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum LimitType {
    Bitmask,
    Bits,
    Exact,
    Max,
    Min,
    Mul,
    Noauto,
    Not,
    Pot,
    Range,
    Struct,
}

#[derive(Deserialize, Debug)]
pub struct Enums {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@bitwidth")]
    pub bitwidth_is_64: Option<()>,
    #[serde(rename = "@type")]
    pub ty: Option<EnumsType>,
    #[serde(default, rename = "$value", deserialize_with = "deserialize_enums")]
    pub enums: Vec<Enum>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EnumsType {
    Bitmask,
    Enum,
    Constants,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Enum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@api")]
    pub api: Option<Api>,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
    #[serde(rename = "@type")]
    pub ty: Option<String>,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@deprecated")]
    pub deprecated: Option<()>,
    #[serde(rename = "@bitpos")]
    pub bitpos: Option<u8>,
}

deserialize_skip! {deserialize_enums, Enum, "comment" => Comment, "unused" => Unused}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Commands {
    #[serde(rename = "@comment")]
    pub comment: String,
    pub command: Vec<Command>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Command {
    #[serde(rename = "@name")]
    pub name: Option<String>,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
    #[serde(rename = "@api")]
    pub api: Option<Api>,
    #[serde(
        default,
        rename = "@successcodes",
        deserialize_with = "comma_separated"
    )]
    pub success_codes: Vec<SuccessCode>,
    #[serde(default, rename = "@errorcodes", deserialize_with = "comma_separated")]
    pub error_codes: Vec<ErrorCode>,
    #[serde(
        default,
        rename = "@cmdbufferlevel",
        deserialize_with = "comma_separated"
    )]
    pub cmd_buffer_level: Vec<CmdBufferLevel>,
    #[serde(default, rename = "@queues", deserialize_with = "comma_separated")]
    pub queues: Vec<Queue>,
    #[serde(default, rename = "@renderpass")]
    pub renderpass: Vec<Renderpass>,
    #[serde(default, rename = "@tasks", deserialize_with = "comma_separated")]
    pub task: Vec<Task>,
    #[serde(default, rename = "@videocoding")]
    pub videocoding: Option<VideoCoding>,
    pub proto: Option<Proto>,
    #[serde(default)]
    pub param: Vec<Param>,
}

pub type SuccessCode = String;
pub type ErrorCode = String;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum CmdBufferLevel {
    Primary,
    Secondary,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Queue {
    Compute,
    Decode,
    Encode,
    Graphics,
    Opticalflow,
    SparseBinding,
    Transfer,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Renderpass {
    Both,
    Inside,
    Outside,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Task {
    Action,
    Indirection,
    State,
    Synchronization,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VideoCoding {
    Both,
    Inside,
    Outside,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Proto {
    #[serde(rename = "type")]
    pub ty: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Param {
    #[serde(rename = "@altlen")]
    pub altlen: Option<String>,
    #[serde(rename = "@api")]
    pub api: Option<Api>,
    #[serde(rename = "@externsync")]
    pub externsync: Option<()>,
    #[serde(rename = "@len")]
    pub len: Option<String>,
    #[serde(rename = "@noautovalidity")]
    pub no_auto_validity: Option<()>,
    #[serde(rename = "@objecttype")]
    pub object_type: Option<()>,
    #[serde(default, rename = "@optional", deserialize_with = "comma_separated")]
    pub optional: Vec<bool>,
    #[serde(rename = "@stride")]
    pub stride: Option<String>,
    #[serde(rename = "@validstructs")]
    pub valid_structs: Option<String>,
    #[serde(rename = "$value")]
    pub content: Vec<ParamContent>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ParamContent {
    Type(String),
    Name(String),
    #[serde(rename = "$text")]
    Text(String),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Feature {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@api", deserialize_with = "comma_separated")]
    pub api: Vec<Api>,
    #[serde(rename = "@number")]
    pub number: String,
    #[serde(rename = "@comment")]
    pub comment: String,
    pub require: Vec<Require>,
    pub remove: Option<Remove>,
}

#[derive(Deserialize, Debug)]
pub struct Require {
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(default, rename = "$value")]
    pub content: Vec<RequireContent>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RequireContent {
    Enum(RequireEnum),
    Type(RequireType),
    Command(RequireCommand),
    Comment(String),
}

#[derive(Deserialize, Debug)]
pub struct RequireEnum {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
    #[serde(rename = "@api")]
    pub api: Option<Api>,
    #[serde(rename = "@deprecated")]
    pub deprecated: Option<()>,
    #[serde(rename = "@extends")]
    pub extends: Option<String>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@bitpos")]
    pub bitpos: Option<u8>,
    #[serde(rename = "@dir")]
    pub dir: Option<()>,
    #[serde(rename = "@extnumber")]
    pub extnumber: Option<u32>,
    #[serde(rename = "@offset")]
    pub offset: Option<u32>,
    #[serde(rename = "@protect")]
    pub protect: Option<()>,
    #[serde(rename = "@type")]
    pub type_is_u32: Option<()>,
    #[serde(rename = "@value")]
    pub value: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RequireCommand {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct RequireType {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Remove {}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Extensions {
    #[serde(rename = "@comment")]
    pub comment: String,
    pub extension: Vec<Extension>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Extension {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number")]
    pub number: u32,
    #[serde(rename = "@supported", deserialize_with = "comma_separated")]
    pub supported: Vec<ExtensionSupported>,
    #[serde(rename = "@comment")]
    pub comment: Option<String>,
    #[serde(rename = "@author")]
    pub author: Option<String>,
    #[serde(rename = "@contact")]
    pub contact: Option<String>,
    #[serde(rename = "@depends")]
    pub depends: Option<String>,
    #[serde(rename = "@deprecatedby")]
    pub deprecated_by: Option<String>,
    #[serde(rename = "@obsoletedby")]
    pub obsoleted_by: Option<String>,
    #[serde(rename = "@platform")]
    pub platform: Option<String>,
    #[serde(rename = "@promotedto")]
    pub promoted_to: Option<String>,
    #[serde(rename = "@provisional")]
    pub provisional: Option<()>,
    #[serde(default, rename = "@ratified", deserialize_with = "comma_separated")]
    pub ratified: Vec<Api>,
    #[serde(rename = "@sortorder")]
    pub sort_order: Option<()>,
    #[serde(default, rename = "@specialuse", deserialize_with = "comma_separated")]
    pub special_use: Vec<SpecialUse>,
    #[serde(rename = "@type")]
    pub ty: Option<ExtensionType>,

    pub require: Vec<Require>,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionSupported {
    Disabled,
    Vulkan,
    Vulkansc,
}

#[derive(Deserialize, Debug)]
pub enum SpecialUse {
    #[serde(rename = "cadsupport")]
    CADSupport,
    #[serde(rename = "d3demulation")]
    D3DEmulation,
    #[serde(rename = "debugging")]
    Debugging,
    #[serde(rename = "devtools")]
    Devtools,
    #[serde(rename = "glemulation")]
    GLEmulation,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExtensionType {
    Device,
    Instance,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Formats {
    pub format: Vec<Format>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Format {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@blockSize")]
    pub block_size: u8,
    #[serde(rename = "@class")]
    pub class: String,
    #[serde(rename = "@texelsPerBlock")]
    pub texels_per_block: u8,
    #[serde(default, rename = "@blockExtent", deserialize_with = "comma_separated")]
    pub block_extent: Vec<u8>,
    #[serde(rename = "@chroma")]
    pub chroma: Option<Chroma>,
    #[serde(rename = "@compressed")]
    pub compressed: Option<String>,
    #[serde(rename = "@packed")]
    pub packed: Option<u8>,

    pub component: Vec<Component>,
    #[serde(default)]
    pub plane: Vec<Plane>,
    #[serde(rename = "spirvimageformat")]
    pub spirv_format: Option<SpirvImage>,
}

#[derive(Deserialize, Debug)]
pub enum Chroma {
    #[serde(rename = "420")]
    Chroma420,
    #[serde(rename = "422")]
    Chroma422,
    #[serde(rename = "444")]
    Chroma444,
}

#[derive(Deserialize, Debug)]
pub struct Component {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@bits")]
    pub bits: String,
    #[serde(rename = "@numericFormat")]
    pub numeric_format: NumericFormat,
    #[serde(rename = "@planeIndex")]
    pub plane_index: Option<PlaneIndex>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum NumericFormat {
    SFloat,
    SInt,
    SNorm,
    SRgb,
    SFixed5,
    SScaled,
    UFloat,
    UInt,
    UNorm,
    UScaled
}

#[derive(Deserialize, Debug)]
pub struct Plane {
    #[serde(rename = "@compatible")]
    pub compatible: String,
    #[serde(rename = "@index")]
    pub index: PlaneIndex,
    #[serde(rename = "@heightDivisor")]
    pub height_divisor: PlaneDivisor,
    #[serde(rename = "@widthDivisor")]
    pub width_divisor: PlaneDivisor,
}

#[derive(Deserialize, Debug)]
pub enum PlaneIndex {
    #[serde(rename = "0")]
    P0,
    #[serde(rename = "1")]
    P1,
    #[serde(rename = "2")]
    P2
}

#[derive(Deserialize, Debug)]
pub enum PlaneDivisor {
    #[serde(rename = "1")]
    D1,
    #[serde(rename = "2")]
    D2,
}

#[derive(Deserialize, Debug)]
pub struct SpirvImage {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct Spirvextensions {
    #[serde(rename = "@comment")]
    pub comment: String,
    #[serde(rename = "spirvextension")]
    pub extensions: Vec<SpirvExtension>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SpirvExtension {
    #[serde(rename = "@name")]
    pub name: String,
    pub enable: Vec<SpirvExtensionEnable>,
}

#[derive(Deserialize, Debug)]
pub struct SpirvExtensionEnable {
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@extension")]
    pub extension: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Spirvcapabilities {
    #[serde(rename = "@comment")]
    pub comment: String,
    #[serde(rename = "spirvcapability")]
    pub capabilities: Vec<SpirvCapability>,
}

#[derive(Deserialize, Debug)]
pub struct SpirvCapability {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct SpirvCapabilityEnable {
    #[serde(rename = "@version")]
    pub version: Option<String>,
    #[serde(rename = "@extension")]
    pub extension: Option<String>,
    #[serde(rename = "@property")]
    pub property: Option<String>,
    #[serde(rename = "@member")]
    pub member: Option<String>,
    #[serde(default, rename = "@requires", deserialize_with = "comma_separated")]
    pub requires: Vec<String>,
    #[serde(rename = "@value")]
    pub value: Option<String>,
    #[serde(rename = "@struct")]
    pub structure: Option<String>,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Sync {
    #[serde(rename = "@comment")]
    pub comment: String,

    #[serde(rename = "syncstage")]
    pub sync_stages: Vec<SyncStage>,
    #[serde(rename = "syncaccess")]
    pub sync_accesses: Vec<SyncAccess>,
    #[serde(rename = "syncpipeline")]
    pub sync_pipelines: Vec<SyncPipeline>,
}

#[derive(Deserialize, Debug)]
pub struct SyncStage {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
    #[serde(rename = "syncequivalent")]
    pub equivalent: Option<SyncEquivalent>,
    #[serde(rename = "syncsupport")]
    pub support: Option<SyncSupport>,
}

#[derive(Deserialize, Debug)]
pub struct SyncEquivalent {
    #[serde(rename = "@stage", deserialize_with = "comma_separated")]
    pub stage: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct SyncSupport {
    #[serde(rename = "@queues", deserialize_with = "comma_separated")]
    pub queues: Vec<Queue>,
}

#[derive(Deserialize, Debug)]
pub struct SyncAccess {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@alias")]
    pub alias: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SyncPipeline {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@depends")]
    pub depends: Option<String>,
}
