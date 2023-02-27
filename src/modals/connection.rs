use gtk::subclass::prelude::*;
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
        type ParentType = gtk::Window;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for ConnectionDialog {}
    impl WidgetImpl for ConnectionDialog {}
    impl WindowImpl for ConnectionDialog {}
    impl DialogImpl for ConnectionDialog {}
}

glib::wrapper! {
    pub struct ConnectionDialog(ObjectSubclass<imp::ConnectionDialog>)
        @extends gtk::Widget, gtk::Window;
}

impl Default for ConnectionDialog {
    fn default() -> Self { glib::Object::new::<Self>() }
}

impl ConnectionDialog {
    pub fn new() -> Self { Self::default() }
}
