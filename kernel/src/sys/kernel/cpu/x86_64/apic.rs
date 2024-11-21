// use core::arch::x86_64::__cpuid;



// static IA32_APIC_BASE_MSR: u32 = 0x1b;
// static IA32_APIC_BASE_MSR_BSP: u32 = 0x100;
// static IA32_APIC_BASE_MSR_ENABLE: u32 = 0x800;

// fn check_apic() -> bool {
//     let edx = unsafe { 
//         __cpuid(1,) 
//     }.edx;
//     return edx & CPU_ID_FEAT_EDX_APIC != 0
// }