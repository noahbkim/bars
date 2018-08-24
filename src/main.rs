extern crate piston_window as piston;
extern crate libpulse_binding as pulse;


struct Visualizer<'v> {
    context: pulse::Context,
    stream: Option<pulse::Stream>
}


impl<'v> Visualizer<'v> {

    /** Create a new visualizer with a context. */
    fn new(mainloop: &'v mut pulse::PulseAudioMainloop) -> Visualizer<'v> {

        /* Create a new audio context.
         * https://docs.rs/libpulse-binding/1.2.2/libpulse_binding/mainloop/standard/index.html */
        let mut properties = pulse::proplist::Proplist::new().unwrap();
        properties.sets(pulse::proplist::properties::APPLICATION_NAME, "Bars").unwrap();
        let mut context = pulse::context::Context::new_with_proplist(
            mainloop.get_api(),
            "Bars Audio Context",
            &properties).unwrap();

        /* Create the visualizer */
        let visualizer = Vizualizer { context };
        return visualizer;

    }

    /** Wait for the context to connect. */
    fn connect(&mut self) {

        /** Connect the context. */
        self.context.connect(None, pulse::context::flags::NOFLAGS, None).unwrap();

        /** Wait for the mainloop and context to be ready. */
        loop {
            match mainloop.iterate(false) {
                pulse::mainloop::standard::IterateResult::Quit(_) |
                pulse::mainloop::standard::IterateResult::Err(_) => {
                    eprintln!("Mainloop iteration failed!");
                    return;
                },
                pulse::mainloop::standard::IterateResult::Success(_) => {}
            }
            match self.context.get_state() {
                pulse::context::State::Failed |
                pulse::context::State::Terminated => {
                    eprintln!("Context state failed!");
                    return;
                },
                pulse::context::State::Ready => { break; },
                _ => {},
            }
        }

    }

    /** Setup stream recording. */
    fn record(&mut self, monitor: &str) {

        /** Create the stream specification. */
        let spec = pulse::sample::Spec {
            format: pulse::sample::SAMPLE_S16NE,
            channels: 2,
            rate: 44100,
        };

        /** Create the stream handle. */
        let mut stream = pulse::stream::Stream::new(
            &mut context,
            "Bars",
            &spec,
            None).unwrap();

        /** Record. */
        stream.set_read_callback(move |stream, size| {
            self.read(stream, size);
        });
        stream.connect_record(Some(monitor), None, None);
        self.stream = Some(stream);

    }

    /** Called when there is data ready for reading in the stream. */
    fn read(&mut self, mut stream: pulse::stream::Stream, size: size_t) {

    }

}


fn draw(context: piston::Context, graphics: &mut piston::G2d) {
    clear([0.0; 4], graphics);
    rectangle([1.0, 0.0, 0.0, 1.0],
              [10.0, 10.0, 110.0, 110.0],
              context.transform,
              graphics);
}


fn main() {
    let settings = WindowSettings::new("Bars", [640, 480]).exit_on_esc(true);
    let mut window: piston::PistonWindow = settings.build().unwrap();
    while let Some(event) = window.next() {
        window.draw_2d(&event, &draw);
    }
}
