use avr_progmem::progmem;
use random_trait::Random;

use crate::random::Rng;
const NO_DATA: u8 = 0xFF;

// TODO: here for now bc train and platform need it
// TODO: is Cargo best name? or should it be like LocationContents?
#[derive(Clone, Copy, Debug)]
pub enum Cargo {
    Empty = 0,
    Full = 1,
}

// TODO: refine naming/usage
// Used to encapsulate location updates from callees (Train and Platform) back to
// caller (Game) that owns the LED driver driver instance and can update
// the display. This seems like a good alternative to avoid callback and lifetime hell.
#[derive(Clone, Copy, Debug)]
pub struct LocationUpdate {
    pub loc: Location,
    pub opt_cargo: Option<Cargo>,
}

impl LocationUpdate {
    pub fn new(loc: Location, opt_cargo: Option<Cargo>) -> Self {
        Self { loc, opt_cargo }
    }
}

/// Represents the direction of travel for a train where
/// Anode is "exiting" a location from the anode and cathode
/// is vice versa.
#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Anode,
    Cathode,
}

/// Lightweight abstraction on top of LOCATION_DATA index
// TODO: index should not be public
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Location {
    pub index: u8,
}

impl Location {
    pub fn next(&self, direction: Direction) -> (Location, Direction) {
        let loc_data = self.get_data();

        if !loc_data.is_track() {
            panic!("LOC NOT TRACK");
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
        let next_loc_data = LOCATION_RAW_DATA.load_at(next_index as usize);
        let next_direction = if next_loc_data.cathode_neighbor == self.index
            || next_loc_data.cathode_neighbor_2 == self.index
        {
            Direction::Anode
        } else {
            Direction::Cathode
        };

        (Location { index: next_index }, next_direction)
    }

    pub fn adjacent_track(&self) -> Location {
        let loc_data = self.get_data();

        if !loc_data.is_platform() {
            panic!("LOC NOT PLATFORM");
        }

        Location {
            index: loc_data.anode_neighbor,
        }
    }

    fn get_data(&self) -> LocationData {
        LOCATION_RAW_DATA.load_at(self.index as usize)
    }

    pub fn is_platform(&self) -> bool {
        self.get_data().is_platform()
    }

    pub fn is_track(&self) -> bool {
        self.get_data().is_track()
    }
}

/// Track/platform graph data is stored in a packed array of LocationData structs.
/// LocationData is built at compile time from the packed u32 array from C impl.
/// Straight tracks have two neighbors, forks have three, and platforms have four.
#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct LocationData {
    anode_neighbor: u8,
    cathode_neighbor: u8,
    anode_neighbor_2: u8,
    cathode_neighbor_2: u8,
}

impl LocationData {
    fn is_platform(&self) -> bool {
        is_platform(*self)
    }

