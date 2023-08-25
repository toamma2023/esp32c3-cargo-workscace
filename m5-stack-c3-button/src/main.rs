use esp_idf_hal::{gpio::*, prelude::*, task};
use esp_idf_sys::{tskTaskControlBlock, TaskHandle_t};
use log::*;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::time::Duration;
use libc::{c_void};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    const BUTTON_NOTIFICATION: u32 = 1;

    let task_handle: AtomicPtr<tskTaskControlBlock> = AtomicPtr::new(std::ptr::null_mut());
    let ptr: TaskHandle_t = task::current().expect("never fail.");
    task_handle.store(ptr as *mut tskTaskControlBlock, Ordering::Relaxed);

    let peripherals = Peripherals::take().expect("never fail");
    let button_pin = peripherals.pins.gpio3;

    let mut button = PinDriver::input(button_pin).expect("never fail");
    let _ = button.set_pull(Pull::Up);
    let _ = button.set_interrupt_type(InterruptType::NegEdge);

    println!("button ready");
    unsafe {
        button.subscribe(move || {
            task::notify(task_handle.load(Ordering::Relaxed) as *mut c_void, BUTTON_NOTIFICATION);
            //println!("button pressed");
        })?;
    }

    loop {
        let res = task::wait_notification(Some(Duration::from_secs(1)));
        if let Some(BUTTON_NOTIFICATION) = res {
            info!("button pressed");
        }
    }
}
