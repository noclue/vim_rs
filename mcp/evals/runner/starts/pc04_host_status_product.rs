//! Fill in the property-path strings.
//! overall_status = "<path>", product_name = "<path>".

use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: HostSystem {
        overall_status = "config_status",
        product_name = "name",
    }
);
