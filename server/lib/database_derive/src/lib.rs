#![recursion_limit="128"]

extern crate syn;
#[macro_use]
extern crate quote;
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_derive(Query, attributes(table_name))]
pub fn query(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_derive_input(&s).unwrap();

    // Build the impl
    let gen = impl_hello_world(&ast);

    // Return the generated impl
    gen.parse().unwrap()
}

fn impl_hello_world(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;

    // get a list of all fields in this struct.
    let field_vec = match ast.body {
        syn::Body::Struct(syn::VariantData::Struct(ref field_vec)) => field_vec,
        _ => panic!("{} is not a struct, while the Query derive needs a struct!", name),
    };

    // creates a string of the form "field_a, field_b, field_c"
    let select_fields = field_vec
        .iter()
        .map(|field| field.ident.as_ref().expect("No identifier found!").as_ref().to_owned())
        .fold(String::new(), |mut a, b| {
            a.push_str(&b);
            a.push_str(", ");
            a
        });

    // find the table name, as noted in #[table_name = "..."]
    let table_name = ast.attrs.iter().filter(|t| t.name() == "table_name").filter_map(|t| match &t.value {
        &syn::MetaItem::NameValue(_, syn::Lit::Str(ref s, _)) => Some(s),
        _ => None,
        }).next().unwrap_or_else(|| panic!("No table_name specified for {}", name.as_ref()));

    // construct the query
    let query = &format!("SELECT {} FROM {};", &select_fields[..select_fields.len() - 2], table_name);

    // create a bunch of field initialisers for the constructor.
    let fields = field_vec.iter()
        .filter_map(|field| field.ident.as_ref()).enumerate()
        .map(|(n, ident)| quote! { #ident: row.get(#n) })
        .collect::<Vec<_>>();

    // output
    quote! {
        impl Query for #name {
            fn load(conn : &::postgres::Connection) -> Result<Vec<Self>, Box<::std::error::Error>> {
                let mut res = Vec::new();
                let query = #query;
                for row in &conn.query(query, &[])? {
                    let el = #name { #(#fields),* };
                    res.push(el);
                }
                Ok(res)
            }
        }

        impl DebugQuery for #name {
            fn debug() -> String {
                #query.to_string()
            }
        }
    }
}
