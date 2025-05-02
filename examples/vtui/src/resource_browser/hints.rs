use crate::resource_type::ResourceType;

pub(crate) const HELP_HINTS: &[&str] = &[
    "q quit",
    "/ search",
    "r resource",
    "0..9 sort",
    "Enter props",
];
const EXPAND_NETWORK: &str = "n network";
const EXPAND_DATASTORE: &str = "d datastore";
const EXPAND_HOST: &str = "h host";
const EXPAND_VM: &str = "v vm";
const EXPAND_TASK: &str = "t task";

const CLUSTER_EXPAND_HINTS: &[&str] = &[
    EXPAND_NETWORK,
    EXPAND_DATASTORE,
    EXPAND_HOST,
    EXPAND_VM,
    EXPAND_TASK,
];
const HOST_EXPAND_HINTS: &[&str] = &[
    EXPAND_NETWORK,
    EXPAND_DATASTORE,
    EXPAND_VM,
    EXPAND_TASK,
];
const DATASTORE_EXPAND_HINTS: &[&str] = &[
    EXPAND_HOST,
    EXPAND_VM,
    EXPAND_TASK,
];
const NETWORK_EXPAND_HINTS: &[&str] = &[
    EXPAND_HOST,
    EXPAND_VM,
    EXPAND_TASK
];

const VM_EXPAND_HINTS: &[&str] = &[
    EXPAND_TASK,
];
pub(crate) fn get_expand_hint(resource_type: ResourceType) -> &'static [&'static str] {
    match resource_type {
        ResourceType::Cluster => CLUSTER_EXPAND_HINTS,
        ResourceType::Host => HOST_EXPAND_HINTS,
        ResourceType::Datastore => DATASTORE_EXPAND_HINTS,
        ResourceType::Network => NETWORK_EXPAND_HINTS,
        ResourceType::VirtualMachine => VM_EXPAND_HINTS,
        _ => &[],
    }
}