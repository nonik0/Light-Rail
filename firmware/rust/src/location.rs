use avr_progmem::progmem;
use heapless::Vec;
use random_trait::Random;

use crate::{panic::set_panic_msg, panic_to_digits, random::Rng};

/// Represents the direction of travel for a train where
/// Anode is "exiting" a location from the anode and cathode
/// is vice versa.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Anode,
    Cathode,
}

/// Lightweight abstraction on top of LOCATION_DATA index
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    index: u8,
}

impl Location {
    pub fn new(index: u8) -> Self {
        // TODO: check bounds
        Self { index }
    }

    // TODO: expose as trait impl? e.g. trait LedIndex { fn index(&self) -> u8; }
    pub fn index(&self) -> u8 {
        self.index
    }

    pub fn next(&self, direction: Direction) -> (Location, Direction) {
        let loc_data = self.location_data();

        if !loc_data.is_track() {
            panic_to_digits!("LOC NOT TRACK");
        }

        let mut next_index = match direction {
            Direction::Anode => loc_data.anode_neighbor,
            Direction::Cathode => loc_data.cathode_neighbor,
        };
        let next_index_2 = match direction {
            Direction::Anode => loc_data.anode_neighbor_2,
            Direction::Cathode => loc_data.cathode_neighbor_2,
        };

        // TODO: randomly choose fork path for now
        if next_index_2 != NO_DATA && Rng::default().get_bool() {
            next_index = next_index_2;
        }

        // exit from next_loc from opposite direction of cur_loc
        let next_loc_data = LOCATION_DATA.load_at(next_index as usize);
        let next_direction = if next_loc_data.cathode_neighbor == self.index
            || next_loc_data.cathode_neighbor_2 == self.index
        {
            Direction::Anode
        } else {
            Direction::Cathode
        };

        (Location::new(next_index), next_direction)
    }

    pub fn platform_locs() -> [Location; NUM_PLATFORMS] {
        PLATFORM_INDICES
            .iter()
            .map(|index| Location { index })
            .collect::<Vec<_, NUM_PLATFORMS>>()
            .into_array()
            .unwrap()
    }

    pub fn adjacent_track(&self) -> Location {
        let loc_data = self.location_data();
        if !loc_data.is_platform() {
            panic_to_digits!("LOC NOT PLAT");
        }

        Location {
            index: loc_data.anode_neighbor,
        }
    }

    fn location_data(&self) -> LocationData {
        LOCATION_DATA.load_at(self.index as usize)
    }

    // pub fn is_platform(&self) -> bool {
    //     self.location_data().is_platform()
    // }

    // pub fn is_track(&self) -> bool {
    //     self.location_data().is_track()
    // }
}

/// Track/platform graph data is stored in a packed array of LocationData structs.
/// LocationData is built at compile time from the packed u32 array from C impl.
/// Straight tracks have two neighbors, forks have three, and crosses have four.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug, PartialEq)]
struct LocationData {
    anode_neighbor: u8,
    cathode_neighbor: u8,
    anode_neighbor_2: u8,
    cathode_neighbor_2: u8,
}

impl LocationData {
    const fn default() -> Self {
        Self {
            anode_neighbor: NO_DATA,
            cathode_neighbor: NO_DATA,
            anode_neighbor_2: NO_DATA,
            cathode_neighbor_2: NO_DATA,
        }
    }

    fn is_platform(&self) -> bool {
        is_platform(*self)
    }

    fn is_track(&self) -> bool {
        !self.is_platform()
    }
}

impl Default for LocationData {
    fn default() -> Self {
        Self::default()
    }
}

impl From<u32> for LocationData {
    fn from(data: u32) -> Self {
        Self {
            anode_neighbor: ((data >> 24) & 0xFF) as u8,
            cathode_neighbor: ((data >> 16) & 0xFF) as u8,
            anode_neighbor_2: ((data >> 8) & 0xFF) as u8,
            cathode_neighbor_2: (data & 0xFF) as u8,
        }
    }
}

// expose number without directly exposing progmem data
//pub const NUM_PLATFORMS: usize = NUM_PLATFORMS_IMPL;
pub const NUM_PLATFORMS: usize = 27;

