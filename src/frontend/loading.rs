use crate::backend::LoadingStep;
use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
pub enum LoadingInput {
    BackendLoading(LoadingStep),
}

#[derive(Debug)]
pub struct LoadingView {
    message: String,
}

#[relm4::component(pub)]
impl Component for LoadingView {
    type Init = ();
    type Input = LoadingInput;
    type Output = ();
    type CommandOutput = ();

    view! {
        #[root]
        gtk::Box {
            set_halign: gtk::Align::Center,
            set_valign: gtk::Align::Center,
            set_vexpand: true,
            set_orientation: gtk::Orientation::Vertical,

            gtk::Spinner {
                start: (),
                set_size_request: (50, 50),
            },

            gtk::Label {
                #[watch]
                set_label: &model.message,
            },
        }
    }

    fn init(
        _init: Self::Init,
        _root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {
            message: String::new(),
        };

        let widgets = view_output!();

        ComponentParts { model, widgets }
    }

    fn update(&mut self, input: Self::Input, _sender: ComponentSender<Self>, _root: &Self::Root) {
        self.process_input(input);
    }
}

impl LoadingView {
    fn process_input(&mut self, input: LoadingInput) {
        match input {
            LoadingInput::BackendLoading(step) => {
                self.message = match step {
                    LoadingStep::LoadingConfiguration => "���ڼ�������...".to_string(),
                    LoadingStep::ReadingConfiguration => "���ڶ�ȡ����...".to_string(),
                    LoadingStep::ConfigurationReadSuccessfully { .. } => {
                        "���ö�ȡ�ɹ�".to_string()
                    },
                    LoadingStep::CheckingConfiguration => "���ڼ������...".to_string(),
                    LoadingStep::ConfigurationIsValid => "������Ч".to_string(),
                    LoadingStep::DecodingChainSpecification => {
                        "���ڽ������淶...".to_string()
                    },
                    LoadingStep::DecodedChainSpecificationSuccessfully => {
                        "���淶����ɹ�".to_string()
                    },
                    LoadingStep::CheckingNodePath => "���ڼ��ڵ�·��...".to_string(),
                    LoadingStep::CreatingNodePath => "���ڴ����ڵ�·��...".to_string(),
                    LoadingStep::NodePathReady => "�ڵ�·������".to_string(),
                    LoadingStep::PreparingNetworkingStack => {
                        "����׼�������ջ...".to_string()
                    },
                    LoadingStep::ReadingNetworkKeypair => "���ڶ�ȡ������Կ��...".to_string(),
                    LoadingStep::GeneratingNetworkKeypair => {
                        "��������������Կ��...".to_string()
                    },
                    LoadingStep::WritingNetworkKeypair => {
                        "���ڽ�������Կ��д�����...".to_string()
                    },
                    LoadingStep::InstantiatingNetworkingStack => {
                        "����ʵ���������ջ...".to_string()
                    },
                    LoadingStep::NetworkingStackCreatedSuccessfully => {
                        "�����ջ�����ɹ�".to_string()
                    },
                    LoadingStep::CreatingConsensusNode => "���ڴ�����ʶ�ڵ�...".to_string(),
                    LoadingStep::ConsensusNodeCreatedSuccessfully => {
                        "��ʶ�ڵ㴴���ɹ�".to_string()
                    },
                    LoadingStep::CreatingFarmer => "���ڴ���ũ��ڵ�...".to_string(),
                    LoadingStep::FarmerCreatedSuccessfully => {
                        "ũ��ڵ㴴���ɹ�".to_string()
                    },
                    LoadingStep::WipingFarm { farm_index, path } => {
                        format!("��������ũ�� {farm_index} λ�� {}...", path.display())
                    },
                    LoadingStep::WipingNode { path } => {
                        format!("��������ڵ�λ�� {}...", path.display())
                    }
                };
            }
        }
    }
}

