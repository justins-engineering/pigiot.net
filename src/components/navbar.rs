// use crate::components::Logo;
use crate::components::OryLogOut;
use crate::{Route, Session};
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
  LdBird, LdHome, LdInfo, LdLogIn, LdMenu, LdMessageCircleQuestion, LdTag, LdX,
};

#[component]
pub fn Navbar() -> Element {
  let mut show_menu: Signal<bool> = use_signal(|| true);

  rsx! {
    header { class: "w-full sticky top-0 z-50 backdrop-blur-md bg-base-200/80 border-b border-stone-800/50 shadow-lg",
      nav { class: "navbar sm:px-4 md:px-8 lg:px-16 xl:px-32 2xl:px-64",
        div { class: "navbar-start space-x-3",
          div { class: "size-10 rounded-full flex items-center justify-center bg-linear-to-br from-teal-600 to-purple-600 animate-glow",
            // Logo {}
            Icon { icon: LdBird, class: "size-6", title: "Logo" }
          }
          span { class: "text-primary text-3xl font-bold", "PigIoT" }
        }
        div { class: "navbar-center hidden md:flex space-x-8",
          ul { class: "menu menu-lg menu-horizontal",
            li {
              Link {
                to: Route::Index {},
                class: "hover:text-primary transition-colors duration-300 relative group",
                "Home"
                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-primary group-hover:w-full transition-all duration-300" }
              }
            }
            li {
              Link {
                to: Route::About {},
                class: "hover:text-primary transition-colors duration-300 relative group",
                "About Us"
                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-primary group-hover:w-full transition-all duration-300" }
              }
            }
            li {
              Link {
                to: Route::Faq {},
                class: "hover:text-primary transition-colors duration-300 relative group",
                "FAQ"
                span { class: "absolute bottom-0 left-0 w-0 h-0.5 bg-primary group-hover:w-full transition-all duration-300" }
              }
            }
          }
        }
        div { class: "navbar-end hidden md:flex space-x-4",
          if *use_context::<Session>().state.read() {
            a {
              class: "btn btn-lg btn-glow hover:animate-glow bg-linear-to-r from-teal-700 to-purple-600 font-semibold",
              href: "https://dashboard.pigiot.net",
              "Dashboard"
            }
            OryLogOut {}
          } else {
            Link {
              class: "btn btn-lg btn-glow hover:animate-glow bg-linear-to-r from-teal-700 to-purple-600 font-semibold",
              to: Route::SignUp {},
              "Get Started"
            }
            Link {
              class: "btn btn-lg bg-stone-800/80 hover:bg-stone-700/80 border border-stone-700 font-bold hover:border-primary/30",
              to: Route::SignIn {},
              Icon { icon: LdLogIn, title: "Login" }
              "Login"
            }
          }
        }
        div { class: "navbar-end md:hidden",
          label {
            class: "swap swap-rotate",
            class: if show_menu() { "swap-active" },
            r#for: "mobile-menu",
            id: "mobile-menu-button",
            aria_label: "open menu",
            onclick: move |_| {
                show_menu.toggle();
            },
            Icon { icon: LdMenu, class: "swap-on" }
            Icon { icon: LdX, class: "swap-off" }
          }
        }
      }
      div {
        class: "md:hidden mt-4 pb-4",
        class: if show_menu() { "hidden" },
        id: "mobile-menu",
        div { class: "menu menu-vertical space-y-3 justify-items-center w-full",
          Link {
            to: Route::Index {},
            class: "font-medium hover:text-primary transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
            Icon {
              icon: LdHome,
              class: "inline align-text-bottom mr-3",
            }
            "Home"
          }
          Link {
            class: "font-medium hover:text-primary transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
            to: Route::About {},
            Icon {
              icon: LdInfo,
              class: "inline align-text-bottom mr-3",
            }
            "About Us"
          }
          Link {
            class: "font-medium hover:text-primary transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
            to: Route::Faq {},
            Icon {
              icon: LdMessageCircleQuestion,
              class: "inline align-text-bottom relative mr-3",
            }
            "FAQ"
          }
          a {
            class: "font-medium hover:text-primary transition-colors py-3 px-4 rounded-lg hover:bg-stone-800/50",
            href: "/#pricing",
            Icon {
              icon: LdTag,
              class: "inline align-text-bottom mr-3",
            }
            "Pricing"
          }
          if *use_context::<Session>().state.read() {
            a {
              class: "btn btn-lg btn-glow hover:animate-glow bg-linear-to-br from-primary to-secondary font-semibold p-0",
              href: "https://dashboard.pigiot.net",
              "Dashboard"
            }
            OryLogOut {}
          } else {
            Link {
              class: "btn btn-lg btn-glow hover:animate-glow bg-linear-to-br from-primary to-secondary font-semibold p-0",
              to: Route::SignUp {},
              "Get Started"
            }
            Link {
              class: "btn btn-lg bg-stone-800/80 hover:bg-stone-700/80 border border-stone-700 font-bold hover:border-primary/30",
              to: Route::SignIn {},
              Icon { icon: LdLogIn, title: "Login" }
              "Login"
            }
          }
        }
      }
    }
  }
}
