use vim_rs::vim_retrievable;

vim_retrievable!(
    struct Target: HostSystem {
        overall_status = "summary.overall_status",
        product_name = "summary.config.product.full_name",
    }
);
