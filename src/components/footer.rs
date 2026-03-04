use crate::Route;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::fa_brands_icons::{FaDiscord, FaGithub, FaYoutube};
use dioxus_free_icons::icons::ld_icons::{LdBird, LdChevronRight};
use web_time::{SystemTime, UNIX_EPOCH};

#[component]
pub fn Footer() -> Element {
  rsx! {
    footer { class: "footer footer-horizontal bg-base-300 border-t border-base-content/10 pt-8",
      div { class: "container mx-auto px-4 sm:px-6",
        div { class: "flex flex-col lg:flex-row justify-between items-start gap-12",
          div { class: "lg:w-2/5",
            div { class: "flex items-center space-x-3 mb-6",
              div { class: "size-12 rounded-full bg-linear-to-r from-teal-500 to-purple-600 flex items-center justify-center animate-glow",
                // Logo {}
                Icon {
                  icon: LdBird,
                  class: "size-7",
                  title: "Logo",
                }
              }
              span { class: "text-3xl font-bold",
                span { class: "text-teal-400", "PigIoT" }
              }
            }
            p { class: "text-base mb-8 leading-relaxed",
              "An open-source IoT platform built in Rust. Security, performance, and reliability by design. Join our community of builders creating the future of connected devices."
            }
            div { class: "flex space-x-4",
              a {
                class: "btn btn-circle bg-base-100 hover:bg-teal-900/50 hover:text-teal-400 transition-all duration-300",
                href: "https://github.com/justins-engineering",
                Icon {
                  icon: FaGithub,
                  class: "size-5",
                  title: "GitHub Logo",
                }
              }
              a {
                class: "btn btn-circle bg-base-100 hover:bg-purple-900/50 hover:text-purple-400 transition-all duration-300",
                href: "#",
                Icon {
                  icon: FaDiscord,
                  class: "size-5",
                  title: "Discord Logo",
                }
              }
              a {
                class: "btn btn-circle bg-base-100 hover:bg-red-900/50 hover:text-red-400 transition-all duration-300",
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
              h4 { class: "footer-title text-lg ml-6 text-teal-300",
                "Product"
              }
              ul { class: "space-y-3",
                li {
                  a {
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
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
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
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
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
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
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
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
              h4 { class: "footer-title text-lg ml-6 text-purple-300",
                "Company"
              }
              ul { class: "space-y-3",
                li {

                  Link {
                    class: "text-base hover:text-purple-400 transition-colors duration-300 flex items-center group",
                    to: Route::About {},
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
                    class: "text-base hover:text-purple-400 transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Blog"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-purple-400 transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Careers"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-purple-400 transition-colors duration-300 flex items-center group",
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
              h4 { class: "footer-title text-lg ml-6 text-teal-300",
                "Community"
              }
              ul { class: "space-y-3",
                li {
                  a {
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
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
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Discord"
                  }
                }
                li {
                  a {
                    class: "text-base hover:text-teal-400 transition-colors duration-300 flex items-center group",
                    href: "#",
                    Icon {
                      icon: LdChevronRight,
                      class: "mr-2 opacity-0 group-hover:opacity-100 transition-opacity",
                      title: "Chevron right",
                    }
                    "Stack Overflow"
                  }
                }
              }
            }
          }
        }
        aside { class: "mt-6 pt-2 border-t border-base-content/10 w-full flex flex-col md:flex-row justify-between items-center",
          p { class: "text-gray-500 mb-1 md:mb-0 text-sm",
            "© {1970 + (SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()/31556926)} Justin's Engineering Services, LLC."
          }
          div { class: "flex items-center space-x-6",
            a {
              class: "text-gray-500 hover:text-teal-400 transition-colors duration-300 text-sm",
              href: "#",
              "Privacy Policy"
            }
            a {
              class: "text-gray-500 hover:text-teal-400 transition-colors duration-300 text-sm",
              href: "#",
              "Terms of Service"
            }
            a {
              class: "text-gray-500 hover:text-teal-400 transition-colors duration-300 text-sm",
              href: "#",
              "Cookie Policy"
            }
          }
        }
      }
    }
  }
}
