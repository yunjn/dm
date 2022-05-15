// @Samuel
#![allow(unused)]

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use crate::data::*;
use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;
use std::env;
use std::fs;

#[derive(Default, NwgUi)]
pub struct DMApp {
    file_path: RefCell<Option<String>>,
    target: RefCell<Option<Target>>,
    loaded_image: RefCell<Option<nwg::Bitmap>>,

    #[nwg_control(size: (300, 200), position: (300, 200), title: "Transform", accept_files: true)]
    #[nwg_events( OnWindowClose: [nwg::stop_thread_dispatch()],OnInit:[DMApp::img_frame,DMApp::check_dir],OnFileDrop: [DMApp::load_text(SELF, EVT_DATA)] )]
    window: nwg::Window,

    #[nwg_control(size: (80, 20), position: (47, 150),collection: vec!["out","skl", "editor", "skl_txt"], selected_index: Some(0))]
    race_input: nwg::ComboBox<&'static str>,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "Edit(*.edit)|TXT(*.txt)|Any (*.*)")]
    dialog: nwg::FileDialog,

    #[nwg_control(size: (80, 30), position: (173, 150),text: "run")]
    #[nwg_events(OnButtonClick:[DMApp::execute])]
    start_btn: nwg::Button,

    #[nwg_control(size: (520, 346), position: (-108, -42))]
    #[nwg_events(OnImageFrameClick:[DMApp::open_file])]
    img: nwg::ImageFrame,

    #[nwg_resource]
    decoder: nwg::ImageDecoder,
}

impl DMApp {
    pub fn check_dir(&self) {
        fs::create_dir_all("assets/out/").unwrap_or_else(|why| {
            println!("! {:?}", why.kind());
        });
    }

    pub fn img_frame(&self) {
        let image: nwg::ImageSource = match self.decoder.from_filename("assets/bg.png") {
            Ok(img) => img,
            Err(_) => {
                println!("Could not read image!");
                return;
            }
        };

        let frame = match image.frame(0) {
            Ok(bmp) => bmp,
            Err(_) => {
                println!("Could not read image frame!");
                return;
            }
        };

        match frame.as_bitmap() {
            Ok(bitmap) => {
                let mut img = self.loaded_image.borrow_mut();
                img.replace(bitmap);
                self.img.set_bitmap(img.as_ref());
            }
            Err(_) => {
                println!("Could not convert image to bitmap!");
            }
        }
    }

    pub fn load_text(&self, data: &nwg::EventData) {
        let drop = data.on_file_drop();
        for file in drop.files() {
            let mut fp = self.file_path.borrow_mut();
            fp.replace(file);
        }
    }

    pub fn open_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }
        if self.dialog.run(Some(&self.window)) {
            if let Ok(directory) = self.dialog.get_selected_item() {
                let mut fp = self.file_path.borrow_mut();
                fp.replace(directory.into_string().unwrap());
            }
        }
    }

    pub fn update_target(&self) {
        let mut fp = self.file_path.borrow_mut();
        let mut tt = self.target.borrow_mut();

        let fp_res = fp.get_or_insert("none".to_string());
        let tt_res = tt.get_or_insert(Target::new());

        if fp_res != "none" && !fp_res.ends_with(".old") {
            let file_vec: Vec<_> = fp_res.split(".").collect();
            let params_update = match file_vec[file_vec.len() - 1] {
                "edit" => Target::from_editor(fp_res),
                "txt" => Target::from_params(fp_res),
                "pcapng" => Target::from_pcap(fp_res),
                _ => Target::new(),
            };

            tt.replace(params_update);
            let mut fp_res = fp_res.to_string();
            fp_res.push_str(".old");
            fp.replace(fp_res);
        }
    }

    pub fn execute(&self) {
        DMApp::update_target(&self);

        let mut fp = self.file_path.borrow_mut();
        let mut tt = self.target.borrow_mut();

        let fp_res = fp.get_or_insert("none".to_string());
        let tt_res = tt.get_or_insert(Target::new());

        if fp_res == "none" {
            return;
        }

        let fp_vec: Vec<_> = fp_res.split(".").collect();

        if cfg!(target_os = "windows") {
            let fp_vec: Vec<_> = fp_vec[fp_vec.len() - 3].split("\\").collect();
        } else if cfg!(target_os = "linux") {
            let fp_vec: Vec<_> = fp_vec[fp_vec.len() - 3].split("/").collect();
        }

        let split_flag = if cfg!(target_os = "windows") {
            "\\"
        } else {
            "/"
        };

        let fp_vec: Vec<_> = fp_vec[fp_vec.len() - 3].split(split_flag).collect();
        let file_name = fp_vec[fp_vec.len() - 1];
        // println!("{}", file_name);
        match self.race_input.selection() {
            Some(1) => tt_res.into_skl(file_name),
            Some(2) => tt_res.into_editor(file_name),
            Some(3) => tt_res.into_skl_txt(file_name),
            _ => (),
        };
    }
}

pub fn run() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("MSYHL").expect("Failed to set default font");
    let _app = DMApp::build_ui(Default::default()).expect("Failed to build UI");
    nwg::dispatch_thread_events();
}
