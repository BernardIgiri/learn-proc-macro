use std::{collections::HashMap, default};

use syn::{DeriveInput, Ident};

fn impl_reflective_trait(ast: DeriveInput) -> proc_macro::TokenStream {
    let ident = ast.ident;
    let ident_str = ident.to_string();

    let field_idents: Vec<Ident> = match ast.data {
        syn::Data::Struct(data) => data.fields.into_iter().filter_map(|f| f.ident).collect(),
        _ => panic!("Only structs are supported by reflective!"),
    };
    let field_idents_strs: Vec<String> = field_idents.iter().map(|i| i.to_string()).collect();

    quote::quote! {
        impl Reflective for #ident {
            fn name(&self) -> &'static str {
                #ident_str
            }
            fn field_names(&self) -> Vec<&'static str> {
                vec![#(#field_idents_strs),*]
            }
        }
    }
    .into()
}

#[proc_macro_derive(Reflective)]
pub fn reflective_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(item).unwrap();
    impl_reflective_trait(ast)
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataFieldAttributes {
    author: String,
}

fn extract_metadata_field_attrs(
    ast: &mut DeriveInput,
) -> deluxe::Result<HashMap<String, MetaDataFieldAttributes>> {
    let mut field_attrs = HashMap::new();
    if let syn::Data::Struct(s) = &mut ast.data {
        for field in s.fields.iter_mut() {
            let field_name = field.ident.as_ref().unwrap().to_string();
            let attrs: MetaDataFieldAttributes = deluxe::extract_attributes(field)?;
            field_attrs.insert(field_name, attrs);
        }
    }
    Ok(field_attrs)
}

#[derive(deluxe::ExtractAttributes)]
#[deluxe(attributes(metadata))]
struct MetaDataStructAttributes {
    author: String,
    #[deluxe(default = 0)]
    serial_version: usize,
}

fn metadata_derive_macro2(
    item: proc_macro2::TokenStream,
) -> deluxe::Result<proc_macro2::TokenStream> {
    let mut ast: DeriveInput = syn::parse2(item)?;
    let MetaDataStructAttributes {
        author,
        serial_version,
    } = deluxe::extract_attributes(&mut ast)?;
    let field_attrs: HashMap<String, MetaDataFieldAttributes> =
        extract_metadata_field_attrs(&mut ast)?;
    let (field_names, field_authors): (Vec<String>, Vec<String>) = field_attrs
        .into_iter()
        .map(|(field, attrs)| (field, *attrs.author))
        .unzip();
    let ident = &ast.ident;
    let (impl_generics, type_generics, where_clause) = ast.generics.split_for_impl();
    Ok(quote::quote! {
        impl #impl_generics MetaData for #ident #type_generics #where_clause {
            fn author(&self) -> &'static str {
                #author
            }
            fn serial_version(&self) -> usize {
                #serial_version
            }
            // Always use fully qualified types for external dependencies so that users do not have errors from unexpected dependencies.
            fn field_authors(&self) -> std::collections::HashMap<&'static str,  &'static str> {
                // We use arrays to avoid heap allocation on macro expansion
                let fields = [#(#field_names),*];
                let authors = [#(#field_authors),*];
                let map:std::collections::HashMap<&'static str,  &'static str> =
                    fields.into_iter()
                        .zip(authors.iter())
                        .map(|(&field, &author)| (field, author))
                        .collect();
                map
            }
        }
    })
}

#[proc_macro_derive(MetaData, attributes(metadata))]
pub fn metadata_derive_macro(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    metadata_derive_macro2(item.into()).unwrap().into()
}
