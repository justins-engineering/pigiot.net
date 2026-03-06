use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdDollarSign, LdRadio, LdShieldHalf, LdZap};

#[component]
pub fn Connectivity() -> Element {
  rsx! {
    section { id: "connectivity", class: "front-page",
      div { class: "bg-linear-to-r from-teal-900/30 via-purple-900/20 to-teal-900/30 border border-teal-800/30 rounded-3xl p-8 md:p-12 shadow-2xl scroll-reveal",
        div { class: "flex flex-col lg:flex-row items-center gap-12",
          div { class: "lg:w-2/3",
            div { class: "flex flex-col items-center py-2 rounded-full bg-black/40 border border-teal-800 mb-6",
              span { class: "text-teal-400 font-bold mr-2 animate-pulse",
                "COMING SOON"
              }
              p { class: "text-sm", "Exclusive partnership" }
            }
            h2 { class: "text-3xl md:text-4xl font-bold mb-6",
              "Wireless, the only US IoT carrier with easy access to Non-IP Data Delivery (NIDD)"
            }
            p { class: "text-xl text-gray-300 mb-8 leading-relaxed",
              "Dirt cheap service and high efficiency! Connect your devices without the overhead of traditional IP stacks."
            }
            div { class: "grid grid-cols-1 md:grid-cols-3 gap-5",
              div { class: "flex flex-row space-x-2 items-start p-4 rounded-xl bg-black/20 border border-teal-800/20 hover:border-teal-500/30 transition-all duration-300",
                Icon {
                  icon: LdZap,
                  class: "mt-1 h-8 w-1/2 stroke-teal-400",
                  title: "Lightning Bolt icon",
                }
                div {
                  h4 { class: "font-bold text-lg mb-2", "High Efficiency" }
                  p { class: "text-gray-400 text-sm",
                    "Reduced data overhead by up to 80% with NIDD technology."
                  }
                }
              }
              div { class: "flex flex-row space-x-2 items-start p-4 rounded-xl bg-black/20 border border-purple-800/20 hover:border-purple-500/30 transition-all duration-300",
                Icon {
                  icon: LdDollarSign,
                  class: "mt-1 h-8 w-1/2 text-purple-400",
                  title: "Dollar sign icon",
                }
                div {
                  h4 { class: "font-bold text-lg mb-2", "Cost Effective" }
                  p { class: "text-gray-400 text-sm",
                    "Pay only for the data you use with our transparent pricing."
                  }
                }
              }
              div { class: "flex flex-row space-x-2 items-start p-4 rounded-xl bg-black/20 border border-teal-800/20 hover:border-teal-500/30 transition-all duration-300",
                Icon {
                  icon: LdShieldHalf,
                  class: "mt-1 h-8 w-1/2 stroke-teal-400",
                  title: "Shield icon",
                }
                div {
                  h4 { class: "font-bold text-lg mb-2", "Enhanced Security" }
                  p { class: "text-gray-400 text-sm",
                    "NIDD provides inherent security benefits over traditional IP delivery."
                  }
                }
              }
            }
          }
          div { class: "lg:w-1/3 flex justify-center",
            div { class: "relative",
              div { class: "w-48 h-48 rounded-full bg-linear-to-br from-teal-700/20 to-purple-700/20 flex items-center justify-center animate-spin-slow border border-teal-800/30",
                div { class: "w-36 h-36 rounded-full bg-linear-to-br from-teal-900/30 to-purple-900/30 flex items-center justify-center",
                  Icon {
                    icon: LdRadio,
                    class: "size-20 stroke-teal-400",
                    title: "Wifi icon",
                  }
                }
              }
              div { class: "absolute -bottom-4 -right-4 w-16 h-16 rounded-full bg-linear-to-br from-purple-600 to-purple-700 flex items-center justify-center shadow-lg animate-bounce-slow",
                Icon {
                  icon: LdDollarSign,
                  class: "size-7 stroke-white",
                  title: "Dollar sign icon",
                }
              }
            }
          }
        }
      }
    }
  }
}
