use quote::quote;
use syn::visit_mut::{visit_item_trait_mut, visit_trait_item_method_mut, VisitMut};
use syn::{parse_file, parse_quote};

enum Mode {
    Runtime,
    Image,
}

struct TraitItemDefauter(String, Mode);
impl VisitMut for TraitItemDefauter {
    fn visit_trait_item_method_mut(&mut self, i: &mut syn::TraitItemMethod) {
        if i.default.is_none() {
            let ident = &i.sig.ident;
            let ident_str = ident.to_string();
            let args = i.sig.inputs.iter().filter_map(|it| match it {
                syn::FnArg::Receiver(_) => None,
                syn::FnArg::Typed(pat) => Some(pat.pat.clone()),
            });

            let (runtime, image) = (
                quote::format_ident!("{}RuntimeService", change_case::pascal_case(&self.0)),
                quote::format_ident!("{}ImageService", change_case::pascal_case(&self.0)),
            );
            let struct_ = match self.1 {
                Mode::Runtime => quote! {
                    crate::runtime_service::#runtime
                },
                Mode::Image => quote! {
                    crate::image_service::#image
                },
            };
            let struct_str = format!("{}", struct_);

            let trait_ = match self.1 {
                Mode::Runtime => quote! {
                    crate::cri::runtime_service_server::RuntimeService
                },
                Mode::Image => quote! {
                    crate::cri::image_service_server::ImageService
                },
            };

            if i.sig.ident == "get_container_events" {
                i.sig.output = parse_quote! {
                    -> Result<
                            tonic::Response<
                                <#struct_ as #trait_>::GetContainerEventsStream,
                            >,
                            tonic::Status,
                        >
                };
            }

            let assertion = quote! {
                debug_assert!(<#struct_ as #trait_>::#ident as *const () != #struct_::#ident as *const (), "{}::{} is not the implemented!", #struct_str, #ident_str);
            };

            i.default = if i.sig.asyncness.is_some() {
                Some(parse_quote! {
                    {
                        #assertion
                        let this = unsafe { std::mem::transmute(&self) };
                        <#struct_>::#ident(this, #(#args),*).await
                    }
                })
            } else {
                Some(parse_quote! {
                    {
                        #assertion
                        <#struct_>::#ident(unsafe { &*(self as *const Self as *const _ as *const #struct_) }, #(#args),*)
                    }
                })
            }
        }
        visit_trait_item_method_mut(self, i);
    }
}

struct TraitTransformer(String);
impl VisitMut for TraitTransformer {
    fn visit_item_trait_mut(&mut self, i: &mut syn::ItemTrait) {
        visit_item_trait_mut(self, i);

        if i.ident == "RuntimeService" {
            TraitItemDefauter(self.0.clone(), Mode::Runtime).visit_item_trait_mut(i);
        } else if i.ident == "ImageService" {
            TraitItemDefauter(self.0.clone(), Mode::Image).visit_item_trait_mut(i);
        }
    }
}

pub fn transform(
    file_path: String,
    project_name: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let file_content = std::fs::read_to_string(file_path)?;

    let mut file_parsed = parse_file(&file_content)?;
    TraitTransformer(project_name.clone()).visit_file_mut(&mut file_parsed);

    Ok(prettyplease::unparse(&file_parsed)
        // hack
        .replace(
            "T::GetContainerEventsStream",
            format!("<crate::runtime_service::{}RuntimeService as RuntimeService>::GetContainerEventsStream", change_case::pascal_case(&project_name)).as_str(),
        )
    )
}
