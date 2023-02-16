//! This module contains the runtime struct, which is used to perform initialization steps, manage
//! peripherals, provides random number generation, and manage an interrupt loop.

use crate::{eeprom::EepromController, random, Timer};
use core::time::Duration;
use tm4c123x_hal::{
    delay::Delay,
    sysctl::{
        self, Clocks, CrystalFrequency, Domain, Oscillator, PllOutputFrequency, PowerControl,
        PowerState, RunMode, Sysctl, SysctlExt, SystemClock,
    },
    tm4c123x::*,
};

/// The runtime struct.
pub struct Runtime<'a> {
    /// The EEPROM controller.
    pub eeprom: EepromController<'a>,
    // TODO: Add controllers.
    hib: &'a HIB,
}

impl<'a> Runtime<'a> {
    /// Initializes the hibernation peripheral.
    fn init_hib(hib: &mut HIB, power_control: &PowerControl) {
        // Enable hibernation module. This is enabled by default, but we enable it here just in case.
        sysctl::control_power(
            power_control,
            Domain::Hibernation,
            RunMode::Run,
            PowerState::On,
        );

        // Reset hibernation module for good measure.
        sysctl::reset(power_control, Domain::Hibernation);

        // Initialize hibernation clock.
        hib.ctl.write(|w| {
            // Use low-frequency oscillator and enable clock.
            w.oscbyp().clear_bit().clk32en().set_bit()
        });

        // Wait for hibernation module to be ready.
        while hib.ctl.read().wrc().bit_is_clear() {}

        // Enable RTC.
        // SAFETY: Writing to this register is safe because it is data-race free. This guarantee
        // comes from the fact that the hibernation peripheral is borrowed mutably.
        hib.ctl
            .modify(|r, w| unsafe { w.bits(r.bits()).rtcen().set_bit() });

        // Wait for hibernation module to be ready.
        while hib.ctl.read().wrc().bit_is_clear() {}
    }

    /// Initializes the runtime.
    pub fn new(rt_peripherals: &'a mut RuntimePeripherals) -> Self {
        random::init_rng(rt_peripherals);

        let eeprom =
            EepromController::new(&mut rt_peripherals.eeprom, &rt_peripherals.power_control)
                .unwrap();

        // todo!("Call init functions of button module, EEPROM controller, etc.).");

        Self::init_hib(&mut rt_peripherals.hib, &rt_peripherals.power_control);

        Runtime {
            eeprom,
            hib: &rt_peripherals.hib,
        }
    }

    /// Runs the event loop.
    pub fn start(&mut self, mut to_run: impl FnMut(&mut Self)) -> ! {
        loop {
            to_run(self);
        }
    }

    // TODO: Add methods for returning controllers.

    /// Creates a timer from a duration.
    pub fn create_timer(&self, duration: Duration) -> Timer {
        Timer::new(&self.hib, duration)
    }

    /// Fills a slice with random bytes from the main CSPRNG.
    pub fn fill_rand_slice(&self, dest: &mut [u8]) {
        random::fill_rand_slice(dest);
    }
}

/// Initializes the system clock and power control, and returns them.
fn initialize_sysctl(mut sysctl: Sysctl) -> (PowerControl, Clocks) {
    // Setup clock.
    sysctl.clock_setup.oscillator = Oscillator::Main(
        CrystalFrequency::_16mhz,
        SystemClock::UsePll(PllOutputFrequency::_80_00mhz),
    );

    (sysctl.power_control, sysctl.clock_setup.freeze())
}

/// All peripherals and core peripherals, but with the system clock, power control, and delay
/// initialized.
#[allow(dead_code, missing_docs)]
pub struct RuntimePeripherals {
    pub cbp: CBP,
    pub cpuid: CPUID,
    pub dcb: DCB,
    pub dwt: DWT,
    pub fpb: FPB,
    pub fpu: FPU,
    pub itm: ITM,
    pub mpu: MPU,
    pub nvic: NVIC,
    pub scb: SCB,
    pub tpiu: TPIU,
    pub watchdog0: WATCHDOG0,
    pub watchdog1: WATCHDOG1,
    pub gpio_porta: GPIO_PORTA,
    pub gpio_portb: GPIO_PORTB,
    pub gpio_portc: GPIO_PORTC,
    pub gpio_portd: GPIO_PORTD,
    pub ssi0: SSI0,
    pub ssi1: SSI1,
    pub ssi2: SSI2,
    pub ssi3: SSI3,
    pub uart0: UART0,
    pub uart1: UART1,
    pub uart2: UART2,
    pub uart3: UART3,
    pub uart4: UART4,
    pub uart5: UART5,
    pub uart6: UART6,
    pub uart7: UART7,
    pub i2c0: I2C0,
    pub i2c1: I2C1,
    pub i2c2: I2C2,
    pub i2c3: I2C3,
    pub gpio_porte: GPIO_PORTE,
    pub gpio_portf: GPIO_PORTF,
    pub pwm0: PWM0,
    pub pwm1: PWM1,
    pub qei0: QEI0,
    pub qei1: QEI1,
    pub timer0: TIMER0,
    pub timer1: TIMER1,
    pub timer2: TIMER2,
    pub timer3: TIMER3,
    pub timer4: TIMER4,
    pub timer5: TIMER5,
    pub wtimer0: WTIMER0,
    pub wtimer1: WTIMER1,
    pub adc0: ADC0,
    pub adc1: ADC1,
    pub comp: COMP,
    pub can0: CAN0,
    pub can1: CAN1,
    pub wtimer2: WTIMER2,
    pub wtimer3: WTIMER3,
    pub wtimer4: WTIMER4,
    pub wtimer5: WTIMER5,
    pub usb0: USB0,
    pub gpio_porta_ahb: GPIO_PORTA_AHB,
    pub gpio_portb_ahb: GPIO_PORTB_AHB,
    pub gpio_portc_ahb: GPIO_PORTC_AHB,
    pub gpio_portd_ahb: GPIO_PORTD_AHB,
    pub gpio_porte_ahb: GPIO_PORTE_AHB,
    pub gpio_portf_ahb: GPIO_PORTF_AHB,
    pub eeprom: EEPROM,
    pub sysexc: SYSEXC,
    pub hib: HIB,
    pub flash_ctrl: FLASH_CTRL,
    pub udma: UDMA,
    pub power_control: PowerControl,
    pub clocks: Clocks,
    pub delay: Delay,
}

