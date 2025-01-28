#[repr(C, packed)]
#[derive(Clone, Copy, Debug)]
struct Track {
    anode_next_loc: u8,
    cathode_next_loc: u8,
    anode_next_loc2: u8,
    cathode_next_loc2: u8,
}

// TODO: how to label track and platform LEDss generically?
// location gives the LED number (0-143)

const PLATFORMS: [u8; 27] = [
    0x01, 0x05, 0x06, 0x0F, 0x13, 0x15, 0x1A, 0x1C, 0x23,
    0x28, 0x2C, 0x31, 0x3A, 0x3C, 0x40, 0x41, 0x43, 0x4B,
    0x56, 0x58, 0x6C, 0x71, 0x75, 0x79, 0x7C, 0x7E, 0x86
];
const PLATFORM_COUNT: usize = PLATFORMS.len();
const LED_COUNT: usize = 144;

const fn unpack_track(data: u32) -> Track {
    Track {
        anode_next_loc: ((data >> 24) & 0xFF) as u8,
        cathode_next_loc: ((data >> 16) & 0xFF) as u8,
        anode_next_loc2: ((data >> 8) & 0xFF) as u8,
        cathode_next_loc2: (data & 0xFF) as u8,
    }
}

// TODO: name
// encodes graph of track layout, index is the led index for IS31
const TRACKS: [Track; LED_COUNT] = [
    unpack_track(0x6111FFFF), // track 0/0x00
    unpack_track(0x20202020), // track 1/0x01 PLATFORM
    unpack_track(0x3003FFFF), // track 2/0x02
    unpack_track(0x7002FFFF), // track 3/0x03
    unpack_track(0x5082FFFF), // track 4/0x04
    unpack_track(0x60606060), // track 5/0x05 PLATFORM
    unpack_track(0x70707070), // track 6/0x06 PLATFORM
    unpack_track(0x8042FFFF), // track 7/0x07
    unpack_track(0x0D69FF1D), // track 8/0x08
    unpack_track(0x682DFFFF), // track 9/0x09
    unpack_track(0x4838FFFF), // track 10/0x0A
    unpack_track(0x3B5CFFFF), // track 11/0x0B
    unpack_track(0x1839FFFF), // track 12/0x0C
    unpack_track(0x0868FFFF), // track 13/0x0D
    unpack_track(0x4C4EFFFF), // track 14/0x0E
    unpack_track(0x88888888), // track 15/0x0F PLATFORM
    unpack_track(0x4920FFFF), // track 16/0x10
    unpack_track(0x0021FFFF), // track 17/0x11
    unpack_track(0x2933FFFF), // track 18/0x12
    unpack_track(0x53535353), // track 19/0x13 PLATFORM
    unpack_track(0x5154FFFF), // track 20/0x14
    unpack_track(0x61616161), // track 21/0x15 PLATFORM
    unpack_track(0x5351FFFF), // track 22/0x16
    unpack_track(0x1E81FFFF), // track 23/0x17
    unpack_track(0x4F0CFFFF), // track 24/0x18
    unpack_track(0x594AFFFF), // track 25/0x19
    unpack_track(0x39393939), // track 26/0x1A PLATFORM
    unpack_track(0x6949FFFF), // track 27/0x1B
    unpack_track(0x59595959), // track 28/0x1C PLATFORM
    unpack_track(0x0848FFFF), // track 29/0x1D
    unpack_track(0x171FFFFF), // track 30/0x1E
    unpack_track(0x1E5FFFFF), // track 31/0x1F
    unpack_track(0x6010FFFF), // track 32/0x20
    unpack_track(0x3211FFFF), // track 33/0x21
    unpack_track(0x6230FFFF), // track 34/0x22
    unpack_track(0x73737373), // track 35/0x23 PLATFORM
    unpack_track(0x5276FFFF), // track 36/0x24
    unpack_track(0x6362FFFF), // track 37/0x25
    unpack_track(0x5272FFFF), // track 38/0x26
    unpack_track(0x8289428C), // track 39/0x27
    unpack_track(0x09090909), // track 40/0x28 PLATFORM
    unpack_track(0x2B12FFFF), // track 41/0x29
    unpack_track(0x6A6BFFFF), // track 42/0x2A
    unpack_track(0x294EFFFF), // track 43/0x2B
    unpack_track(0x5A5A5A5A), // track 44/0x2C PLATFORM
    unpack_track(0x096AFFFF), // track 45/0x2D
    unpack_track(0x8D7AFFFF), // track 46/0x2E
    unpack_track(0x8A7AFFFF), // track 47/0x2F
    unpack_track(0x2202FFFF), // track 48/0x30
    unpack_track(0x12121212), // track 49/0x31 PLATFORM
    unpack_track(0x2162FFFF), // track 50/0x32
    unpack_track(0x8312FFFF), // track 51/0x33
    unpack_track(0x5350FFFF), // track 52/0x34
    unpack_track(0x6336FFFF), // track 53/0x35
    unpack_track(0x7335FFFF), // track 54/0x36
    unpack_track(0x8447FFFF), // track 55/0x37
    unpack_track(0x390AFFFF), // track 56/0x38
    unpack_track(0x380CFFFF), // track 57/0x39
    unpack_track(0x5B5B5B5B), // track 58/0x3A PLATFORM
    unpack_track(0x0B4DFFFF), // track 59/0x3B
    unpack_track(0x5D5D5D5D), // track 60/0x3C PLATFORM
    unpack_track(0x2A6FFFFF), // track 61/0x3D
    unpack_track(0x7B6FFFFF), // track 62/0x3E
    unpack_track(0x8B8EFFFF), // track 63/0x3F
    unpack_track(0x03030303), // track 64/0x40 PLATFORM
    unpack_track(0x16161616), // track 65/0x41 PLATFORM
    unpack_track(0x0727FFFF), // track 66/0x42
    unpack_track(0x72727272), // track 67/0x43 PLATFORM
    unpack_track(0x5455FFFF), // track 68/0x44
    unpack_track(0x6764FFFF), // track 69/0x45
    unpack_track(0x8774FFFF), // track 70/0x46
    unpack_track(0x8537FF83), // track 71/0x47
    unpack_track(0x1D0AFFFF), // track 72/0x48
    unpack_track(0x1B10FFFF), // track 73/0x49
    unpack_track(0x194EFFFF), // track 74/0x4A
    unpack_track(0x3B3B3B3B), // track 75/0x4B PLATFORM
    unpack_track(0x6E0EFFFF), // track 76/0x4C
    unpack_track(0x3B88FFFF), // track 77/0x4D
    unpack_track(0x2B0E4A78), // track 78/0x4E
    unpack_track(0x188CFFFF), // track 79/0x4F
    unpack_track(0x0434FFFF), // track 80/0x50
    unpack_track(0x1614FFFF), // track 81/0x51
    unpack_track(0x2426FFFF), // track 82/0x52
    unpack_track(0x3416FFFF), // track 83/0x53
    unpack_track(0x1444FFFF), // track 84/0x54
    unpack_track(0x4466FFFF), // track 85/0x55
    unpack_track(0x87878787), // track 86/0x56 PLATFORM
    unpack_track(0x7785FFFF), // track 87/0x57
    unpack_track(0x0C0C0C0C), // track 88/0x58 PLATFORM
    unpack_track(0x7F19FFFF), // track 89/0x59
    unpack_track(0x5B8AFFFF), // track 90/0x5A
    unpack_track(0x5D5AFFFF), // track 91/0x5B
    unpack_track(0x780BFFFF), // track 92/0x5C
    unpack_track(0x5B6DFFFF), // track 93/0x5D
    unpack_track(0x8D5FFFFF), // track 94/0x5E
    unpack_track(0x5E1FFFFF), // track 95/0x5F
    unpack_track(0x6120FFFF), // track 96/0x60
    unpack_track(0x6000FFFF), // track 97/0x61
    unpack_track(0x2532FF22), // track 98/0x62
    unpack_track(0x2535FFFF), // track 99/0x63
    unpack_track(0x4565FFFF), // track 100/0x64
    unpack_track(0x6664FFFF), // track 101/0x65
    unpack_track(0x657655FF), // track 102/0x66
    unpack_track(0x4574FFFF), // track 103/0x67
    unpack_track(0x0D09FFFF), // track 104/0x68
    unpack_track(0x1B08FFFF), // track 105/0x69
    unpack_track(0x2D2AFFFF), // track 106/0x6A
    unpack_track(0x8E2AFFFF), // track 107/0x6B
    unpack_track(0x4D4D4D4D), // track 108/0x6C PLATFORM
    unpack_track(0x8B5DFFFF), // track 109/0x6D
    unpack_track(0x8F4CFFFF), // track 110/0x6E
    unpack_track(0x3E3DFFFF), // track 111/0x6F
    unpack_track(0x0380FFFF), // track 112/0x70
    unpack_track(0x51515151), // track 113/0x71 PLATFORM
    unpack_track(0x7326FFFF), // track 114/0x72
    unpack_track(0x7236FFFF), // track 115/0x73
    unpack_track(0x4667FFFF), // track 116/0x74
    unpack_track(0x46464646), // track 117/0x75 PLATFORM
    unpack_track(0x6624FFFF), // track 118/0x76
    unpack_track(0x8757FFFF), // track 119/0x77
    unpack_track(0x4E5CFFFF), // track 120/0x78
    unpack_track(0x1E1E1E1E), // track 121/0x79 PLATFORM
    unpack_track(0x2E2FFFFF), // track 122/0x7A
    unpack_track(0x883EFFFF), // track 123/0x7B
    unpack_track(0x17171717), // track 124/0x7C PLATFORM
    unpack_track(0x7F8DFFFF), // track 125/0x7D
    unpack_track(0x6E6E6E6E), // track 126/0x7E PLATFORM
    unpack_track(0x7D59FFFF), // track 127/0x7F
    unpack_track(0x7007FFFF), // track 128/0x80
    unpack_track(0x8417FFFF), // track 129/0x81
    unpack_track(0x0427FFFF), // track 130/0x82
    unpack_track(0x4733FFFF), // track 131/0x83
    unpack_track(0x8137FFFF), // track 132/0x84
    unpack_track(0x5747FFFF), // track 133/0x85
    unpack_track(0x74747474), // track 134/0x86 PLATFORM
    unpack_track(0x4677FFFF), // track 135/0x87
    unpack_track(0x7B4DFFFF), // track 136/0x88
    unpack_track(0x8F27FFFF), // track 137/0x89
    unpack_track(0x2F5AFFFF), // track 138/0x8A
    unpack_track(0x6D3FFFFF), // track 139/0x8B
    unpack_track(0x4F27FFFF), // track 140/0x8C
    unpack_track(0x2E5EFF7D), // track 141/0x8D
    unpack_track(0x6B3FFFFF), // track 142/0x8E
    unpack_track(0x896EFFFF), // track 143/0x8F
];

fn main() {
    // Your embedded logic goes here
}
