use crate::Route;
use crate::components::{Footer, Navbar, ThemeController};
use dioxus::prelude::*;

#[component]
pub fn Wrapper() -> Element {
  rsx! {
    Navbar {}
    main { class: "flex-1", Outlet::<Route> {} }
    ThemeController {}
    Footer {}
  }
}
