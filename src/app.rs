use relm4::{
    adw,
    actions::{ActionGroupName, RelmAction, RelmActionGroup},
    gtk, main_application, Component, ComponentController, ComponentParts,
    ComponentSender, Controller, SimpleComponent,
};

use gtk::prelude::{
    ApplicationExt, ApplicationWindowExt, ButtonExt, GtkWindowExt, 
    OrientableExt, SettingsExt, ToggleButtonExt, WidgetExt
};
use gtk::{gio, glib};

use crate::config::{APP_ID, PROFILE};
use crate::modals::about::AboutDialog;

pub(super) struct App {
    about_dialog: Controller<AboutDialog>,
    flap_shown: bool
}

#[derive(Debug)]
pub(super) enum AppMsg {
    Quit,
    ToggleFlap(bool)
}

relm4::new_action_group!(pub(super) WindowActionGroup, "win");
relm4::new_stateless_action!(PreferencesAction, WindowActionGroup, "preferences");
relm4::new_stateless_action!(pub(super) ShortcutsAction, WindowActionGroup, "show-help-overlay");
relm4::new_stateless_action!(AboutAction, WindowActionGroup, "about");

#[relm4::component(pub)]
impl SimpleComponent for App {
    type Init = ();
    type Input = AppMsg;
    type Output = ();
    type Widgets = AppWidgets;

    menu! {
        primary_menu: {
            section! {
                "_Preferences" => PreferencesAction,
                "_Keyboard" => ShortcutsAction,
                "_About Based" => AboutAction,
            }
        }
    }

    view! {
        #[root]
        main_window = adw::ApplicationWindow::new(&main_application()) {
            connect_close_request[sender] => move |_| {
                sender.input(AppMsg::Quit);
                gtk::Inhibit(true)
            },
            adw::Flap {
                #[watch]
                set_reveal_flap: model.flap_shown,

                #[wrap(Some)]
                set_flap = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    adw::HeaderBar {
                        set_show_end_title_buttons: false,

                        pack_start = &gtk::Button {
                            set_icon_name: "list-add-symbolic"
                        }
                    }
                },
                #[wrap(Some)]
                set_content = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    adw::HeaderBar {
                        pack_start = &gtk::ToggleButton {
                            set_icon_name: "sidebar-show-symbolic",
                            set_active: true,
                            connect_toggled[sender] => move |btn| {
                                sender.input(AppMsg::ToggleFlap(btn.is_active()))
                            }
                        },
                        pack_end = &gtk::MenuButton {
                            set_icon_name: "open-menu-symbolic",
                            set_menu_model: Some(&primary_menu),
                        }
                    },
                    gtk::ScrolledWindow {
                        set_height_request: 180,
                        gtk::TextView {
                            add_css_class: "scripts-input"
                        }
                    }
                },
                #[wrap(Some)]
                set_separator = &gtk::Separator::default(),
            },

            #[wrap(Some)]
            set_help_overlay: shortcuts = &gtk::Builder::from_resource(
                    "/com/kylobytes/Based/gtk/help-overlay.ui"
                )
                .object::<gtk::ShortcutsWindow>("help_overlay")
                .unwrap() -> gtk::ShortcutsWindow {
                    set_transient_for: Some(&main_window),
                    set_application: Some(&main_application()),
            },

            add_css_class?: if PROFILE == "Devel" {
                Some("devel")
            } else {
                None
            },
        }
    }

    fn init(
        _init: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let about_dialog = AboutDialog::builder()
            .transient_for(root)
            .launch(())
            .detach();

        let model = Self {
            about_dialog,
            flap_shown: true
        };

        let widgets = view_output!();

        let actions = RelmActionGroup::<WindowActionGroup>::new();

        let shortcuts_action = {
            let shortcuts = widgets.shortcuts.clone();
            RelmAction::<ShortcutsAction>::new_stateless(move |_| {
                shortcuts.present();
            })
        };

        let about_action = {
            let sender = model.about_dialog.sender().clone();
            RelmAction::<AboutAction>::new_stateless(move |_| {
                sender.send(());
            })
        };

        actions.add_action(&shortcuts_action);
        actions.add_action(&about_action);

        widgets
            .main_window
            .insert_action_group(WindowActionGroup::NAME, Some(&actions.into_action_group()));

        widgets.load_window_size();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::Quit => main_application().quit(),
            AppMsg::ToggleFlap(shown) => self.flap_shown = shown
        }
    }

    fn shutdown(&mut self, widgets: &mut Self::Widgets, _output: relm4::Sender<Self::Output>) {
        widgets.save_window_size().unwrap();
    }
}

impl AppWidgets {
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = gio::Settings::new(APP_ID);
        let (width, height) = self.main_window.default_size();

        settings.set_int("window-width", width)?;
        settings.set_int("window-height", height)?;

        settings.set_boolean("is-maximized", self.main_window.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        let settings = gio::Settings::new(APP_ID);

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.main_window.set_default_size(width, height);

        if is_maximized {
            self.main_window.maximize();
        }
    }
}
