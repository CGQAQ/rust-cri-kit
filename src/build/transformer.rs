use syn::visit_mut::{visit_item_trait_mut, visit_trait_item_method_mut, VisitMut};
use syn::{parse_file, parse_quote};

struct TraitItemDefauter;
impl VisitMut for TraitItemDefauter {
    fn visit_trait_item_method_mut(&mut self, i: &mut syn::TraitItemMethod) {
        if i.default.is_none() {
            let ident = &i.sig.ident;
            let args = i.sig.inputs.iter().filter_map(|it| match it {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat) => Some(pat.pat.clone()),
            });

            i.default = if i.sig.asyncness.is_some() {
                Some(parse_quote! {
                    {self.#ident(#(#args),*).await}
                })
            } else {
                Some(parse_quote! {
                    {self.#ident(#(#args),*)}
                })
            }
        }
        visit_trait_item_method_mut(self, i);
    }
}

struct TraitTransformer;
impl VisitMut for TraitTransformer {
    fn visit_item_trait_mut(&mut self, i: &mut syn::ItemTrait) {
        visit_item_trait_mut(self, i);

        if i.ident == "RuntimeService" || i.ident == "ImageService" {
            TraitItemDefauter.visit_item_trait_mut(i);
        }
    }
}

pub fn transform(file_path: String) -> Result<String, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;

    let mut file_parsed = parse_file(&file_content)?;
    TraitTransformer.visit_file_mut(&mut file_parsed);

    Ok(prettyplease::unparse(&file_parsed))
}
