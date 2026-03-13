// Ory Kratos public endpoint for browser flows
pub const KRATOS_BROWSER_URL: &str = match option_env!("KRATOS_BROWSER_URL") {
  Some(url) => url,
  None => "http://127.0.0.1:4433", // A safe local fallback
};

// Path for dashboard (dovecote), a seperate app on the same domain
pub const DASHBOARD_URL: &str = match option_env!("DASHBOARD_URL") {
  Some(url) => url,
  None => "http://127.0.0.1:8787",
};
