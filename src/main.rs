mod file_handling;
mod fl_core;

extern crate native_windows_derive as nwd;
use crate::file_handling::*;
use crate::fl_core::*;

use bstr::BString;
use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;
use std::ffi::OsStr;
use std::path::{PathBuf, Path};

use std::thread;
use directories::UserDirs;

#[derive(Default, NwgUi)]
pub struct AboutWindow {
    #[nwg_control(
        flags: "WINDOW|VISIBLE",
        ex_flags: 0x00020000|0x00000008,
        size: (300, 100),
        center: true,
        title: "About")]
    #[nwg_events(OnWindowClose: [AboutWindow::close])]
    about_window: nwg::Window,

    #[nwg_resource(family: "Segoe UI", size: 15)]
    font: nwg::Font,

    #[nwg_control(
        parent: about_window, 
        flags: "VISIBLE|MULTI_LINE",
        text: "Freelancer Save Convert v0.1.0\r\ngithub.com/BC46/freelancer-hd-edition\r\ngithub.com/BC46/freelancer-hd-edition", 
        size: (300, 100),
        position: (40, 30),
        font: Some(&data.font))]
    about_label: nwg::RichLabel,
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
    #[nwg_control(
        flags: "WINDOW|MINIMIZE_BOX|VISIBLE",
        // length, height
        size: (500, 400),
        center: true,
        title: "FL Save Convert")]
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
    #[nwg_resource(
        title: "Open Save",
        action: nwg::FileDialogAction::Open,
        filters: "FL(*.fl)|TXT(*.txt)|Any (*.*)")]
    dialog: nwg::FileDialog,

    // Main layout items
    #[nwg_control(parent: window, text: "Open", focus: true, size: (100, 200))]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 0, col_span: 1)]
    #[nwg_events(OnButtonClick: [FLSaveConvert::open_file])]
    open_btn: nwg::Button,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 0, col_span: 4)]
    file_name: nwg::TextInput,

    #[nwg_resource(family: "Consolas", size: 18)]
    console_font: nwg::Font,

    #[nwg_control(font: Some(&data.console_font), readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 1, col_span: 5, row_span: 3)]
    msg_box: nwg::RichTextBox,

    #[nwg_control(parent: window, enabled: false, text: "Convert Only")]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 4, col_span: 2)]
    #[nwg_events(OnButtonClick: [FLSaveConvert::convert_save])]
    convert_btn: nwg::Button,

    #[nwg_control(parent: window, enabled: false, text: "Fix Save")]
    #[nwg_layout_item(layout: main_layout, col: 3, row: 4, col_span: 1)]
    #[nwg_events(OnButtonClick: [FLSaveConvert::fix_save])]
    fix_btn: nwg::Button,

    orig_path: RefCell<PathBuf>,
    fl_save_contents: RefCell<BString>,
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
                    let dir_path: PathBuf = directory.try_into().expect("Failed to convert directory to PathBuf.");
                    
                    *self.orig_path.borrow_mut() = dir_path;

                    let fl_name_ptr: *mut PathBuf = self.orig_path.as_ptr();
                    let fl_path: *mut PathBuf = self.orig_path.as_ptr();

                    unsafe {
                        /*let fl_name: &str = fl_name_ptr.as_ref()
                            .expect("Save path should not be null.")
                            .file_name().expect("Save name should not be empty.")
                            .to_str().expect("Unable to convert save name to str.");*/

                        let fl_path_str: &str = fl_path.as_ref()
                            .expect("Save path should not be null.")
                            .to_str().expect("Cannot convert file path ptr to str.");                

                        // Set file name text input to FL save path.
                        self.file_name.set_text(fl_path_str);
                            
                        self.process_file(fl_path_str);
                    }
                }
            }
        }
    }

    fn process_file(&self, file_path: &str) {//, file_name: &str) {
        self.msg_box.set_text("[INFO]: Reading Freelancer save.\r\n");

        if let Ok(fl_save) = read_save(file_path) {
            self.msg_box.append("[INFO]: ");
            unsafe {
                let fl_name_ptr = self.orig_path.as_ptr();
                let file_name = fl_name_ptr.as_ref()
                    .expect("File name should not be null.")
                    .file_name().expect("Could not get file name.")
                    .to_str().expect("Cannot convert file name ptr to str.");
                self.msg_box.append(file_name);
            }
            self.msg_box.append(" successfully read.\r\n");

            *self.fl_save_contents.borrow_mut() = fl_save;

            self.convert_btn.set_enabled(true);
            self.fix_btn.set_enabled(true);
        } else {
            self.convert_btn.set_enabled(false);
            self.fix_btn.set_enabled(false);

            self.msg_box.append("[ERROR]: Save file may be empty or corrupt.\r\n");
        };
    }

    fn convert_save(&self) {
        let fl_path: *mut PathBuf = self.orig_path.as_ptr();
        let fl_save: *mut BString = self.fl_save_contents.as_ptr();

        //let fl_save_name = fl_path.file_name();
        
        self.msg_box.append("[INFO]: Backing up your Freelancer save.\r\n");
        
        unsafe {
            match backup_save(fl_path.as_ref().unwrap().as_path()) {
                Ok(o) => {
                    self.msg_box.append(o);
                    //let my_buf = decrypt(fl_save.as_ref().unwrap());
                    if let Ok(my_buf) = decrypt(fl_save.as_ref().unwrap()) {
                        let orig_dir = self.orig_path.as_ptr();
                        let save_dir: &Path = orig_dir.as_ref().unwrap().parent().unwrap();

                        let fl_name_ptr = self.orig_path.as_ptr();
                        let save_name = fl_name_ptr.as_ref()
                            .expect("File name should not be null.").file_name();

                        if let Ok(..) = write_out(save_dir.to_path_buf(), save_name, my_buf) {
                            self.msg_box.append("[INFO]: New save successfully written.")
                        } else {
                            self.msg_box.append("[ERROR]: Failed to write new save file.")
                        };
                    } else {
                        self.msg_box.append("[ERROR]: Failed to decipher save.");
                    };
                    //write_out(save_dir, save_name, buf);
                    //decrypt(fl_save.as_ref().unwrap());
                },
                Err(e) => self.msg_box.append(e),
            }
        }
    }

    fn fix_save(&self) {
        unsafe {
            let fl_path = self.orig_path.as_ptr();
            self.msg_box.append("[INFO]: Backing up your Freelancer save.\r\n");
            match backup_save(fl_path.as_ref().unwrap().as_path()) {
                Ok(o) => {
                    self.msg_box.append(o);               
                    //println!("{:?}", decrypt(&fl_save));
                },
                Err(e) => self.msg_box.append(e),
            }
        }
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