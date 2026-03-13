use crate::components::{SetSessionCookie, session_cookie_valid};
use crate::config::{DASHBOARD_URL, KRATOS_BROWSER_URL};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
use ory_kratos_client_wasm::apis::configuration::Configuration;
use unic_langid::langid;
use views::{
  About, AccountRecovery, Faq, Index, LoginFlow, PageNotFound, RecoveryFlow, RegisterFlow,
  ServerError, SessionInfo, Settings, SettingsFlow, SignIn, SignUp, VerificationFlow, Verify,
  Wrapper,
};

mod components;
mod config;
mod partials;
mod views;

const SESSION_COOKIE_NAME: &str = "session_expiry";

#[derive(Clone, Copy, Debug)]
struct Session {
  state: Signal<bool>,
}

trait Create {
  fn create() -> Configuration;
}

impl Create for Configuration {
  fn create() -> Configuration {
    Configuration {
      base_path: KRATOS_BROWSER_URL.to_owned(),
      user_agent: None,
      basic_auth: None,
      oauth_access_token: None,
      bearer_access_token: None,
      api_key: None,
    }
  }
}

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
  #[layout(Wrapper)]
    #[route("/")]
    Index {},
    #[route("/faq")]
    Faq {},
    #[route("/about")]
    About {},
    #[route("/session")]
    SessionInfo {},
    #[route("/sign-in")]
    SignIn {},
    #[route("/login?:flow")]
    LoginFlow { flow: String },
    #[route("/sign-up")]
    SignUp {},
    #[route("/registration?:flow")]
    RegisterFlow { flow: String },
    #[route("/verify")]
    Verify {},
    #[route("/verification?:flow")]
    VerificationFlow { flow: String },
    #[route("/my-settings")]
    Settings {},
    #[route("/settings?:flow")]
    SettingsFlow { flow: String },
    #[route("/account-recovery")]
    AccountRecovery {},
    #[route("/recovery?:flow")]
    RecoveryFlow { flow: String },
    #[route("/session/local?:state")]
    SetSessionCookie { state: bool },
    #[end_layout]
  #[route("/error?:id")]
  ServerError { id: String },
  #[route("/:..route")]
  PageNotFound { route: Vec<String> },
}

const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

// The server function at the endpoint "static_routes" will be called by the CLI to generate the list of static
// routes. You must explicitly set the endpoint to `"static_routes"` in the server function attribute instead of
// the default randomly generated endpoint.
// #[server(endpoint = "static_routes", output = server_fn::codec::Json)]
// async fn static_routes() -> Result<Vec<String>, ServerFnError> {
//   // The `Routable` trait has a `static_routes` method that returns all static routes in the enum
//   Ok(
//     Route::static_routes()
//       .iter()
//       .map(ToString::to_string)
//       .collect(),
//   )
// }

fn main() {
  // dioxus::LaunchBuilder::new()
  //   // Set the server config only if we are building the server target
  //   .with_cfg(server_only! {
  //       ServeConfig::builder()
  //           // Enable incremental rendering
  //           .incremental(
  //               dioxus::server::IncrementalRendererConfig::new()
  //                   // Store static files in the public directory where other static assets like wasm are stored
  //                   .static_dir(
  //                       std::env::current_exe()
  //                           .unwrap()
  //                           .parent()
  //                           .unwrap()
  //                           .join("public")
  //                   )
  //                   // Don't clear the public folder on every build. The public folder has other files including the wasm
  //                   // binary and static assets required for the app to run
  //                   .clear_cache(false)
  //           )
  //           .enable_out_of_order_streaming()
  //   })
  //   .launch(App);
  dioxus::launch(App);
}

#[component]
fn App() -> Element {
  use_init_i18n(|| {
    I18nConfig::new(langid!("en-US")).with_locale(Locale::new_static(
      langid!("en-US"),
      include_str!("../locales/en-US.ftl"),
    ))
  });

  use_effect(crate::components::set_lang);

  use_context_provider(|| Session {
    state: Signal::new(false),
  });

  let set_state = use_resource(move || async move { session_cookie_valid().await });
  (set_state)();

  rsx! {
    document::Link { rel: "stylesheet", href: MAIN_CSS }
    document::Link {
      rel: "icon",
      href: asset!("/assets/images/icon-light.ico"),
      sizes: "32x32",
    }
    document::Link {
      rel: "icon",
      href: asset!("/assets/images/icon-light.ico"),
      sizes: "32x32",
      media: "prefers-color-scheme: light",
    }
    document::Link {
      rel: "icon",
      href: asset!("/assets/images/icon-dark.ico"),
      sizes: "32x32",
      media: "prefers-color-scheme: dark",
    }
    document::Link {
      rel: "icon",
      r#type: "image/svg+xml",
      href: asset!("/assets/images/icon-light.svg"),
    }
    document::Link {
      rel: "icon",
      r#type: "image/svg+xml",
      href: asset!("/assets/images/icon-light.svg"),
      media: "prefers-color-scheme: light",
    }
    document::Link {
      rel: "icon",
      r#type: "image/svg+xml",
      href: asset!("/assets/images/icon-dark.svg"),
      media: "prefers-color-scheme: dark",
    }
    document::Meta {
      name: "description",
      content: "An open-source IoT platform built in Rust. Security, performance, and reliability by design.",
    }
    Router::<Route> {}
  }
}
