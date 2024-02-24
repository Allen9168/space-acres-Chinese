use crate::backend::config::Farm;
use crate::frontend::configuration::MaybeValid;
use bytesize::ByteSize;
use gtk::prelude::*;
use relm4::prelude::*;
use relm4_icons::icon_name;
use std::path::PathBuf;
use std::str::FromStr;
use tracing::warn;

// 2 GB
const MIN_FARM_SIZE: u64 = 1000 * 1000 * 1000 * 2;

#[derive(Debug, Default)]
pub(super) struct FarmWidgetInit {
    pub(super) path: MaybeValid<PathBuf>,
    pub(super) size: MaybeValid<String>,
}

#[derive(Debug)]
pub(super) enum FarmWidgetInput {
    DirectorySelected(PathBuf),
    FarmSizeChanged(String),
}

#[derive(Debug)]
pub(super) enum FarmWidgetOutput {
    OpenDirectory(DynamicIndex),
    ValidityUpdate,
    Delete(DynamicIndex),
}

#[derive(Debug)]
pub(super) struct FarmWidget {
    index: DynamicIndex,
    path: MaybeValid<PathBuf>,
    size: MaybeValid<String>,
    valid: bool,
}

#[relm4::factory(pub(super))]
impl FactoryComponent for FarmWidget {
    type Init = FarmWidgetInit;
    type Input = FarmWidgetInput;
    type Output = FarmWidgetOutput;
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::ListBoxRow {
            set_activatable: false,
            set_margin_bottom: 10,
            set_selectable: false,

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,

                gtk::Label {
                    add_css_class: "heading",
                    set_halign: gtk::Align::Start,
                    #[watch]
                    set_label: &format!("农场 {} 的路径及其大小", self.index.current_index()),
                },

                gtk::Box {
                    set_spacing: 10,

                    gtk::Box {
                        add_css_class: "linked",

                        gtk::Entry {
                            set_can_focus: false,
                            set_editable: false,
                            set_hexpand: true,
                            set_placeholder_text: Some(
                                if cfg!(windows) {
                                    "D:\\subspace-farm"
                                } else {
                                    "/media/subspace-farm"
                                },
                            ),
                            set_primary_icon_name: Some(icon_name::SSD),
                            set_primary_icon_activatable: false,
                            set_primary_icon_sensitive: false,
                            #[watch]
                            set_secondary_icon_name: self.path.icon(),
                            set_secondary_icon_activatable: false,
                            set_secondary_icon_sensitive: false,
                            #[watch]
                            set_text: self.path.display().to_string().as_str(),
                            set_tooltip_markup: Some(
                                "存储农场文件的绝对路径 \
                                任何SSD均可，无需高耐用性"
                            ),
                        },

                        gtk::Button {
                            connect_clicked[sender, index] => move |_| {
                                if sender.output(FarmWidgetOutput::OpenDirectory(index.clone())).is_err() {
                                    warn!("无法发送打开目录输出");
                                }
                            },
                            set_label: "选择",
                        },
                    },

                    gtk::Entry {
                        connect_activate[sender] => move |entry| {
                            sender.input(FarmWidgetInput::FarmSizeChanged(entry.text().into()));
                        },
                        connect_changed[sender] => move |entry| {
                            sender.input(FarmWidgetInput::FarmSizeChanged(entry.text().into()));
                        },
                        set_placeholder_text: Some(
                            "大小:200Gib, 或者2Tib等等",
                        ),
                        set_primary_icon_name: Some(icon_name::SIZE_HORIZONTALLY),
                        set_primary_icon_activatable: false,
                        set_primary_icon_sensitive: false,
                        #[watch]
                        set_secondary_icon_name: self.size.icon(),
                        set_secondary_icon_activatable: false,
                        set_secondary_icon_sensitive: false,
                        #[track = "self.size.unknown()"]
                        set_text: self.size.as_str(),
                        set_tooltip_markup: Some(
                            "农场的大小 \
                            最少需要2Gib空间"
                        ),
                    },

                    gtk::Button {
                        connect_clicked[sender, index] => move |_| {
                            if sender.output(FarmWidgetOutput::Delete(index.clone())).is_err() {
                                warn!("无法发送删除输出");
                            }
                        },
                        set_icon_name: icon_name::CROSS,
                        set_tooltip: "删除该农场",
                    },
                },
            },
        }
    }

    fn init_model(value: Self::Init, index: &DynamicIndex, _sender: FactorySender<Self>) -> Self {
        Self {
            index: index.clone(),
            path: value.path,
            size: value.size,
            valid: false,
        }
    }

    fn update(&mut self, input: Self::Input, sender: FactorySender<Self>) {
        match input {
            FarmWidgetInput::DirectorySelected(path) => {
                self.path = MaybeValid::Valid(path);
            }
            FarmWidgetInput::FarmSizeChanged(size) => {
                let size = if ByteSize::from_str(&size)
                    .map(|size| size.as_u64() >= MIN_FARM_SIZE)
                    .unwrap_or_default()
                {
                    MaybeValid::Valid(size)
                } else {
                    MaybeValid::Invalid(size)
                };
                self.size = size;
            }
        }

        let valid = self.valid();
        if self.valid != valid {
            self.valid = valid;

            // Send notification up that validity was updated, such that parent view can re-render
            // view if necessary
            if sender.output(FarmWidgetOutput::ValidityUpdate).is_err() {
                warn!("无法发送有效性更新输出");
            }
        }
    }
}

impl FarmWidget {
    pub(super) fn valid(&self) -> bool {
        self.path.valid() && self.size.valid()
    }

    pub(super) fn farm(&self) -> Farm {
        Farm {
            path: PathBuf::clone(&self.path),
            size: String::clone(&self.size),
        }
    }
}
