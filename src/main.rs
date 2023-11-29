use std::env;

fn main() {
    let url = env::var("SENTRY_URL").unwrap();
    let _guard: sentry::ClientInitGuard = sentry::init((url, sentry::ClientOptions {
    release: sentry::release_name!(),
    ..Default::default()
    }));

     // Sentry will capture this
    panic!("Everything is on fire!");
}
