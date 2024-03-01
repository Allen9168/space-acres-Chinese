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
                    LoadingStep::LoadingConfiguration => "正在加载配置...".to_string(),
                    LoadingStep::ReadingConfiguration => "正在读取配置...".to_string(),
                    LoadingStep::ConfigurationReadSuccessfully { .. } => "配置读取成功".to_string(),
                    LoadingStep::CheckingConfiguration => "正在检查配置...".to_string(),
                    LoadingStep::ConfigurationIsValid => "配置有效".to_string(),
                    LoadingStep::DecodingChainSpecification => "正在解码链规范...".to_string(),
                    LoadingStep::DecodedChainSpecificationSuccessfully => {
                        "链规范解码成功".to_string()
                    }
                    LoadingStep::CheckingNodePath => "正在检查节点路径...".to_string(),
                    LoadingStep::CreatingNodePath => "正在创建节点路径...".to_string(),
                    LoadingStep::NodePathReady => "节点路径就绪".to_string(),
                    LoadingStep::PreparingNetworkingStack => "正在准备网络堆栈...".to_string(),
                    LoadingStep::ReadingNetworkKeypair => "正在读取网络密钥对...".to_string(),
                    LoadingStep::GeneratingNetworkKeypair => "正在生成网络密钥对...".to_string(),
                    LoadingStep::WritingNetworkKeypair => "正在将网络密钥对写入磁盘...".to_string(),
                    LoadingStep::InstantiatingNetworkingStack => {
                        "正在实例化网络堆栈...".to_string()
                    }
                    LoadingStep::NetworkingStackCreatedSuccessfully => {
                        "网络堆栈创建成功".to_string()
                    }
                    LoadingStep::CreatingConsensusNode => "正在创建共识节点...".to_string(),
                    LoadingStep::ConsensusNodeCreatedSuccessfully => "共识节点创建成功".to_string(),
                    LoadingStep::CreatingFarmer => "正在创建农民节点...".to_string(),
                    LoadingStep::FarmerCreatedSuccessfully => "农民节点创建成功".to_string(),
                    LoadingStep::WipingFarm { farm_index, path } => {
                        format!("正在清理农场 {farm_index} 位于 {}...", path.display())
                    }
                    LoadingStep::WipingNode { path } => {
                        format!("正在清理节点位于 {}...", path.display())
                    }
                };
            }
        }
    }
}
