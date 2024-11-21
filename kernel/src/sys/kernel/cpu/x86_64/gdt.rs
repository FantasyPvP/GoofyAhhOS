use x86_64::{
    instructions::tables::load_tss, registers::segmentation::{Segment, CS, DS, ES, SS}, structures::{
        gdt::{
            Descriptor, GlobalDescriptorTable, SegmentSelector
        },
        tss::TaskStateSegment,
    }, VirtAddr
};

use lazy_static::lazy_static;

use crate::println_log;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 8;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });
            let stack_end = stack_start + STACK_SIZE.try_into().unwrap();
            stack_end
        };
        tss
    };
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        x86_64::instructions::interrupts::disable();
        let mut gdt = GlobalDescriptorTable::new();
        // Kernel code segment
        let code_selector = gdt.append(Descriptor::kernel_code_segment());
        // Kernel data segment (needed for proper interrupt handling)
        let data_selector = gdt.append(Descriptor::kernel_data_segment());
        // User segments (even if not used, helps with some hardware)
        let user_data_selector = gdt.append(Descriptor::user_data_segment());
        let user_code_selector = gdt.append(Descriptor::user_code_segment());
        // TSS segment for interrupt stack switching
        let tss_selector = gdt.append(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                data_selector,
                // user_code_selector,
                // user_data_selector,
                tss_selector,
            }
        )
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
    // user_code_selector: SegmentSelector,
    // user_data_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub fn init() {
    GDT.0.load();
    println_log!("Loaded GDT...");

    unsafe {
        // Load the segment selectors
        CS::set_reg(GDT.1.code_selector);
        load_tss(GDT.1.tss_selector);
        
        // // Set up data segments
        // DS::set_reg(GDT.1.data_selector);
        // ES::set_reg(GDT.1.data_selector);
        SS::set_reg(GDT.1.data_selector);
    }
}