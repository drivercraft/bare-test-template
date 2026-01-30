#![no_std]
#![no_main]
#![feature(used_with_arg)]

extern crate alloc;
extern crate bare_test;

#[bare_test::tests]
mod tests {
    use bare_test::{mem::iomap, println};
    use log::info;
    use project_name::MyDriver;

    #[test]
    fn it_works() {
        let demo_reg_base = 0x1000_0000;
        let mmio = iomap(demo_reg_base.into(), 0x1000);
        info!("MMIO mapped at: {:p}", mmio.as_ptr());
        let mut driver = MyDriver::new(mmio);
        driver.initialize();

        println!("test passed!");
    }
}
