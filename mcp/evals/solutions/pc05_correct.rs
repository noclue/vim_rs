//! Reference-correct pc05 solution (grader self-test).
use vim_rs::types::convert::CastInto;
use vim_rs::types::traits::VirtualEthernetCardTrait;
use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        devices = "config.hardware.device",
    }
);

pub fn labeled_macs(t: &Target) -> Vec<(String, String)> {
    t.devices
        .as_ref()
        .map(|devs| {
            devs.iter()
                .filter_map(|d| {
                    let eth: &dyn VirtualEthernetCardTrait = d.as_ref().into_ref()?;
                    // MAC is on the ethernet subtype.
                    let mac = eth.get_virtual_ethernet_card().mac_address.clone()?;
                    // Label is on the BASE VirtualDevice.device_info (via supertrait),
                    // which is itself a DescriptionTrait.
                    let label = eth
                        .get_virtual_device()
                        .device_info
                        .as_ref()?
                        .get_description()
                        .label
                        .clone();
                    Some((label, mac))
                })
                .collect()
        })
        .unwrap_or_default()
}
