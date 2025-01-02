/// Display Controller Commands
/// TODO clean up the dups and see if they can share names. C enum had repeats, but not allowed in rust
#[derive(Debug, Copy, Clone)]
pub enum LcdCommand {
    SWRESET = 0x01, // Software Reset
    SLPOUT = 0x11,  // Sleep Out
    PTLON = 0x12,   // Partial Display Mode On
    NORON = 0x13,   // Normal Display Mode On
    INVOFF = 0x20,  // Display Inversion Off
    INVON = 0x21,   // Display Inversion On
    ALLPOFF = 0x22, // All Pixels Off
    ALLPON = 0x23,  // All Pixels On
    GAMSET = 0x26,  // Gamma Set
    DISPOFF = 0x28, // Display Off
    DISPON = 0x29,  // Display On
    TEOFF = 0x34,   // Tearing Effect Line Off (kinda vsync)
    TEON = 0x35,    // Tearing Effect Line On (kinda vsync)
    MADCTL = 0x36,  // Display data access control
    IDMOFF = 0x38,  // Idle Mode Off
    IDMON = 0x39,   // Idle Mode On
    COLMOD = 0x3A,  // Interface Pixel Format
    GSL = 0x45,     // Get Scan Line
    // Command2_BK0
    PVGAMCTRL = 0xB0, // Positive Voltage Gamma Control
    NVGAMCTRL = 0xB1, // Negative Voltage Gamma Control
    // DGMEN = 0xB8,     // Digital Gamma Enable
    DGMLUTR = 0xB9, // Digital Gamma LUT for Red
    DGMLUTB = 0xBA, // Digital Gamma LUT for Blue
    LNESET = 0xC0,  // Display Line Setting
    PORCTRL = 0xC1, // Porch Control
    INVSET = 0xC2,  // Inversion Selection & Frame Rate Control
    RGBCTRL = 0xC3, // RGB Control
    PARCTRL = 0xC5, // Partial Mode Control
    SDIR = 0xC7,    // X-direction Control
    // PDOSET = 0xC8,  // Pseudo-Dot Inversion Driving Setting
    COLCTRL = 0xCD, // Colour Control
    SRECTRL = 0xE0, // Sunlight Readable Enhancement
    NRCTRL = 0xE1,  // Noise Reduce Control
    SECTRL = 0xE2,  // Sharpness Control
    CCCTRL = 0xE3,  // Color Calibration Control
    SKCTRL = 0xE4,  // Skin Tone Preservation Control
    // Command2_BK1
    // VHRS = 0xB0, // Vop amplitude
    // VCOMS = 0xB1,   // VCOM amplitude
    VGHSS = 0xB2,   // VGH voltage
    TESTCMD = 0xB3, // TEST command
    VGLS = 0xB5,    // VGL voltage
    VRHDV = 0xB6,   // VRH_DV voltage
    PWCTRL1 = 0xB7, // Power Control 1
    PWCTRL2 = 0xB8, // Power Control 2
    // PCLKS1 = 0xBA,  // Power pumping clock selection 1
    PCLKS2 = 0xBC, // Power pumping clock selection 2
    // PDR1 = 0xC1,   // Source pre-drive timing set 1
    // PDR2 = 0xC2,   // Source pre-drive timing set 2
    // Command2_BK3
    NVMEN = 0xC8,   // NVM enable
    NVMSET = 0xCA,  // NVM manual control
    PROMACT = 0xCC, // NVM program active
    // Other
    CND2BKxSEL = 0xFF, // Command2 BKx Select

    //Forbidden commands
    // FORBIDDEN1 = 0xE0,
    // FORBIDDEN2 = 0xE1,
    // FORBIDDEN3 = 0xE2,
    // FORBIDDEN4 = 0xE3,
    // FORBIDDEN5 = 0xE4,
    FORBIDDEN6 = 0xE5,
    FORBIDDEN7 = 0xE6,
    FORBIDDEN8 = 0xE7,
    FORBIDDEN9 = 0xE8,
    FORBIDDEN10 = 0xEB,
    FORBIDDEN11 = 0xEC,
    FORBIDDEN12 = 0xED,
}
