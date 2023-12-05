use std::env;

 fn main() {
    let url = env::var("SENTRY_URL").unwrap();
    let _guard: sentry::ClientInitGuard = sentry::init((url, sentry::ClientOptions {
    release: sentry::release_name!(),
    ..Default::default()
    }));

     // Sentry will capture this

     let mut count = 0u32;

     loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);  
        sentry::capture_message("Something went wrong", sentry::Level::Info);

        if count == 50000 {
            println!("OK, that's enough");
            panic!("Party party party - Everything is on fire!");

            // Exit this loop
            //break;
        }

    }
}
 