use crate::Route;
use chrono::Datelike;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::{FaDiscord, FaGithub, FaYoutube};
use dioxus_free_icons::icons::ld_icons::{LdBird, LdChevronRight};

#[component]
pub fn Footer() -> Element {
  rsx! {
    footer { class: "footer footer-horizontal bg-neutral border-t border-neutral-content/10 pt-8",
      div { class: "mx-auto px-4 sm:px-6",
        div { class: "flex flex-col lg:flex-row justify-between items-start gap-12",
          div { class: "lg:w-2/5",
            div { class: "flex items-center space-x-3 mb-6",
              div { class: "size-12 rounded-full bg-primary/60 flex items-center justify-center animate-glow",
                // Logo {}
                Icon {
                  icon: LdBird,
                  class: "size-7",
                  title: "Logo",
                }
              }
              span { class: "text-3xl font-bold",
                span { class: "text-secondary", "PigIoT" }
              }
            }
            p { class: "text-base mb-8 leading-relaxed",
              "An open-source IoT platform built in Rust. Security, performance, and reliability by design. Join our community of builders creating the future of connected devices."
            }
            div { class: "flex space-x-4",
              a {
                class: "btn btn-circle bg-base-100 hover:bg-[#0FBF3E] hover:text-[#0A241B] transition-all duration-300",
                href: "https://github.com/justins-engineering",
                Icon {
                  icon: FaGithub,
                  class: "size-5",
                  title: "GitHub Logo",
                }
              }
              a {
                class: "btn btn-circle bg-base-100 hover:bg-[#E0E3FF] hover:text-[#5865F2] transition-all duration-300",
                href: "https://discord.gg/W2vjtpeP",
                Icon {
                  icon: FaDiscord,
                  class: "size-5",
                  title: "Discord Logo",
                }
              }
              a {
                class: "btn btn-circle bg-base-100 hover:bg-[#212121] hover:text-[#FF0033] transition-all duration-300",
                href: "#",
                Icon {
                  icon: FaYoutube,
                  class: "size-5",
                  title: "Youtube Logo",
                }
              }
            }
          }
          div { class: "grid grid-cols-2 md:grid-cols-3 gap-8 lg:w-3/5",
            nav {
              h4 { class: "footer-title text-lg ml-6 text-primary", "Product" }
              ul { class: "space-y-3",
                li {
                  a {
                    class: "text-base hover:text-primary transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Features"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-primary transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Pricing"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-primary transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Documentation"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-primary transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "API Reference"
                  }
                }
              }
            }
            nav {
              h4 { class: "footer-title text-lg ml-6 text-secondary",
                "Company"
              }
              ul { class: "space-y-3",
                li {

                  Link {
                    class: "text-base hover:text-secondary transition-colors duration-300 flex items-center group",
                    to: Route::AboutUs {},
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "About Us"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-secondary transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Contact"
                  }
                }
              }
            }
            nav {
              h4 { class: "footer-title text-lg ml-6 text-primary", "Community" }
              ul { class: "space-y-3",
                li {
                  a {
                    class: "text-base hover:text-primary transition-colors duration-300 flex items-center group",
                    href: "https://github.com/justins-engineering",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "GitHub"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-primary transition-colors duration-300 flex items-center group",
                    href: "https://discord.gg/W2vjtpeP",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Discord"
                  }
                }
              }
            }
          }
        }
        aside { class: "mt-6 pt-2 border-t border-base-content/10 w-full flex flex-col md:flex-row justify-between items-center",
          p { class: "mb-1 md:mb-0 text-sm",
            "© {chrono::Utc::now().year()} Justin's Engineering Services, LLC."
          }
          div { class: "flex items-center space-x-6",
            a {
              class: "hover:text-primary transition-colors duration-300 text-sm",
              href: "#",
              "Privacy Policy"
            }
            a {
              class: "hover:text-primary transition-colors duration-300 text-sm",
              href: "#",
              "Terms of Service"
            }
            a {
              class: "hover:text-primary transition-colors duration-300 text-sm",
              href: "#",
              "Cookie Policy"
            }
          }
        }
      }
    }
  }
}
