use quote::__private::Span;
use quote::{format_ident, quote, ToTokens};
use syn::visit_mut::{visit_attribute_mut, visit_item_use_mut, visit_path_mut, VisitMut};
use syn::{parse_file, parse_quote, Meta, Token};

struct PathTransformer;

impl VisitMut for PathTransformer {
    fn visit_path_mut(&mut self, i: &mut syn::Path) {
        visit_path_mut(self, i);

        if i.leading_colon.is_some() {
            i.leading_colon = None;
        }
        if i.segments.len() >= 1 {
            let seg = i.segments.first_mut().unwrap();
            if seg.ident == "prost" || seg.ident == "tonic" {
                i.segments.insert(0, format_ident!("rust_cri_kit").into());
            }
        }
    }

    fn visit_item_use_mut(&mut self, i: &mut syn::ItemUse) {
        visit_item_use_mut(self, i);

        if i.leading_colon.is_some() {
            i.leading_colon = None;
        }

        i.tree = match i.tree {
            syn::UseTree::Path(ref mut p) => {
                if p.ident == "prost" || p.ident == "tonic" {
                    syn::UseTree::Path(syn::UsePath {
                        ident: format_ident!("rust_cri_kit"),
                        tree: Box::new(syn::UseTree::Path(p.clone())),
                        colon2_token: Token![::](Span::mixed_site()),
                    })
                } else {
                    syn::UseTree::Path(p.clone())
                }
            }
            _ => i.tree.clone(),
        };
    }

    fn visit_attribute_mut(&mut self, i: &mut syn::Attribute) {
        visit_attribute_mut(self, i);

        if i.path.is_ident("derive") {
            let mut meta = i.parse_meta().unwrap();

            match meta {
                Meta::List(ref mut ml) => {
                    if ml.path.is_ident("derive") {
                        ml.nested.iter_mut().for_each(|it| match it {
                            syn::NestedMeta::Meta(ref mut m) => match m {
                                Meta::Path(ref mut p) => {
                                    if p.leading_colon.is_some() {
                                        p.leading_colon = None;
                                    }

                                    if p.segments.first().unwrap().ident == "prost" {
                                        p.segments.insert(0, format_ident!("rust_cri_kit").into());
                                    }
                                }
                                _ => {}
                            },
                            _ => {}
                        })
                    }

                    let nested = ml.nested.clone();
                    i.tokens = proc_macro2::Group::new(
                        proc_macro2::Delimiter::Parenthesis,
                        nested.to_token_stream(),
                    )
                    .to_token_stream();
                }
                _ => {}
            }
        }
    }
}

pub fn transform(file_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;

    let mut file_parsed = parse_file(&file_content)?;
    PathTransformer.visit_file_mut(&mut file_parsed);

    Ok(prettyplease::unparse(&file_parsed))
}
