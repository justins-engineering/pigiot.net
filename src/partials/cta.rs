use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdFileCode, LdGift, LdPlay};

#[component]
pub fn Cta() -> Element {
  rsx! {
    section { id: "cta", class: "py-20",
      div { class: "container mx-auto px-4 sm:px-6",
        div { class: "bg-linear-to-r from-teal-900/30 via-purple-900/20 to-teal-900/30 border border-stone-700 rounded-3xl p-8 md:p-12 text-center shadow-2xl scroll-reveal",
          div { class: "max-w-3xl mx-auto",
            h2 { class: "text-3xl md:text-4xl lg:text-5xl font-bold mb-6",
              "Ready to Build Secure IoT Solutions?"
            }
            p { class: "text-xl text-gray-300 mb-10 leading-relaxed",
              "Join thousands of engineers and businesses who trust Pigeon IoT for their mission-critical applications."
            }
            div { class: "flex flex-col sm:flex-row justify-center space-y-4 sm:space-y-0 sm:space-x-6 mb-8",
              Link {
                class: "btn btn-xl sm:w-1/2 grow btn-glow font-bold bflex bg-linear-to-r from-teal-600 to-purple-600",
                to: Route::SignUp {},
                Icon {
                  icon: LdPlay,
                  class: "mr-2 animate-pulse-slow",
                  title: "Start Now",
                }
                "Start Now"
              }
              a {
                class: "btn btn-xl sm:w-1/2 grow bg-stone-800/80 hover:bg-stone-700/80 border border-stone-700 font-bold hover:border-primary/30",
                href: "https://github.com/justins-engineering",
                Icon {
                  icon: LdFileCode,
                  class: "mr-2",
                  title: "Read the Documentation",
                }
                "Read the Docs"
              }
            }
            div { class: "inline-flex items-center px-4 py-2 rounded-full bg-black/30 border border-teal-800/30",
              Icon {
                icon: LdGift,
                class: "size-10 sm:size-5 mr-2 stroke-teal-400",
                title: "Gift",
              }
              p { class: "text-sm",
                "No credit card required. Free tier includes up to 100 devices."
              }
            }
          }
        }
      }
    }
  }
}
