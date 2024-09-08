use std::borrow::Cow;

use proc_macro2::TokenStream;
use quote::quote;

use crate::{backend::Backend, table::Table};

mod insert;

#[derive(Clone)]
pub struct MySqlBackend;

impl Backend for MySqlBackend {
    const QUOTE: char = '`';
    type Bindings = MySqlBindings;

    fn query_result() -> TokenStream {
        quote!(sqlx::mysql::MySqlQueryResult)
    }

    fn impl_insert(table: &Table<Self>) -> TokenStream {
        insert::impl_insert(table)
    }
}

#[derive(Default)]
pub struct MySqlBindings;

impl Iterator for MySqlBindings {
    type Item = Cow<'static, str>;

    fn next(&mut self) -> Option<Self::Item> {
        Some(Cow::Borrowed("?"))
    }
}
