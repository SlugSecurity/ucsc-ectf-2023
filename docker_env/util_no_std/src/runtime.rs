//! This module contains the runtime struct, which is used to perform initialization steps, manage
//! peripherals, provides random number generation, and manage an interrupt loop.

use crate::{
    button::Sw1ButtonController,
    communication::{Uart0Controller, Uart1Controller},
    eeprom::EepromController,
    hib::HibController,
    random,
};
use chacha20poly1305::Key;
use tm4c123x_hal::{
    delay::Delay,
    gpio::{
        gpiob::{PB0, PB1},
        gpiof::PF4,
        AlternateFunction, GpioExt, Input, PullUp, PushPull, AF1,
    },
    serial::{NewlineMode, Rx, RxPin, Serial, Tx, TxPin},
    sysctl::{
        Clocks, CrystalFrequency, Oscillator, PllOutputFrequency, PowerControl, Sysctl, SysctlExt,
        SystemClock,
    },
    time::Bps,
    tm4c123x::*,
};

/// Bits-per-second for UART communications.
const BPS: u32 = 57600;

/// The TX pin for UART 1.
type Uart1TxPin = PB1<AlternateFunction<AF1, PullUp>>;

/// The RX pin for UART 1.
type Uart1RxPin = PB0<AlternateFunction<AF1, PushPull>>;

/// The runtime struct.
pub struct Runtime<'a> {
    /// The EEPROM controller.
    pub eeprom_controller: EepromController<'a>,

    /// The hibernation controller.
    pub hib_controller: HibController<'a>,

    /// The SW1 button controller.
    pub sw1_button_controller: Sw1ButtonController<'a>,

    /// The controller for UART0. See the documentation for [`Uart0Controller`] for more details.
    pub uart0_controller: Uart0Controller<'a, (), ()>,

    /// The controller for UART1. See the documentation for [`Uart1Controller`] for more details.
    pub uart1_controller: Uart1Controller<'a, Uart1TxPin, Uart1RxPin>,
}

impl<'a> Runtime<'a> {
    /// Initializes the runtime.
    ///
    /// # Panics
    ///
    /// Panics if the EEPROM controller cannot be initialized.
    pub fn new(
        peripherals: &'a mut RuntimePeripherals,
        uart1_rx_key: &Key,
        uart1_tx_key: &Key,
    ) -> Self {
        random::init_rng(peripherals);

        let eeprom_controller =
            EepromController::new(&mut peripherals.eeprom, &peripherals.power_control).unwrap();

        let hib_controller = HibController::new(&mut peripherals.hib, &peripherals.power_control);

        let sw1_button_controller =
            Sw1ButtonController::new(&mut peripherals.pf4, &mut peripherals.nvic);

        let uart0_controller =
            Uart0Controller::without_key(&mut peripherals.uart0_tx, &mut peripherals.uart0_rx);

        let uart1_controller = Uart1Controller::new(
            &mut peripherals.uart1_tx,
            &mut peripherals.uart1_rx,
            uart1_rx_key,
            uart1_tx_key,
        );

        Runtime {
            eeprom_controller,
            hib_controller,
            sw1_button_controller,
            uart0_controller,
            uart1_controller,
        }
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

macro_rules! init_uart {
    ($typ:ty, $fn_name: ident, $to_call: ident) => {
        fn $fn_name<TX, RX>(
            uart: $typ,
            tx: TX,
            rx: RX,
            clocks: &Clocks,
            pc: &PowerControl,
        ) -> (Tx<$typ, TX, ()>, Rx<$typ, RX, ()>)
        where
            TX: TxPin<$typ>,
            RX: RxPin<$typ>,
        {
            Serial::$to_call(
                uart,
                tx,
                rx,
                (),
                (),
                Bps(BPS),
                NewlineMode::Binary,
                clocks,
                pc,
            )
            .split()
        }
    };
}

init_uart!(UART0, initialize_uart0, uart0);
init_uart!(UART1, initialize_uart1, uart1);

/// All peripherals and core peripherals, but with the system clock, power control, PF4 GPIO pin,
/// delay, and UART pins initialized.
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
    pub gpio_portc: GPIO_PORTC,
    pub gpio_portd: GPIO_PORTD,
    pub ssi0: SSI0,
    pub ssi1: SSI1,
    pub ssi2: SSI2,
    pub ssi3: SSI3,
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
    pub pf4: PF4<Input<PullUp>>,
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
    pub uart0_tx: Tx<UART0, (), ()>,
    pub uart0_rx: Rx<UART0, (), ()>,
    pub uart1_tx: Tx<UART1, PB1<AlternateFunction<AF1, PullUp>>, ()>,
    pub uart1_rx: Rx<UART1, PB0<AlternateFunction<AF1, PushPull>>, ()>,
}

impl From<(CorePeripherals, Peripherals)> for RuntimePeripherals {
    fn from((core_peripherals, peripherals): (CorePeripherals, Peripherals)) -> Self {
        let sysctl = initialize_sysctl(peripherals.SYSCTL.constrain());
        let (uart0_tx, uart0_rx) =
            initialize_uart0(peripherals.UART0, (), (), &sysctl.1, &sysctl.0);
        let mut portb = peripherals.GPIO_PORTB.split(&sysctl.0);
        let (uart1_tx, uart1_rx) = initialize_uart1(
            peripherals.UART1,
            portb.pb1.into_af_pull_up::<AF1>(&mut portb.control),
            portb.pb0.into_af_push_pull::<AF1>(&mut portb.control),
            &sysctl.1,
            &sysctl.0,
        );

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
            gpio_portc: peripherals.GPIO_PORTC,
            gpio_portd: peripherals.GPIO_PORTD,
            ssi0: peripherals.SSI0,
            ssi1: peripherals.SSI1,
            ssi2: peripherals.SSI2,
            ssi3: peripherals.SSI3,
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
            pf4: peripherals
                .GPIO_PORTF
                .split(&sysctl.0)
                .pf4
                .into_pull_up_input(),
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
            uart0_tx,
            uart0_rx,
            uart1_tx,
            uart1_rx,
        }
    }
}
