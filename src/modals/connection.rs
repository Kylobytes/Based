// Based - connection.rs

// Copyright (C) 2023  Kent Delante <leftybournes@pm.me>
// This file is part of Based.

// Based is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Based is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Based. If not, see <https://www.gnu.org/licenses/>.

use adw::subclass::prelude::*;
use gtk::glib;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/com/kylobytes/Based/ui/modals/connection.ui")]
    pub struct ConnectionDialog {}

    #[glib::object_subclass]
    impl ObjectSubclass for ConnectionDialog {
        const NAME: &'static str = "ConnectionDialog";
        type Type = super::ConnectionDialog;
        type ParentType = adw::Dialog;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ConnectionDialog {}
    impl WidgetImpl for ConnectionDialog {}
    impl AdwDialogImpl for ConnectionDialog {}
}

glib::wrapper! {
    pub struct ConnectionDialog(ObjectSubclass<imp::ConnectionDialog>)
    @extends gtk::Widget, adw::Dialog;
}

impl Default for ConnectionDialog {
    fn default() -> Self {
        glib::Object::new::<Self>()
    }
}

impl ConnectionDialog {
    pub fn new() -> Self {
        Self::default()
    }
}
