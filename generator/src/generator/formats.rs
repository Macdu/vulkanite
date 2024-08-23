use super::Generator;
use anyhow::Result;
use proc_macro2::TokenStream;
use quote::quote;

pub fn generate<'a>(gen: &Generator<'a>) -> Result<String> {
    let mut names = Vec::new();
    let mut block_texels = Vec::new();
    let mut block_sizes = Vec::new();
    let mut block_extents = Vec::new();
    let mut component_counts = Vec::new();
    let mut compressed_formats = Vec::new();

    for format in gen
        .registry
        .formats
        .iter()
        .flat_map(|formats| &formats.format)
    {
        let format_name: TokenStream = gen.get_mapping_name(&format.name)?.parse().unwrap();
        names.push(format_name.clone());

        if format.compressed.is_some() {
            compressed_formats.push(format_name.clone());
        }

        if format.texels_per_block > 1 {
            let texels_per_block = format.texels_per_block;
            block_texels.push(quote! {#format_name => #texels_per_block,});
        }

        block_sizes.push(format.block_size);

        component_counts.push(format.component.len() as u8);

        if format.block_extent.len() >= 3 {
            assert!(format.block_extent.len() == 3);
            let x = format.block_extent[0];
            let y = format.block_extent[1];
            let z = format.block_extent[2];

            block_extents.push(quote! {#format_name => [#x, #y, #z],});
        };
    }

    // Note: is it better to panic or return a default value for vk::Format::Undefined ?
    let result = quote! {
        use super::Format;

        impl Format {
            /// Return the number of components of this format.
            pub const fn component_count(self) -> u8 {
                match self {
                    Format::Undefined => panic!("Trying to get the component count of vk::Format::Undefined"),
                    #(#names => #component_counts,)*
                }
            }

            /// Return the texel block size of this format in bytes
            pub const fn block_size(self) -> u8 {
                match self {
                    Format::Undefined => panic!("Trying to get the block size of vk::Format::Undefined"),
                    #(#names => #block_sizes,)*
                }
            }

            /// Return the number of texels in a texel block
            pub const fn texels_per_block(self) -> u8 {
                match self {
                    Format::Undefined => panic!("Trying to get the number of texels per block of vk::Format::Undefined"),
                    #(#block_texels)*
                    _ => 1,
                }
            }

            /// Return the three-dimensional extent of texel blocks
            pub const fn block_extent(self) -> [u8; 3] {
                match self {
                    Format::Undefined => panic!("Trying to get the block extent of vk::Format::Undefined"),
                    #(#block_extents)*
                    _ => [1, 1, 1],
                }
            }

            /// Return true if this format is a compressed format
            pub const fn is_compressed(self) -> bool {
                match self {
                    #(#compressed_formats)|* => true,
                    _ => false,
                }
            }
        }
    }
    .to_string();

    Generator::format_result(result)
}