// location data built from raw data in const fn below and stored in progmem, const fn data discarded
progmem! {
    static progmem PLATFORM_INDICES: [u8; NUM_PLATFORMS] = {
        let mut platforms = [0u8; NUM_PLATFORMS];
        let mut count = 0;
        let mut loc = 0;
        while loc < NUM_LOCATIONS {
            if is_platform(get_location_data(loc)) {
                platforms[count] = loc as u8;
                count += 1;
            }
            loc += 1;
        }
        platforms
    };

    static progmem LOCATION_DATA: [LocationData; NUM_LOCATIONS] = {
        let mut locations = [LocationData::default(); NUM_LOCATIONS];
        let mut i = 0;
        while i < NUM_LOCATIONS {
            let loc_data = get_location_data(i);

            locations[i] = loc_data;
            i += 1;
        }
        locations
    };
}

const NO_DATA: u8 = 0xFF;
const NUM_LOCATIONS: usize = is31fl3731::LED_COUNT as usize;

// platforms are encoded with all fields equal and referencing the adjacent track
const fn is_platform(location: LocationData) -> bool {
    location.anode_neighbor == location.cathode_neighbor // just check one match
}

const fn unpack_location_data(data: u32) -> LocationData {
    LocationData {
        anode_neighbor: ((data >> 24) & 0xFF) as u8,
        cathode_neighbor: ((data >> 16) & 0xFF) as u8,
        anode_neighbor_2: ((data >> 8) & 0xFF) as u8,
        cathode_neighbor_2: (data & 0xFF) as u8,
    }
}

