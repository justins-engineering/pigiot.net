use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::ld_icons::{LdChevronsDown, LdRocket};

#[component]
pub fn Home() -> Element {
  rsx! {
    section {
      id: "home",
      class: "my-12 w-full h-dvh lg:w-7/10 2xl:w-1/3 animate-slide-in",
      h1 { class: "text-center text-4xl md:text-6xl lg:text-7xl font-bold leading-tight",
        "The"
        span { class: "gradient-text font-extrabold block", "No Compromise" }
        "IoT Platform"
      }
      article { class: "text-xl text-center my-8 leading-relaxed",
        p {
          "An open-source, distributed, and efficient IoT platform. Built in "
          span { class: "text-orange-400", "Rust" }
          ", from front to back. Designed with security, privacy, and flexibility in mind."
        }
        br {}
        p {
          span { class: "text-primary", " Your data." }
          span { class: "text-secondary", " Your control." }
        }
      }
      div { class: "flex flex-col sm:flex-row sm:space-x-6 space-y-4 sm:space-y-0",
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
          Icon { icon: FaGithub, class: "mr-2", title: "GitHub logo" }
          "View on GitHub"
        }
      }
      div { class: "size-full my-auto",
        Icon {
          icon: LdChevronsDown,
          class: "size-15 mx-auto stroke-primary animate-pulse-slow",
          title: "Scroll for more info",
        }
      }
    }
  }
}
