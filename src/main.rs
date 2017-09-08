#![no_std]

extern crate bluepill_usbhid;
extern crate r0;
#[macro_use] extern crate stm32f103xx;
use bluepill_usbhid::*;

/* setup interrupts */
exception!(NMI, nmi_handler);
exception!(HARD_FAULT, hardfault_handler);
//exception!(MEM_MANAGE, mem_manage_handler);
exception!(BUS_FAULT, bus_fault_handler);
exception!(SVCALL, svc_handler);
exception!(PENDSV, pend_sv_handler);
exception!(SYS_TICK, systick_handler);
interrupt!(CAN1_RX0, usb_lp_can1_rx0_irqhandler);

fn bss_init_bugfix() {
    extern "C" {
        // Boundaries of the .bss section
        static mut _ebss: u32;
        static mut _sdata: u32;
    }
    unsafe {
        r0::zero_bss(&mut _ebss, &mut _sdata);
    }
}

fn main() {
    bss_init_bugfix();
    hal_init();
    system_clock_config();
    //gpio_init();
    usb_init();
    const MOMO: &str = "momo ";
    let mut hid_send_data: [u8; 128] = [0; 128];
    let mut hid_cmd_data: [u8; 128] = [0; 128];
    let mut hid_send_len: usize = 0;
    let mut hid_cmd_len: usize = 5;
    hid_cmd_data[..5].clone_from_slice(MOMO.as_bytes());
    loop {
        /* copy the received data from C buffer */
        unsafe {
            if hid_recv_len > 0 {
                let l = hid_recv_len as usize;
                hid_cmd_data[hid_cmd_len..hid_cmd_len+l]
                    .clone_from_slice(&hid_recv_data[..l]);
                hid_cmd_len += l;
                hid_recv_len = 0;
            }
        }

        if hid_cmd_len > 0 && hid_cmd_data[hid_cmd_len - 1] == b'\n'
        {
            hid_send_data[hid_send_len..hid_send_len+hid_cmd_len]
                .clone_from_slice(&hid_cmd_data[..hid_cmd_len]);
            hid_send_len += hid_cmd_len;
            hid_cmd_len = 5;
        }
        
        if hid_send_len > 0 {
            while !hid_send(&mut hid_send_data, hid_send_len) {
                hal_delay(100);
            }
            hid_send_len = 0;
        }
    }
}
