use std::os::raw::c_int;

const FREENOVE_DVP_PINS: DvpPins = DvpPins {
    pwdn: -1,
    rst: -1,
    xclk: 21,
    pclk: 22,
    vsync: 25,
    href: 23,
    sda: 26,
    scl: 27,
    d7: 35,
    d6: 34,
    d5: 39,
    d4: 36,
    d3: 19,
    d2: 18,
    d1: 5,
    d0: 4,
};

const AITHINKER_DVP_PINS: DvpPins = DvpPins {
    pwdn: 32,
    rst: -1,
    xclk: 0,
    pclk: 22,
    vsync: 25,
    href: 23,
    sda: 26,
    scl: 27,
    d7: 35,
    d6: 34,
    d5: 39,
    d4: 36,
    d3: 21,
    d2: 19,
    d1: 18,
    d0: 5,
};

// Pin assignment for MIPI interface
pub struct MipiPins {
    // Power down
    pub pwdn: c_int,
    // Reset
    pub rst: c_int,
    // Master clock
    pub xclk: c_int,
    // SDA two-wire line
    pub sda: c_int,
    // SCLK two-wire line
    pub scl: c_int,

    // Pixel data lines
    pub d9: c_int,
    pub d8: c_int,
    pub d7: c_int,
    pub d6: c_int,
    pub d5: c_int,
    pub d4: c_int,
}

// Pin assignment for DVP interface
#[derive(Clone, Copy)]
pub struct DvpPins {
    // Power down
    pub pwdn: c_int,
    // Sensor reset
    pub rst: c_int,
    // Master clock
    pub xclk: c_int,
    // Pixel clock
    pub pclk: c_int,
    // Frame valid (active high: indicates active frame)
    pub vsync: c_int,
    // Pixels valid (active high: indicates active pixels)
    pub href: c_int,
    // SDA two-wire line
    pub sda: c_int,
    // SCLK two-wire line
    pub scl: c_int,

    // Pixel data lines
    pub d7: c_int,
    pub d6: c_int,
    pub d5: c_int,
    pub d4: c_int,
    pub d3: c_int,
    pub d2: c_int,
    pub d1: c_int,
    pub d0: c_int,
}

#[derive(Debug)]
pub enum Board {
    Freenove,
    AIThinker,
}

impl Board {
    pub fn dvp_pins(self) -> DvpPins {
        match self {
            Board::Freenove => FREENOVE_DVP_PINS,
            Board::AIThinker => AITHINKER_DVP_PINS,
        }
    }
}
