use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::ld_icons::{LdTriangleAlert, LdX};

#[component]
pub fn Alert() -> Element {
  let mut dismissed: Signal<bool> = use_signal(|| false);
  rsx! {
    div {
      role: "alert",
      class: "sticky top-16 sm:top-18 z-25 alert alert-warning alert-soft sm:px-4 md:px-8 lg:px-16 xl:px-32 2xl:px-64",
      class: if dismissed() { "hidden" },
      Icon { icon: LdTriangleAlert, title: "Important" }
      span {
        "This platform is in pre-alpha, the information presented here may not be accurate and will change!"
      }
      button {
        class: "btn btn-sm btn-square btn-warning btn-soft",
        aria_label: "Close alert",
        onclick: move |_| {
            dismissed.set(true);
        },
        Icon { icon: LdX, title: "Dismiss" }
      }
    }
  }
}