const fn get_location_data(index: usize) -> LocationData {
    match index {
        0 => unpack_location_data(0x6111FFFF),   // track 0/0x00
        1 => unpack_location_data(0x20202020),   // platf 1/0x01
        2 => unpack_location_data(0x3003FFFF),   // track 2/0x02
        3 => unpack_location_data(0x7002FFFF),   // track 3/0x03
        4 => unpack_location_data(0x5082FFFF),   // track 4/0x04
        5 => unpack_location_data(0x60606060),   // platf 5/0x05
        6 => unpack_location_data(0x70707070),   // platf 6/0x06
        7 => unpack_location_data(0x8042FFFF),   // track 7/0x07
        8 => unpack_location_data(0x0D69FF1D),   // track 8/0x08
        9 => unpack_location_data(0x682DFFFF),   // track 9/0x09
        10 => unpack_location_data(0x4838FFFF),  // track 10/0x0A
        11 => unpack_location_data(0x3B5CFFFF),  // track 11/0x0B
        12 => unpack_location_data(0x1839FFFF),  // track 12/0x0C
        13 => unpack_location_data(0x0868FFFF),  // track 13/0x0D
        14 => unpack_location_data(0x4C4EFFFF),  // track 14/0x0E
        15 => unpack_location_data(0x88888888),  // platf 15/0x0F
        16 => unpack_location_data(0x4920FFFF),  // track 16/0x10
        17 => unpack_location_data(0x0021FFFF),  // track 17/0x11
        18 => unpack_location_data(0x2933FFFF),  // track 18/0x12
        19 => unpack_location_data(0x53535353),  // platf 19/0x13
        20 => unpack_location_data(0x5154FFFF),  // track 20/0x14
        21 => unpack_location_data(0x61616161),  // platf 21/0x15
        22 => unpack_location_data(0x5351FFFF),  // track 22/0x16
        23 => unpack_location_data(0x1E81FFFF),  // track 23/0x17
        24 => unpack_location_data(0x4F0CFFFF),  // track 24/0x18
        25 => unpack_location_data(0x594AFFFF),  // track 25/0x19
        26 => unpack_location_data(0x39393939),  // platf 26/0x1A
        27 => unpack_location_data(0x6949FFFF),  // track 27/0x1B
        28 => unpack_location_data(0x59595959),  // platf 28/0x1C
        29 => unpack_location_data(0x0848FFFF),  // track 29/0x1D
        30 => unpack_location_data(0x171FFFFF),  // track 30/0x1E
        31 => unpack_location_data(0x1E5FFFFF),  // track 31/0x1F
        32 => unpack_location_data(0x6010FFFF),  // track 32/0x20
        33 => unpack_location_data(0x3211FFFF),  // track 33/0x21
        34 => unpack_location_data(0x6230FFFF),  // track 34/0x22
        35 => unpack_location_data(0x73737373),  // platf 35/0x23
        36 => unpack_location_data(0x5276FFFF),  // track 36/0x24
        37 => unpack_location_data(0x6362FFFF),  // track 37/0x25
        38 => unpack_location_data(0x5272FFFF),  // track 38/0x26
        39 => unpack_location_data(0x8289428C),  // track 39/0x27
        40 => unpack_location_data(0x09090909),  // platf 40/0x28
        41 => unpack_location_data(0x2B12FFFF),  // track 41/0x29
        42 => unpack_location_data(0x6A6BFF3D),  // track 42/0x2A
        43 => unpack_location_data(0x294EFFFF),  // track 43/0x2B
        44 => unpack_location_data(0x5A5A5A5A),  // platf 44/0x2C
        45 => unpack_location_data(0x096AFFFF),  // track 45/0x2D
        46 => unpack_location_data(0x8D7AFFFF),  // track 46/0x2E
        47 => unpack_location_data(0x8A7AFFFF),  // track 47/0x2F
        48 => unpack_location_data(0x2202FFFF),  // track 48/0x30
        49 => unpack_location_data(0x12121212),  // platf 49/0x31
        50 => unpack_location_data(0x2162FFFF),  // track 50/0x32
        51 => unpack_location_data(0x8312FFFF),  // track 51/0x33
        52 => unpack_location_data(0x5350FFFF),  // track 52/0x34
        53 => unpack_location_data(0x6336FFFF),  // track 53/0x35
        54 => unpack_location_data(0x7335FFFF),  // track 54/0x36
        55 => unpack_location_data(0x8447FFFF),  // track 55/0x37
        56 => unpack_location_data(0x390AFFFF),  // track 56/0x38
        57 => unpack_location_data(0x380CFFFF),  // track 57/0x39
        58 => unpack_location_data(0x5B5B5B5B),  // platf 58/0x3A
        59 => unpack_location_data(0x0B4DFFFF),  // track 59/0x3B
        60 => unpack_location_data(0x5D5D5D5D),  // platf 60/0x3C
        61 => unpack_location_data(0x2A6FFFFF),  // track 61/0x3D
        62 => unpack_location_data(0x7B6FFFFF),  // track 62/0x3E
        63 => unpack_location_data(0x8B8EFFFF),  // track 63/0x3F
        64 => unpack_location_data(0x03030303),  // platf 64/0x40
        65 => unpack_location_data(0x16161616),  // platf 65/0x41
        66 => unpack_location_data(0x0727FFFF),  // track 66/0x42
        67 => unpack_location_data(0x72727272),  // platf 67/0x43
        68 => unpack_location_data(0x5455FFFF),  // track 68/0x44
        69 => unpack_location_data(0x6764FFFF),  // track 69/0x45
        70 => unpack_location_data(0x8774FFFF),  // track 70/0x46
        71 => unpack_location_data(0x8537FF83),  // track 71/0x47
        72 => unpack_location_data(0x1D0AFFFF),  // track 72/0x48
        73 => unpack_location_data(0x1B10FFFF),  // track 73/0x49
        74 => unpack_location_data(0x194EFFFF),  // track 74/0x4A
        75 => unpack_location_data(0x3B3B3B3B),  // platf 75/0x4B
        76 => unpack_location_data(0x6E0EFFFF),  // track 76/0x4C
        77 => unpack_location_data(0x3B88FFFF),  // track 77/0x4D
        78 => unpack_location_data(0x2B0E4A78),  // track 78/0x4E
        79 => unpack_location_data(0x188CFFFF),  // track 79/0x4F
        80 => unpack_location_data(0x0434FFFF),  // track 80/0x50
        81 => unpack_location_data(0x1614FFFF),  // track 81/0x51
        82 => unpack_location_data(0x2426FFFF),  // track 82/0x52
        83 => unpack_location_data(0x3416FFFF),  // track 83/0x53
        84 => unpack_location_data(0x1444FFFF),  // track 84/0x54
        85 => unpack_location_data(0x4466FFFF),  // track 85/0x55
        86 => unpack_location_data(0x87878787),  // platf 86/0x56
        87 => unpack_location_data(0x7785FFFF),  // track 87/0x57
        88 => unpack_location_data(0x0C0C0C0C),  // platf 88/0x58
        89 => unpack_location_data(0x7F19FFFF),  // track 89/0x59
        90 => unpack_location_data(0x5B8AFFFF),  // track 90/0x5A
        91 => unpack_location_data(0x5D5AFFFF),  // track 91/0x5B
        92 => unpack_location_data(0x780BFFFF),  // track 92/0x5C
        93 => unpack_location_data(0x5B6DFFFF),  // track 93/0x5D
        94 => unpack_location_data(0x8D5FFFFF),  // track 94/0x5E
        95 => unpack_location_data(0x5E1FFFFF),  // track 95/0x5F
        96 => unpack_location_data(0x6120FFFF),  // track 96/0x60
        97 => unpack_location_data(0x6000FFFF),  // track 97/0x61
        98 => unpack_location_data(0x2532FF22),  // track 98/0x62
        99 => unpack_location_data(0x2535FFFF),  // track 99/0x63
        100 => unpack_location_data(0x4565FFFF), // track 100/0x64
        101 => unpack_location_data(0x6664FFFF), // track 101/0x65
        102 => unpack_location_data(0x657655FF), // track 102/0x66
        103 => unpack_location_data(0x4574FFFF), // track 103/0x67
        104 => unpack_location_data(0x0D09FFFF), // track 104/0x68
        105 => unpack_location_data(0x1B08FFFF), // track 105/0x69
        106 => unpack_location_data(0x2D2AFFFF), // track 106/0x6A
        107 => unpack_location_data(0x8E2AFFFF), // track 107/0x6B
        108 => unpack_location_data(0x4D4D4D4D), // platf 108/0x6C
        109 => unpack_location_data(0x8B5DFFFF), // track 109/0x6D
        110 => unpack_location_data(0x8F4CFFFF), // track 110/0x6E
        111 => unpack_location_data(0x3E3DFFFF), // track 111/0x6F
        112 => unpack_location_data(0x0380FFFF), // track 112/0x70
        113 => unpack_location_data(0x51515151), // platf 113/0x71
        114 => unpack_location_data(0x7326FFFF), // track 114/0x72
        115 => unpack_location_data(0x7236FFFF), // track 115/0x73
        116 => unpack_location_data(0x4667FFFF), // track 116/0x74
        117 => unpack_location_data(0x46464646), // platf 117/0x75
        118 => unpack_location_data(0x6624FFFF), // track 118/0x76
        119 => unpack_location_data(0x8757FFFF), // track 119/0x77
        120 => unpack_location_data(0x4E5CFFFF), // track 120/0x78
        121 => unpack_location_data(0x1E1E1E1E), // platf 121/0x79
        122 => unpack_location_data(0x2E2FFFFF), // track 122/0x7A
        123 => unpack_location_data(0x883EFFFF), // track 123/0x7B
        124 => unpack_location_data(0x17171717), // platf 124/0x7C
        125 => unpack_location_data(0x7F8DFFFF), // track 125/0x7D
        126 => unpack_location_data(0x6E6E6E6E), // platf 126/0x7E
        127 => unpack_location_data(0x7D59FFFF), // track 127/0x7F
        128 => unpack_location_data(0x7007FFFF), // track 128/0x80
        129 => unpack_location_data(0x8417FFFF), // track 129/0x81
        130 => unpack_location_data(0x0427FFFF), // track 130/0x82
        131 => unpack_location_data(0x4733FFFF), // track 131/0x83
        132 => unpack_location_data(0x8137FFFF), // track 132/0x84
        133 => unpack_location_data(0x5747FFFF), // track 133/0x85
        134 => unpack_location_data(0x74747474), // platf 134/0x86
        135 => unpack_location_data(0x4677FFFF), // track 135/0x87
        136 => unpack_location_data(0x7B4DFFFF), // track 136/0x88
        137 => unpack_location_data(0x8F27FFFF), // track 137/0x89
        138 => unpack_location_data(0x2F5AFFFF), // track 138/0x8A
        139 => unpack_location_data(0x6D3FFFFF), // track 139/0x8B
        140 => unpack_location_data(0x4F27FFFF), // track 140/0x8C
        141 => unpack_location_data(0x2E5EFF7D), // track 141/0x8D
        142 => unpack_location_data(0x6B3FFFFF), // track 142/0x8E
        143 => unpack_location_data(0x896EFFFF), // track 143/0x8F
        _ => LocationData::default(),            // Default for out-of-range
    }
}
