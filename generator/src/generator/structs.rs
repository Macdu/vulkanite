use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

use anyhow::{anyhow, Result};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::{
    structs::{AdvancedType, Struct, StructBasetype, StructField, StructStandard},
    xml,
};

use super::{make_doc_link, Generator};

pub fn generate<'a>(gen: &Generator<'a>) -> Result<String> {
    let listed_structs = RefCell::new(HashSet::from(
        // We use our own Bool32 type
        ["VkBool32"],
    ));

    let generate_group_struct = |require: &'a xml::Require| -> Result<TokenStream> {
        let structs = require
            .content
            .iter()
            .filter_map(|item| match item {
                xml::RequireContent::Type(xml::RequireType { name: ty_name, .. }) => gen
                    .structs
                    .get(ty_name.as_str())
                    .filter(|_| listed_structs.borrow_mut().insert(&ty_name))
                    .map(|my_struct| match my_struct {
                        Struct::BaseType(StructBasetype {
                            name,
                            ty,
                            has_lifetime,
                        }) => {
                            let advanced_type = gen.compute_advanced_type(ty);
                            let ty = gen
                                .generate_type_inner(&advanced_type, true)
                                .expect("Failed to get type");
                            let name = format_ident!("{name}");
                            let doc_tag = make_doc_link(ty_name);
                            let lifetime = has_lifetime.get().unwrap().then(|| quote! (<'a>));
                            Ok(quote! (#doc_tag pub type #name #lifetime = #ty;))
                        }
                        Struct::Standard(my_struct) => generate_struct(gen, my_struct, ty_name),
                    }),
                _ => None,
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(quote! { #(#structs)*})
    };

    let struct_features = gen
        .filtered_features()
        .flat_map(|feature| &feature.require)
        .map(|req| generate_group_struct(req))
        .collect::<Result<Vec<_>>>()?;
    let struct_extensions = gen
        .filtered_extensions()
        .flat_map(|ext| &ext.require)
        .map(|req| generate_group_struct(req))
        .collect::<Result<Vec<_>>>()?;

    let result = quote! {
        use crate::*;
        use crate::vk::*;
        use crate::vk::raw::*;
        use std::ptr;
        use std::array;
        use std::marker::PhantomData;
        use std::ffi::{c_char, c_int, c_void};
        use std::mem::ManuallyDrop;

        #(#struct_features)*
        #(#struct_extensions)*
    }
    .to_string();

    let formatted_result = Generator::format_result(result)?;
    Ok(formatted_result)
}

fn generate_struct<'a>(
    gen: &Generator<'a>,
    my_struct: &StructStandard<'a>,
    struct_vk_name: &str,
) -> Result<TokenStream> {
    let mapping = gen.mapping.borrow();
    let all_fields: HashMap<_, _> = my_struct.fields.iter().map(|f| (f.vk_name, f)).collect();
    let mut simple_fields: HashSet<_> = all_fields.keys().cloned().collect();
    let mut char_arr_fields = Vec::new();
    if my_struct.s_type.is_some() {
        // remove preemptively s_type and p_next
        simple_fields.remove("sType");
        simple_fields.remove("pNext");
    }

    struct FieldWithLen<'a, 'b> {
        len_field: &'b StructField<'a>,
        array_fields: Vec<&'b StructField<'a>>,
    }
    let mut length_fields = HashMap::new();

    // retrieve all arrays with a len
    for field in &my_struct.fields {
        let mut len = match &field.xml.len {
            Some(len) => len.as_str(),
            _ => continue,
        };

        if len == "null-terminated" {
            if let Some(AdvancedType::CharArray(size)) = field.advanced_ty.get() {
                simple_fields.remove(field.vk_name);
                char_arr_fields.push((field, size));
            }
            continue;
        }

        if len.ends_with(",null-terminated") {
            // just ask for an array of const char* for the time being
            len = &len[..(len.len() - ",null-terminated".len())]
        }

        if field.xml.alt_len.is_some() {
            // handled with specific code (only concern 5 structs so far)
            simple_fields.remove(field.vk_name);
            continue;
        }

        if len.contains(',') {
            // TODO: handle
            len = len.split(',').next().unwrap();
        }

        let len_field = *all_fields
            .get(len)
            .ok_or_else(|| anyhow!("Failed to find length field {len} for {struct_vk_name}"))?;

        simple_fields.remove(field.vk_name);
        simple_fields.remove(len_field.vk_name);

        length_fields
            .entry(len)
            .or_insert_with(|| FieldWithLen {
                len_field,
                array_fields: Vec::new(),
            })
            .array_fields
            .push(field);
    }

    for (_, field_with_len) in &length_fields {
        if field_with_len
            .array_fields
            .iter()
            .all(|field| field.optional)
        {
            // if all slice fields are optional, we must give the option to set the length alone
            // (for example descriptorCount in VkDescriptorSetLayoutBinding )
            simple_fields.insert(field_with_len.len_field.vk_name);
        }
    }

    // remove length parameter associated with altlen fields
    match struct_vk_name {
        "VkShaderModuleCreateInfo" => {
            simple_fields.remove("codeSize");
        }
        "VkPipelineMultisampleStateCreateInfo" => {
            simple_fields.remove("rasterizationSamples");
        }
        _ => (),
    };

    let iter = my_struct
        .fields
        .iter()
        .map(|field| {
            let field_name = format_ident!("{}", field.name);
            let (ty_name, vis, default_impl) = match field.vk_name {
                "sType" if my_struct.s_type.is_some() => (
                    quote!(StructureType),
                    quote! (pub(crate)),
                    quote! {#field_name: Self::STRUCTURE_TYPE},
                ),
                "pNext" if my_struct.s_type.is_some() => (
                    quote!(Cell<*const Header>),
                    quote! (pub(crate)),
                    quote! {p_next: Cell::new(ptr::null())},
                ),
                _ => {
                    let default_value = gen.generate_default(&field.advanced_ty.get().unwrap())?;
                    let mut ty_inner =
                        gen.generate_type_inner(&field.advanced_ty.get().unwrap(), true)?;
                    if my_struct.is_union && gen.compute_type_lifetime(&field.ty) {
                        ty_inner = quote! (ManuallyDrop<#ty_inner>)
                    };
                    (
                        ty_inner,
                        if my_struct.is_union || simple_fields.get(field.vk_name).is_some() {
                            quote!(pub)
                        } else {
                            quote! (pub(crate))
                        },
                        quote! (#field_name: #default_value),
                    )
                }
            };
            Ok((quote! {#vis #field_name: #ty_name,}, default_impl))
        })
        .collect::<Result<Vec<_>>>()?;

    let (fields, default_impl): (Vec<_>, Vec<_>) = iter.into_iter().unzip();

    let simple_accessors = my_struct
        .fields
        .iter()
        .filter(|field| simple_fields.contains(field.vk_name))
        .map(|field| {
            let name = format_ident!("{}", field.name);
            let fn_name = if field.name.starts_with("p_") {
                &field.name[2..]
            } else if field.name.starts_with("pp_") {
                &field.name[3..]
            } else {
                &field.name
            };
            let fn_name = format_ident!("{fn_name}");
            let ty = field.advanced_ty.get().unwrap();
            let ty_name = gen.generate_type_outer(&ty, field.optional, true)?;
            let value =
                gen.generate_type_outer_to_inner(&ty, field.optional, format_ident!("value"))?;
            Ok(quote! {
                #[inline]
                pub fn #fn_name(mut self, value: #ty_name) -> Self {
                    self.#name = #value;
                    self
                }
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let char_arr_setters = char_arr_fields.into_iter().map(|(field, size)| {
        let name = format_ident!("{}", field.name);
        let fn_name = format_ident!("get_{}", field.name);
        let size_cst = format_ident!("{}", &size["VK_".len()..]);

        quote! {
            pub fn #fn_name(&self) -> &CStr {
                CStr::from_bytes_until_nul(
                    unsafe {
                        mem::transmute::<_, &[u8; #size_cst as _]>(&self.#name)
                    }
                    .as_slice(),
                )
                .unwrap()
            }
        }
    });

    // Some functions which are not worth fully automatically generating
    let custom_accessors = match struct_vk_name {
        "VkShaderModuleCreateInfo" => quote! {
            #[inline]
            pub fn code(mut self, value: &'a [u32]) -> Self {
                self.p_code = value.as_ptr();
                self.code_size = value.len() * 4;
                self
            }
        },
        "VkPipelineMultisampleStateCreateInfo" => quote! {
            #[inline]
            pub fn rasterization_samples_with_mask(mut self, samples: SampleCountFlags, mask: Option<&'a [u32]>) -> Self {
                let count = samples.bits();
                assert!(count.is_power_of_two());
                assert!(mask.is_none() || mask.is_some_and(|arr| arr.len() as u32 == (count + 31) / 32));
                self.rasterization_samples = samples;
                self.p_sample_mask = mask.map(|arr| arr.as_ptr()).unwrap_or(ptr::null());
                self
            }
        },
        "VkAccelerationStructureVersionInfoKHR" | "VkMicromapVersionInfoEXT" => quote! {
            #[inline]
            pub fn version_data(mut self, value: &'a [u8; (2*UUID_SIZE) as _]) -> Self {
                self.p_version_data = value.as_ptr();
                self
            }
        },
        _ => quote!(),
    };

    let array_accessors = my_struct
        .fields
        .iter()
        .filter_map(|field| length_fields.get(field.vk_name))
        .map(|length_field| {
            if my_struct.return_only {
                // TODO: still add getters
                return Ok(quote! ());
            }

            let len_field = length_field.len_field;
            let var_name = if length_field.array_fields.len() == 1 && length_field.array_fields[0].name.starts_with("p_") {
                &length_field.array_fields[0].name[("p_".len())..]
            } else if len_field.name.ends_with("_count") {
                 &len_field.name[..(len_field.name.len() - "_count".len())]
            } else if len_field.name.ends_with("_size") {
                &len_field.name[..(len_field.name.len() - "_size".len())]
            } else if len_field.name == "size" {
                "values"  
            } else {
                return Err(anyhow!("field length name not expected: {}", len_field.name))
            };
            let setter_name = format_ident!("{}", var_name);
            //let getter_name = format_ident!("get_{}", field.name[2..]);
            let length_name = format_ident!("{}", length_field.len_field.name);
            // a slice of size 0 can be seen as a none, no need for options if there is just one field
            let can_be_optional = length_field.array_fields.len() > 1;
            let ty_tokens = length_field.array_fields.iter().enumerate().map(|(idx, field)|{
                gen.generate_slice_type(field.advanced_ty.get().unwrap(), idx as u32, format_ident!("{}", field.name), true, true, can_be_optional && field.optional)
            }).collect::<Result<Vec<_>>>()?;
            let field_names = length_field.array_fields.iter().map(|field| {
                format_ident!("{}",field.name)
            }).collect::<Vec<_>>();
            let len_value = if let Some((idx,_)) = length_field.array_fields.iter().enumerate().find(|(_,field)| !can_be_optional || !field.optional) {
                let used_field = &field_names[idx];
                quote! (#used_field.len() as _)
            } else {
                let first_field = &field_names[0];
                quote! (#first_field.map(|p| p.len()).unwrap_or_default() as _)
            };
            let template_arg = ty_tokens.iter().map(|(x,_,_)| x).filter(|x| !x.is_empty());
            let slice_ty = ty_tokens.iter().map(|(_,y,_)| y);
            let attr = ty_tokens.iter().map(|(_,_,z)| z);
            Ok(quote! {
                #[inline]
                pub fn #setter_name<#(#template_arg),*>(mut self, #(#field_names: #slice_ty),*) -> Self {
                    #(#attr;)*
                    self.#length_name = #len_value;
                    self
                }
            })
        })
        .collect::<Result<Vec<_>>>()?;

    let aliases = my_struct
        .aliases
        .borrow()
        .iter()
        .map(|name| format_ident!("{}", mapping.get(name).unwrap().name))
        .collect::<Vec<_>>();

    let name = format_ident!("{}", my_struct.name);
    let doc_tag = make_doc_link(struct_vk_name);

    let has_lifetime = my_struct.has_lifetime.get().unwrap();
    let lifetime = has_lifetime.then(|| quote! (<'a>));
    let require_phantom = has_lifetime;
    let phantom_decl = require_phantom.then(|| quote! (phantom: PhantomData<&'a ()>,));
    let phantom_default = require_phantom.then(|| quote! (phantom: PhantomData,));
    let (s_type_impl, p_next_impl) = if let Some(s_type) = my_struct.s_type {
        let s_type_value: TokenStream = mapping
            .get(s_type)
            .ok_or_else(|| anyhow!("Failed to find structure type for {s_type}"))?
            .name
            .parse()
            .unwrap();
        (
            Some(quote! {
                unsafe impl #lifetime ExtendableStructure for #name #lifetime {
                    const STRUCTURE_TYPE: StructureType = #s_type_value;
                }
            }),
            Some(quote! {
                pub fn push_next<T: ExtendingStructure<Self>>(&mut self, ext: &'a mut T) {
                    unsafe { self.push_next_unchecked(ext) }
                }
            }),
        )
    } else {
        (None, None)
    };
    let struct_extensions = my_struct
        .extends
        .iter()
        .map(|name| {
            mapping
                .get(name.as_str())
                .map(|entry| format_ident!("{}", entry.name))
                .ok_or_else(|| anyhow!("Failed to find extension {}", name))
        })
        .collect::<Result<Vec<_>>>()?;

    // for the time being, do not implement clone for types with a lifetime
    let derives = (!has_lifetime).then(|| quote! (#[derive(Clone, Copy)]));

    if my_struct.is_union {
        // union are much more lightweight
        // we should have wrapper around them (as they are unsafe)

        // For the default implementation, take the first field and use its default value
        let first_field = my_struct.fields.first().unwrap();
        let first_field_name = format_ident!("{}", first_field.name);
        let mut first_field_default =
            gen.generate_default(&first_field.advanced_ty.get().unwrap())?;
        if gen.compute_type_lifetime(&first_field.ty) {
            first_field_default = quote! (ManuallyDrop::new(#first_field_default))
        }
        return Ok(quote! {
            #[repr(C)]
            #derives
            #doc_tag
            pub union #name #lifetime {
                #(#fields)*
            }

            impl #lifetime Default for #name #lifetime {
                fn default() -> Self {
                    Self {
                        #first_field_name: #first_field_default,
                    }
                }
            }

            #(pub type #aliases #lifetime = #name #lifetime;)*
        });
    }

    Ok(quote! {
        #[repr(C)]
        #derives
        #doc_tag
        pub struct #name #lifetime {
            #(#fields)*
            #phantom_decl
        }
        #s_type_impl
        unsafe impl #lifetime Send for #name #lifetime {}
        unsafe impl #lifetime Sync for #name #lifetime {}
        #(unsafe impl<'a, 'b> ExtendingStructure<#struct_extensions<'b>> for #name<'a> {})*

        impl #lifetime Default for #name #lifetime {
            fn default() -> Self {
                Self {
                    #(#default_impl,)*
                    #phantom_default
                }
            }
        }

        impl #lifetime #name #lifetime {
            #(#simple_accessors)*
            #(#char_arr_setters)*
            #custom_accessors
            #(#array_accessors)*
            #p_next_impl
        }

        #(pub type #aliases #lifetime = #name #lifetime;)*
    })
}
