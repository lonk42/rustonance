

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg<T>(result: serenity::Result<T>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

