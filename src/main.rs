use std::{thread::sleep, time::Duration};

use gstreamer::{glib::ObjectExt, prelude::*, ElementFactory, Pipeline, State};

fn main() {
    gstreamer::init().unwrap();

    let pipeline = Pipeline::new(None);

    {
        let queue = ElementFactory::make("queue", None).unwrap();

        dbg!(queue.ref_count()); // 1

        pipeline.add(&queue).unwrap();

        queue.set_state(State::Playing).unwrap();
        queue.set_state(State::Null).unwrap();

        pipeline.remove(&queue).unwrap();

        dbg!(queue.ref_count()); // 9
    }

    // At this point I'd expect to see in logs that element was disposed.

    println!("Sleeping ...");

    sleep(Duration::from_secs(1));

    println!("Dropping pipeline ...");

    drop(pipeline);

    // But element disposal happens only when pipeline is dropped

    println!("Sleeping ...");

    sleep(Duration::from_secs(1));

    println!("Exiting process ...");
}
