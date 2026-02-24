use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{LdGithub, LdKeyRound, LdMail, LdX};
use dioxus_free_icons::Icon;

#[component]
pub fn LoginModal() -> Element {
  rsx! {
    dialog { class: "modal", id: "login_modal",
      div { class: "modal-box relative max-w-xs md:max-w-sm",
        form { class: "absolute end-2 top-2", method: "dialog",
          button { class: "btn btn-sm btn-circle btn-ghost",
            Icon { icon: LdX, class: "size-4" }
          }
        }
        div { class: "text-center text-xl font-medium", "Login" }
        fieldset { class: "fieldset mt-5",
          legend { class: "fieldset-legend", "Email Address" }
          label { class: "input w-full focus:outline-0",
            Icon {
              icon: LdMail,
              class: "text-base-content/80 size-5",
            }
            input {
              class: "grow focus:outline-0",
              placeholder: "Email Address",
              r#type: "email",
            }
          }
        }
        fieldset { class: "fieldset",
          legend { class: "fieldset-legend", "Password" }
          label { class: "input w-full focus:outline-0",
            Icon {
              icon: LdKeyRound,
              class: "text-base-content/80 size-5",
            }
            input {
              class: "grow focus:outline-0",
              placeholder: "Password",
              r#type: "password",
            }
          }
        }
        div { class: "mt-5 flex items-center justify-end gap-3",
          button { class: "btn", "Register" }
          button { class: "btn btn-primary", "Login" }
        }
        div { class: "mt-5 flex items-center gap-3",
          hr { class: "border-base-300 grow" }
          span { class: "text-base-content/70 text-sm", "Continue with" }
          hr { class: "border-base-300 grow" }
        }
        div { class: "mt-5 flex gap-4",
          button { class: "btn grow",
            Icon { icon: LdGithub, class: "size-4" }
            "Github"
          }
        }
      }
      form { class: "modal-backdrop", method: "dialog",
        button { "close" }
      }
    }
  }
}