impl From<(CorePeripherals, Peripherals)> for RuntimePeripherals {
    fn from((core_peripherals, peripherals): (CorePeripherals, Peripherals)) -> Self {
        let sysctl = initialize_sysctl(peripherals.SYSCTL.constrain());

        RuntimePeripherals {
            cbp: core_peripherals.CBP,
            cpuid: core_peripherals.CPUID,
            dcb: core_peripherals.DCB,
            dwt: core_peripherals.DWT,
            fpb: core_peripherals.FPB,
            fpu: core_peripherals.FPU,
            itm: core_peripherals.ITM,
            mpu: core_peripherals.MPU,
            nvic: core_peripherals.NVIC,
            scb: core_peripherals.SCB,
            tpiu: core_peripherals.TPIU,
            watchdog0: peripherals.WATCHDOG0,
            watchdog1: peripherals.WATCHDOG1,
            gpio_porta: peripherals.GPIO_PORTA,
            gpio_portb: peripherals.GPIO_PORTB,
            gpio_portc: peripherals.GPIO_PORTC,
            gpio_portd: peripherals.GPIO_PORTD,
            ssi0: peripherals.SSI0,
            ssi1: peripherals.SSI1,
            ssi2: peripherals.SSI2,
            ssi3: peripherals.SSI3,
            uart0: peripherals.UART0,
            uart1: peripherals.UART1,
            uart2: peripherals.UART2,
            uart3: peripherals.UART3,
            uart4: peripherals.UART4,
            uart5: peripherals.UART5,
            uart6: peripherals.UART6,
            uart7: peripherals.UART7,
            i2c0: peripherals.I2C0,
            i2c1: peripherals.I2C1,
            i2c2: peripherals.I2C2,
            i2c3: peripherals.I2C3,
            gpio_porte: peripherals.GPIO_PORTE,
            gpio_portf: peripherals.GPIO_PORTF,
            pwm0: peripherals.PWM0,
            pwm1: peripherals.PWM1,
            qei0: peripherals.QEI0,
            qei1: peripherals.QEI1,
            timer0: peripherals.TIMER0,
            timer1: peripherals.TIMER1,
            timer2: peripherals.TIMER2,
            timer3: peripherals.TIMER3,
            timer4: peripherals.TIMER4,
            timer5: peripherals.TIMER5,
            wtimer0: peripherals.WTIMER0,
            wtimer1: peripherals.WTIMER1,
            adc0: peripherals.ADC0,
            adc1: peripherals.ADC1,
            comp: peripherals.COMP,
            can0: peripherals.CAN0,
            can1: peripherals.CAN1,
            wtimer2: peripherals.WTIMER2,
            wtimer3: peripherals.WTIMER3,
            wtimer4: peripherals.WTIMER4,
            wtimer5: peripherals.WTIMER5,
            usb0: peripherals.USB0,
            gpio_porta_ahb: peripherals.GPIO_PORTA_AHB,
            gpio_portb_ahb: peripherals.GPIO_PORTB_AHB,
            gpio_portc_ahb: peripherals.GPIO_PORTC_AHB,
            gpio_portd_ahb: peripherals.GPIO_PORTD_AHB,
            gpio_porte_ahb: peripherals.GPIO_PORTE_AHB,
            gpio_portf_ahb: peripherals.GPIO_PORTF_AHB,
            eeprom: peripherals.EEPROM,
            sysexc: peripherals.SYSEXC,
            hib: peripherals.HIB,
            flash_ctrl: peripherals.FLASH_CTRL,
            udma: peripherals.UDMA,
            power_control: sysctl.0,
            clocks: sysctl.1,
            delay: Delay::new(core_peripherals.SYST, &sysctl.1),
        }
    }
}
