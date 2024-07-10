//! Make any serializable struct or enum encryptable.
//!
//! Usage is entirely straightforward, and is best demonstrated with an example:
//!
//! ```rust
//! use strong_box::{generate_key, StaticStrongBox, StrongBox};
//! use struct_box::StructBox;
//! use serde::{Serialize, Deserialize};
//!
//! #[derive(Serialize, Deserialize, StructBox)]
//! struct SensitiveData {
//!     data: String,
//! }
//!
//! # fn main() -> Result<(), strong_box::Error> {
//! // This is an example of how to setup a StrongBox, which is the encryption engine we use behind
//! // the scenes
//! let strong_key = generate_key();
//! let strong_box = StaticStrongBox::new(strong_key, vec![strong_key]);
//!
//! // This is the data we want to securely encrypt
//! let data = SensitiveData { data: "something very important and secret".to_string() };
//!
//! // The contents of ciphertext is a Vec<u8> that can be safely shared, stored, etc, without
//! // worrying about anyone who doesn't have the key being able to read it
//! let ciphertext = data.encrypt(&strong_box, b"encryption context")?;
//!
//! // Of course, it's not much use if we can't *decrypt* it again, though...
//! let decrypted_data = SensitiveData::decrypt(&ciphertext, &strong_box, b"encryption context")?;
//!
//! assert_eq!(data.data, decrypted_data.data);
//! # Ok(())
//! # }
//! ```
//!
//! As this derive macro is little more than a wrapper around
//! [`strong-box`](https://docs.rs/strong-box), consult that crate's documentation for details
//! about available StrongBox types, and the importance of the "encryption context" that is passed
//! into the `encrypt` and `decrypt` calls.

use proc_macro2::{Ident, TokenStream, TokenTree};
use quote::quote;

#[proc_macro_derive(StructBox)]
pub fn struct_box(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
	let mut tokens = TokenStream::from(input).into_iter().peekable();

	let mut vis = TokenStream::new();
	let mut struct_name: Option<Ident> = None;

	while struct_name.is_none() {
		match tokens.next() {
			Some(TokenTree::Ident(ident)) => match ident.to_string().as_str() {
				"pub" => {
					vis.extend([TokenTree::Ident(ident.clone())]);
					if let Some(TokenTree::Group(g)) = tokens.peek() {
						vis.extend([TokenTree::Group(g.clone())]);
						// Consume that next token, too
						tokens.next();
					}
				}
				"struct" | "enum" => (),
				_ => struct_name = Some(ident.clone()),
			},
			Some(_) => (),
			None => return quote! { compile_error!("no type name found?!") }.into(),
		}
	}

	let struct_name = struct_name.unwrap();

	quote! {
		impl #struct_name {
			#vis fn encrypt(&self, strong_box: &impl ::strong_box::StrongBox, ctx: impl AsRef<[u8]>) -> Result<Vec<u8>, ::strong_box::Error> {
				let mut serialized = Vec::<u8>::new();

				::strong_box::ciborium::into_writer(self, &mut serialized)?;
				strong_box.encrypt(&serialized, ctx.as_ref())
			}

			#vis fn decrypt(ciphertext: impl AsRef<[u8]>, strong_box: &impl ::strong_box::StrongBox, ctx: impl AsRef<[u8]>) -> Result<Self, ::strong_box::Error> {
				let plaintext = strong_box.decrypt(ciphertext.as_ref(), ctx.as_ref())?;
				Ok(::strong_box::ciborium::from_reader::<Self, &[u8]>(&plaintext)?)
			}
		}
	}.into()
}
