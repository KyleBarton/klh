extern crate cursive;

use std::env;
use std::fs::File;
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;
use cursive::traits::*;
use cursive::event::{Event, Key};
use cursive::views::{Dialog, EditView, TextArea, OnEventView};
use cursive::Cursive;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("Can only take one input file as a parameter!");
    }
    // Ok this is fine, now let's actually open into a buffer
    let file_name = &args[1];
	let path = Path::new(&file_name);
	let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("Couldn't open file because {}!", why.to_string()),
        Ok(file) => file
    };
    let mut contents = String::new();
	// Welp, anything past this is gonna need a view
    match file.read_to_string(&mut contents) {
        Err(why) => panic!("couldn't read {}: {}", display,
                                                   why.to_string()),
        Ok(_) => print!("Nice, sending the contents of {} to the view", display),
    };
    //view work
    //CURSIVE BB
    let mut siv = cursive::default();

    siv.add_layer(
        Dialog::new()
            .title(file_name)
            .padding_lrtb(1, 1, 1, 0)
            .content(TextArea::new().content(contents).with_name("text"))
            .button("Ok", Cursive::quit)
            .full_screen(),
     );
    siv.add_layer(Dialog::info("Hint: press Ctrl-F to find in text!"));

    //not really sure what move does, but I'm not borrowing right and this is a PoC anyway
    //didn't work :( I need to understand how I can let it borrow some contents
//    siv.add_global_callback(Event::CtrlChar('s'), |s| {
//        //save contents here.
//        let text = s
//            .call_on_name("edit", |view: &mut EditView| {
//                view.get_content()
//        }).unwrap();
//        match save(&text, &path) {
//            Err(why) => panic!("Couldn't save {}!", "filename"),
//            Ok(_) => s.add_layer(Dialog::info(format!("Sucessfully saved {}", "&file_name")).button("Ok", Cursive::quit))
//
//        };
//    });

    siv.add_global_callback(Event::CtrlChar('f'), |s| {
        // When Ctrl-F is pressed, show the Find popup.
        // Pressing the Escape key will discard it.
        s.add_layer(
            OnEventView::new(
                Dialog::new()
                    .title("Find")
                    .content(
                        EditView::new()
                            .on_submit(find)
                            .with_name("edit")
                            .min_width(10),
                    )
                    .button("Ok", |s| {
                        let text = s
                            .call_on_name("edit", |view: &mut EditView| {
                                view.get_content()
                            })
                            .unwrap();
                        find(s, &text);
                    })
                    .dismiss_button("Cancel"),
            )
            .on_event(Event::Key(Key::Esc), |s| {
                s.pop_layer();
            }),
        )
    });

    siv.run();

}

fn find(siv: &mut Cursive, text: &str) {
    // First, remove the find popup
    siv.pop_layer();

    let res = siv.call_on_name("text", |v: &mut TextArea| {
        // Find the given text from the text area content
        // Possible improvement: search after the current cursor.
        if let Some(i) = v.get_content().find(text) {
            // If we found it, move the cursor
            v.set_cursor(i);
            Ok(())
        } else {
            // Otherwise, return an error so we can show a warning.
            Err(())
        }
    });

    if let Some(Err(())) = res {
        // If we didn't find anything, tell the user!
        siv.add_layer(Dialog::info(format!("`{}` not found", text)));
    }
}

fn save(text: &str, file_path: &Path) -> Result<String, Error> {
    return Ok(String::from("Ok, did it"));
}
