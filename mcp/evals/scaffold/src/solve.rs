//! Agent-writable contract — fill in VIM property path strings only.
//! Default paths are intentionally wrong-but-compiling so the warm image
//! build succeeds without leaking oracle answers into the sealed tree.

use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: VirtualMachine {
        power_state = "summary.runtime.power_state",
        guest_host = "name",
    }
);
