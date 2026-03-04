use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdSatelliteDish, LdServer};

#[component]
pub fn Infrastructure() -> Element {
  rsx! {
    section { id: "infrastructure", class: "relative w-full lg:w-9/10",
      div { class: "bg-stone-800/30 border border-stone-700 rounded-3xl overflow-hidden shadow-2xl scroll-reveal",
        div { class: "relative h-64 md:h-80 lg:h-96",
          img {
            alt: "Server Infrastructure",
            class: "w-full h-full object-cover",
            src: "https://images.unsplash.com/photo-1762163516269-3c143e04175c?crop=entropy&cs=tinysrgb&fit=max&fm=jpg&ixid=M3w4MDcxMzN8MHwxfHNlYXJjaHwxfHxzZXJ2ZXIlMjBkYXRhJTIwY2VudGVyJTIwdGVjaG5vbG9neSUyMGluZnJhc3RydWN0dXJlfGVufDB8MHx8fDE3NzE4OTEzMzl8MA&ixlib=rb-4.1.0&q=80&w=1080",
          }
          div { class: "absolute inset-0 bg-linear-to-t from-stone-950 via-stone-950/50 to-transparent" }
          div { class: "absolute bottom-0 left-0 right-0 p-8",
            div { class: "flex flex-col md:flex-row items-center justify-between",
              div {
                p { class: "text-2xl md:text-3xl font-bold mb-2",
                  "Enterprise-Grade Infrastructure"
                }
                p { "Global data centers with 99.99% uptime guarantee" }
              }
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
    }
  }
}
