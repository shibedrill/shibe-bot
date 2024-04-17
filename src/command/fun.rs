
use rand::Rng;
use rand::seq::SliceRandom;

use crate::Context;
use crate::Error;

/// MAKE HER BLEAT
#[poise::command(slash_command)]
pub async fn bleat(ctx: Context<'_>) -> Result<(), Error> {

    let sounds: Vec<&str> = [
        "rah",
        "grr",
        "bah",
        "bleat",
        "yippee",
        "woohoo",
        "huh",
        "wha",
        "buh",
        "whuh",
        "oh",
        "yeag",
        "yeab",
        "yeas",
        "mweee",
        "mweh",
        "bwah",
    ].to_vec();
    
    let faces: Vec<&str> = vec![
        "xp",
        "x3",
        ":3",
        ":3c",
        ";3",
        ";3c",
        "=p",
    ].to_vec();

    let exclamation: Vec<&str> = vec![
        "!",
        "1",
        "?",
        "-",
        ",",
        ".",
    ].to_vec();

    fn modify_sound(input: &str) -> String {

        // Create an RNG
        let mut rng = rand::thread_rng();
        let mut output;

        // Possibly uppercase or lowercase
        output = match rng.gen_bool(0.5) {
            true => input.to_ascii_uppercase(),
            false => input.to_ascii_uppercase(),
        };

        let n = rng.gen_range(0..3);
        let mut new_string = "*".repeat(n);
        new_string.push_str(&output);
        new_string.push_str(&"*".repeat(n));
        output = new_string;

        // Return modified string
        output
    }

    let sound: String = match rand::thread_rng().gen_range(0..3) {
        0 => {
            let count = rand::thread_rng().gen_range(1..3);
            let mut new_sound: String = String::new();
            for _i in 0..count {
                new_sound.push_str(&modify_sound(sounds.choose(&mut rand::thread_rng()).unwrap()));
                new_sound.push(' ');
            }
            new_sound
        }
        1 => {
            format!("am so {}ing awesome", modify_sound(sounds.choose(&mut rand::thread_rng()).unwrap()))
        }
        2 => {
            format!("feel so {} like a {} machine", modify_sound(sounds.choose(&mut rand::thread_rng()).unwrap()), modify_sound(sounds.choose(&mut rand::thread_rng()).unwrap()))
        }
        3 => {
            format!("do it {}", modify_sound(sounds.choose(&mut rand::thread_rng()).unwrap()))
        }
        _ => unreachable!(),

    };
    
    ctx.say(sound) // This unwrap will never return None. We promise this slice will always be non empty.
    .await?;
    info!("Executed command `bleat` successfully");
    Ok(())
}
