// use crate::components::LoginModal;
use crate::components::Logo;
// use crate::components::RegisterModal;
use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::LdMenu;

#[component]
pub fn Navbar() -> Element {
  rsx! {
    // RegisterModal {}
    // LoginModal {}
    header {
      id: "layout-topbar",
      class: "bg-base-100 lg:bg-base-100/90 border-base-300 sticky top-0 z-10 border-b data-[at-top=true]:border-transparent lg:backdrop-blur-sm",
      div { class: "container",
        nav { class: "flex items-center justify-between py-2",
          div { class: "flex items-center gap-2",
            label {
              class: "px-2 swap swap-rotate",
              r#for: "menu-drawer",
              id: "menu-drawer-trigger",
              aria_label: "open sidebar",
              class: "btn btn-square btn-ghost btn-sm lg:hidden",
              Icon { icon: LdMenu }
            }
            Link { to: Route::Index {}, aria_label: "Home", Logo {} }
          }
          div { class: "max-lg:hidden",
            ul { class: "menu menu-horizontal gap-2 px-1 text-base",
              li { class: "font-medium",
                a { href: "/#features", "Features" }
              }
              li { class: "font-medium",
                a { href: "/#pricing", "Pricing" }
              }
              li { class: "font-medium",
                a { href: "/#faq", "FAQ" }
              }
            }
          }
          div { class: "space-x-2",
            Link {
              class: "btn btn-ghost btn-sm",
              to: Route::SignUp {},
              "Register"
            }
            // button { class: "btn btn-ghost btn-sm",
            //   onclick: move |_| {
            //       document::eval(r#"document.getElementById("register_modal").showModal();"#);
            //   },
            //   "Register"
            // }
            Link {
              class: "btn btn-primary btn-sm",
              to: Route::SignIn {},
              "Login"
            }
                    // button {
          //   class: "btn btn-primary btn-sm",
          //   onclick: move |_| {
          //       document::eval(r#"document.getElementById("login_modal").showModal();"#);
          //   },
          //   "Login"
          // }
          }
        }
      }
      div { class: "drawer",
        input {
          id: "menu-drawer",
          r#type: "checkbox",
          class: "drawer-toggle",
        }
        div { class: "drawer-side",
          label {
            r#for: "menu-drawer",
            aria_label: "close sidebar",
            class: "drawer-overlay",
          }
          div { class: "bg-base-100 min-h-full w-60 p-5",
            Link { to: Route::Index {}, aria_label: "Home", Logo {} }
            ul { class: "menu w-full gap-2 p-0 pt-4",
              li { class: "font-medium",
                a { href: "#features", "Features" }
              }
              li { class: "font-medium",
                a { href: "#pricing", "Pricing" }
              }
              li { class: "font-medium",
                a { href: "#faq", "FAQ" }
              }
            }
          }
        }
      }
    }
  }
}
