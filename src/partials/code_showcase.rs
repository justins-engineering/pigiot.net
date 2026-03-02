use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaRust;
use dioxus_free_icons::icons::ld_icons::LdCheck;

#[component]
pub fn CodeShowcase() -> Element {
  rsx! {
    section {
      id: "showcase",
      class: "py-20 bg-linear-to-b from-stone-900/50 to-stone-950/50",
      div { class: "container mx-auto px-4 sm:px-6",
        div { class: "flex flex-col lg:flex-row items-center gap-12",
          div { class: "lg:w-1/2 scroll-reveal",
            div { class: "inline-flex items-center px-4 py-2 rounded-full bg-stone-800/80 border border-stone-700 mb-6",
              Icon {
                icon: FaRust,
                class: "text-orange-500 mr-2",
                title: "Rust Icon",
              }
              span { class: "text-sm font-medium", "Rust-Powered Excellence" }
            }
            h2 { class: "text-3xl md:text-4xl font-bold mb-6",
              span { class: "gradient-text", "Memory Safety" }
              " Meets Performance"
            }
            p { class: "text-gray-400 mb-8 text-lg leading-relaxed",
              "Experience the power of Rust's compile-time guarantees. Our platform leverages Rust's ownership system to eliminate entire classes of bugs and security vulnerabilities while delivering C++-level performance."
            }
            div { class: "space-y-4",
              div { class: "flex items-start",
                div { class: "w-8 h-8 rounded-full bg-teal-900/50 flex items-center justify-center mr-4 mt-1",
                  Icon {
                    icon: LdCheck,
                    class: "text-teal-400",
                    title: "Check Mark",
                  }
                }
                div {
                  h4 { class: "font-bold text-lg mb-1", "Zero Runtime Overhead" }
                  p { class: "text-gray-400",
                    "No garbage collector, no runtime. Pure performance."
                  }
                }
              }
              div { class: "flex items-start",
                div { class: "w-8 h-8 rounded-full bg-purple-900/50 flex items-center justify-center mr-4 mt-1",
                  Icon {
                    icon: LdCheck,
                    class: "text-purple-400",
                    title: "Check Mark",
                  }
                }
                div {
                  h4 { class: "font-bold text-lg mb-1", "Fearless Concurrency" }
                  p { class: "text-gray-400",
                    "Write concurrent code without data races."
                  }
                }
              }
              div { class: "flex items-start",
                div { class: "w-8 h-8 rounded-full bg-teal-900/50 flex items-center justify-center mr-4 mt-1",
                  Icon {
                    icon: LdCheck,
                    class: "text-teal-400",
                    title: "Check Mark",
                  }
                }
                div {
                  h4 { class: "font-bold text-lg mb-1", "Compile-Time Safety" }
                  p { class: "text-gray-400",
                    "Catch bugs before they reach production."
                  }
                }
              }
            }
          }
          div {
            class: "max-w-full lg:w-1/2 scroll-reveal",
            style: "animation-delay: 0.3s;",
            div { class: "bg-stone-900/80 border border-stone-700 rounded-2xl overflow-hidden shadow-2xl",
              div { class: "bg-stone-800 px-6 py-4 border-b border-stone-700 flex items-center",
                div { class: "flex space-x-2",
                  div { class: "w-3 h-3 rounded-full bg-red-500" }
                  div { class: "w-3 h-3 rounded-full bg-yellow-500" }
                  div { class: "w-3 h-3 rounded-full bg-green-500" }
                }
                span { class: "ml-4 text-sm font-medium text-gray-300",
                  "main.rs"
                }
              }
              div { class: "p-6 text-sm overflow-scroll",
                pre {
                  code { class: "text-gray-300 mb-2",
                    span { class: "text-purple-400 mb-2",
                      "use pigeon_iot::DeviceManager;"
                    }
                    br {}
                    br {}
                    span { class: "text-blue-400 mb-2", "async fn " }
                    span { class: "text-yellow-400 mb-2", "connect_device " }
                    "(device_id: &str) -> Result<Device, Error> {{ "
                    br {}
                    span { class: "text-blue-400", "  let " }
                    "mut manager = DeviceManager::new()
    .with_security(SecurityLevel::Maximum)
    .with_encryption(Encryption::AES256GCM)
    .await?;"

                    // Compile-time safety ensures no data races"
                    br {}
                    span { class: "text-blue-400", "  let " }
                    "device = manager.connect(device_id).await?;
  Ok(device)
}}"
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
