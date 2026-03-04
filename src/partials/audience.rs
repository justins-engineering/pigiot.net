use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdBuilding, LdCheck, LdCopyleft, LdUserCog};

#[component]
pub fn Audience() -> Element {
  rsx! {
    section { id: "audience", class: "py-20",
      div { class: "container mx-auto px-4 sm:px-6",
        div { class: "text-center mb-16 scroll-reveal",
          h2 { class: "text-3xl md:text-4xl font-bold mb-4", "Built for the Builders" }
          p { class: "text-xl text-gray-400 max-w-3xl mx-auto",
            "PigIoT is designed for engineers and businesses who demand performance, security, and control."
          }
        }
        div { class: "grid grid-cols-1 md:grid-cols-3 gap-8",
          div { class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
            div { class: "flex items-center mb-6",
              div { class: "size-14 rounded-full bg-linear-to-br from-teal-900/50 to-teal-800/50 flex items-center justify-center mr-4 feature-icon",
                Icon {
                  icon: LdCopyleft,
                  class: "size-8 stroke-teal-400",
                  title: "Copyleft icon",
                }
              }
              h3 { class: "text-2xl font-bold", "Open Source" }
            }
            p { class: "text-gray-400 mb-6 leading-relaxed",
              "Fully transparent codebase with an active community. Contribute, fork, or customize to your needs."
            }
            ul { class: "space-y-4",
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-teal-500",
                    title: "Check mark",
                  }
                }
                span { "AGPL-3.0 Licensed" }
              }
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-teal-500",
                    title: "Check mark",
                  }
                }
                span { "Active community contributions" }
              }
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-teal-500",
                    title: "Check mark",
                  }
                }
                span { "Comprehensive documentation" }
              }
            }
          }
          div {
            class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
            style: "animation-delay: 0.2s;",
            div { class: "flex items-center mb-6",
              div { class: "size-14 rounded-full bg-linear-to-br from-purple-900/50 to-purple-800/50 flex items-center justify-center mr-4 feature-icon",
                Icon {
                  icon: LdUserCog,
                  class: "size-8 stroke-purple-400",
                  title: "User Cog icon",
                }
              }
              h3 { class: "text-2xl font-bold", "Independent Engineers" }
            }
            p { class: "text-gray-400 mb-6 leading-relaxed",
              "Powerful tools for prototyping and production. From hobby projects to commercial deployments."
            }
            ul { class: "space-y-4",
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-purple-900/40 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-purple-500",
                    title: "Check mark",
                  }
                }
                span { "Easy local development" }
              }
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-purple-900/40 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-purple-500",
                    title: "Check mark",
                  }
                }
                span { "Extensive device library" }
              }
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-purple-900/40 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-purple-500",
                    title: "Check mark",
                  }
                }
                span { "Free tier for personal use" }
              }
            }
          }
          div {
            class: "bg-stone-800/30 border border-stone-700 rounded-2xl p-8 card-hover scroll-reveal",
            style: "animation-delay: 0.4s;",
            div { class: "flex items-center mb-6",
              div { class: "size-14 rounded-full bg-linear-to-br from-teal-900/50 to-purple-900/50 flex items-center justify-center mr-4 feature-icon",
                Icon {
                  icon: LdBuilding,
                  class: "size-8 stroke-teal-300",
                  title: "Building icon",
                }
              }
              h3 { class: "text-2xl font-bold", "B2B & Enterprise" }
            }
            p { class: "text-gray-400 mb-6 leading-relaxed",
              "            Scalable, secure, and compliant solutions for industrial IoT, smart cities, and enterprise applications.        "
            }
            ul { class: "space-y-4",
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-teal-400",
                    title: "Check mark",
                  }
                }
                span { "SLA guarantees" }
              }
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-teal-400",
                    title: "Check mark",
                  }
                }
                span { "Enterprise support" }
              }
              li { class: "flex items-center",
                div { class: "size-6 rounded-full bg-teal-900/50 flex items-center justify-center mr-3",
                  Icon {
                    icon: LdCheck,
                    class: "stroke-teal-400",
                    title: "Check mark",
                  }
                }
                span { "Custom deployment options" }
              }
            }
          }
        }
      }
    }
  }
}
