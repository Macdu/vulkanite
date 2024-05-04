use core::fmt;
use std::marker::PhantomData;

use serde::{
    de::{self, DeserializeOwned, Visitor},
    Deserialize, Deserializer,
};

#[macro_export]
macro_rules! deserialize_skip {
    ($fn_name:ident,$ty:ident, $($remove_name:literal => $remove_ty:ident),*) => {
        fn $fn_name<'de, D>(deserializer: D) -> Result<Vec<$ty>, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Debug, Deserialize)]
            #[serde(rename_all = "snake_case")]
            enum MyEnum {
                $ty($ty),
                $(
                    #[serde(rename = $remove_name)]
                    $remove_ty,
                )*
            }
            let result = Vec::<MyEnum>::deserialize(deserializer)?
                .into_iter()
                .filter_map(|el| {
                    if let MyEnum::$ty(ty) = el {
                        Some(ty)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            Ok(result)
        }
    };
}

#[macro_export]
macro_rules! deserialize_multiple {
    ($fn_name:ident, $class_name:ident, $(
        $type_name:ident => $type_ty:ident
    ),* $(,)?) => {
        fn $fn_name<'de, D>(deserializer: D) -> Result<$class_name, D::Error>
        where
            D: Deserializer<'de>,
        {
            #[derive(Debug, Deserialize)]
            #[serde(rename_all = "snake_case")]
            enum MyEnum {
                $(
                    $type_ty($type_ty),
                )*
            }
            $(
                let mut $type_name = Vec::new();
            )*
            for el in Vec::<MyEnum>::deserialize(deserializer)? {
                match el {
                    $(
                        MyEnum::$type_ty(el) => $type_name.push(el),
                    )*
                }
            }
            Ok($class_name{
                $($type_name,)*
            })
        }
    };
}

// from https://github.com/serde-rs/serde/issues/581
pub(crate) fn comma_separated<'de, V, T, D>(deserializer: D) -> Result<V, D::Error>
where
    V: FromIterator<T>,
    T: DeserializeOwned,
    D: Deserializer<'de>,
{
    struct CommaSeparated<V, T>(PhantomData<V>, PhantomData<T>);

    #[derive(Deserialize)]
    struct Wrapper<T> {
        ty: T,
    }

    impl<'de, V, T> Visitor<'de> for CommaSeparated<V, T>
    where
        V: FromIterator<T>,
        T: DeserializeOwned,
    {
        type Value = V;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string containing comma-separated elements")
        }

        fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            let iter = s.split(",").map(|item| {
                quick_xml::de::from_str::<Wrapper<T>>(&format!("<root><ty>{item}</ty></root>"))
                    .map(|res| res.ty)
            });
            Result::from_iter(iter).map_err(de::Error::custom)
        }
    }

    let visitor = CommaSeparated(PhantomData, PhantomData);
    deserializer.deserialize_str(visitor)
}

pub(crate) fn screaming_snake_to_pascal_case(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    // start with an uppercase caracter
    let mut next_is_uppercase = true;

    for c in name.chars() {
        match c {
            '_' => next_is_uppercase = true,
            _ if c.is_ascii_digit() => {
                next_is_uppercase = true;
                result.push(c);
            }
            _ if next_is_uppercase => {
                next_is_uppercase = false;
                result.push(c);
            }
            _ => {
                result.push(c.to_ascii_lowercase());
            }
        }
    }
    result
}

pub(crate) fn camel_case_to_snake_case(name: &str) -> String {
    let mut result: String = String::with_capacity(name.len());
    // Contiguous uppercase letters must not have an underscore between them
    // prevent UUID from being translated to u_u_i_d
    let mut can_put_separation = false;
    for c in name.chars() {
        if c == '_' {
            // just for textureCompressionASTC_HDR..
            can_put_separation = true;
            continue
        } else if c.is_ascii_uppercase() && can_put_separation {
            result.push('_');
            can_put_separation = false;
        } else if !c.is_ascii_uppercase() {
            can_put_separation = true;
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

pub(crate) fn longuest_common_prefix<'a, 'b>(str1: &'a str, str2: &'b str) -> &'a str {
    let prefix_size = str1
        .chars()
        .zip(str2.chars())
        .take_while(|(c1, c2)| c1 == c2)
        .count();
    &str1[..prefix_size]
}
