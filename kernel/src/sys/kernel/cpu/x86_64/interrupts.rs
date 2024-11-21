use core::arch::asm;

use lazy_static::lazy_static;
use spin::lazy;
use x86_64::{instructions::{self, hlt, interrupts}, structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode}};

use crate::{println_log, serial_println, sys::kernel::cpu::gdt};

// use super::pics::ChainedPics;

use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
    unsafe { 
        ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) 
    }
);

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.general_protection_fault.set_handler_fn(general_protection_fault_handler);


        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }

        idt[InterruptIndex::Timer.as_u8()].set_handler_fn(timer_interrupt_handler);

        idt[InterruptIndex::Keyboard.as_u8()]
            .set_handler_fn(keyboard_interrupt_handler);

        idt
    };
}

pub fn init() {
    IDT.load();
    println_log!("Loaded IDT...");

    unsafe {
        PICS.lock().initialize();
        PICS.lock().write_masks(0xfc, 0xff);
        // println_log!("PIC initialized with masks: {:02x}, {:02x}", 
        //     PICS.lock().read_masks()[0],
        //     PICS.lock().read_masks()[1]
        // );
        println_log!("weird");
    }

    unsafe { asm!("sti"); }
    println_log!("Enabled interrupts...");
}

extern "x86-interrupt" fn timer_interrupt_handler(stack_frame: InterruptStackFrame) {
    without(|| {
        println_log!("Timer interrupt!");
    });

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) {
    without(|| {
        // hlt();

        let rsp: u64;
        unsafe {
            asm!("mov {}, rsp", out(reg) rsp);
        }
        serial_println!("EXCEPTION: GENERAL PROTECTION FAULT");
        serial_println!("Error Code: {:#x}", error_code);
        serial_println!("RSP: {:#x}", rsp);
        serial_println!("Instruction Pointer: {:#x}", stack_frame.instruction_pointer.as_u64());
        serial_println!("Stack Pointer: {:#x}", stack_frame.stack_pointer.as_u64());
        serial_println!("CPU Flags: {:#x}", stack_frame.cpu_flags.bits());
        serial_println!("Code Segment: {:?}", stack_frame.code_segment);
        serial_println!("Stack Segment: {:?}", stack_frame.stack_segment);
    });
    
    panic!("EXCEPTION: GENERAL PROTECTION FAULT");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    without(|| {
        println_log!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
        serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    });
    
    loop {}
}

extern "x86-interrupt" fn double_fault_handler(stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    without(|| {
        serial_println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    });
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame)
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    without(|| {
        serial_println!("EXCEPTION: PAGE FAULT");
        serial_println!("Accessed Address: {:?}", Cr2::read());
        serial_println!("Error Code: {:?}", error_code);
        serial_println!("Stack Frame: {:#?}", stack_frame);
    });
    
    panic!("EXCEPTION: PAGE FAULT");
}

pub fn without(func: impl Fn()) {
    if are_enabled() {
        unsafe { asm!("cli"); }
        func();
        unsafe { asm!("sti"); }
    } else {
        func();
    }
}

pub fn are_enabled() -> bool {
    let flags: u64;
    unsafe {
        asm!("pushfq; pop {}", out(reg) flags);
    }
    flags & (1 << 9) != 0  // Bit 9 is the Interrupt Flag (IF)
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    let mut port = Port::new(0x60);

    serial_println!("KEYBOARD INTERRUPT");

    let scancode: u8 = unsafe { port.read() };

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}