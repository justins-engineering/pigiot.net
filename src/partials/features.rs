use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{
  LdArrowRight, LdCodeXml, LdDatabaseBackup, LdLockKeyhole,
};

#[component]
pub fn Features() -> Element {
  rsx! {
    section { id: "features", class: "my-24",
      div { class: "text-center mb-16",
        h2 { class: "text-3xl md:text-4xl lg:text-5xl font-bold mb-4",
          "Built for the Builders"
        }
        p { class: "text-xl text-gray-400 max-w-3xl mx-auto",
          "A complete IoT platform with native high availability, designed for open-source enthusiasts and B2B applications."
        }
      }
      div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8",
        div {
          class: "card card-xl card-border space-y-8 justify-around bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover",
          style: "animation-delay: 0.2s;",
          div { class: "card-title space-x-4",
            div { class: "p-2 rounded-2xl bg-linear-to-br from-teal-900 to-teal-800 feature-icon shadow-lg",
              Icon {
                icon: LdCodeXml,
                class: "size-10 stroke-teal-400",
                title: "Code Icon",
              }
            }
            h3 { class: "text-2xl font-bold", "100% Rust" }
          }
          p { class: "leading-relaxed",
            "From frontend to backend, everything is written in Rust. Enjoy memory safety, zero-cost abstractions, and fearless concurrency."
          }
          div { class: "card-actions justify-end",
            a {
              class: "flex items-center text-teal-400 font-medium hover:translate-x-2 transition-transform duration-300",
              href: "#",
              "Learn more"
              Icon {
                icon: LdArrowRight,
                class: "ml-2",
                title: "Arrow right",
              }
            }
          }
        }
        div {
          class: "card card-xl card-border space-y-8 justify-around bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover",
          style: "animation-delay: 0.2s;",
          div { class: "card-title space-x-4",
            div { class: "p-2 rounded-2xl bg-linear-to-br from-purple-900 to-purple-800 flex items-center justify-center feature-icon shadow-lg",
              Icon {
                icon: LdLockKeyhole,
                class: "size-10 stroke-purple-400",
                title: "Lock Icon",
              }
            }
            h3 { class: "text-2xl font-bold", "Security by Design" }
          }
          p { class: "leading-relaxed",
            "Rust's ownership model eliminates entire classes of security vulnerabilities. No more buffer overflows or data races."
          }
          div { class: "card-actions justify-end",
            a {
              class: "flex items-center text-purple-400 font-medium hover:translate-x-2 transition-transform duration-300",
              href: "#",
              "Security whitepaper"
              Icon {
                icon: LdArrowRight,
                class: "ml-2",
                title: "Arrow right",
              }
            }
          }
        }
        div {
          class: "card card-xl card-border space-y-8 justify-around bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover",
          style: "animation-delay: 0.2s;",
          div { class: "card-title space-x-4",
            div { class: "p-2 rounded-2xl bg-linear-to-br from-teal-900 to-purple-900 flex items-center justify-center feature-icon shadow-lg",
              Icon {
                icon: LdDatabaseBackup,
                class: "size-10 stroke-teal-300",
                title: "Lock Icon",
              }
            }
            h3 { class: "text-2xl font-bold", "High Availability" }
          }
          p { class: "text-gray-400 leading-relaxed",
            "Native high-availability architecture ensures your IoT infrastructure remains operational even during partial failures."
          }
          div { class: "card-actions justify-end",
            a {
              class: "flex items-center text-teal-300 font-medium hover:translate-x-2 transition-transform duration-300",
              href: "#",
              "Architecture docs"
              Icon {
                icon: LdArrowRight,
                class: "ml-2",
                title: "Arrow right",
              }
            }
          }
        }
      }
    }
  }
}
