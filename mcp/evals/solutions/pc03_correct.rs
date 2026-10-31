//! Reference-correct pc03 solution (grader self-test).
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        devices = "config.hardware.device",
    }
);

pub fn ethernet_macs(t: &Target) -> Vec<String> {
    t.devices
        .as_ref()
        .map(|devs| {
            devs.iter()
                .filter_map(|d| {
                    let eth: Option<&dyn VirtualEthernetCardTrait> = d.as_ref().into_ref();
                    eth.and_then(|e| e.mac_address.clone())
                })
                .collect()
        })
        .unwrap_or_default()
}
