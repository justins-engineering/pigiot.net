use crate::{Configuration, Create};
use dioxus::logger::tracing::error;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdLogOut;
use ory_kratos_client_wasm::apis::frontend_api::create_browser_logout_flow;

#[component]
pub fn OryLogOut() -> Element {
  let create_flow = use_resource(move || async move {
    create_browser_logout_flow(&Configuration::create(), None, None).await
  });

  return match &*create_flow.read() {
    Some(new_flow) => match new_flow {
      Ok(res) => {
        rsx! {
          a {
            class: "btn btn-lg bg-stone-800/80 hover:bg-stone-700/80 border border-stone-700 font-bold hover:border-primary/30",
            href: res.logout_url.clone(),
            Icon { icon: LdLogOut, title: "Logout" }
            "Logout"
          }
        }
      }
      Err(ory_kratos_client_wasm::apis::Error::ResponseError(_res)) => {
        // error!("{res:#?}");
        rsx! {}
      }

      Err(err) => {
        error!("{err:#?}");
        rsx! {}
      }
    },
    None => rsx! {},
  };
}
