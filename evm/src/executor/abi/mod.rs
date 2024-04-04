use corebc::{
    types::{Network, Selector, H160, H176},
    utils::to_ican,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

pub use foxar_abi::{
    console::{self, ConsoleEvents, CONSOLE_ABI},
    hardhat_console::{self, HardhatConsoleCalls, HARDHATCONSOLE_ABI as HARDHAT_CONSOLE_ABI},
    hevm::{self, HEVMCalls, HEVM_ABI},
};

/// The cheatcode handler address (0xce60fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8).
///
/// This is the same address as the one used in DappTools's HEVM.
/// `address(bytes20(uint160(uint256(sha3('hevm cheat code')))))`
/// cb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8 CORETODO: Change this
pub const CHEATCODE_ADDRESS: H160 = H160([
    0xfc, 0x06, 0xa1, 0x2b, 0x7a, 0x6f, 0x30, 0xe2, 0xa3, 0xc1, 0x6a, 0x3b, 0x5d, 0x50, 0x2c, 0xd7,
    0x1c, 0x20, 0xf2, 0xf8,
]);

/// Default cheatcode address
pub fn default_cheatcode_address(mut network: Option<Network>) -> H176 {
    if network.is_none() {
        network = Some(Network::Private(1337))
    }
    to_ican(&CHEATCODE_ADDRESS, &network.unwrap())
}

/// The Hardhat console address (0xcb82000000000000000000636f6e736f6c652e6c6f67).
///
/// See: https://github.com/nomiclabs/hardhat/blob/master/packages/hardhat-core/console.sol
pub const HARDHAT_CONSOLE_ADDRESS: H160 = H160([
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x63, 0x6f, 0x6e, 0x73, 0x6f, 0x6c, 0x65,
    0x2e, 0x6c, 0x6f, 0x67,
]);

/// Default hardhat console address
pub fn default_hardhat_address(mut network: Option<Network>) -> H176 {
    if network.is_none() {
        network = Some(Network::Private(1337))
    }
    to_ican(&HARDHAT_CONSOLE_ADDRESS, &network.unwrap())
}
/// If the input starts with a known `hardhat/console.log` `uint` selector, then this will replace
/// it with the selector `abigen!` bindings expect.
pub fn patch_hardhat_console_selector(input: &mut [u8]) {
    if input.len() < 4 {
        return;
    }

    // SAFETY: length checked above; see [<[T]>::split_array_mut].
    let selector = unsafe { &mut *(input.get_unchecked_mut(..4) as *mut [u8] as *mut [u8; 4]) };
    if let Some(abigen_selector) = HARDHAT_CONSOLE_SELECTOR_PATCHES.get(selector) {
        *selector = *abigen_selector;
    }
}

/// This contains a map with all the  `hardhat/console.log` log selectors that use `uint` or `int`
/// as key and the selector of the call with `uint256`,
///
/// This is a bit terrible but a workaround for the differing selectors used by hardhat and the call
/// bindings which `abigen!` creates. `hardhat/console.log` logs its events in functions that accept
/// `uint` manually as `abi.encodeWithSignature("log(int)", p0)`, but `abigen!` uses `uint256` for
/// its call bindings (`HardhatConsoleCalls`) as generated by solc.
pub static HARDHAT_CONSOLE_SELECTOR_PATCHES: Lazy<HashMap<Selector, Selector>> = Lazy::new(|| {
    HashMap::from([
        // log(bool,uint256,uint256,address)
        ([45, 123, 12, 117], [183, 183, 32, 241]),
        // log(uint256,address,address,string)
        ([82, 234, 189, 182], [199, 185, 37, 98]),
        // log(uint256,bool,address,uint256)
        ([133, 161, 42, 215], [170, 129, 142, 134]),
        // log(bool,address,bool,uint256)
        ([94, 155, 124, 146], [202, 181, 96, 64]),
        // log(bool,uint256,address)
        ([224, 1, 49, 23], [44, 79, 199, 91]),
        // log(uint256,address,address,bool)
        ([101, 99, 237, 90], [191, 74, 211, 242]),
        // log(address,bool,uint256,string)
        ([71, 255, 9, 159], [81, 99, 34, 80]),
        // log(bool,bool,uint256,uint256)
        ([205, 177, 89, 248], [16, 20, 242, 193]),
        // log(bool,address,address,uint256)
        ([109, 210, 99, 166], [109, 91, 241, 23]),
        // log(uint256,address,uint256,uint256)
        ([186, 227, 249, 175], [107, 136, 92, 91]),
        // log(string,address,uint256)
        ([106, 177, 195, 218], [249, 215, 122, 21]),
        // log(address,string,uint256,bool)
        ([199, 146, 111, 109], [15, 68, 254, 248]),
        // log(address,uint256,address,uint256)
        ([107, 117, 220, 33], [190, 40, 129, 28]),
        // log(string,string,uint256,address)
        ([74, 249, 72, 11], [37, 220, 126, 33]),
        // log(bool,string,uint256)
        ([241, 187, 32, 98], [193, 216, 189, 253]),
        // log(bool,bool,uint256)
        ([146, 176, 165, 52], [141, 196, 137, 166]),
        // log(bool,address,uint256,address)
        ([226, 75, 251, 65], [225, 195, 119, 104]),
        // log(bool,uint256,address,uint256)
        ([219, 200, 11, 215], [16, 165, 17, 228]),
        // log(bool,string,uint256,address)
        ([172, 130, 131, 26], [75, 15, 229, 178]),
        // log(address,string,string,uint256)
        ([92, 18, 54, 134], [101, 247, 248, 149]),
        // log(uint256,address,uint256,address)
        ([43, 68, 3, 199], [56, 73, 12, 3]),
        // log(uint256,uint256,address,bool)
        ([79, 188, 153, 13], [100, 47, 28, 143]),
        // log(bool,string,bool,uint256)
        ([226, 121, 214, 84], [208, 0, 21, 18]),
        // log(address,address,uint256)
        ([229, 235, 177, 115], [181, 79, 253, 196]),
        // log(uint256,uint256,uint256,uint256)
        ([88, 62, 142, 208], [188, 167, 216, 21]),
        // log(bool,string,uint256,string)
        ([104, 169, 113, 12], [234, 211, 237, 65]),
        // log(bool,uint256,address,string)
        ([61, 255, 221, 73], [221, 59, 171, 46]),
        // log(string,uint256,address)
        ([218, 163, 194, 87], [105, 189, 16, 71]),
        // log(uint256,bool)
        ([124, 9, 46, 112], [190, 192, 209, 171]),
        // log(address,uint256,address,string)
        ([234, 137, 190, 64], [87, 168, 208, 106]),
        // log(address,string,uint256,uint256)
        ([217, 167, 220, 91], [78, 188, 63, 136]),
        // log(uint256,bool,uint256)
        ([191, 33, 176, 87], [231, 147, 244, 55]),
        // log(uint256,bool,bool)
        ([129, 136, 113, 207], [193, 11, 191, 201]),
        // log(address,uint256,uint256,address)
        ([50, 248, 47, 100], [231, 65, 57, 16]),
        // log(uint256,string,string,string)
        ([206, 132, 29, 64], [249, 194, 156, 161]),
        // log(address,uint256,bool,uint256)
        ([30, 177, 24, 67], [43, 69, 31, 221]),
        // log(uint256,address,address,address)
        ([6, 102, 149, 166], [231, 24, 188, 93]),
        // log(string,bool,string,uint256)
        ([1, 153, 100, 168], [109, 115, 170, 63]),
        // log(bool,uint256,address,address)
        ([177, 152, 146, 128], [170, 5, 219, 124]),
        // log(uint256,uint256,string,string)
        ([153, 29, 165, 81], [224, 210, 134, 201]),
        // log(bool,string,uint256,uint256)
        ([90, 109, 4, 82], [145, 145, 118, 241]),
        // log(uint256,bool,string,uint256)
        ([76, 58, 205, 145], [24, 161, 70, 47]),
        // log(address,uint256,uint256,uint256)
        ([128, 60, 243, 149], [136, 143, 50, 239]),
        // log(uint256,bool,address)
        ([166, 144, 121, 173], [70, 195, 228, 175]),
        // log(string,uint256,bool,bool)
        ([76, 172, 124, 51], [152, 36, 59, 184]),
        // log(bool,uint256,uint256)
        ([114, 35, 18, 232], [146, 81, 149, 65]),
        // log(bool,uint256,uint256,uint256)
        ([134, 230, 53, 23], [87, 250, 201, 157]),
        // log(uint256,string,uint256)
        ([144, 204, 94, 38], [64, 140, 128, 224]),
        // log(address,bool,uint256,uint256)
        ([38, 228, 135, 45], [241, 137, 196, 133]),
        // log(address,address,bool,uint256)
        ([90, 98, 96, 124], [238, 49, 220, 213]),
        // log(bool,uint256)
        ([97, 208, 142, 244], [95, 175, 247, 26]),
        // log(uint256,string,uint256,address)
        ([231, 93, 215, 111], [103, 198, 82, 100]),
        // log(address,uint256,bool,bool)
        ([116, 62, 100, 31], [253, 229, 244, 182]),
        // log(uint256,address,string,string)
        ([16, 95, 130, 220], [162, 16, 79, 86]),
        // log(string,address,bool,uint256)
        ([82, 110, 166, 3], [103, 80, 142, 82]),
        // log(uint256,uint256,string,address)
        ([116, 221, 46, 173], [141, 106, 208, 84]),
        // log(address,string,uint256,string)
        ([62, 9, 94, 95], [161, 206, 141, 224]),
        // log(uint256,bool,address,bool)
        ([68, 229, 47, 72], [114, 222, 59, 170]),
        // log(address,string,address,uint256)
        ([176, 245, 109, 233], [64, 10, 181, 76]),
        // log(uint256,address,string,uint256)
        ([107, 237, 161, 85], [192, 179, 39, 6]),
        // log(uint256,uint256,bool)
        ([87, 97, 177, 109], [5, 209, 47, 211]),
        // log(address,uint256,address,address)
        ([18, 230, 218, 92], [142, 9, 34, 248]),
        // log(address,uint256,uint256,string)
        ([206, 162, 72, 234], [51, 17, 145, 210]),
        // log(bool,bool,address,uint256)
        ([210, 2, 23, 21], [152, 33, 65, 140]),
        // log(uint256,string,bool)
        ([166, 169, 131, 96], [45, 9, 114, 190]),
        // log(string,uint256,address,uint256)
        ([100, 61, 144, 212], [231, 47, 129, 184]),
        // log(address,string,bool,uint256)
        ([230, 191, 215, 40], [87, 33, 148, 35]),
        // log(bool,address,uint256,string)
        ([158, 93, 51, 128], [160, 172, 187, 193]),
        // log(bool,bool,uint256,address)
        ([145, 187, 79, 27], [76, 173, 79, 184]),
        // log(uint256,uint256,address,address)
        ([33, 203, 10, 235], [76, 244, 94, 4]),
        // log(string,string,uint256)
        ([181, 166, 211, 178], [191, 41, 24, 6]),
        // log(string,uint256,string)
        ([195, 220, 222, 228], [246, 45, 9, 65]),
        // log(uint256,uint256,uint256,string)
        ([137, 232, 22, 151], [247, 47, 43, 187]),
        // log(string,address,uint256,string)
        ([91, 140, 221, 180], [79, 178, 97, 226]),
        // log(uint256,address,uint256)
        ([179, 212, 109, 178], [174, 123, 219, 9]),
        // log(string,uint256,string,string)
        ([187, 104, 72, 48], [182, 151, 31, 192]),
        // log(uint256,address,bool,uint256)
        ([200, 170, 101, 255], [207, 115, 181, 62]),
        // log(address,uint256,string,address)
        ([111, 205, 216, 206], [27, 163, 39, 113]),
        // log(uint256,uint256,address)
        ([10, 58, 68, 47], [169, 192, 233, 226]),
        // log(string,bool,address,uint256)
        ([6, 95, 140, 143], [242, 206, 106, 146]),
        // log(string,string,uint256,string)
        ([1, 42, 251, 10], [219, 198, 237, 98]),
        // log(uint256,uint256,string,uint256)
        ([139, 34, 139, 24], [169, 255, 94, 53]),
        // log(string,uint256,address,address)
        ([78, 228, 172, 183], [125, 8, 178, 184]),
        // log(uint256,address,uint256,bool)
        ([151, 67, 62, 202], [153, 142, 93, 82]),
        // log(bool,address,uint256)
        ([87, 157, 211, 205], [95, 187, 38, 61]),
        // log(uint256,string,address,address)
        ([49, 84, 243, 38], [97, 160, 92, 124]),
        // log(bool,bool,uint256,bool)
        ([42, 127, 141, 25], [71, 115, 94, 58]),
        // log(address,string,uint256,address)
        ([144, 56, 190, 197], [63, 242, 32, 169]),
        // log(uint256,address,string)
        ([63, 33, 173, 126], [195, 33, 139, 237]),
        // log(string,address,uint256,address)
        ([26, 179, 98, 120], [203, 160, 9, 199]),
        // log(uint256,string)
        ([92, 209, 161, 135], [168, 104, 37, 20]),
        // log(string,bool,uint256,uint256)
        ([197, 9, 30, 33], [255, 206, 199, 111]),
        // log(address,uint256,uint256,bool)
        ([89, 98, 247, 1], [249, 17, 90, 30]),
        // log(address,uint256,bool)
        ([170, 251, 175, 112], [113, 252, 25, 161]),
        // log(address,string,uint256)
        ([9, 4, 108, 177], [5, 32, 212, 163]),
        // log(uint256,bool,string,string)
        ([129, 84, 143, 16], [167, 217, 71, 192]),
        // log(uint256,string,uint256,bool)
        ([95, 134, 48, 97], [155, 147, 60, 160]),
        // log(uint256,address)
        ([95, 13, 75, 150], [31, 231, 47, 23]),
        // log(uint256,bool,bool,address)
        ([5, 223, 178, 179], [106, 174, 27, 50]),
        // log(bool,uint256,string,uint256)
        ([172, 57, 226, 134], [204, 169, 194, 127]),
        // log(bool,string,uint256,bool)
        ([66, 0, 150, 84], [62, 111, 224, 5]),
        // log(uint256,uint256,address,string)
        ([137, 65, 85, 176], [13, 24, 70, 188]),
        // log(bool,bool,bool,uint256)
        ([221, 54, 163, 126], [0, 189, 207, 93]),
        // log(uint256,uint256,string)
        ([218, 165, 30, 177], [19, 24, 188, 252]),
        // log(uint256,address,address,uint256)
        ([143, 19, 226, 81], [150, 102, 135, 134]),
        // log(string,bool,uint256,string)
        ([55, 177, 96, 0], [27, 78, 101, 254]),
        // log(uint256,bool,bool,uint256)
        ([94, 103, 56, 36], [25, 199, 90, 89]),
        // log(string,uint256,uint256,bool)
        ([61, 84, 94, 96], [239, 160, 248, 135]),
        // log(uint256,uint256,string,bool)
        ([17, 28, 151, 122], [11, 10, 161, 60]),
        // log(uint256,string,address)
        ([180, 227, 239, 142], [25, 41, 4, 16]),
        // log(address,uint256,address)
        ([114, 98, 11, 22], [123, 112, 110, 97]),
        // log(bool,string,string,uint256)
        ([70, 96, 141, 220], [186, 165, 164, 166]),
        // log(bool,address,uint256,uint256)
        ([83, 77, 22, 51], [13, 214, 14, 163]),
        // log(string,uint256,string,address)
        ([49, 155, 132, 81], [5, 250, 2, 131]),
        // log(string,string,address,uint256)
        ([161, 51, 219, 183], [244, 227, 224, 30]),
        // log(string,uint256,string,bool)
        ([79, 123, 78, 0], [52, 97, 243, 140]),
        // log(bool,bool,uint256,string)
        ([39, 222, 56, 53], [192, 110, 245, 164]),
        // log(bool,uint256,bool,uint256)
        ([97, 39, 104, 108], [204, 41, 74, 62]),
        // log(address,bool,string,uint256)
        ([190, 145, 161, 181], [254, 142, 161, 23]),
        // log(string,uint256,address,bool)
        ([254, 158, 229, 219], [72, 211, 240, 18]),
        // log(uint256,string,uint256,uint256)
        ([7, 71, 243, 76], [52, 92, 175, 36]),
        // log(address,uint256)
        ([188, 5, 13, 240], [225, 67, 220, 133]),
        // log(string,uint256,uint256,string)
        ([143, 100, 5, 88], [74, 224, 228, 209]),
        // log(uint256,bool,string)
        ([37, 238, 251, 168], [83, 105, 243, 198]),
        // log(address,uint256,string,string)
        ([206, 91, 189, 178], [131, 87, 2, 40]),
        // log(uint256,bool,uint256,address)
        ([233, 91, 203, 156], [164, 210, 98, 174]),
        // log(uint256,uint256,address,uint256)
        ([48, 121, 220, 129], [252, 224, 51, 22]),
        // log(string,bool,uint256,bool)
        ([136, 23, 198, 218], [158, 228, 119, 66]),
        // log(address,bool,bool,uint256)
        ([144, 168, 74, 115], [47, 131, 152, 112]),
        // log(address,address,uint256,address)
        ([156, 88, 78, 222], [25, 43, 102, 117]),
        // log(string,bool,bool,uint256)
        ([30, 192, 153, 118], [199, 209, 202, 59]),
        // log(bool,uint256,uint256,string)
        ([2, 117, 127, 239], [142, 227, 70, 79]),
        // log(string,string,string,uint256)
        ([192, 204, 162, 224], [70, 198, 119, 190]),
        // log(string,address,address,uint256)
        ([186, 184, 131, 170], [167, 199, 36, 137]),
        // log(uint256,string,address,bool)
        ([9, 71, 17, 45], [116, 166, 34, 252]),
        // log(uint256,address,bool,string)
        ([215, 231, 82, 100], [47, 122, 235, 65]),
        // log(bool,uint256,bool,string)
        ([245, 211, 224, 17], [2, 124, 233, 182]),
        // log(uint256,bool,uint256,bool)
        ([72, 138, 177, 182], [215, 145, 201, 45]),
        // log(string,address,string,uint256)
        ([210, 158, 73, 52], [5, 21, 160, 114]),
        // log(string,bool,uint256,address)
        ([231, 209, 101, 100], [9, 211, 13, 186]),
        // log(address,address,address,uint256)
        ([82, 20, 113, 251], [242, 38, 43, 79]),
        // log(uint256,uint256,bool,address)
        ([158, 65, 39, 193], [196, 8, 13, 61]),
        // log(bool,uint256,bool,address)
        ([204, 84, 75, 10], [68, 244, 8, 204]),
        // log(address,address,uint256,bool)
        ([96, 149, 167, 160], [78, 223, 145, 235]),
        // log(uint256,address,bool)
        ([107, 164, 106, 21], [164, 92, 100, 210]),
        // log(uint256,string,address,string)
        ([148, 32, 133, 130], [79, 156, 29, 191]),
        // log(address,bool,uint256)
        ([111, 82, 213, 253], [29, 163, 88, 29]),
        // log(uint256,address,string,address)
        ([244, 79, 223, 168], [99, 125, 10, 168]),
        // log(string,uint256,address,string)
        ([61, 253, 70, 227], [170, 180, 111, 208]),
        // log(address,uint256,address,bool)
        ([104, 207, 165, 26], [54, 206, 115, 238]),
        // log(uint256,bool,address,address)
        ([103, 84, 151, 10], [75, 225, 8, 65]),
        // log(address,uint256,string)
        ([26, 34, 28, 68], [68, 131, 195, 216]),
        // log(address,uint256,bool,address)
        ([172, 101, 239, 231], [25, 126, 114, 81]),
        // log(uint256,uint256,bool,string)
        ([118, 174, 32, 166], [133, 52, 161, 226]),
        // log(bool,string,address,uint256)
        ([119, 183, 229, 85], [183, 130, 240, 183]),
        // log(address,bool,address,uint256)
        ([81, 17, 238, 206], [121, 166, 1, 18]),
        // log(string,uint256,uint256,uint256)
        ([211, 237, 11, 42], [101, 75, 91, 89]),
        // log(uint256,uint256,bool,bool)
        ([174, 246, 127, 155], [11, 142, 70, 193]),
        // log(string,uint256,bool,string)
        ([54, 19, 69, 184], [238, 189, 128, 10]),
        // log(uint256,bool,address,string)
        ([190, 87, 160, 45], [206, 53, 156, 85]),
        // log(uint256,string,bool,address)
        ([120, 68, 236, 201], [78, 58, 253, 0]),
        // log(uint256,string,string,uint256)
        ([21, 161, 231, 110], [57, 0, 203, 116]),
        // log(uint256,string,string)
        ([221, 242, 107, 212], [146, 156, 221, 17]),
        // log(uint256,string,string,bool)
        ([70, 102, 157, 171], [42, 62, 151, 254]),
        // log(bool,uint256,address,bool)
        ([223, 167, 142, 198], [73, 15, 73, 15]),
        // log(string,uint256)
        ([248, 92, 56, 212], [246, 195, 245, 166]),
        // log(address,uint256,uint256)
        ([72, 71, 3, 133], [11, 54, 112, 240]),
        // log(uint256,bool,bool,bool)
        ([138, 207, 193, 14], [65, 208, 237, 18]),
        // log(uint256,string,uint256,string)
        ([247, 223, 88, 48], [57, 100, 162, 178]),
        // log(uint256,string,bool,bool)
        ([8, 35, 160, 19], [81, 152, 113, 61]),
        // log(uint256,address,address)
        ([55, 236, 133, 211], [100, 241, 179, 184]),
        // log(address,address,uint256,uint256)
        ([129, 13, 38, 65], [234, 98, 151, 154]),
        // log(bool,uint256,uint256,bool)
        ([153, 120, 191, 248], [9, 230, 17, 212]),
        // log(address,uint256,string,uint256)
        ([57, 217, 242, 105], [2, 136, 85, 53]),
        // log(bool,address,string,uint256)
        ([241, 115, 191, 255], [212, 158, 44, 142]),
        // log(string,string,uint256,bool)
        ([170, 2, 252, 23], [206, 178, 4, 241]),
        // log(bool,uint256,string)
        ([97, 20, 169, 63], [41, 97, 185, 237]),
        // log(address,bool,uint256,bool)
        ([193, 248, 81, 156], [39, 80, 79, 3]),
        // log(uint256,uint256,uint256,bool)
        ([229, 121, 18, 49], [181, 53, 145, 231]),
        // log(address,uint256,bool,string)
        ([106, 215, 37, 5], [16, 220, 182, 232]),
        // log(string,uint256,string,uint256)
        ([15, 97, 102, 31], [132, 242, 84, 255]),
        // log(uint256,bool,uint256,uint256)
        ([47, 231, 23, 132], [14, 33, 185, 249]),
        // log(string,bool,uint256)
        ([188, 181, 248, 76], [245, 108, 75, 1]),
        // log(string,uint256,uint256)
        ([253, 95, 196, 218], [254, 189, 150, 237]),
        // log(string,uint256,bool)
        ([233, 218, 237, 159], [1, 243, 53, 117]),
        // log(uint256,address,string,bool)
        ([243, 148, 38, 63], [113, 254, 206, 100]),
        // log(address,bool,uint256,address)
        ([22, 157, 71, 4], [242, 207, 153, 242]),
        // log(bool,uint256,bool,bool)
        ([109, 242, 40, 6], [156, 9, 160, 130]),
        // log(uint256,string,bool,uint256)
        ([161, 32, 76, 80], [244, 42, 81, 101]),
        // log(address,uint256,string,bool)
        ([153, 251, 93, 79], [233, 152, 36, 58]),
        // log(uint256,uint256,uint256)
        ([185, 56, 247, 2], [125, 242, 227, 147]),
        // log(uint256,string,bool,string)
        ([254, 218, 232, 65], [48, 143, 148, 66]),
        // log(uint256,string,string,address)
        ([194, 190, 104, 229], [219, 25, 176, 87]),
        // log(bool,address,uint256,bool)
        ([17, 202, 129, 198], [177, 186, 148, 227]),
        // log(string,string,bool,uint256)
        ([27, 109, 79, 97], [89, 145, 161, 205]),
        // log(uint256,address,uint256,string)
        ([146, 254, 89, 189], [238, 18, 66, 126]),
        // log(uint256,bool,bool,string)
        ([100, 12, 204, 139], [109, 67, 142, 255]),
        // log(uint256,bool,uint256,string)
        ([9, 181, 191, 73], [160, 59, 169, 107]),
        // log(string,uint256,bool,address)
        ([163, 129, 202, 237], [212, 189, 192, 0]),
        // log(string,uint256,uint256,address)
        ([249, 169, 188, 126], [95, 252, 25, 64]),
        // log(uint256,address,bool,bool)
        ([157, 233, 204, 79], [185, 182, 242, 49]),
        // log(bool,bool,string,uint256)
        ([112, 139, 91, 159], [200, 247, 184, 166]),
        // log(string,uint256,bool,uint256)
        ([203, 231, 33, 135], [4, 169, 222, 214]),
        // log(bool,uint256,string,bool)
        ([152, 169, 145, 109], [95, 134, 114, 155]),
        // log(uint256,string,address,uint256)
        ([241, 36, 128, 215], [22, 59, 198, 247]),
        // log(bool,uint256,bool)
        ([59, 197, 54, 224], [168, 181, 103, 212]),
        // log(uint256,uint256,bool,uint256)
        ([66, 35, 167, 108], [12, 23, 168, 100]),
        // log(uint256,bool,string,bool)
        ([38, 86, 169, 221], [131, 210, 166, 235]),
        // log(address,address,string,uint256)
        ([137, 100, 242, 29], [202, 21, 227, 89]),
        // log(uint256,bool,string,address)
        ([111, 155, 29, 24], [108, 27, 231, 46]),
        // log(uint256,address,bool,address)
        ([26, 156, 25, 165], [159, 157, 81, 140]),
        // log(string,string,uint256,uint256)
        ([110, 170, 60, 188], [19, 197, 116, 3]),
        // log(bool,uint256,string,string)
        ([120, 247, 131, 3], [180, 38, 71, 187]),
        // log(uint256,uint256)
        ([10, 237, 184, 121], [11, 227, 232, 169]),
        // log(uint256) and logUint(uint256)
        ([108, 111, 112, 197], [91, 179, 46, 203]),
        // log(string,address,uint256,uint256)
        ([253, 241, 26, 242], [22, 181, 164, 123]),
        // log(uint256,uint256,uint256,address)
        ([129, 230, 220, 229], [176, 63, 98, 212]),
        // log(string,address,uint256,bool)
        ([215, 56, 108, 186], [196, 220, 132, 229]),
        // log(address,address,uint256,string)
        ([71, 162, 77, 21], [140, 66, 81, 1]),
        // log(bool,uint256,string,address)
        ([247, 40, 34, 232], [251, 149, 107, 209]),
        // logInt(int256)
        ([79, 45, 254, 16], [51, 113, 71, 198]),
        // logBytes(bytes)
        ([80, 31, 147, 186], [198, 31, 230, 90]),
        // logBytes1(bytes1)
        ([204, 7, 227, 94], [99, 189, 222, 115]),
        // logBytes2(bytes2)
        ([60, 44, 110, 224], [236, 254, 141, 196]),
        // logBytes3(bytes3)
        ([82, 70, 255, 221], [21, 194, 80, 147]),
        // logBytes4(bytes4)
        ([221, 133, 169, 22], [62, 163, 74, 30]),
        // logBytes5(bytes5)
        ([143, 15, 248, 233], [50, 223, 68, 240]),
        // logBytes6(bytes6)
        ([239, 25, 14, 194], [169, 38, 216, 242]),
        // logBytes7(bytes7)
        ([193, 72, 225, 240], [247, 94, 19, 4]),
        // logBytes8(bytes8)
        ([51, 72, 81, 157], [213, 117, 249, 56]),
        // logBytes9(bytes9)
        ([66, 110, 195, 23], [189, 14, 101, 159]),
        // logBytes10(bytes10)
        ([70, 12, 254, 221], [19, 24, 33, 50]),
        // logBytes11(bytes11)
        ([197, 76, 42, 67], [174, 143, 243, 119]),
        // logBytes12(bytes12)
        ([64, 85, 113, 68], [25, 255, 170, 198]),
        // logBytes13(bytes13)
        ([63, 73, 0, 173], [255, 224, 76, 57]),
        // logBytes14(bytes14)
        ([126, 14, 194, 84], [132, 66, 19, 85]),
        // logBytes15(bytes15)
        ([145, 168, 75, 181], [53, 176, 177, 193]),
        // logBytes16(bytes16)
        ([176, 2, 129, 201], [86, 193, 166, 111]),
        // logBytes17(bytes17)
        ([114, 192, 99, 109], [182, 208, 66, 226]),
        // logBytes18(bytes18)
        ([37, 254, 55, 148], [231, 12, 49, 221]),
        // logBytes19(bytes19)
        ([55, 96, 1, 49], [43, 109, 94, 38]),
        // logBytes20(bytes20)
        ([202, 79, 17, 195], [43, 124, 77, 44]),
        // logBytes21(bytes21)
        ([145, 22, 141, 48], [133, 243, 250, 36]),
        // logBytes22(bytes22)
        ([158, 72, 206, 236], [212, 64, 119, 214]),
        // logBytes23(bytes23)
        ([157, 119, 200, 23], [136, 220, 43, 93]),
        // logBytes24(bytes24)
        ([122, 51, 160, 32], [111, 226, 56, 179]),
        // logBytes25(bytes25)
        ([217, 4, 76, 48], [1, 183, 4, 154]),
        // logBytes26(bytes26)
        ([126, 142, 220, 8], [121, 5, 51, 220]),
        // logBytes27(bytes27)
        ([20, 231, 94, 251], [162, 115, 247, 200]),
        // logBytes28(bytes28)
        ([188, 213, 203, 71], [75, 13, 121, 117]),
        // logBytes29(bytes29)
        ([168, 68, 155, 79], [42, 154, 95, 245]),
        // logBytes30(bytes30)
        ([80, 87, 237, 0], [96, 17, 227, 92]),
        // logBytes31(bytes31)
        ([134, 247, 217, 144], [32, 26, 23, 187]),
        // logBytes32(bytes32)
        ([213, 22, 199, 128], [66, 127, 115, 151]),
    ])
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hardhat_console_path_works() {
        for (hh, abigen) in HARDHAT_CONSOLE_SELECTOR_PATCHES.iter() {
            let mut hh = *hh;
            patch_hardhat_console_selector(&mut hh);
            assert_eq!(*abigen, hh);
        }
    }
}
