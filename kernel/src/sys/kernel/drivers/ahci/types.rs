
#[repr(u32)]
pub enum DeviceSignature {
    Ata = 0x00000101,
    Atapi = 0xEB140101,
    Semb = 0xC33C0101,
    Pm = 0x96690101,
    Invalid = 0x00000000,
}

#[repr(u32)]
pub enum PortSignature {
    Null = 0,
    Sata = 1,
    Semb = 2,
    Pm = 3,
    Satapi = 4,
}

impl From<u32> for PortSignature {
    fn from(value: u32) -> Self {
        match value {
            0 => PortSignature::Null,
            1 => PortSignature::Sata,
            2 => PortSignature::Semb,
            3 => PortSignature::Pm,
            4 => PortSignature::Satapi,
            _ => PortSignature::Null,
        }
    }
}

// useful constants
pub const HBA_PORT_IPM_ACTIVE: u8 = 1;
pub const HBA_PORT_DET_PRESENT: u8 = 3;

pub const HBA_PXCMD_ST: u32 = 0x0001;
pub const HBA_PXCMD_FRE: u32 = 0x0010;
pub const HBA_PXCMD_FR: u32 = 0x4000;
pub const HBA_PXCMD_CR: u32 = 0x8000;



// Following code defines different kinds of FIS specified in Serial ATA Revision 3.0.
#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum FisType {
    RegH2D = 0x27,    // Register FIS - host to device
    RegD2H = 0x34,    // Register FIS - device to host
    DmaAct = 0x39,    // DMA activate FIS - device to host
    DmaSetup = 0x41,  // DMA setup FIS - bidirectional
    Data = 0x46,      // Data FIS - bidirectional
    Bist = 0x58,      // BIST activate FIS - bidirectional
    PioSetup = 0x5F,  // PIO setup FIS - device to host
    DevBits = 0xA1,   // Set device bits FIS - device to host
}

// A host to device register FIS is used by the host to send command or control to a device.
// It contains IDE registers such as command, LBA, device, feature, count and control.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisRegH2D {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_REG_H2D
    pub pmport_c: u8,     // Port multiplier [7:4], Reserved [3:1], Command[0]
    pub command: u8,      // Command register
    pub featurel: u8,     // Feature register, 7:0
    
    // DWORD 1
    pub lba0: u8,         // LBA low register, 7:0
    pub lba1: u8,         // LBA mid register, 15:8
    pub lba2: u8,         // LBA high register, 23:16
    pub device: u8,       // Device register
    
    // DWORD 2
    pub lba3: u8,         // LBA register, 31:24
    pub lba4: u8,         // LBA register, 39:32
    pub lba5: u8,         // LBA register, 47:40
    pub featureh: u8,     // Feature register, 15:8
    
    // DWORD 3
    pub countl: u8,       // Count register, 7:0
    pub counth: u8,       // Count register, 15:8
    pub icc: u8,          // Isochronous command completion
    pub control: u8,      // Control register
    
    // DWORD 4
    pub rsv1: [u8; 4],    // Reserved
}

// A device to host register FIS is used by the device to notify the host 
// that some ATA register has changed.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisRegD2H {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_REG_D2H
    pub pmport_i: u8,     // Port multiplier [7:4], Reserved [3:2], Interrupt [1], Reserved [0]
    pub status: u8,       // Status register
    pub error: u8,        // Error register
    
    // DWORD 1
    pub lba0: u8,         // LBA low register, 7:0
    pub lba1: u8,         // LBA mid register, 15:8
    pub lba2: u8,         // LBA high register, 23:16
    pub device: u8,       // Device register
    
    // DWORD 2
    pub lba3: u8,         // LBA register, 31:24
    pub lba4: u8,         // LBA register, 39:32
    pub lba5: u8,         // LBA register, 47:40
    pub rsv2: u8,         // Reserved
    
    // DWORD 3
    pub countl: u8,       // Count register, 7:0
    pub counth: u8,       // Count register, 15:8
    pub rsv3: [u8; 2],    // Reserved
    
    // DWORD 4
    pub rsv4: [u8; 4],    // Reserved
}

// This FIS is used by the host or device to send data payload.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisData {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_DATA
    pub pmport: u8,       // Port multiplier [7:4], Reserved [3:0]
    pub rsv1: [u8; 2],    // Reserved
    
    // DWORD 1 ~ N
    pub data: [u32; 1],   // Payload
}

// This FIS is used by the device to tell the host that it's about to send 
// or ready to receive a PIO data payload.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisPioSetup {
    // DWORD 0
    pub fis_type: u8,     // FIS_TYPE_PIO_SETUP
    pub pmport_flags: u8,  // Port multiplier [7:4], Reserved [3], Data transfer direction [2], Interrupt [1], Reserved [0]
    pub status: u8,       // Status register
    pub error: u8,        // Error register
    
    // DWORD 1
    pub lba0: u8,         // LBA low register, 7:0
    pub lba1: u8,         // LBA mid register, 15:8
    pub lba2: u8,         // LBA high register, 23:16
    pub device: u8,       // Device register
    
    // DWORD 2
    pub lba3: u8,         // LBA register, 31:24
    pub lba4: u8,         // LBA register, 39:32
    pub lba5: u8,         // LBA register, 47:40
    pub rsv2: u8,         // Reserved
    
    // DWORD 3
    pub countl: u8,       // Count register, 7:0
    pub counth: u8,       // Count register, 15:8
    pub rsv3: u8,         // Reserved
    pub e_status: u8,     // New value of status register
    
    // DWORD 4
    pub tc: u16,          // Transfer count
    pub rsv4: [u8; 2],    // Reserved
}

