# bare-test-template

基于 `bare-test` 的裸机测试模板项目，用于在裸机环境下运行 Rust 测试。

## 运行测试

### 安装工具链

安装 `ostool`:

```bash
cargo install ostool
```

### 运行测试

```bash
# 基本测试
cargo test --test test

# 带 u-boot 的开发板测试

# 配置开发板参数
ostool menuconfig uboot  
# 运行测试
cargo test --test test -- uboot
```

## API 用法

### 基本测试

`bare-test` 提供了简洁的测试框架，使用 `#[bare_test::tests]` 属性宏标记测试模块：

```rust
#[bare_test::tests]
mod tests {
    use bare_test::println;

    #[test]
    fn it_works() {
        let a = 2;
        let b = 2;
        assert_eq!(a + b, 4);
        println!("test passed!");
    }
}
```

### 获取设备树

bare-test 通过 `sparreal-rt` 重新导出了设备树相关的 API。可以通过 `global_val()` 访问设备树：

```rust
use bare_test::*;
use globals::{PlatformInfoKind, global_val};
use fdt_parser::Fdt;

#[test]
fn test_device_tree() {
    // 获取设备树
    let fdt = match &global_val().platform_info {
        PlatformInfoKind::DeviceTree(fdt) => fdt,
        _ => return,
    };

    // 解析设备树
    let fdt = fdt.get();

    // 获取模型名称
    if let Some(model) = fdt.model() {
        println!("Model: {}", model);
    }

    // 查找特定节点
    if let Some(node) = fdt.find_node("/uart") {
        println!("Found UART node");
    }
}
```

### 注册中断

使用 `bare_test::irq::register_handler` 注册中断处理函数：

```rust
use bare_test::*;
use core::sync::atomic::{AtomicU32, Ordering};

static INTERRUPT_COUNT: AtomicU32 = AtomicU32::new(0);

#[test]
fn test_interrupt() {
    // 定义中断处理函数
    extern "C" fn irq_handler() {
        let count = INTERRUPT_COUNT.fetch_add(1, Ordering::SeqCst);
        println!("Interrupt triggered, count: {}", count);

        // 确认中断（根据具体硬件实现）
        // crate::hal::al::cpu::systick_ack();
    }

    // 获取系统中断 ID（示例使用系统滴答中断）
    let irq_id = crate::hal::al::cpu::systick_irq_id();

    // 注册中断处理函数
    bare_test::irq::register_handler(irq_id, irq_handler);

    println!("Interrupt handler registered");

    // 等待中断发生
    // ...
}
```

### 日志输出

```rust
use bare_test::*;
use log::{info, warn, error};

#[test]
fn test_logging() {
    info!("This is an info message");
    warn!("This is a warning");
    error!("This is an error");

    // 也可以直接使用 println
    println!("Direct output");
}
```

### 中断相关

- `IrqId` - 中断标识符
- `NoIrqGuard` - RAII 风格的中断禁用守卫

## 相关资源

- [bare-test crate](https://docs.rs/bare-test)
- [sparreal-kernel](https://github.com/ZR233/sparreal-os)
- [fdt_parser](https://docs.rs/fdt-parser)

## License

MPL-2.0
