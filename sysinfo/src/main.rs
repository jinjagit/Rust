use sysinfo::{ProcessorExt, SystemExt};
// use sysinfo::{ProcessExt, SystemExt};
use std::{thread, time::Duration};
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc, WidgetExt};

const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<SysState> = LocalizedString::new("sysinfo");

#[derive(Clone, Data, Lens)]
struct SysState {
    val: String,
}

impl SysState {
    fn change(&mut self) {
        loop {
            self.val = String::from("bibble!");

            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn main () {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let mut initial_state = SysState {
        val: "---".into(),
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    let mut system = sysinfo::System::new_all();

    // loop {
    //     // First we update all information of our system struct.
    //     system.refresh_all();

    //     let mut usages: Vec<u8> = vec![];

    //     for processor in system.get_processors() {
    //         let usage = processor.get_cpu_usage() as u8;

    //         usages.push(usage);
    //     }

    //     println!("{}%", usages[0]);

    //     thread::sleep(Duration::from_secs(1));
    // }

    initial_state.change();

    // println!("system: {:#?}", system);
    // println!("components: {:#?}", system.get_components());

    // // Then let's print the temperature of the different components:
    // for component in system.get_components() {
    //     println!("{:?}", component);
    // }

    // And then all disks' information:
    // for disk in system.get_disks() {
    //     println!("{:?}", disk);
    // }

    // // And finally the RAM and SWAP information:
    // println!("total memory: {} KB", system.get_total_memory());
    // println!("used memory : {} KB", system.get_used_memory());
    // println!("total swap  : {} KB", system.get_total_swap());
    // println!("used swap   : {} KB", system.get_used_swap());
}

fn build_root_widget() -> impl Widget<SysState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &SysState, _env: &Env| format!("{}%", data.val));
    // a textbox that modifies `name`.

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column()
        .with_child(label);
        // .with_spacer(VERTICAL_WIDGET_SPACING)
        // .with_child(textbox);

    // center the two widgets in the available space
    Align::centered(layout)
}