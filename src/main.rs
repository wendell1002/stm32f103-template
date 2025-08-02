#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use {{panic_crate}} as _;

use stm32f1xx_hal::{
    pac,
    prelude::*,
};
#[entry]
fn main() -> ! {
    //初始化和获取外设对象
    // 获取cortex-m 相关的核心外设
    let cp = cortex_m::Peripherals::take().unwrap();
    //获取stm32f1xx_hal硬件外设
    let dp = pac::Peripherals::take().unwrap();
    // 初始化并获取flash和rcc设备的所有权
    let mut rcc = dp.RCC.constrain();
    let mut flash = dp.FLASH.constrain();
    //冻结系统中所有时钟的配置，并将冻结后的频率值存储在“clocks”中
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = dp.GPIOB.split();

    let mut led = gpiob.pb12.into_push_pull_output(&mut gpiob.crh);

    //精度 1us
    // let mut delay = FTimerUs::new(dp.TIM2, &clocks).delay();
    //精度 1ms
    //   let mut delay = FTimerMs::new(dp.TIM2, &clocks).delay();
    // or
    let mut delay = dp.TIM2.delay_us(&clocks);
    hprintln!("Hello, world!");
    loop {
        // 设置高电平
        led.set_high();
        delay.delay_ms(500_u16);

        // 设置低电平
        led.set_low();
        delay.delay_ms(500_u16);
    }
}
