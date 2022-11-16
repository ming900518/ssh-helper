use adw::prelude::*;
use gtk::glib;
use relm4::prelude::*;

struct App {
    current_section: u32,
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = u32;
    type Output = ();
    type Init = ();

    view! {
        adw::Window {
            set_default_width: 450,
            set_default_height: 500,
            #[name = "leaflet"]
            adw::Leaflet {
                set_can_navigate_back: true,
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    #[name = "sidebar_header"]
                    adw::HeaderBar {
                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            set_title: "SSH Helper",
                        }
                    },
                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        gtk::ListBox {
                            set_selection_mode: gtk::SelectionMode::Single,
                            set_vexpand: true,
                            set_hexpand: true,
                            add_css_class: "navigation-sidebar",
                            adw::ActionRow {
                                set_icon_name: Some("network-transmit-symbolic"),
                                set_title: "Connection",
                            },

                            adw::ActionRow {
                                set_icon_name: Some("dialog-information-symbolic"),
                                set_title: "About"
                            },

                            connect_row_selected[sender] => move |_, row| {
                                if let Some(row) = row {
                                    sender.input((row.index() + 1) as u32);
                                }
                            }
                        },
                        gtk::Separator{},
                    }
                },
                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_hexpand: true,
                    #[name = "content_header"]
                    adw::HeaderBar {
                        #[name = "back_button"]
                        pack_start = &gtk::Button {
                            set_icon_name: "go-previous-symbolic",
                            connect_clicked[leaflet] => move |_| {
                                leaflet.navigate(adw::NavigationDirection::Back);
                            }
                        },

                        #[wrap(Some)]
                        set_title_widget = &adw::WindowTitle {
                            #[watch]
                            set_title: if model.current_section == 1 {
                                "Connection"
                            } else {
                                "About"
                            },
                        }
                    },

                    gtk::Label {
                        add_css_class: "title-1",
                        set_vexpand: true,

                        #[watch]
                        set_text: &format!("Page {}", model.current_section),
                    }
                },
            }
        }
    }

    fn update(&mut self, msg: u32, _: ComponentSender<Self>) {
        self.current_section = msg;
    }

    fn init(_: (), root: &Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
        let model = App { current_section: 1 };

        let widgets = view_output!();

        widgets
            .leaflet
            .bind_property("folded", &widgets.sidebar_header, "show-end-title-buttons")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        widgets
            .leaflet
            .bind_property(
                "folded",
                &widgets.content_header,
                "show-start-title-buttons",
            )
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();
        widgets
            .leaflet
            .bind_property("folded", &widgets.back_button, "visible")
            .flags(glib::BindingFlags::SYNC_CREATE)
            .build();

        ComponentParts { model, widgets }
    }

    fn pre_view() {
        widgets.leaflet.navigate(adw::NavigationDirection::Forward);
    }
}

fn main() {
    let app = RelmApp::new("tw.mingchang.ssh-helper");
    app.run::<App>(());
    relm4::set_global_css_from_file("style.css")
}
