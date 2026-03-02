use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::ld_icons::{
  LdCpu, LdRocket, LdSatelliteDish, LdServer, LdShieldHalf, LdZap,
};

#[component]
pub fn Home() -> Element {
  rsx! {
    section { id: "home", class: "container py-12",
      div { class: "flex flex-col lg:flex-row items-center",
        div { class: "lg:w-1/2 mb-12 lg:mb-0 animate-slide-in",
          // div { class: "inline-flex items-center px-4 py-2 rounded-full bg-stone-800/80 border border-stone-700 mb-6 animate-slide-in",
          //   span { class: "w-2 h-2 rounded-full bg-teal-500 mr-2 animate-pulse" }
          //   span { class: "text-sm font-medium", "PigIoT Open-Source" }
          //   span { class: "ml-2 px-2 py-0.5 bg-teal-900/50 rounded text-xs font-bold text-teal-300",
          //     "v0.1.0"
          //   }
          // }
          h1 { class: "text-4xl text-center md:text-6xl lg:text-7xl font-bold mb-6 leading-tight",
            "The "
            span { class: "gradient-text block", "No Compromise" }
            "IoT Platform"
          }
          p { class: "text-xl text-gray-400 mb-8 max-w-2xl leading-relaxed",
            "An open-source IoT platform built in Rust, from front to back."
            span { class: "font-semibold text-teal-300",
              " You can be sure you're not leaving any performance on the table or holes in your firewall."
            }
          }
          div { class: "flex flex-col sm:flex-row sm:space-x-6 space-y-4 sm:space-y-0 mb-12",
            Link {
              class: "btn btn-xl grow btn-glow font-bold hover:animate-glow bflex bg-linear-to-r from-teal-600 to-purple-600",
              to: Route::SignUp {},
              Icon {
                icon: LdRocket,
                class: "mr-2 animate-bounce-slow",
                title: "Rocket icon",
              }
              "Get Started Free"
            }
            a {
              class: "btn btn-xl grow bg-stone-800/80 hover:bg-stone-700/80 border border-stone-700 font-bold hover:border-primary/30",
              href: "https://github.com/justins-engineering",
              Icon {
                icon: FaGithub,
                class: "mr-2",
                title: "GitHub logo",
              }
              "View on GitHub"
            }
          }
                // div { class: "grid grid-cols-1 sm:grid-cols-2 gap-6",
        //   div { class: "flex items-center p-4 rounded-xl bg-stone-800/50 border border-stone-700 hover:border-teal-500/30 transition-all duration-300",
        //     div { class: "w-12 h-12 rounded-full bg-teal-900/50 border border-teal-800 flex items-center justify-center mr-4 feature-icon",
        //       Icon {
        //         icon: LdShieldHalf,
        //         class: "size-8 text-teal-400",
        //         title: "Shield icon",
        //       }
        //     }
        //     div {
        //       p { class: "font-bold text-lg", "Security First" }
        //       p { class: "text-sm text-gray-400", "Memory-safe by design" }
        //     }
        //   }
        //   div { class: "flex items-center p-4 rounded-xl bg-stone-800/50 border border-stone-700 hover:border-purple-500/30 transition-all duration-300",
        //     div { class: "w-12 h-12 rounded-full bg-purple-900/50 border border-purple-800 flex items-center justify-center mr-4 feature-icon",
        //       Icon {
        //         icon: LdZap,
        //         class: "size-8 text-purple-400",
        //         title: "Lightning Bolt icon",
        //       }
        //     }
        //     div {
        //       p { class: "font-bold text-lg", "High Performance" }
        //       p { class: "text-sm text-gray-400", "Native speed with Rust" }
        //     }
        //   }
        // }
        }
        div { class: "lg:w-1/2 flex justify-center relative",
          div { class: "relative w-full max-w-lg",
            div { class: "relative rounded-2xl overflow-hidden shadow-2xl image-hover border border-stone-700",
              img {
                alt: "IoT Technology Network",
                class: "w-full h-64 md:h-80 object-cover",
                src: "https://images.unsplash.com/photo-1718866033984-c3ddab9af2a0?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3ixlib=rb-4.1.0&q=80&w=1080",
              }
              div { class: "absolute inset-0 bg-linear-to-t from-stone-950/80 via-transparent to-transparent" }
              div { class: "absolute bottom-4 left-4 right-4",
                div { class: "flex items-center space-x-2",
                  div { class: "w-3 h-3 rounded-full bg-teal-500 animate-pulse" }
                  span { class: "text-sm font-medium",
                    "Live IoT Network Visualization"
                  }
                }
              }
            }
            div { class: "absolute -top-6 -left-6 w-24 h-24 rounded-2xl bg-teal-900/30 border border-teal-800/50 flex items-center justify-center animate-float shadow-lg",
              Icon {
                icon: LdServer,
                class: "size-8 stroke-teal-400",
                title: "Server icon",
              }
            }
            div {
              class: "absolute -bottom-6 -right-6 w-24 h-24 rounded-2xl bg-purple-900/30 border border-purple-800/50 flex items-center justify-center animate-float shadow-lg",
              style: "animation-delay: 1s;",
              Icon {
                icon: LdSatelliteDish,
                class: "size-8 stroke-purple-400",
                title: "Satellite icon",
              }
            }
            div {
              class: "absolute top-1/2 -right-8 w-20 h-20 rounded-2xl bg-accent-200/70 border border-accent-100/50 flex items-center justify-center shadow-lg animate-float",
              style: "animation-delay: 2s;",
              Icon {
                icon: LdCpu,
                class: "size-8 stroke-accent-100",
                title: "Microchip icon",
              }
            }
          }
        }
      }
    }
  }
}
