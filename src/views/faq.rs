use dioxus::prelude::*;

#[component]
pub fn Faq() -> Element {
  rsx! {
    h1 { class: "m-4 text-4xl", "FAQs" }
    div { class: "container h-full space-y-2",
      div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
        input { r#type: "radio", name: "my-accordion-2", checked: true }
        div { class: "collapse-title font-semibold", "Who?" }
        p { class: "collapse-content text-sm",
          "Justin's Engineering Services, LLC. A company founded by me, Justin Forgue, to create the IoT/embedded ecosystem I've always wanted. A better option than what we currently have."
        }
      }
    }
  }
}
