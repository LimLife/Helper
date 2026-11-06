use application::data_use_cases::{Loader, Mutator};
use transport::gateways::{
    article::ArticleGatewayDioxus, content::ContentGatewayDioxus, node::NodeGatewayDioxus,
};
pub type AppLoader = Loader<ContentGatewayDioxus, NodeGatewayDioxus, ArticleGatewayDioxus>;
pub type AppMutator = Mutator<ContentGatewayDioxus, NodeGatewayDioxus, ArticleGatewayDioxus>;
