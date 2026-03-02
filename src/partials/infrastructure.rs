use dioxus::prelude::*;

#[component]
pub fn Infrastructure() -> Element {
  rsx! {
    section { id: "infrastructure", class: "py-20",
      div { class: "container mx-auto px-4 sm:px-6",
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
                  h3 { class: "text-2xl md:text-3xl font-bold mb-2",
                    "Enterprise-Grade Infrastructure"
                  }
                  p { class: "text-gray-300",
                    "Global data centers with 99.99% uptime guarantee"
                  }
                }
                div { class: "mt-4 md:mt-0",
                  div { class: "flex items-center space-x-2",
                    div { class: "w-3 h-3 rounded-full bg-green-500 animate-pulse" }
                    span { class: "text-sm font-medium", "All Systems Operational" }
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
