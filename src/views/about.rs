use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
  rsx! {
    h1 { class: "m-4 text-4xl", "About Us" }
    div { class: "space-y-2",
      h2 { class: "text-3xl", "The Five Ws" }

      div { class: "w-full collapse collapse-arrow bg-base-100 border border-base-300",
        input { r#type: "radio", name: "about-accordion", checked: true }
        div { class: "collapse-title text-lg font-semibold", "Who?" }
        p { class: "collapse-content",
          "Justin's Engineering Services, LLC. A company founded by me, Justin Forgue, to create the IoT/embedded ecosystem I've always wanted. A better option than what we currently have."
        }
      }

      div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
        input { r#type: "radio", name: "about-accordion" }
        div { class: "collapse-title text-lg font-semibold", "What?" }
        p { class: "collapse-content",
          "An open source IoT platform built in Rust. Security, performance, and reliability by design. Freedom from vendor lock in and full control over your data!"
        }
      }

      div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
        input { r#type: "radio", name: "about-accordion" }
        div { class: "collapse-title text-lg font-semibold", "When?" }
        p { class: "collapse-content",
          "Now! And more to come in the future. Just wait until you see what we have cooking for cellular connectivity..."
        }
      }

      div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
        input { r#type: "radio", name: "about-accordion" }
        div { class: "collapse-title text-lg font-semibold", "Where?" }
        article { class: "collapse-content",
          p {
            "Everywhere! Thanks to Cloudflare's distributed network, containers, and data storage, your code execution and data operate local to you (or your device) for low latency and high availability."
          }
          br {}
          p {
            "All non time sensitive data (credentials, logs, etc.) are stored in a distributed Yugabyte (PostgreSQL) database."
          }
        }
      }

      div { class: "collapse collapse-arrow bg-base-100 border border-base-300",
        input { r#type: "radio", name: "about-accordion" }
        div { class: "collapse-title text-lg font-semibold", "Why?" }
        p { class: "collapse-content",
          "Have you ever found an easy solution that's cost prohibitive? Or a solution that's a great value but would require dedicating an entire team to setting up, integrating, and maintaining? This is the middle ground. Your data, under your control, made easy."
        }
      }
    }
  }
}
