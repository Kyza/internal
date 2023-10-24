extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{quote};
use syn::{Visibility, token::Pub, ImplItem, Item, Attribute, AttrStyle, parse_quote, ForeignItem};

fn internalize(public_item: Item, private_item: Item) -> TokenStream2 {
	quote! {
		#[cfg(not(feature = "internal"))]
		#private_item
		#[cfg(feature = "internal")]
		#public_item
	}
}

fn is_fully_public(vis: &Visibility) -> bool {
	match vis {
		Visibility::Public(_) => true,
		_ => false,
	}
}

fn process_item(item: Item) -> Item {
	let pub_vis = Visibility::Public(Pub { span: Span::call_site() });

	match item.clone() {
		Item::Const(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Const(public);
		}
		Item::Enum(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Enum(public);
		}
		Item::ExternCrate(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::ExternCrate(public);
		}
		Item::Fn(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Fn(public);
		}
		Item::ForeignMod(mut public) => {
			for mut item in &mut public.items {
				match &mut item {
					ForeignItem::Fn(item) => {
						if !is_fully_public(&item.vis) {
							add_internal_doc_comment(&mut item.attrs);
						}
						item.vis = pub_vis.clone();
					}
					ForeignItem::Static(item) => {
						if !is_fully_public(&item.vis) {
							add_internal_doc_comment(&mut item.attrs);
						}
						item.vis = pub_vis.clone();
					}
					ForeignItem::Type(item) => {
						if !is_fully_public(&item.vis) {
							add_internal_doc_comment(&mut item.attrs);
						}
						item.vis = pub_vis.clone();
					}
					ForeignItem::Macro(_) => {}
					ForeignItem::Verbatim(_) => {}
					_ => {}
				}
			}
		}
		Item::Impl(mut public) => {
			for mut item in &mut public.items {
				match &mut item {
					ImplItem::Const(item) => {
						if !is_fully_public(&item.vis) {
							add_internal_doc_comment(&mut item.attrs);
						}
						item.vis = pub_vis.clone();
					}
					ImplItem::Fn(item) => {
						if !is_fully_public(&item.vis) {
							add_internal_doc_comment(&mut item.attrs);
						}
						item.vis = pub_vis.clone();
					}
					ImplItem::Type(item) => {
						if !is_fully_public(&item.vis) {
							add_internal_doc_comment(&mut item.attrs);
						}
						item.vis = pub_vis.clone();
					}
					ImplItem::Macro(_) => {}
					ImplItem::Verbatim(_) => {}
					_ => {}
				}
			}
			return Item::Impl(public);
		}
		Item::Macro(_) => {}
		Item::Mod(mut public) => {
			if let Some((_, ref mut items)) = public.content {
				for item in items {
					*item = process_item(item.clone())
				}
			}
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Mod(public);
		}
		Item::Static(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Static(public);
		}
		Item::Struct(mut public) => {
			for field in &mut public.fields {
				if field.vis == Visibility::Inherited {
					add_internal_doc_comment(&mut field.attrs);
				}
				field.vis = pub_vis.clone();
			}
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Struct(public);
		}
		Item::Trait(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Trait(public);
		}
		Item::TraitAlias(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::TraitAlias(public);
		}
		Item::Type(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Type(public);
		}
		Item::Union(mut public) => {
			for field in &mut public.fields.named {
				if field.vis == Visibility::Inherited {
					add_internal_doc_comment(&mut field.attrs);
				}
				field.vis = pub_vis.clone();
			}
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Union(public);
		}
		Item::Use(mut public) => {
			if !is_fully_public(&public.vis) {
				add_internal_doc_comment(&mut public.attrs);
			}
			public.vis = pub_vis.clone();
			return Item::Use(public);
		}
		Item::Verbatim(_) => {}
		_ => {}
	};

	item
}

static INTERNAL_DOCS_WARNING: [&str; 2] = [
	"This item is internal and could change or be removed without warning.",
	"Be careful when relying on it."
];

fn add_internal_doc_comment(attrs: &mut Vec<Attribute>) {
	let was_empty = attrs.is_empty();
	let mut end_index = 0;
	for line in INTERNAL_DOCS_WARNING.iter().rev() {
		end_index += 2;
		let line = format!(" {}", line);
		attrs.insert(0, Attribute {
			pound_token: Default::default(),
			style: AttrStyle::Outer,
			bracket_token: Default::default(),
			meta: parse_quote! {
				doc = #line
			},
		});
		attrs.insert(1, Attribute {
			pound_token: Default::default(),
			style: AttrStyle::Outer,
			bracket_token: Default::default(),
			meta: parse_quote! {
				doc = ""
			},
		});
	}
	if !was_empty {
		attrs.insert(end_index, Attribute {
			pound_token: Default::default(),
			style: AttrStyle::Outer,
			bracket_token: Default::default(),
			meta: parse_quote! {
				doc = " ---"
			},
		});
		attrs.insert(end_index + 1, Attribute {
			pound_token: Default::default(),
			style: AttrStyle::Outer,
			bracket_token: Default::default(),
			meta: parse_quote! {
				doc = ""
			},
		});
	} else {
		attrs.pop();
	}
}

#[proc_macro_attribute]
pub fn internal(_attr: TokenStream, input: TokenStream) -> TokenStream {
	let item = syn::parse2::<Item>(TokenStream2::from(input));

	if let Ok(item) = item {
		TokenStream::from(
			internalize(
				process_item(item.clone()),
				item
			)
		)
	} else {
		panic!("`#[internal]` wasn't called on an item");
	}
}