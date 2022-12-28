use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Ident, Lifetime, LifetimeDef, TypeParam};

#[proc_macro_derive(IterableSequence)]
pub fn derive_iter(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);

    // Lifetime for the reference to the sequence to be iterated.
    let this = Lifetime::new("'this", Span::call_site());

    // Define impl generics for `SequenceIter`: `this` plus the generics of the
    // target.
    let mut seq_iter_impl_generics = generics.clone();
    seq_iter_impl_generics
        .params
        .push(LifetimeDef::new(this.clone()).into());

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let output = quote! {
        impl #seq_iter_impl_generics ::sqnc::traits::SequenceIter<#this> for #ident #ty_generics
        where
            Self: ::sqnc::traits::Sequence + ::sqnc::traits::IndexableSequence,
        {
            type Iter = ::sqnc::derive::Iter<'this, Self>;
        }

        impl #impl_generics ::sqnc::traits::IterableSequence for #ident #ty_generics
        where
            Self: ::sqnc::traits::Sequence + ::sqnc::traits::IndexableSequence,
        {
            #[inline]
            fn iter(&self) -> ::sqnc::derive::Iter<'_, Self> {
                ::sqnc::derive::Iter::new(self)
            }
        }
    };
    output.into()
}

#[proc_macro_derive(SequenceIntoIterator)]
pub fn derive_into_iter(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);

    // Generic for the item type.
    let item = Ident::new("Item", Span::call_site());
    let mut impl_generics = generics.clone();
    impl_generics.params.push(TypeParam::from(item).into());
    let (impl_generics, _, _) = impl_generics.split_for_impl();

    let (_, ty_generics, _) = generics.split_for_impl();
    let output = quote! {
        impl #impl_generics IntoIterator for #ident #ty_generics
        where
            Self: ::sqnc::traits::Sequence + ::sqnc::traits::IndexableSequence
                + for<'a> ::sqnc::traits::SequenceItem<'a, Item = Item>,
        {
            type Item = Item;
            type IntoIter = ::sqnc::derive::IntoIter<Self>;

            #[inline]
            fn into_iter(self) -> Self::IntoIter {
                Self::IntoIter::new(self)
            }
        }
    };
    output.into()
}
