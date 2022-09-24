mod file_handling;
mod fl_core;

extern crate native_windows_derive as nwd;
use crate::file_handling::*;
use crate::fl_core::*;

use nwd::NwgUi;
use nwg::NativeUi;
use std::path::{PathBuf, Path};
use std::thread;
use directories::UserDirs;

#[derive(Default, NwgUi)]
pub struct AboutWindow {
    #[nwg_control(flags: "WINDOW|VISIBLE", size: (300, 200), center: true, title: "About")]
    #[nwg_events( OnWindowClose: [AboutWindow::close])]
    about_window: nwg::Window,
}

impl AboutWindow {
    fn popup(sender: nwg::NoticeSender) {
        thread::spawn(move || {
            // Create the UI just like in the main function
            let _app = AboutWindow::build_ui(Default::default()).expect("Failed to build UI");
            nwg::dispatch_thread_events();

            // Notice the main thread that the dialog completed
            sender.notice();
        });
    }

    fn close(&self) {
        nwg::stop_thread_dispatch();
    }
}

#[derive(Default, NwgUi)]
pub struct FLSaveConvert {
    #[nwg_control(flags: "WINDOW|MINIMIZE_BOX|VISIBLE", size: (500, 320), center: true, title: "FL Save Convert")]
    #[nwg_events( OnWindowClose: [FLSaveConvert::exit])]
    window: nwg::Window,

    #[nwg_layout(parent: window, max_row: Some(5), max_column: Some(5))]
    main_layout: nwg::GridLayout,

    // File Menu
    #[nwg_control(parent: window, text: "&File")]
    menu_file: nwg::Menu,

    #[nwg_control(parent: menu_file, text: "&Open...")]
    #[nwg_events(OnMenuItemSelected: [FLSaveConvert::open_file])]
    menu_file_open: nwg::MenuItem,

    #[nwg_control(parent: menu_file)]
    menu_file_sep0: nwg::MenuSeparator,

    #[nwg_control(parent: menu_file, text: "E&xit")]
    #[nwg_events(OnMenuItemSelected: [FLSaveConvert::exit])]
    menu_file_exit: nwg::MenuItem,

    // About Menu
    #[nwg_control(parent: window, text: "&About")]
    #[nwg_events(OnMenuItemSelected: [FLSaveConvert::open_about])]
    menu_about: nwg::MenuItem,

    #[nwg_control]
    #[nwg_events(OnNotice: [FLSaveConvert::enable_about])]
    about_notice: nwg::Notice,

    // File browser dialog
    #[nwg_resource(title: "Open Save", action: nwg::FileDialogAction::Open, filters: "FL(*.fl)|TXT(*.txt)|Any (*.*)")]
    dialog: nwg::FileDialog,

    // Main layout items
    #[nwg_control(text: "Open", focus: true, size: (100, 200))]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 0, col_span: 1)]
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
                    let file_name = dir;
                    // Set file name text input to FL save path.
                    self.file_name.set_text(dir);
                        
                    self.process_file(file_name, &orig_path);
                    
                }
            }
        }
    }

    fn process_file(&self, file_name: &str, orig_path: &PathBuf) {
        self.msg_box.set_text("[INFO]: Reading Freelancer save.\r\n");

        if let Ok(fl_save) = read_save(file_name) {
            self.msg_box.append("[INFO]: Read successful.\r\n");

            self.msg_box.append("[INFO]: Backing up your Freelancer save.\r\n");
            
            match backup_save(&orig_path) {
                Ok(o) => self.msg_box.append(o),
                Err(e) => self.msg_box.append(e),
            };

            self.msg_box.append("[INFO]: Backing up your Freelancer save.\r\n");

        } else {
            self.msg_box.append("[ERROR]: Save file may be empty or corrupt.\r\n");
        };
    }

    fn open_about(&self) {
        // Disable the button to stop the user from spawning multiple dialogs
        self.menu_about.set_enabled(false);

        AboutWindow::popup(self.about_notice.sender());
    }

    fn enable_about(&self) {
        self.menu_about.set_enabled(true);
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = FLSaveConvert::build_ui(Default::default()).expect("Failed to build UI");

    nwg::dispatch_thread_events();
}