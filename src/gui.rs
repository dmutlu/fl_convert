extern crate native_windows_derive as nwd;
use crate::file_handling::*;

use bstr::{ByteSlice, BString};
use nwd::NwgUi;
use nwg::NativeUi;
use std::path::{PathBuf, Path};
use directories::UserDirs;

#[derive(Default, NwgUi)]
pub struct FLSaveConvert {
    #[nwg_control(size: (400, 300), position: (400, 150), title: "FL Save Convert")]
    #[nwg_events( OnWindowClose: [FLSaveConvert::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, max_row: Some(5), max_column: Some(5) )]
    main_layout: nwg::GridLayout,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "FL(*.fl)|TXT(*.txt)|Any (*.*)")]
    dialog: nwg::FileDialog,

    #[nwg_control(text: "Open", focus: true)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 0)]
    #[nwg_events(OnButtonClick: [FLSaveConvert::open_file])]
    open_btn: nwg::Button,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 0, col_span: 4)]
    file_name: nwg::TextInput,

    #[nwg_resource(family: "Consolas", size: 18)]
    console_font: nwg::Font,

    #[nwg_control(font: Some(&data.console_font), readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 1, col_span: 5, row_span: 4)]
    msg_box: nwg::RichTextBox,
}

impl FLSaveConvert {
    fn open_file(&self) {
        if let Some(user_dirs) = UserDirs::new() {
            let user_docs: &Path = user_dirs.document_dir().unwrap();
            let sp_path: PathBuf = user_docs.join(r"My Games\Freelancer\Accts\SinglePlayer");

            if let Some(dir) = sp_path.to_str() {
                self.dialog.clear_client_data();
                let nwg_default_dir = self.dialog.set_default_folder(dir);
                
                // If FL save dir exists ok, otherwise set to Documents root.
                if let Ok(..) = nwg_default_dir{
                    nwg_default_dir.expect("Failed to set default folder.");
                } else {
                    self.dialog.set_default_folder(user_docs.to_str().unwrap()).expect("Failed to set default folder.");
                }
            }    
        
            if self.dialog.run(Some(&self.window)) {
                // Set file name text input to blank.
                self.file_name.set_text("");
                if let Ok(directory) = self.dialog.get_selected_item() {
                    let orig_path: PathBuf = directory.try_into().expect("Failed to convert directory to PathBuf.");
                    let fl_name = orig_path.file_name();
                    //let dir = directory.into_string().unwrap();
                    let dir = orig_path.to_str().unwrap();
                    let file_name = &dir;
                    // Set file name text input to FL save path.
                    self.file_name.set_text(&dir);
                           
                    self.msg_box.set_text("[INFO]: Reading Freelancer save.\r\n");
                    if let Ok(fl_save) = read_save(file_name) {
                        self.msg_box.append("[INFO]: Read successful.\r\n");
                        self.msg_box.append("[INFO]: Backing up your Freelancer save.\r\n");

                        //println!("{:?}", fl_save);
                        println!("{:?}", orig_path.parent());
                        println!("{:?}", fl_name);
                        backup_save(&orig_path);

                    } else {
                        self.msg_box.append("[ERROR]: Save file may be empty or corrupt.\r\n");
                    };
                    
                }
            }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

/*impl ImageDecoderApp {

    fn open_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog.set_default_folder(d).expect("Failed to set default folder.");
            }
        }
        
        if self.dialog.run(Some(&self.window)) {
            self.file_name.set_text("");
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.file_name.set_text(&dir);
                self.read_file();
            }
        }
    }

    fn read_file(&self) {
        println!("{}", self.file_name.text());
        let image = match self.decoder.from_filename(&self.file_name.text()) {
            Ok(img) => img,
            Err(_) => { println!("Could not read image!"); return; }
        };
        
        println!("Frame count: {}", image.frame_count());
        println!("Format: {:?}", image.container_format());

        let frame = match image.frame(0) {
            Ok(bmp) => bmp,
            Err(_) => { println!("Could not read image frame!"); return; }
        };

        println!("Resolution: {:?}", frame.resolution());
        println!("Size: {:?}", frame.size());

        // Create a new Bitmap image from the image data
        match frame.as_bitmap() {
            Ok(bitmap) => {
                let mut img = self.loaded_image.borrow_mut();
                img.replace(bitmap);
                self.img.set_bitmap(img.as_ref());
            },
            Err(_) => { println!("Could not convert image to bitmap!"); }
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}*/

pub(crate) fn main_wnd() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = FLSaveConvert::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}