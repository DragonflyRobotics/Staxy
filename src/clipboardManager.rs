use futures::executor::block_on;
use std::{thread, time};
use std::sync::mpsc::Sender;
use arboard::Clipboard;

use rdev::{listen, Event, EventType, Key, simulate};

async fn clipboardManager(clipboard: &mut Clipboard, stack: &mut Vec<String>) {
    println!("MANAGER");
    // loop {
        clipboardUpdater(clipboard, stack).await.to_vec();
    //     println!("Clipboard: {:?}", stack);
    //     thread::sleep(std::time::Duration::from_millis(1000));
    // }

}
//
async fn clipboardUpdater(clipboard: &mut Clipboard, stack: &mut Vec<String>) -> Vec<String> {
    let mut stackReset = false;
    if stack.len() <= 0 {
        stackReset = true;
        stack.push(clipboard.get_text().unwrap());
    }
    while (stack.len() > 0 && clipboard.get_text().unwrap() == stack[stack.len() - 1]) {
        // println!("Waiting for clipboard to change...");
        thread::sleep(std::time::Duration::from_millis(100));
    }
    if stackReset {
        stack.remove(0);
    }
    stack.push(clipboard.get_text().unwrap());
    stack.to_vec()
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    // thread::sleep(delay);
    println!("Send done...");
}

fn paste() {
    send(&EventType::KeyPress(Key::ControlLeft));
    send(&EventType::KeyPress(Key::KeyV));
    send(&EventType::KeyRelease(Key::ControlLeft));
    send(&EventType::KeyRelease(Key::KeyV));
}

pub(crate) fn callback(event: Event, ctrlPressed: &mut bool, stack: &mut Vec<String>, clipboard: &mut Clipboard, sender: Sender<Vec<String>>) {
    sender.send(stack.clone()).expect("TODO: panic message");
    match event.event_type {
        EventType::KeyPress(Key::ScrollLock) => {
            // println!("ScrlLK");
            if stack.len() > 0 {
                clipboard.set_text(&stack[0]).unwrap();
                println!("PASTED: {:?}", &stack[0]);
                paste();
                stack.remove(0);
            }
        }

        EventType::KeyPress(Key::Escape) => {
            if ctrlPressed == &true
            {
                println!("CTRL + Esc pressed");
                println!("ClipboardESC: {:?}", stack);
                stack.retain(|x| false);
                println!("ClipboardESC: {:?}", stack);

            }
        }

        EventType::KeyPress(Key::KeyC) => {
            if ctrlPressed == &true
            {
                println!("CTRL + C pressed");
                block_on(clipboardUpdater(clipboard, stack));
                println!("Clipboard: {:?}", stack);
            }
        }

        EventType::KeyPress(Key::ControlLeft) => {
            *ctrlPressed = true;
        }
        EventType::KeyRelease(Key::ControlLeft) => {
            *ctrlPressed = false;
        }
        _ => {}
    }
}