    fn is_track(&self) -> bool {
        !self.is_platform()
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

// platforms are encoded with all fields equal and referencing the adjacent track
const fn is_platform(location: LocationData) -> bool {
    location.anode_neighbor == location.cathode_neighbor // just check one match
}


pub const NUM_PLATFORMS: usize = 10;
pub const PLATFORM_INDICES: [usize; NUM_PLATFORMS] = [1, 5, 6, 15, 19, 21, 26, 28, 39, 44];
// pub const NUM_PLATFORMS: usize = {
//     let mut count = 0;
//     let mut loc = 0;
//     while loc < NUM_LOCATIONS {
//         let loc_data = LOCATION_RAW_DATA.load_at(loc);
//         if is_platform(LocationData::from(LOCATION_RAW_DATA.load_at(loc).into
//         ())) {
//             count += 1;
//         }
//         loc += 1;
//     }
//     count
// };
// pub const PLATFORM_INDICES: [usize; NUM_PLATFORMS] = {
//     let mut platforms = [0; NUM_PLATFORMS]; // Replace with meaningful value if possible
//     let mut count = 0;
//     let mut loc = 0;
//     while loc < NUM_LOCATIONS {
//         if is_platform(LOCATION_RAW_DATA.load_at(loc)) {
//             platforms[count] = loc;
//             count += 1;
//         }
//         loc += 1;
//     }
//     platforms
// };
const NUM_LOCATIONS: usize = is31fl3731::LED_COUNT as usize;

progmem! {
    static progmem LOCATION_RAW_DATA: [u32; NUM_LOCATIONS] = [
        0x6111FFFF, // track 0/0x00
        0x20202020, // track 1/0x01 PLATFORM
        0x3003FFFF, // track 2/0x02
        0x7002FFFF, // track 3/0x03
        0x5082FFFF, // track 4/0x04
        0x60606060, // track 5/0x05 PLATFORM
        0x70707070, // track 6/0x06 PLATFORM
        0x8042FFFF, // track 7/0x07
        0x0D69FF1D, // track 8/0x08
        0x682DFFFF, // track 9/0x09
        0x4838FFFF, // track 10/0x0A
        0x3B5CFFFF, // track 11/0x0B
        0x1839FFFF, // track 12/0x0C
        0x0868FFFF, // track 13/0x0D
        0x4C4EFFFF, // track 14/0x0E
        0x88888888, // track 15/0x0F PLATFORM
        0x4920FFFF, // track 16/0x10
        0x0021FFFF, // track 17/0x11
        0x2933FFFF, // track 18/0x12
        0x53535353, // track 19/0x13 PLATFORM
        0x5154FFFF, // track 20/0x14
        0x61616161, // track 21/0x15 PLATFORM
        0x5351FFFF, // track 22/0x16
        0x1E81FFFF, // track 23/0x17
        0x4F0CFFFF, // track 24/0x18
        0x594AFFFF, // track 25/0x19
        0x39393939, // track 26/0x1A PLATFORM
        0x6949FFFF, // track 27/0x1B
        0x59595959, // track 28/0x1C PLATFORM
        0x0848FFFF, // track 29/0x1D
        0x171FFFFF, // track 30/0x1E
        0x1E5FFFFF, // track 31/0x1F
        0x6010FFFF, // track 32/0x20
        0x3211FFFF, // track 33/0x21
        0x6230FFFF, // track 34/0x22
        0x73737373, // track 35/0x23 PLATFORM
        0x5276FFFF, // track 36/0x24
        0x6362FFFF, // track 37/0x25
        0x5272FFFF, // track 38/0x26
        0x8289428C, // track 39/0x27
        0x09090909, // track 40/0x28 PLATFORM
        0x2B12FFFF, // track 41/0x29
        0x6A6BFFFF, // track 42/0x2A
        0x294EFFFF, // track 43/0x2B
        0x5A5A5A5A, // track 44/0x2C PLATFORM
        0x096AFFFF, // track 45/0x2D
        0x8D7AFFFF, // track 46/0x2E
        0x8A7AFFFF, // track 47/0x2F
        0x2202FFFF, // track 48/0x30
        0x12121212, // track 49/0x31 PLATFORM
        0x2162FFFF, // track 50/0x32
        0x8312FFFF, // track 51/0x33
        0x5350FFFF, // track 52/0x34
        0x6336FFFF, // track 53/0x35
        0x7335FFFF, // track 54/0x36
        0x8447FFFF, // track 55/0x37
        0x390AFFFF, // track 56/0x38
        0x380CFFFF, // track 57/0x39
        0x5B5B5B5B, // track 58/0x3A PLATFORM
        0x0B4DFFFF, // track 59/0x3B
        0x5D5D5D5D, // track 60/0x3C PLATFORM
        0x2A6FFFFF, // track 61/0x3D
        0x7B6FFFFF, // track 62/0x3E
        0x8B8EFFFF, // track 63/0x3F
        0x03030303, // track 64/0x40 PLATFORM
        0x16161616, // track 65/0x41 PLATFORM
        0x0727FFFF, // track 66/0x42
        0x72727272, // track 67/0x43 PLATFORM
        0x5455FFFF, // track 68/0x44
        0x6764FFFF, // track 69/0x45
        0x8774FFFF, // track 70/0x46
        0x8537FF83, // track 71/0x47
        0x1D0AFFFF, // track 72/0x48
        0x1B10FFFF, // track 73/0x49
        0x194EFFFF, // track 74/0x4A
        0x3B3B3B3B, // track 75/0x4B PLATFORM
        0x6E0EFFFF, // track 76/0x4C
        0x3B88FFFF, // track 77/0x4D
        0x2B0E4A78, // track 78/0x4E
        0x188CFFFF, // track 79/0x4F
        0x0434FFFF, // track 80/0x50
        0x1614FFFF, // track 81/0x51
        0x2426FFFF, // track 82/0x52
        0x3416FFFF, // track 83/0x53
        0x1444FFFF, // track 84/0x54
        0x4466FFFF, // track 85/0x55
        0x87878787, // track 86/0x56 PLATFORM
        0x7785FFFF, // track 87/0x57
        0x0C0C0C0C, // track 88/0x58 PLATFORM
        0x7F19FFFF, // track 89/0x59
        0x5B8AFFFF, // track 90/0x5A
        0x5D5AFFFF, // track 91/0x5B
        0x780BFFFF, // track 92/0x5C
        0x5B6DFFFF, // track 93/0x5D
        0x8D5FFFFF, // track 94/0x5E
        0x5E1FFFFF, // track 95/0x5F
        0x6120FFFF, // track 96/0x60
        0x6000FFFF, // track 97/0x61
        0x2532FF22, // track 98/0x62
        0x2535FFFF, // track 99/0x63
        0x4565FFFF, // track 100/0x64
        0x6664FFFF, // track 101/0x65
        0x657655FF, // track 102/0x66
        0x4574FFFF, // track 103/0x67
        0x0D09FFFF, // track 104/0x68
        0x1B08FFFF, // track 105/0x69
        0x2D2AFFFF, // track 106/0x6A
        0x8E2AFFFF, // track 107/0x6B
        0x4D4D4D4D, // track 108/0x6C PLATFORM
        0x8B5DFFFF, // track 109/0x6D
        0x8F4CFFFF, // track 110/0x6E
        0x3E3DFFFF, // track 111/0x6F
        0x0380FFFF, // track 112/0x70
        0x51515151, // track 113/0x71 PLATFORM
        0x7326FFFF, // track 114/0x72
        0x7236FFFF, // track 115/0x73
        0x4667FFFF, // track 116/0x74
        0x46464646, // track 117/0x75 PLATFORM
        0x6624FFFF, // track 118/0x76
        0x8757FFFF, // track 119/0x77
        0x4E5CFFFF, // track 120/0x78
        0x1E1E1E1E, // track 121/0x79 PLATFORM
        0x2E2FFFFF, // track 122/0x7A
        0x883EFFFF, // track 123/0x7B
        0x17171717, // track 124/0x7C PLATFORM
        0x7F8DFFFF, // track 125/0x7D
        0x6E6E6E6E, // track 126/0x7E PLATFORM
        0x7D59FFFF, // track 127/0x7F
        0x7007FFFF, // track 128/0x80
        0x8417FFFF, // track 129/0x81
        0x0427FFFF, // track 130/0x82
        0x4733FFFF, // track 131/0x83
        0x8137FFFF, // track 132/0x84
        0x5747FFFF, // track 133/0x85
        0x74747474, // track 134/0x86 PLATFORM
        0x4677FFFF, // track 135/0x87
        0x7B4DFFFF, // track 136/0x88
        0x8F27FFFF, // track 137/0x89
        0x2F5AFFFF, // track 138/0x8A
        0x6D3FFFFF, // track 139/0x8B
        0x4F27FFFF, // track 140/0x8C
        0x2E5EFF7D, // track 141/0x8D
        0x6B3FFFFF, // track 142/0x8E
        0x896EFFFF, // track 143/0x8F
    ];
}

/*
const LOCATION_DATA: [LocationData; NUM_LOCATIONS] = {
    const fn unpack_location_data(data: u32) -> LocationData {
        LocationData {
            anode_neighbor: ((data >> 24) & 0xFF) as u8,
            cathode_neighbor: ((data >> 16) & 0xFF) as u8,
            anode_neighbor_2: ((data >> 8) & 0xFF) as u8,
            cathode_neighbor_2: (data & 0xFF) as u8,
        }
    }

    [
        unpack_location_data(0x6111FFFF), // track 0/0x00
        unpack_location_data(0x20202020), // platf 1/0x01 PLATFORM
        unpack_location_data(0x3003FFFF), // track 2/0x02
        unpack_location_data(0x7002FFFF), // track 3/0x03
        unpack_location_data(0x5082FFFF), // track 4/0x04
        unpack_location_data(0x60606060), // platf 5/0x05 PLATFORM
        unpack_location_data(0x70707070), // platf 6/0x06 PLATFORM
        unpack_location_data(0x8042FFFF), // track 7/0x07
        unpack_location_data(0x0D69FF1D), // track 8/0x08
        unpack_location_data(0x682DFFFF), // track 9/0x09
        unpack_location_data(0x4838FFFF), // track 10/0x0A
        unpack_location_data(0x3B5CFFFF), // track 11/0x0B
        unpack_location_data(0x1839FFFF), // track 12/0x0C
        unpack_location_data(0x0868FFFF), // track 13/0x0D
        unpack_location_data(0x4C4EFFFF), // track 14/0x0E
        unpack_location_data(0x88888888), // platf 15/0x0F PLATFORM
        unpack_location_data(0x4920FFFF), // track 16/0x10
        unpack_location_data(0x0021FFFF), // track 17/0x11
        unpack_location_data(0x2933FFFF), // track 18/0x12
        unpack_location_data(0x53535353), // platf 19/0x13 PLATFORM
        unpack_location_data(0x5154FFFF), // track 20/0x14
        unpack_location_data(0x61616161), // track 21/0x15 PLATFORM
        unpack_location_data(0x5351FFFF), // track 22/0x16
        unpack_location_data(0x1E81FFFF), // track 23/0x17
        unpack_location_data(0x4F0CFFFF), // track 24/0x18
        unpack_location_data(0x594AFFFF), // track 25/0x19
        unpack_location_data(0x39393939), // platf 26/0x1A PLATFORM
        unpack_location_data(0x6949FFFF), // track 27/0x1B
        unpack_location_data(0x59595959), // platf 28/0x1C PLATFORM
        unpack_location_data(0x0848FFFF), // track 29/0x1D
        unpack_location_data(0x171FFFFF), // track 30/0x1E
        unpack_location_data(0x1E5FFFFF), // track 31/0x1F
        unpack_location_data(0x6010FFFF), // track 32/0x20
        unpack_location_data(0x3211FFFF), // track 33/0x21
        unpack_location_data(0x6230FFFF), // track 34/0x22
        unpack_location_data(0x73737373), // platf 35/0x23 PLATFORM
        unpack_location_data(0x5276FFFF), // track 36/0x24
        unpack_location_data(0x6362FFFF), // track 37/0x25
        unpack_location_data(0x5272FFFF), // track 38/0x26
        unpack_location_data(0x8289428C), // track 39/0x27
        unpack_location_data(0x09090909), // platf 40/0x28 PLATFORM
        unpack_location_data(0x2B12FFFF), // track 41/0x29
        unpack_location_data(0x6A6BFFFF), // track 42/0x2A
        unpack_location_data(0x294EFFFF), // track 43/0x2B
        unpack_location_data(0x5A5A5A5A), // platf 44/0x2C PLATFORM
        unpack_location_data(0x096AFFFF), // track 45/0x2D
        unpack_location_data(0x8D7AFFFF), // track 46/0x2E
        unpack_location_data(0x8A7AFFFF), // track 47/0x2F
        unpack_location_data(0x2202FFFF), // track 48/0x30
        unpack_location_data(0x12121212), // platf 49/0x31 PLATFORM
        unpack_location_data(0x2162FFFF), // track 50/0x32
        unpack_location_data(0x8312FFFF), // track 51/0x33
        unpack_location_data(0x5350FFFF), // track 52/0x34
        unpack_location_data(0x6336FFFF), // track 53/0x35
        unpack_location_data(0x7335FFFF), // track 54/0x36
        unpack_location_data(0x8447FFFF), // track 55/0x37
        unpack_location_data(0x390AFFFF), // track 56/0x38
        unpack_location_data(0x380CFFFF), // track 57/0x39
        unpack_location_data(0x5B5B5B5B), // platf 58/0x3A PLATFORM
        unpack_location_data(0x0B4DFFFF), // track 59/0x3B
        unpack_location_data(0x5D5D5D5D), // platf 60/0x3C PLATFORM
        unpack_location_data(0x2A6FFFFF), // track 61/0x3D
        unpack_location_data(0x7B6FFFFF), // track 62/0x3E
        unpack_location_data(0x8B8EFFFF), // track 63/0x3F
        unpack_location_data(0x03030303), // platf 64/0x40 PLATFORM
        unpack_location_data(0x16161616), // platf 65/0x41 PLATFORM
        unpack_location_data(0x0727FFFF), // track 66/0x42
        unpack_location_data(0x72727272), // platf 67/0x43 PLATFORM
        unpack_location_data(0x5455FFFF), // track 68/0x44
        unpack_location_data(0x6764FFFF), // track 69/0x45
        unpack_location_data(0x8774FFFF), // track 70/0x46
        unpack_location_data(0x8537FF83), // track 71/0x47
        unpack_location_data(0x1D0AFFFF), // track 72/0x48
        unpack_location_data(0x1B10FFFF), // track 73/0x49
        unpack_location_data(0x194EFFFF), // track 74/0x4A
        unpack_location_data(0x3B3B3B3B), // platf 75/0x4B PLATFORM
        unpack_location_data(0x6E0EFFFF), // track 76/0x4C
        unpack_location_data(0x3B88FFFF), // track 77/0x4D
        unpack_location_data(0x2B0E4A78), // track 78/0x4E
        unpack_location_data(0x188CFFFF), // track 79/0x4F
        unpack_location_data(0x0434FFFF), // track 80/0x50
        unpack_location_data(0x1614FFFF), // track 81/0x51
        unpack_location_data(0x2426FFFF), // track 82/0x52
        unpack_location_data(0x3416FFFF), // track 83/0x53
        unpack_location_data(0x1444FFFF), // track 84/0x54
        unpack_location_data(0x4466FFFF), // track 85/0x55
        unpack_location_data(0x87878787), // platf 86/0x56 PLATFORM
        unpack_location_data(0x7785FFFF), // track 87/0x57
        unpack_location_data(0x0C0C0C0C), // platf 88/0x58 PLATFORM
        unpack_location_data(0x7F19FFFF), // track 89/0x59
        unpack_location_data(0x5B8AFFFF), // track 90/0x5A
        unpack_location_data(0x5D5AFFFF), // track 91/0x5B
        unpack_location_data(0x780BFFFF), // track 92/0x5C
        unpack_location_data(0x5B6DFFFF), // track 93/0x5D
        unpack_location_data(0x8D5FFFFF), // track 94/0x5E
        unpack_location_data(0x5E1FFFFF), // track 95/0x5F
        unpack_location_data(0x6120FFFF), // track 96/0x60
        unpack_location_data(0x6000FFFF), // track 97/0x61
        unpack_location_data(0x2532FF22), // track 98/0x62
        unpack_location_data(0x2535FFFF), // track 99/0x63
        unpack_location_data(0x4565FFFF), // track 100/0x64
        unpack_location_data(0x6664FFFF), // track 101/0x65
        unpack_location_data(0x657655FF), // track 102/0x66
        unpack_location_data(0x4574FFFF), // track 103/0x67
        unpack_location_data(0x0D09FFFF), // track 104/0x68
        unpack_location_data(0x1B08FFFF), // track 105/0x69
        unpack_location_data(0x2D2AFFFF), // track 106/0x6A
        unpack_location_data(0x8E2AFFFF), // track 107/0x6B
        unpack_location_data(0x4D4D4D4D), // platf 108/0x6C PLATFORM
        unpack_location_data(0x8B5DFFFF), // track 109/0x6D
        unpack_location_data(0x8F4CFFFF), // track 110/0x6E
        unpack_location_data(0x3E3DFFFF), // track 111/0x6F
        unpack_location_data(0x0380FFFF), // track 112/0x70
        unpack_location_data(0x51515151), // platf 113/0x71 PLATFORM
        unpack_location_data(0x7326FFFF), // track 114/0x72
        unpack_location_data(0x7236FFFF), // track 115/0x73
        unpack_location_data(0x4667FFFF), // track 116/0x74
        unpack_location_data(0x46464646), // platf 117/0x75 PLATFORM
        unpack_location_data(0x6624FFFF), // track 118/0x76
        unpack_location_data(0x8757FFFF), // track 119/0x77
        unpack_location_data(0x4E5CFFFF), // track 120/0x78
        unpack_location_data(0x1E1E1E1E), // platf 121/0x79 PLATFORM
        unpack_location_data(0x2E2FFFFF), // track 122/0x7A
        unpack_location_data(0x883EFFFF), // track 123/0x7B
        unpack_location_data(0x17171717), // platf 124/0x7C PLATFORM
        unpack_location_data(0x7F8DFFFF), // track 125/0x7D
        unpack_location_data(0x6E6E6E6E), // platf 126/0x7E PLATFORM
        unpack_location_data(0x7D59FFFF), // track 127/0x7F
        unpack_location_data(0x7007FFFF), // track 128/0x80
        unpack_location_data(0x8417FFFF), // track 129/0x81
        unpack_location_data(0x0427FFFF), // track 130/0x82
        unpack_location_data(0x4733FFFF), // track 131/0x83
        unpack_location_data(0x8137FFFF), // track 132/0x84
        unpack_location_data(0x5747FFFF), // track 133/0x85
        unpack_location_data(0x74747474), // platf 134/0x86 PLATFORM
        unpack_location_data(0x4677FFFF), // track 135/0x87
        unpack_location_data(0x7B4DFFFF), // track 136/0x88
        unpack_location_data(0x8F27FFFF), // track 137/0x89
        unpack_location_data(0x2F5AFFFF), // track 138/0x8A
        unpack_location_data(0x6D3FFFFF), // track 139/0x8B
        unpack_location_data(0x4F27FFFF), // track 140/0x8C
        unpack_location_data(0x2E5EFF7D), // track 141/0x8D
        unpack_location_data(0x6B3FFFFF), // track 142/0x8E
        unpack_location_data(0x896EFFFF), // track 143/0x8F
    ]
};
*/