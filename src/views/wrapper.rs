use crate::Route;
use crate::components::{Alert, Footer, Navbar, ThemeController};
use dioxus::prelude::*;

#[component]
pub fn Wrapper() -> Element {
  rsx! {
    Navbar {}
    Alert {}
    main { Outlet::<Route> {} }
    ThemeController {}
    Footer {}
  }
}