// DMA Setup - Device to Host
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct FisDmaSetup {
    // DWORD 0
    pub fis_type: u8,         // FIS_TYPE_DMA_SETUP
    pub pmport_flags: u8,     // Port multiplier [7:4], Reserved [3], Data transfer direction [2], Interrupt [1], Auto-activate [0]
    pub rsved: [u8; 2],       // Reserved
    
    // DWORD 1&2
    pub dma_buffer_id: u64,   // DMA Buffer Identifier
    
    // DWORD 3
    pub rsvd: u32,            // More reserved
    
    // DWORD 4
    pub dma_buf_offset: u32,  // Byte offset into buffer. First 2 bits must be 0
    
    // DWORD 5
    pub transfer_count: u32,  // Number of bytes to transfer. Bit 0 must be 0
    
    // DWORD 6
    pub resvd: u32,          // Reserved
}

// HBA Memory Registers Structure
#[repr(C)]
#[derive(Debug)]
pub struct HbaMem {
    // 0x00 - 0x2B, Generic Host Control
    pub cap: u32,             // 0x00, Host capability
    pub ghc: u32,             // 0x04, Global host control
    pub is: u32,              // 0x08, Interrupt status
    pub pi: u32,              // 0x0C, Port implemented
    pub vs: u32,              // 0x10, Version
    pub ccc_ctl: u32,         // 0x14, Command completion coalescing control
    pub ccc_pts: u32,         // 0x18, Command completion coalescing ports
    pub em_loc: u32,          // 0x1C, Enclosure management location
    pub em_ctl: u32,          // 0x20, Enclosure management control
    pub cap2: u32,            // 0x24, Host capabilities extended
    pub bohc: u32,            // 0x28, BIOS/OS handoff control and status
    
    // 0x2C - 0x9F, Reserved
    pub rsv: [u8; 0xA0-0x2C],
    
    // 0xA0 - 0xFF, Vendor specific registers
    pub vendor: [u8; 0x100-0xA0],
    
    // 0x100 - 0x10FF, Port control registers
    pub ports: [HbaPort; 1],  // 1 ~ 32
}

// HBA Port Structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HbaPort {
    pub clb: u32,             // 0x00, command list base address, 1K-byte aligned
    pub clbu: u32,            // 0x04, command list base address upper 32 bits
    pub fb: u32,              // 0x08, FIS base address, 256-byte aligned
    pub fbu: u32,             // 0x0C, FIS base address upper 32 bits
    pub is: u32,              // 0x10, interrupt status
    pub ie: u32,              // 0x14, interrupt enable
    pub cmd: u32,             // 0x18, command and status
    pub rsv0: u32,            // 0x1C, Reserved
    pub tfd: u32,             // 0x20, task file data
    pub sig: u32,             // 0x24, signature
    pub ssts: u32,            // 0x28, SATA status (SCR0:SStatus)
    pub sctl: u32,            // 0x2C, SATA control (SCR2:SControl)
    pub serr: u32,            // 0x30, SATA error (SCR1:SError)
    pub sact: u32,            // 0x34, SATA active (SCR3:SActive)
    pub ci: u32,              // 0x38, command issue
    pub sntf: u32,            // 0x3C, SATA notification (SCR4:SNotification)
    pub fbs: u32,             // 0x40, FIS-based switch control
    pub rsv1: [u32; 11],      // 0x44 ~ 0x6F, Reserved
    pub vendor: [u32; 4],     // 0x70 ~ 0x7F, vendor specific
}

// HBA FIS Structure
#[repr(C)]
#[derive(Debug)]
pub struct HbaFis {
    // 0x00
    pub dsfis: FisDmaSetup,   // DMA Setup FIS
    pub pad0: [u8; 4],
    
    // 0x20
    pub psfis: FisPioSetup,   // PIO Setup FIS
    pub pad1: [u8; 12],
    
    // 0x40
    pub rfis: FisRegD2H,      // Register – Device to Host FIS
    pub pad2: [u8; 4],
    
    // 0x58
    pub sdbfis: [u8; 8],      // Set Device Bit FIS
    
    // 0x60
    pub ufis: [u8; 64],
    
    // 0xA0
    pub rsv: [u8; 0x100-0xA0],
}

// Command List Structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HbaCmdHeader {
    // DW0
    pub flags: u16,           // Command FIS length and flags
    pub prdtl: u16,          // Physical region descriptor table length in entries
    
    // DW1
    pub prdbc: u32,          // Physical region descriptor byte count transferred
    
    // DW2, 3
    pub ctba: u32,           // Command table descriptor base address
    pub ctbau: u32,          // Command table descriptor base address upper 32 bits
    
    // DW4 - 7
    pub rsv1: [u32; 4],      // Reserved
}

// Command Table Structure
#[repr(C)]
#[derive(Debug)]
pub struct HbaCmdTbl {
    // 0x00
    pub cfis: [u8; 64],      // Command FIS
    
    // 0x40
    pub acmd: [u8; 16],      // ATAPI command, 12 or 16 bytes
    
    // 0x50
    pub rsv: [u8; 48],       // Reserved
    
    // 0x80
    pub prdt_entry: [HbaPrdtEntry; 1], // Physical region descriptor table entries, 0 ~ 65535
}

// Physical Region Descriptor Table Entry
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct HbaPrdtEntry {
    pub dba: u32,            // Data base address
    pub dbau: u32,           // Data base address upper 32 bits
    pub rsv0: u32,           // Reserved
    
    // DW3
    pub dbc_and_flags: u32,  // Byte count and flags
}