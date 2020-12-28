use gstreamer::gst_element_error;
use gstreamer::prelude::*;

use anyhow::Error;
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
#[display(fmt = "Received error from {}: {} (debug: {:?})", src, error, debug)]
struct ErrorMessage {
    src: String,
    error: String,
    debug: Option<String>,
}

// This is the callback function called by the demuxer, when a new stream was detected.
fn handle_demux_pad_added(
    demuxer: &gstreamer::Element,
    demux_src_pad: &gstreamer::Pad,
    queue: &gstreamer::Element,
    muxer: &gstreamer::Element,
) {
    // Pipe the detected stream through our multiqueue to the muxer.
    // For that, we need to request a sink pad that fits our needs.
    let link_to_muxer = || -> Result<(), Error> {
        let queue_sink_pad = queue
            .get_request_pad("sink_%u")
            .expect("If this happened, something is terribly wrong");
        demux_src_pad.link(&queue_sink_pad).unwrap();
        // Now that we requested a sink pad fitting our needs from the multiqueue,
        // the multiqueue automatically created a fitting src pad on the other side.
        // sink and src pad are linked internally, so we can iterate this internal link chain
        // and dependably retrieve the src pad corresponding to our requested sink pad.
        let queue_src_pad = queue_sink_pad
            .iterate_internal_links()
            .next()?
            .expect("Failed to iterate the multiqueue's internal link chain");

        // Link the multiqueue's output for this stream to the matroskamuxer.
        // For that, we request an appropriate pad at the muxer, that fits our needs.
        let muxer_sink_pad = muxer
            .get_compatible_pad(&queue_src_pad, None)
            .expect("Aww, you found a format that matroska doesn't support!");
        queue_src_pad.link(&muxer_sink_pad).unwrap();

        Ok(())
    };

    if let Err(err) = link_to_muxer() {
        gst_element_error!(
            demuxer,
            gstreamer::LibraryError::Failed,
            ("Failed to insert sink"),
            ["{}", err]
        );
    }
}

//input_file = "file:///C:/Users/test.mp4";
//output_file = "/home/test.flv";
pub fn conversion_video(input_file: &str, output_file: &str) -> Result<(), Error> {
    gstreamer::init().unwrap();
    let pipeline = gstreamer::Pipeline::new(None);
    let src = gstreamer::Element::make_from_uri(gstreamer::URIType::Src, input_file, None)
        .expect("We do not seem to support this uri");
    let typefinder =
        gstreamer::ElementFactory::make("typefind", None).unwrap();
    let queue =
        gstreamer::ElementFactory::make("multiqueue", None).unwrap();
    let muxer = gstreamer::ElementFactory::make("matroskamux", None).unwrap();
    let sink =
        gstreamer::ElementFactory::make("filesink", None).unwrap();

    sink.set_property("location", &output_file)
        .expect("setting location property failed");
    // Increase the queue capacity to 100MB to avoid a stalling pipeline
    queue
        .set_property("max-size-buffers", &0u32)
        .expect("changing capacity of multiqueue failed");
    queue
        .set_property("max-size-time", &0u64)
        .expect("changing capacity of multiqueue failed");
    queue
        .set_property("max-size-bytes", &(1024u32 * 1024 * 100))
        .expect("changing capacity of multiqueue failed");

    pipeline
        .add_many(&[&src, &typefinder, &queue, &muxer, &sink])
        .expect("failed to add elements to pipeline");

    src.link(&typefinder).unwrap();
    muxer.link(&sink).unwrap();

    let pipeline_clone = pipeline.clone();
    let typefinder_clone = typefinder.clone();
    typefinder
        .connect("have-type", false, move |values| {
            let (pipeline, typefinder) = (&pipeline_clone, &typefinder_clone);

            // Use the detected format to select between a small set of supported demuxers
            // Hint: This should probably never be done manually, for stuff like this,
            // the decodebin should be used, that does this stuff automatically and handles
            // much more corner-cases. This is just for the sake of being an example.
            let caps = values[2]
                .get::<gstreamer::Caps>()
                .expect("typefinder \"have-type\" signal values[2]")
                .expect("typefinder \"have-type\" signal values[2]: no `caps`");
            let format_name = caps
                .get_structure(0)
                .expect("Failed to get format name")
                .get_name();

            let demuxer = match format_name {
                "video/x-matroska" | "video/webm" => {
                    gstreamer::ElementFactory::make("matroskademux", None).expect("matroskademux missing")
                }
                "video/quicktime" => {
                    gstreamer::ElementFactory::make("qtdemux", None).expect("qtdemux missing")
                }
                _ => {
                    eprintln!("Sorry, this format is not supported by this example.");
                    std::process::exit(-1);
                }
            };

            // We found a supported format and created the appropriate demuxer -> link it
            pipeline
                .add(&demuxer)
                .expect("Failed to build remux pipeline");
            // We simply keep the typefinder element and pipe the data through it.
            // Removing is non-trivial since it started reading data from the pipeline
            // that the next element (the format specific demuxer) would need.
            typefinder
                .link(&demuxer)
                .expect("Failed to build remux pipeline");

            let queue_clone = queue.clone();
            let muxer_clone = muxer.clone();
            demuxer.connect_pad_added(move |demux, src_pad| {
                handle_demux_pad_added(demux, src_pad, &queue_clone, &muxer_clone)
            });
            demuxer
                .sync_state_with_parent()
                .expect("Failed to build remux pipeline");

            None
        })
        .expect("Failed to register have-type signal of typefind");

    pipeline.set_state(gstreamer::State::Playing).unwrap();

    let bus = pipeline
        .get_bus()
        .expect("Pipeline without bus. Shouldn't happen!");

    for msg in bus.iter_timed(gstreamer::CLOCK_TIME_NONE) {
        use gstreamer::MessageView;

        match msg.view() {
            MessageView::Eos(..) => break,
            MessageView::Error(err) => {
                pipeline.set_state(gstreamer::State::Null).unwrap();

                return Err(ErrorMessage {
                    src: msg
                        .get_src()
                        .map(|s| String::from(s.get_path_string()))
                        .unwrap_or_else(|| String::from("None")),
                    error: err.get_error().to_string(),
                    debug: err.get_debug(),
                }
                    .into());
            }
            MessageView::StateChanged(s) => {
                println!(
                    "State changed from {:?}: {:?} -> {:?} ({:?})",
                    s.get_src().map(|s| s.get_path_string()),
                    s.get_old(),
                    s.get_current(),
                    s.get_pending()
                );
            }
            _ => (),
        }
    }

    pipeline.set_state(gstreamer::State::Null).unwrap();

    Ok(())
}

#[test]
fn conversion_video_test() {
    use crate::tool::file_tool::now_dir_path;
    let mut test_path = String::from("file:///");
    let tmp_dir = now_dir_path();
    test_path.push_str(tmp_dir.as_str());
    test_path.push_str("/test/input.mp4");
    println!("{}", test_path);
    let result = conversion_video(test_path.as_str(), format!("{}{}", tmp_dir, "/test/output.mp4").as_str());
    assert_eq!(result.is_ok(), true);
}