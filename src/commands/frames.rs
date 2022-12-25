// use std::fs::File;
// use std::io::Write;
use std::fs;
use std::path::Path;
use std::string::String;
use crate::{Context, Error};
use crate::{Frames, MoveAliases, ImageLinks, Nicknames, check};

/// Displays the frame data of a move along with an image.
#[poise::command(prefix_command, slash_command, aliases("f"))]
pub async fn frames(
    ctx: Context<'_>,
    #[description = "Character name or nickname."] character_arg: String,
    #[description = "Move name, input or alias."] mut character_move_arg: String,
) -> Result<(), Error> {

    // This will store the full character name in case user input was an alias
    let mut character_arg_altered = String::new();
    // Flags that will be used for logic to determine output
    let mut character_found = false;
    let mut move_found = false;

    // Checking if character user argument is correct
    if let Some(error_msg) = check::correct_character_arg(&character_arg){
        ctx.say(&error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
    }

    // Checking if move user argument is correct
    if let Some(error_msg) = check::correct_character_move_arg(&character_move_arg){
        ctx.say(&error_msg).await?;
        print!("\n");
        panic!("{}", error_msg);
    }

    // Checking if data folder exists
    if let Some(error_msg) = check::data_folder_exists(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Checking if character folders exist
    if let Some(error_msg) = check::character_folders_exist(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }
    
    // Checking if character jsons exist
    if let Some(error_msg) = check::character_jsons_exist(false) {
        ctx.say(&error_msg.replace("'", "`")).await?;
        print!("\n");
        panic!("{}", error_msg.replace("\n", " "));
    }

    // Initializing variables for the embed
    // They must not be empty cause then the embed wont send
    let mut image_embed = "https://raw.githubusercontent.com/yakiimoninja/baiken/main/data/images/no_image.png".to_string();


    // Reading the nicknames json
    let data_from_file = fs::read_to_string("data/nicknames.json")
        .expect("\nFailed to read 'nicknames.json' file.");
    
    // Deserializing from nicknames json
    let vec_nicknames = serde_json::from_str::<Vec<Nicknames>>(&data_from_file).unwrap();

    // Iterating through the nicknames.json character entries
    for x_nicknames in vec_nicknames {

        // If user input is part of a characters full name or the full name itself
        // Then pass the full and correct charactet name to the new var 'character_arg_altered'
        if x_nicknames.character.to_lowercase().replace("-", "").contains(&character_arg.to_lowercase()) == true ||
        x_nicknames.character.to_lowercase().contains(&character_arg.to_lowercase()) == true {
            
            character_found = true;
            character_arg_altered = x_nicknames.character.to_owned();
            break;
        }

        // Iterating through the nicknames.json nickname entries
        for y_nicknames in x_nicknames.nicknames {

            // If user input equals a character nickname then pass the full character name
            // To the new variable 'character_arg_altered'
            if y_nicknames.to_lowercase() == character_arg.to_lowercase().trim() {

                character_found = true;
                character_arg_altered = x_nicknames.character.to_owned();
                break;
            }   
        }
    }

    // If user input isnt the full name, part of a full name or a nickname
    // Error out cause requested character was not found in the json
    if character_found == false {
        let error_msg= &("Character `".to_owned() + &character_arg + "` was not found!");
        ctx.say(error_msg).await?;
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    // Reading the character json
    let char_file_path = "data/".to_owned() + &character_arg_altered + "/" + &character_arg_altered + ".json";
    let char_file_data = fs::read_to_string(char_file_path)
        .expect(&("\nFailed to read '".to_owned() + &character_arg + ".json" + "' file."));
    
    // Deserializing from character json
    let move_frames = serde_json::from_str::<Vec<Frames>>(&char_file_data).unwrap();            
    
    println!("\nCommand: '{} {} {}'", ctx.command().qualified_name, character_arg, character_move_arg);
    println!("Succesfully read '{}.json' file.", character_arg_altered);
    

    // Checking if aliases for this characters moves exist
    let aliases_path = "data/".to_owned() + &character_arg_altered + "/aliases.json";
    if Path::new(&aliases_path).exists() == true {
        
        // Reading the aliases json
        let aliases_data = fs::read_to_string(&aliases_path)
            .expect(&("\nFailed to read '".to_owned() + &aliases_path + "' file."));
        
        // Deserializing the aliases json
        let aliases_data = serde_json::from_str::<Vec<MoveAliases>>(&aliases_data).unwrap();

        for alias_data in aliases_data {
            for x_aliases in alias_data.aliases {
                
                // If the requested argument (character_move) is an alias for any of the moves listed in aliases.json
                // Change the given argument (character_move) to the actual move name instead of the alias
                if x_aliases.to_lowercase().trim().replace(".", "")
                == character_move_arg.to_lowercase().trim().replace(".", "") {
                    character_move_arg = alias_data.input.to_string();
                }
            }
        }
    }

    // Reading images.json for this character
    let image_links = fs::read_to_string(&("data/".to_owned() + &character_arg_altered + "/images.json"))
        .expect(&("\nFailed to read 'data/".to_owned() + &character_arg_altered + "'/images.json' file."));

    // Deserializing images.json for this character
    let image_links= serde_json::from_str::<Vec<ImageLinks>>(&image_links).unwrap();
    

    for mframes in move_frames {
        
        // Iterating through the moves of the json file to find the move requested
        if mframes.input.to_string().to_lowercase().replace(".", "") 
        == character_move_arg.to_string().to_lowercase().replace(".", "")
        || mframes.name.to_string().to_lowercase().contains(&character_move_arg.to_string().to_lowercase()) == true {
            
            
            move_found = true;
            println!("Succesfully read move '{}' in '{}.json' file.", &mframes.input.to_string(), character_arg_altered);

            let content_embed = "https://dustloop.com/wiki/index.php?title=GGST/".to_owned() + &character_arg_altered + "/Frame_Data";
            let title_embed = "Move: ".to_owned() + &mframes.input.to_string();

            // Checking if the respective data field in the json file is empty
            // If they aren't empty, the variables initialized above will be replaced
            // With the corresponind data from the json file
            // Otherwise they will remain as '-'
            for img_links in image_links {
                // Iterating through the image.json to find the move's image links
                if mframes.input == img_links.input {
                    if img_links.move_img.is_empty() == false {
                        image_embed = img_links.move_img.to_string();
                        break;
                    }
                }
            }

            // Debugging prints
            // println!("{}", content_embed);
            // println!("{}", image_embed);
            // println!("{}", title_embed);
            // println!("{}", damage_embed);
            // println!("{}", guard_embed);
            // println!("{}", invin_embed);
            // println!("{}", startup_embed);
            // println!("{}", hit_embed);
            // println!("{}", block_embed);
            // println!("{}", active_embed);
            // println!("{}", recovery_embed);
            // println!("{}", counter_embed);

            // New version notification
            //ctx.say(r"Baiken enters season 2 with a new version 0.5.0!
//As always a link to the patch notes is below.
//__<https://github.com/yakiimoninja/baiken/releases>__").await?;

            // Sending the data as an embed
            let _msg = ctx.send(|m| {
                m.content(&content_embed);
                m.embed(|e| {
                    e.color((140,75,64));
                    e.title(&title_embed);
                    //e.description("This is a description");
                    e.image(&image_embed);
                    e.fields(vec![
                        ("Damage", &mframes.damage.to_string(), true),
                        ("Guard", &mframes.guard.to_string(), true),
                        ("Invinciblity", &mframes.invincibility.to_string(), true),
                        ("Startup", &mframes.startup.to_string(), true),
                        ("Active", &mframes.active.to_string(), true),
                        ("Recovery", &mframes.recovery.to_string(), true),
                        ("On Hit", &mframes.hit.to_string(), true),
                        ("On Block", &mframes.block.to_string(), true),
                        ("Level", &mframes.level.to_string(), true),
                        ("Risc Gain", &mframes.riscgain.to_string(), true),
                        ("Scaling", &mframes.scaling.to_string(), true),
                        ("Counter", &mframes.counter.to_string(), true)]);
                    //e.field("This is the third field", "This is not an inline field", false);
                    e
                });
                m
            }).await;

            break;
        }
    }

    // Error message cause given move wasnt found in the json
    if character_found == true && move_found == false {
        let error_msg= &("Move `".to_owned() + &character_move_arg + "` was not found!" + "\nView moves of a character by executing `/moves`.\nView aliases of a character by executing `/aliases`.");
        ctx.say(error_msg).await?;
        // Console error print 
        let error_msg= &("Move `".to_owned() + &character_move_arg + "` was not found!");
        print!("\n");
        panic!("{}", error_msg.replace("`", "'"));
    }

    Ok(())
}
