pub use hevm::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod hevm {
    #[rustfmt::skip]
    const __ABI: &str = "[\nstruct Log { bytes32[] topics; bytes data; }\nstruct Rpc { string name; string url; }\nstruct DirEntry { string errorMessage; string path; uint64 depth; bool isDir; bool isSymlink; }\nstruct FsMetadata { bool isDir; bool isSymlink; uint256 length; bool readOnly; uint256 modified; uint256 accessed; uint256 created; }\n\nallowCheatcodes(address)\n\nffi(string[])(bytes)\n\nbreakpoint(string)\nbreakpoint(string,bool)\n\nroll(uint256)\nwarp(uint256)\ndifficulty(uint256)\nfee(uint256)\ncoinbase(address)\nstore(address,bytes32,bytes32)\nload(address,bytes32)(bytes32)\n\nsetEnv(string,string)\nenvBool(string)(bool)\nenvUint(string)(uint256)\nenvInt(string)(int256)\nenvAddress(string)(address)\nenvBytes32(string)(bytes32)\nenvString(string)(string)\nenvBytes(string)(bytes)\nenvBool(string,string)(bool[])\nenvUint(string,string)(uint256[])\nenvInt(string,string)(int256[])\nenvAddress(string,string)(address[])\nenvBytes32(string,string)(bytes32[])\nenvString(string,string)(string[])\nenvBytes(string,string)(bytes[])\nenvOr(string,bool)(bool)\nenvOr(string,uint256)(uint256)\nenvOr(string,int256)(int256)\nenvOr(string,address)(address)\nenvOr(string,bytes32)(bytes32)\nenvOr(string,string)(string)\nenvOr(string,bytes)(bytes)\nenvOr(string,string,bool[])(bool[])\nenvOr(string,string,uint256[])(uint256[])\nenvOr(string,string,int256[])(int256[])\nenvOr(string,string,address[])(address[])\nenvOr(string,string,bytes32[])(bytes32[])\nenvOr(string,string,string[])(string[])\nenvOr(string,string,bytes[])(bytes[])\n\naddr(string)(address)\nsign(string,bytes32)(bytes)\nderiveKey(string,uint32)(uint256)\nderiveKey(string,string,uint32)(uint256)\nderiveKey(string,uint32,string)(uint256)\nderiveKey(string,string,uint32,string)(uint256)\nrememberKey(string)(address)\n\nprank(address)\nprank(address,address)\nreadCallers()(uint256,address,address)\nstartPrank(address)\nstartPrank(address,address)\nstopPrank()\n\ndeal(address,uint256)\netch(address,bytes)\nexpectRevert()\nexpectRevert(bytes)\nexpectRevert(bytes4)\nrecord()\naccesses(address)(bytes32[],bytes32[])\nskip(bool)\n\nrecordLogs()\ngetRecordedLogs()(Log[])\n\nexpectEmit()\nexpectEmit(address)\nexpectEmit(bool,bool,bool,bool)\nexpectEmit(bool,bool,bool,bool,address)\n\nmockCall(address,bytes,bytes)\nmockCall(address,uint256,bytes,bytes)\nmockCallRevert(address,bytes,bytes)\nmockCallRevert(address,uint256,bytes,bytes)\nclearMockedCalls()\n\nexpectCall(address,bytes)\nexpectCall(address,bytes,uint64)\nexpectCall(address,uint256,bytes)\nexpectCall(address,uint256,bytes,uint64)\nexpectCall(address,uint256,uint64,bytes)\nexpectCall(address,uint256,uint64,bytes,uint64)\nexpectCallMinGas(address,uint256,uint64,bytes)\nexpectCallMinGas(address,uint256,uint64,bytes,uint64)\nexpectSafeMemory(uint64,uint64)\nexpectSafeMemoryCall(uint64,uint64)\n\ngetCode(string)\ngetDeployedCode(string)\nlabel(address,string)\ngetLabel(address)(string)\nassume(bool)\nsetNonce(address,uint64)\ngetNonce(address)\nresetNonce(address)\nsetNonceUnsafe(address,uint64)\nchainId(uint256)\ntxGasPrice(uint256)\n\ncomputeCreateAddress(address,uint256)\ncomputeCreate2Address(bytes32,bytes32,address)\ncomputeCreate2Address(bytes32,bytes32)\n\nbroadcast()\nbroadcast(address)\nbroadcast(string)\nstartBroadcast()\nstartBroadcast(address)\nstartBroadcast(string)\nstopBroadcast()\n\nprojectRoot()(string)\nreadFile(string)(string)\nreadFileBinary(string)(bytes)\nwriteFile(string,string)\nwriteFileBinary(string,bytes)\nopenFile(string)\nreadLine(string)(string)\nwriteLine(string,string)\ncloseFile(string)\nremoveFile(string)\ncreateDir(string, bool)\nremoveDir(string, bool)\nreadDir(string)(DirEntry[])\nreadDir(string, uint64)(DirEntry[])\nreadDir(string, uint64, bool)(DirEntry[])\nreadLink(string)(string)\nfsMetadata(string)(FsMetadata)\n\ntoString(bytes)\ntoString(address)\ntoString(uint256)\ntoString(int256)\ntoString(bytes32)\ntoString(bool)\nparseBytes(string)(bytes)\nparseAddress(string)(address)\nparseUint(string)(uint256)\nparseInt(string)(int256)\nparseBytes32(string)(bytes32)\nparseBool(string)(bool)\n\nsnapshot()(uint256)\nrevertTo(uint256)(bool)\ncreateFork(string,uint256)(uint256)\ncreateFork(string,bytes32)(uint256)\ncreateFork(string)(uint256)\ncreateSelectFork(string,uint256)(uint256)\ncreateSelectFork(string,bytes32)(uint256)\ncreateSelectFork(string)(uint256)\nselectFork(uint256)\nactiveFork()(uint256)\ntransact(bytes32)\ntransact(uint256,bytes32)\nmakePersistent(address)\nmakePersistent(address,address)\nmakePersistent(address,address,address)\nmakePersistent(address[])\nrevokePersistent(address)\nrevokePersistent(address[])\nisPersistent(address)(bool)\nrollFork(uint256)\nrollFork(bytes32)\nrollFork(uint256,uint256)\nrollFork(uint256,bytes32)\nrpcUrl(string)(string)\nrpcUrls()(string[2][])\nrpcUrlStructs()(Rpc[])\n\nwriteJson(string, string)\nwriteJson(string, string, string)\nparseJson(string)(bytes)\nparseJson(string, string)(bytes)\nparseJsonUint(string, string)(uint256)\nparseJsonUintArray(string, string)(uint256[])\nparseJsonInt(string, string)(int256)\nparseJsonIntArray(string, string)(int256[])\nparseJsonString(string, string)(string)\nparseJsonStringArray(string, string)(string[])\nparseJsonAddress(string, string)(address)\nparseJsonAddressArray(string, string)(address[])\nparseJsonBool(string, string)(bool)\nparseJsonBoolArray(string, string)(bool[])\nparseJsonBytes(string, string)(bytes)\nparseJsonBytesArray(string, string)(bytes[])\nparseJsonBytes32(string, string)(bytes32)\nparseJsonBytes32Array(string, string)(bytes32[])\nserializeBool(string,string,bool)(string)\nserializeBool(string,string,bool[])(string)\nserializeUint(string,string,uint256)(string)\nserializeUint(string,string,uint256[])(string)\nserializeInt(string,string,int256)(string)\nserializeInt(string,string,int256[])(string)\nserializeAddress(string,string,address)(string)\nserializeAddress(string,string,address[])(string)\nserializeBytes32(string,string,bytes32)(string)\nserializeBytes32(string,string,bytes32[])(string)\nserializeString(string,string,string)(string)\nserializeString(string,string,string[])(string)\nserializeBytes(string,string,bytes)(string)\nserializeBytes(string,string,bytes[])(string)\n\npauseGasMetering()\nresumeGasMetering()\nstartMappingRecording()\nstopMappingRecording()\ngetMappingLength(address,bytes32)\ngetMappingSlotAt(address,bytes32,uint256)\ngetMappingKeyAndParentOf(address,bytes32)\n]";
    ///The parsed human-readable ABI of the contract.
    pub static HEVM_ABI: ::corebc_contract::Lazy<::corebc_core::abi::Abi> = ::corebc_contract::Lazy::new(||
    ::corebc_core::abi::parse_abi_str(__ABI).expect("ABI is always valid"));
    pub struct HEVM<M>(::corebc_contract::Contract<M>);
    impl<M> ::core::clone::Clone for HEVM<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HEVM<M> {
        type Target = ::corebc_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HEVM<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HEVM<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(HEVM)).field(&self.address()).finish()
        }
    }
    impl<M: ::corebc_providers::Middleware> HEVM<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `corebc::Contract` object.
        pub fn new<T: Into<::corebc_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::corebc_contract::Contract::new(
                    address.into(),
                    HEVM_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `accesses` (0x6e606627) function
        pub fn accesses(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            (::std::vec::Vec<[u8; 32]>, ::std::vec::Vec<[u8; 32]>),
        > {
            self.0
                .method_hash([110, 96, 102, 39], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `activeFork` (0x2a978c58) function
        pub fn active_fork(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([42, 151, 140, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `addr` (0xf91daf04) function
        pub fn addr(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::corebc_core::types::Address,
        > {
            self.0
                .method_hash([249, 29, 175, 4], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `allowCheatcodes` (0x21bd8ec4) function
        pub fn allow_cheatcodes(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 189, 142, 196], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `assume` (0xca480fa9) function
        pub fn assume(
            &self,
            p0: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 72, 15, 169], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `breakpoint` (0x3af777a9) function
        pub fn breakpoint_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 247, 119, 169], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `breakpoint` (0x18af918f) function
        pub fn breakpoint_1(
            &self,
            p0: ::std::string::String,
            p1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 175, 145, 143], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0x12f46a0c) function
        pub fn broadcast_0(&self) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 244, 106, 12], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0x3aec5261) function
        pub fn broadcast_1(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 236, 82, 97], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `broadcast` (0x85b0a416) function
        pub fn broadcast_2(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 176, 164, 22], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `chainId` (0x5858cd87) function
        pub fn chain_id(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 88, 205, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `clearMockedCalls` (0x7eaff7de) function
        pub fn clear_mocked_calls(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 175, 247, 222], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `closeFile` (0xa3a9f387) function
        pub fn close_file(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 169, 243, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `coinbase` (0x53a5c6de) function
        pub fn coinbase(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 165, 198, 222], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate2Address` (0x447f5ae9) function
        pub fn compute_create_2_address_1(
            &self,
            p0: [u8; 32],
            p1: [u8; 32],
            p2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 127, 90, 233], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreate2Address` (0xf27a5040) function
        pub fn compute_create_2_address_0(
            &self,
            p0: [u8; 32],
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 122, 80, 64], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `computeCreateAddress` (0x7486025d) function
        pub fn compute_create_address(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 134, 2, 93], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createDir` (0x8b295957) function
        pub fn create_dir(
            &self,
            p0: ::std::string::String,
            p1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([139, 41, 89, 87], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x27dc0128) function
        pub fn create_fork_1(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([39, 220, 1, 40], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0x472ee61e) function
        pub fn create_fork_2(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([71, 46, 230, 30], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createFork` (0xc8282be0) function
        pub fn create_fork_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([200, 40, 43, 224], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0xcc15574b) function
        pub fn create_select_fork_1(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([204, 21, 87, 75], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x47cbbd7a) function
        pub fn create_select_fork_2(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([71, 203, 189, 122], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `createSelectFork` (0x6bcf2c60) function
        pub fn create_select_fork_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([107, 207, 44, 96], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deal` (0x5ae137d7) function
        pub fn deal(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 225, 55, 215], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0xf6f7e9e1) function
        pub fn derive_key_0(
            &self,
            p0: ::std::string::String,
            p1: u32,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([246, 247, 233, 225], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0xd5a72210) function
        pub fn derive_key_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: u32,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([213, 167, 34, 16], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0xee602b67) function
        pub fn derive_key_2(
            &self,
            p0: ::std::string::String,
            p1: u32,
            p2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([238, 96, 43, 103], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `deriveKey` (0xcc819384) function
        pub fn derive_key_3(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: u32,
            p3: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([204, 129, 147, 132], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `difficulty` (0xfc75ba15) function
        pub fn difficulty(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 117, 186, 21], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envAddress` (0x5992c5c6) function
        pub fn env_address_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::corebc_core::types::Address,
        > {
            self.0
                .method_hash([89, 146, 197, 198], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envAddress` (0xdd088083) function
        pub fn env_address_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::Address>,
        > {
            self.0
                .method_hash([221, 8, 128, 131], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBool` (0xfd40bd11) function
        pub fn env_bool_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([253, 64, 189, 17], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBool` (0xdf739509) function
        pub fn env_bool_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([223, 115, 149, 9], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes` (0xb1676ba0) function
        pub fn env_bytes_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([177, 103, 107, 160], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes` (0xd9577bb1) function
        pub fn env_bytes_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::Bytes>,
        > {
            self.0
                .method_hash([217, 87, 123, 177], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes32` (0x2d261418) function
        pub fn env_bytes_320(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([45, 38, 20, 24], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envBytes32` (0x5c1826d3) function
        pub fn env_bytes_321(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([92, 24, 38, 211], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envInt` (0xdba2cad8) function
        pub fn env_int_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::I256> {
            self.0
                .method_hash([219, 162, 202, 216], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envInt` (0x347968b0) function
        pub fn env_int_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::I256>,
        > {
            self.0
                .method_hash([52, 121, 104, 176], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x8efd04a9) function
        pub fn env_or_0(
            &self,
            p0: ::std::string::String,
            p1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([142, 253, 4, 169], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x1c6db43b) function
        pub fn env_or_1(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([28, 109, 180, 59], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x35711e3f) function
        pub fn env_or_2(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::I256,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::I256> {
            self.0
                .method_hash([53, 113, 30, 63], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x18745f8c) function
        pub fn env_or_3(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::corebc_core::types::Address,
        > {
            self.0
                .method_hash([24, 116, 95, 140], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xcb48937a) function
        pub fn env_or_4(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([203, 72, 147, 122], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x42d1d0ef) function
        pub fn env_or_5(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([66, 209, 208, 239], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x2dd17959) function
        pub fn env_or_6(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([45, 209, 121, 89], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x9430ce1e) function
        pub fn env_or_7(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<bool>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([148, 48, 206, 30], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0xbf6e43ef) function
        pub fn env_or_8(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::U256>,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::U256>,
        > {
            self.0
                .method_hash([191, 110, 67, 239], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x60a8eb5e) function
        pub fn env_or_9(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::I256>,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::I256>,
        > {
            self.0
                .method_hash([96, 168, 235, 94], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x0dbd12fc) function
        pub fn env_or_10(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::Address>,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::Address>,
        > {
            self.0
                .method_hash([13, 189, 18, 252], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x864e3a3c) function
        pub fn env_or_11(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<[u8; 32]>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([134, 78, 58, 60], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x9eb2a8c4) function
        pub fn env_or_12(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::std::string::String>,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([158, 178, 168, 196], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envOr` (0x206f7cdd) function
        pub fn env_or_13(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::Bytes>,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::Bytes>,
        > {
            self.0
                .method_hash([32, 111, 124, 221], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envString` (0xe613c3f6) function
        pub fn env_string_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([230, 19, 195, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envString` (0x535a86f8) function
        pub fn env_string_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([83, 90, 134, 248], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envUint` (0x22b1fcf4) function
        pub fn env_uint_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([34, 177, 252, 244], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `envUint` (0x09437722) function
        pub fn env_uint_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::U256>,
        > {
            self.0
                .method_hash([9, 67, 119, 34], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `etch` (0x61106001) function
        pub fn etch(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 16, 96, 1], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xc244877f) function
        pub fn expect_call_0(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 68, 135, 127], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0x35ce3631) function
        pub fn expect_call_1(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Bytes,
            p2: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 206, 54, 49], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0x5a1f47d6) function
        pub fn expect_call_2(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 31, 71, 214], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0xde6e6e54) function
        pub fn expect_call_3(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: ::corebc_core::types::Bytes,
            p3: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 110, 110, 84], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0x536671ce) function
        pub fn expect_call_4(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: u64,
            p3: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 102, 113, 206], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCall` (0x66ca6367) function
        pub fn expect_call_5(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: u64,
            p3: ::corebc_core::types::Bytes,
            p4: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 202, 99, 103], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCallMinGas` (0x782f8cdb) function
        pub fn expect_call_min_gas_0(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: u64,
            p3: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([120, 47, 140, 219], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectCallMinGas` (0x7f5de23e) function
        pub fn expect_call_min_gas_1(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: u64,
            p3: ::corebc_core::types::Bytes,
            p4: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 93, 226, 62], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0x51bd56ad) function
        pub fn expect_emit_0(&self) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 189, 86, 173], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0xbff0146e) function
        pub fn expect_emit_1(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 240, 20, 110], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0xe2e01dd4) function
        pub fn expect_emit_2(
            &self,
            p0: bool,
            p1: bool,
            p2: bool,
            p3: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 224, 29, 212], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectEmit` (0xdb3d4fd8) function
        pub fn expect_emit_3(
            &self,
            p0: bool,
            p1: bool,
            p2: bool,
            p3: bool,
            p4: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 61, 79, 216], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0x10403acb) function
        pub fn expect_revert_0(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 64, 58, 203], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0xe37ab4ed) function
        pub fn expect_revert_1(
            &self,
            p0: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 122, 180, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectRevert` (0x4e7614bc) function
        pub fn expect_revert_2(
            &self,
            p0: [u8; 4],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([78, 118, 20, 188], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectSafeMemory` (0x46bb5493) function
        pub fn expect_safe_memory(
            &self,
            p0: u64,
            p1: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 187, 84, 147], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `expectSafeMemoryCall` (0x9b61c661) function
        pub fn expect_safe_memory_call(
            &self,
            p0: u64,
            p1: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 97, 198, 97], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fee` (0xaa32efc1) function
        pub fn fee(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 50, 239, 193], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `ffi` (0x64ba2b32) function
        pub fn ffi(
            &self,
            p0: ::std::vec::Vec<::std::string::String>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([100, 186, 43, 50], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fsMetadata` (0x895b0357) function
        pub fn fs_metadata(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            (
                bool,
                bool,
                ::corebc_core::types::U256,
                bool,
                ::corebc_core::types::U256,
                ::corebc_core::types::U256,
                ::corebc_core::types::U256,
            ),
        > {
            self.0
                .method_hash([137, 91, 3, 87], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCode` (0xf4311698) function
        pub fn get_code(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 49, 22, 152], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getDeployedCode` (0xe04cfba1) function
        pub fn get_deployed_code(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 76, 251, 161], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getLabel` (0x56c8fc59) function
        pub fn get_label(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([86, 200, 252, 89], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMappingKeyAndParentOf` (0x1b402ced) function
        pub fn get_mapping_key_and_parent_of(
            &self,
            p0: ::corebc_core::types::Address,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 64, 44, 237], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMappingLength` (0x5e0a6a20) function
        pub fn get_mapping_length(
            &self,
            p0: ::corebc_core::types::Address,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 10, 106, 32], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getMappingSlotAt` (0x6028eb0c) function
        pub fn get_mapping_slot_at(
            &self,
            p0: ::corebc_core::types::Address,
            p1: [u8; 32],
            p2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([96, 40, 235, 12], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getNonce` (0x7ee608d7) function
        pub fn get_nonce(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([126, 230, 8, 215], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getRecordedLogs` (0x6aa9ac15) function
        pub fn get_recorded_logs(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<(::std::vec::Vec<[u8; 32]>, ::corebc_core::types::Bytes)>,
        > {
            self.0
                .method_hash([106, 169, 172, 21], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPersistent` (0x27051a9d) function
        pub fn is_persistent(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([39, 5, 26, 157], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `label` (0xb167f0ca) function
        pub fn label(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 103, 240, 202], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `load` (0xfac88623) function
        pub fn load(
            &self,
            p0: ::corebc_core::types::Address,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([250, 200, 134, 35], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x9ff5ebb8) function
        pub fn make_persistent_0(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 245, 235, 184], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0x05bd09f0) function
        pub fn make_persistent_2(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([5, 189, 9, 240], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0xd0f24610) function
        pub fn make_persistent_3(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Address,
            p2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([208, 242, 70, 16], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `makePersistent` (0xa6180f41) function
        pub fn make_persistent_1(
            &self,
            p0: ::std::vec::Vec<::corebc_core::types::Address>,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 24, 15, 65], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCall` (0x4b25c120) function
        pub fn mock_call_0(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Bytes,
            p2: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 37, 193, 32], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCall` (0xe05abf37) function
        pub fn mock_call_1(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: ::corebc_core::types::Bytes,
            p3: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 90, 191, 55], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCallRevert` (0xe9bb00fb) function
        pub fn mock_call_revert_0(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Bytes,
            p2: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 187, 0, 251], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `mockCallRevert` (0xd216223d) function
        pub fn mock_call_revert_1(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::U256,
            p2: ::corebc_core::types::Bytes,
            p3: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 22, 34, 61], (p0, p1, p2, p3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `openFile` (0x87badc3e) function
        pub fn open_file(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([135, 186, 220, 62], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseAddress` (0xbf03d4f6) function
        pub fn parse_address(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::corebc_core::types::Address,
        > {
            self.0
                .method_hash([191, 3, 212, 246], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBool` (0xc711f34c) function
        pub fn parse_bool(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([199, 17, 243, 76], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBytes` (0x3dc7ecf1) function
        pub fn parse_bytes(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([61, 199, 236, 241], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseBytes32` (0x6ea2d6ed) function
        pub fn parse_bytes_32(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([110, 162, 214, 237], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseInt` (0xcacaea8a) function
        pub fn parse_int(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::I256> {
            self.0
                .method_hash([202, 202, 234, 138], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJson` (0x1b14d0fa) function
        pub fn parse_json_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([27, 20, 208, 250], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJson` (0x9ed12636) function
        pub fn parse_json_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([158, 209, 38, 54], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonAddress` (0x6d037439) function
        pub fn parse_json_address(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::corebc_core::types::Address,
        > {
            self.0
                .method_hash([109, 3, 116, 57], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonAddressArray` (0x73ae0e2b) function
        pub fn parse_json_address_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::Address>,
        > {
            self.0
                .method_hash([115, 174, 14, 43], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBool` (0x1ddc1e05) function
        pub fn parse_json_bool(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([29, 220, 30, 5], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBoolArray` (0x1b7a2c28) function
        pub fn parse_json_bool_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::vec::Vec<bool>> {
            self.0
                .method_hash([27, 122, 44, 40], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytes` (0x4ae7c4b9) function
        pub fn parse_json_bytes(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([74, 231, 196, 185], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytes32` (0x2088d639) function
        pub fn parse_json_bytes_32(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([32, 136, 214, 57], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytes32Array` (0x6ed6bf6d) function
        pub fn parse_json_bytes_32_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::vec::Vec<[u8; 32]>> {
            self.0
                .method_hash([110, 214, 191, 109], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonBytesArray` (0xc7f2b390) function
        pub fn parse_json_bytes_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::Bytes>,
        > {
            self.0
                .method_hash([199, 242, 179, 144], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonInt` (0x7e3d173a) function
        pub fn parse_json_int(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::I256> {
            self.0
                .method_hash([126, 61, 23, 58], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonIntArray` (0xc37c99a1) function
        pub fn parse_json_int_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::I256>,
        > {
            self.0
                .method_hash([195, 124, 153, 161], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonString` (0x9d9b1273) function
        pub fn parse_json_string(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([157, 155, 18, 115], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonStringArray` (0x6499a16c) function
        pub fn parse_json_string_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::std::string::String>,
        > {
            self.0
                .method_hash([100, 153, 161, 108], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonUint` (0x7aa5f1ff) function
        pub fn parse_json_uint(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([122, 165, 241, 255], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseJsonUintArray` (0x3118994d) function
        pub fn parse_json_uint_array(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::corebc_core::types::U256>,
        > {
            self.0
                .method_hash([49, 24, 153, 77], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `parseUint` (0xd3fdc787) function
        pub fn parse_uint(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([211, 253, 199, 135], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pauseGasMetering` (0x48b8fe9f) function
        pub fn pause_gas_metering(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 184, 254, 159], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prank` (0x10d64053) function
        pub fn prank_0(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 214, 64, 83], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `prank` (0x5c3c5387) function
        pub fn prank_1(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 60, 83, 135], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `projectRoot` (0x6e197bfb) function
        pub fn project_root(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([110, 25, 123, 251], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readCallers` (0xc89ced7f) function
        pub fn read_callers(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            (
                ::corebc_core::types::U256,
                ::corebc_core::types::Address,
                ::corebc_core::types::Address,
            ),
        > {
            self.0
                .method_hash([200, 156, 237, 127], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readDir` (0xf108a04c) function
        pub fn read_dir_0(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<
                (::std::string::String, ::std::string::String, u64, bool, bool),
            >,
        > {
            self.0
                .method_hash([241, 8, 160, 76], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readDir` (0xb2f4318c) function
        pub fn read_dir_1(
            &self,
            p0: ::std::string::String,
            p1: u64,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<
                (::std::string::String, ::std::string::String, u64, bool, bool),
            >,
        > {
            self.0
                .method_hash([178, 244, 49, 140], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readDir` (0xc87aafac) function
        pub fn read_dir_2(
            &self,
            p0: ::std::string::String,
            p1: u64,
            p2: bool,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<
                (::std::string::String, ::std::string::String, u64, bool, bool),
            >,
        > {
            self.0
                .method_hash([200, 122, 175, 172], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readFile` (0x875a759d) function
        pub fn read_file(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([135, 90, 117, 157], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readFileBinary` (0x06623015) function
        pub fn read_file_binary(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([6, 98, 48, 21], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readLine` (0xadd52189) function
        pub fn read_line(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([173, 213, 33, 137], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `readLink` (0xbb3ef03a) function
        pub fn read_link(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([187, 62, 240, 58], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `record` (0x620bef66) function
        pub fn record(&self) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 11, 239, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `recordLogs` (0x9abda091) function
        pub fn record_logs(&self) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 189, 160, 145], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rememberKey` (0x40eac48c) function
        pub fn remember_key(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::corebc_core::types::Address,
        > {
            self.0
                .method_hash([64, 234, 196, 140], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeDir` (0x45febd7d) function
        pub fn remove_dir(
            &self,
            p0: ::std::string::String,
            p1: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 254, 189, 125], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `removeFile` (0x32d0a050) function
        pub fn remove_file(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 208, 160, 80], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resetNonce` (0x3a1289c1) function
        pub fn reset_nonce(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([58, 18, 137, 193], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `resumeGasMetering` (0x626bfc65) function
        pub fn resume_gas_metering(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([98, 107, 252, 101], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revertTo` (0x255a8c36) function
        pub fn revert_to(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([37, 90, 140, 54], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePersistent` (0x135c6569) function
        pub fn revoke_persistent_0(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 92, 101, 105], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `revokePersistent` (0x9bf340e8) function
        pub fn revoke_persistent_1(
            &self,
            p0: ::std::vec::Vec<::corebc_core::types::Address>,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 243, 64, 232], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `roll` (0xe8fcf06b) function
        pub fn roll(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 252, 240, 107], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0xe3666077) function
        pub fn roll_fork_0(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 102, 96, 119], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0x0d538c69) function
        pub fn roll_fork_1(
            &self,
            p0: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 83, 140, 105], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0x439cddd8) function
        pub fn roll_fork_2(
            &self,
            p0: ::corebc_core::types::U256,
            p1: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 156, 221, 216], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rollFork` (0x74340412) function
        pub fn roll_fork_3(
            &self,
            p0: ::corebc_core::types::U256,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 52, 4, 18], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrl` (0x931ea701) function
        pub fn rpc_url(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([147, 30, 167, 1], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrlStructs` (0x725410b7) function
        pub fn rpc_url_structs(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<(::std::string::String, ::std::string::String)>,
        > {
            self.0
                .method_hash([114, 84, 16, 183], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `rpcUrls` (0x266095bf) function
        pub fn rpc_urls(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<
            M,
            ::std::vec::Vec<[::std::string::String; 2]>,
        > {
            self.0
                .method_hash([38, 96, 149, 191], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `selectFork` (0x4669f876) function
        pub fn select_fork(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 105, 248, 118], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeAddress` (0x1b9dc788) function
        pub fn serialize_address_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([27, 157, 199, 136], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeAddress` (0x9ef1ff0b) function
        pub fn serialize_address_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::Address>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([158, 241, 255, 11], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBool` (0x06c97128) function
        pub fn serialize_bool_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 201, 113, 40], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBool` (0xc40c3d45) function
        pub fn serialize_bool_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<bool>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([196, 12, 61, 69], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes` (0xd2d2b28a) function
        pub fn serialize_bytes_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([210, 210, 178, 138], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes` (0x0af30cb5) function
        pub fn serialize_bytes_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::Bytes>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([10, 243, 12, 181], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes32` (0x271a3830) function
        pub fn serialize_bytes_320(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([39, 26, 56, 48], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeBytes32` (0x6324bb27) function
        pub fn serialize_bytes_321(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<[u8; 32]>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([99, 36, 187, 39], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeInt` (0x38b12609) function
        pub fn serialize_int_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::corebc_core::types::I256,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([56, 177, 38, 9], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeInt` (0xb2decdb3) function
        pub fn serialize_int_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::I256>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([178, 222, 205, 179], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeString` (0x5c7ccb97) function
        pub fn serialize_string_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([92, 124, 203, 151], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeString` (0xa5df3cd6) function
        pub fn serialize_string_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::std::string::String>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([165, 223, 60, 214], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeUint` (0xa11b741a) function
        pub fn serialize_uint_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([161, 27, 116, 26], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `serializeUint` (0x6d152d8f) function
        pub fn serialize_uint_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::vec::Vec<::corebc_core::types::U256>,
        ) -> ::corebc_contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([109, 21, 45, 143], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setEnv` (0xd7347a08) function
        pub fn set_env(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([215, 52, 122, 8], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNonce` (0x38831543) function
        pub fn set_nonce(
            &self,
            p0: ::corebc_core::types::Address,
            p1: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 131, 21, 67], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `setNonceUnsafe` (0xdc395990) function
        pub fn set_nonce_unsafe(
            &self,
            p0: ::corebc_core::types::Address,
            p1: u64,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 57, 89, 144], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `sign` (0xd0c67f5f) function
        pub fn sign(
            &self,
            p0: ::std::string::String,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::Bytes> {
            self.0
                .method_hash([208, 198, 127, 95], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `skip` (0xe6365bdc) function
        pub fn skip(
            &self,
            p0: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([230, 54, 91, 220], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `snapshot` (0x843c63f8) function
        pub fn snapshot(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ::corebc_core::types::U256> {
            self.0
                .method_hash([132, 60, 99, 248], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0x423144a6) function
        pub fn start_broadcast_0(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 49, 68, 166], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0x14d2714b) function
        pub fn start_broadcast_1(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([20, 210, 113, 75], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startBroadcast` (0x09b04225) function
        pub fn start_broadcast_2(
            &self,
            p0: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 176, 66, 37], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startMappingRecording` (0x01b71662) function
        pub fn start_mapping_recording(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 183, 22, 98], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startPrank` (0xf1236596) function
        pub fn start_prank_0(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 35, 101, 150], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `startPrank` (0x6e46fc4f) function
        pub fn start_prank_1(
            &self,
            p0: ::corebc_core::types::Address,
            p1: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([110, 70, 252, 79], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopBroadcast` (0x741490df) function
        pub fn stop_broadcast(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 20, 144, 223], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopMappingRecording` (0x3c3d719c) function
        pub fn stop_mapping_recording(
            &self,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 61, 113, 156], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `stopPrank` (0xad743cce) function
        pub fn stop_prank(&self) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 116, 60, 206], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `store` (0x404ab67b) function
        pub fn store(
            &self,
            p0: ::corebc_core::types::Address,
            p1: [u8; 32],
            p2: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 74, 182, 123], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xe1c5ec89) function
        pub fn to_string_0(
            &self,
            p0: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 197, 236, 137], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xa79a7df1) function
        pub fn to_string_1(
            &self,
            p0: ::corebc_core::types::Address,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 154, 125, 241], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x26e977fd) function
        pub fn to_string_2(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 233, 119, 253], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xf5d91aec) function
        pub fn to_string_3(
            &self,
            p0: ::corebc_core::types::I256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 217, 26, 236], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0xe9dcfb8e) function
        pub fn to_string_4(
            &self,
            p0: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([233, 220, 251, 142], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `toString` (0x359df6f8) function
        pub fn to_string_5(
            &self,
            p0: bool,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 157, 246, 248], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transact` (0x92cd3fce) function
        pub fn transact_0(
            &self,
            p0: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([146, 205, 63, 206], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transact` (0x04f2e146) function
        pub fn transact_1(
            &self,
            p0: ::corebc_core::types::U256,
            p1: [u8; 32],
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 242, 225, 70], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `txGasPrice` (0xc69545f8) function
        pub fn tx_gas_price(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 149, 69, 248], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `warp` (0x037a7f89) function
        pub fn warp(
            &self,
            p0: ::corebc_core::types::U256,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 122, 127, 137], p0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeFile` (0x6b52a3c0) function
        pub fn write_file(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 82, 163, 192], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeFileBinary` (0x103030bc) function
        pub fn write_file_binary(
            &self,
            p0: ::std::string::String,
            p1: ::corebc_core::types::Bytes,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 48, 48, 188], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeJson` (0xb5e0aa0c) function
        pub fn write_json_0(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 224, 170, 12], (p0, p1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeJson` (0xa57e5e2d) function
        pub fn write_json_1(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
            p2: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 126, 94, 45], (p0, p1, p2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `writeLine` (0x592b1bb6) function
        pub fn write_line(
            &self,
            p0: ::std::string::String,
            p1: ::std::string::String,
        ) -> ::corebc_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 43, 27, 182], (p0, p1))
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::corebc_providers::Middleware> From<::corebc_contract::Contract<M>>
    for HEVM<M> {
        fn from(contract: ::corebc_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `accesses` function with signature `accesses(address)` and selector `0x6e606627`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "accesses", abi = "accesses(address)")]
    pub struct AccessesCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `activeFork` function with signature `activeFork()` and selector `0x2a978c58`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "activeFork", abi = "activeFork()")]
    pub struct ActiveForkCall;
    ///Container type for all input parameters for the `addr` function with signature `addr(string)` and selector `0xf91daf04`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "addr", abi = "addr(string)")]
    pub struct AddrCall(pub ::std::string::String);
    ///Container type for all input parameters for the `allowCheatcodes` function with signature `allowCheatcodes(address)` and selector `0x21bd8ec4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "allowCheatcodes", abi = "allowCheatcodes(address)")]
    pub struct AllowCheatcodesCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `assume` function with signature `assume(bool)` and selector `0xca480fa9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "assume", abi = "assume(bool)")]
    pub struct AssumeCall(pub bool);
    ///Container type for all input parameters for the `breakpoint` function with signature `breakpoint(string)` and selector `0x3af777a9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "breakpoint", abi = "breakpoint(string)")]
    pub struct Breakpoint0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `breakpoint` function with signature `breakpoint(string,bool)` and selector `0x18af918f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "breakpoint", abi = "breakpoint(string,bool)")]
    pub struct Breakpoint1Call(pub ::std::string::String, pub bool);
    ///Container type for all input parameters for the `broadcast` function with signature `broadcast()` and selector `0x12f46a0c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "broadcast", abi = "broadcast()")]
    pub struct Broadcast0Call;
    ///Container type for all input parameters for the `broadcast` function with signature `broadcast(address)` and selector `0x3aec5261`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "broadcast", abi = "broadcast(address)")]
    pub struct Broadcast1Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `broadcast` function with signature `broadcast(string)` and selector `0x85b0a416`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "broadcast", abi = "broadcast(string)")]
    pub struct Broadcast2Call(pub ::std::string::String);
    ///Container type for all input parameters for the `chainId` function with signature `chainId(uint256)` and selector `0x5858cd87`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "chainId", abi = "chainId(uint256)")]
    pub struct ChainIdCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `clearMockedCalls` function with signature `clearMockedCalls()` and selector `0x7eaff7de`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "clearMockedCalls", abi = "clearMockedCalls()")]
    pub struct ClearMockedCallsCall;
    ///Container type for all input parameters for the `closeFile` function with signature `closeFile(string)` and selector `0xa3a9f387`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "closeFile", abi = "closeFile(string)")]
    pub struct CloseFileCall(pub ::std::string::String);
    ///Container type for all input parameters for the `coinbase` function with signature `coinbase(address)` and selector `0x53a5c6de`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "coinbase", abi = "coinbase(address)")]
    pub struct CoinbaseCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32,address)` and selector `0x447f5ae9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "computeCreate2Address",
        abi = "computeCreate2Address(bytes32,bytes32,address)"
    )]
    pub struct ComputeCreate2Address1Call(
        pub [u8; 32],
        pub [u8; 32],
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `computeCreate2Address` function with signature `computeCreate2Address(bytes32,bytes32)` and selector `0xf27a5040`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "computeCreate2Address",
        abi = "computeCreate2Address(bytes32,bytes32)"
    )]
    pub struct ComputeCreate2Address0Call(pub [u8; 32], pub [u8; 32]);
    ///Container type for all input parameters for the `computeCreateAddress` function with signature `computeCreateAddress(address,uint256)` and selector `0x7486025d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "computeCreateAddress",
        abi = "computeCreateAddress(address,uint256)"
    )]
    pub struct ComputeCreateAddressCall(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `createDir` function with signature `createDir(string,bool)` and selector `0x8b295957`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createDir", abi = "createDir(string,bool)")]
    pub struct CreateDirCall(pub ::std::string::String, pub bool);
    ///Container type for all input parameters for the `createFork` function with signature `createFork(string,uint256)` and selector `0x27dc0128`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createFork", abi = "createFork(string,uint256)")]
    pub struct CreateFork1Call(
        pub ::std::string::String,
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `createFork` function with signature `createFork(string,bytes32)` and selector `0x472ee61e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createFork", abi = "createFork(string,bytes32)")]
    pub struct CreateFork2Call(pub ::std::string::String, pub [u8; 32]);
    ///Container type for all input parameters for the `createFork` function with signature `createFork(string)` and selector `0xc8282be0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createFork", abi = "createFork(string)")]
    pub struct CreateFork0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `createSelectFork` function with signature `createSelectFork(string,uint256)` and selector `0xcc15574b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createSelectFork", abi = "createSelectFork(string,uint256)")]
    pub struct CreateSelectFork1Call(
        pub ::std::string::String,
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `createSelectFork` function with signature `createSelectFork(string,bytes32)` and selector `0x47cbbd7a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createSelectFork", abi = "createSelectFork(string,bytes32)")]
    pub struct CreateSelectFork2Call(pub ::std::string::String, pub [u8; 32]);
    ///Container type for all input parameters for the `createSelectFork` function with signature `createSelectFork(string)` and selector `0x6bcf2c60`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "createSelectFork", abi = "createSelectFork(string)")]
    pub struct CreateSelectFork0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `deal` function with signature `deal(address,uint256)` and selector `0x5ae137d7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deal", abi = "deal(address,uint256)")]
    pub struct DealCall(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,uint32)` and selector `0xf6f7e9e1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,uint32)")]
    pub struct DeriveKey0Call(pub ::std::string::String, pub u32);
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,string,uint32)` and selector `0xd5a72210`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,string,uint32)")]
    pub struct DeriveKey1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub u32,
    );
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,uint32,string)` and selector `0xee602b67`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,uint32,string)")]
    pub struct DeriveKey2Call(
        pub ::std::string::String,
        pub u32,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `deriveKey` function with signature `deriveKey(string,string,uint32,string)` and selector `0xcc819384`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "deriveKey", abi = "deriveKey(string,string,uint32,string)")]
    pub struct DeriveKey3Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub u32,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `difficulty` function with signature `difficulty(uint256)` and selector `0xfc75ba15`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "difficulty", abi = "difficulty(uint256)")]
    pub struct DifficultyCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `envAddress` function with signature `envAddress(string)` and selector `0x5992c5c6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envAddress", abi = "envAddress(string)")]
    pub struct EnvAddress0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envAddress` function with signature `envAddress(string,string)` and selector `0xdd088083`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envAddress", abi = "envAddress(string,string)")]
    pub struct EnvAddress1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envBool` function with signature `envBool(string)` and selector `0xfd40bd11`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envBool", abi = "envBool(string)")]
    pub struct EnvBool0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envBool` function with signature `envBool(string,string)` and selector `0xdf739509`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envBool", abi = "envBool(string,string)")]
    pub struct EnvBool1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envBytes` function with signature `envBytes(string)` and selector `0xb1676ba0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envBytes", abi = "envBytes(string)")]
    pub struct EnvBytes0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envBytes` function with signature `envBytes(string,string)` and selector `0xd9577bb1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envBytes", abi = "envBytes(string,string)")]
    pub struct EnvBytes1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envBytes32` function with signature `envBytes32(string)` and selector `0x2d261418`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envBytes32", abi = "envBytes32(string)")]
    pub struct EnvBytes320Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envBytes32` function with signature `envBytes32(string,string)` and selector `0x5c1826d3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envBytes32", abi = "envBytes32(string,string)")]
    pub struct EnvBytes321Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envInt` function with signature `envInt(string)` and selector `0xdba2cad8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envInt", abi = "envInt(string)")]
    pub struct EnvInt0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envInt` function with signature `envInt(string,string)` and selector `0x347968b0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envInt", abi = "envInt(string,string)")]
    pub struct EnvInt1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,bool)` and selector `0x8efd04a9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,bool)")]
    pub struct EnvOr0Call(pub ::std::string::String, pub bool);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,uint256)` and selector `0x1c6db43b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,uint256)")]
    pub struct EnvOr1Call(pub ::std::string::String, pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,int256)` and selector `0x35711e3f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,int256)")]
    pub struct EnvOr2Call(pub ::std::string::String, pub ::corebc_core::types::I256);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,address)` and selector `0x18745f8c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,address)")]
    pub struct EnvOr3Call(pub ::std::string::String, pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,bytes32)` and selector `0xcb48937a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,bytes32)")]
    pub struct EnvOr4Call(pub ::std::string::String, pub [u8; 32]);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string)` and selector `0x42d1d0ef`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string)")]
    pub struct EnvOr5Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,bytes)` and selector `0x2dd17959`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,bytes)")]
    pub struct EnvOr6Call(pub ::std::string::String, pub ::corebc_core::types::Bytes);
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,bool[])` and selector `0x9430ce1e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,bool[])")]
    pub struct EnvOr7Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<bool>,
    );
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,uint256[])` and selector `0xbf6e43ef`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,uint256[])")]
    pub struct EnvOr8Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::U256>,
    );
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,int256[])` and selector `0x60a8eb5e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,int256[])")]
    pub struct EnvOr9Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::I256>,
    );
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,address[])` and selector `0x0dbd12fc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,address[])")]
    pub struct EnvOr10Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::Address>,
    );
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,bytes32[])` and selector `0x864e3a3c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,bytes32[])")]
    pub struct EnvOr11Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<[u8; 32]>,
    );
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,string[])` and selector `0x9eb2a8c4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,string[])")]
    pub struct EnvOr12Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::std::string::String>,
    );
    ///Container type for all input parameters for the `envOr` function with signature `envOr(string,string,bytes[])` and selector `0x206f7cdd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envOr", abi = "envOr(string,string,bytes[])")]
    pub struct EnvOr13Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::Bytes>,
    );
    ///Container type for all input parameters for the `envString` function with signature `envString(string)` and selector `0xe613c3f6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envString", abi = "envString(string)")]
    pub struct EnvString0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envString` function with signature `envString(string,string)` and selector `0x535a86f8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envString", abi = "envString(string,string)")]
    pub struct EnvString1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `envUint` function with signature `envUint(string)` and selector `0x22b1fcf4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envUint", abi = "envUint(string)")]
    pub struct EnvUint0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `envUint` function with signature `envUint(string,string)` and selector `0x09437722`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "envUint", abi = "envUint(string,string)")]
    pub struct EnvUint1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `etch` function with signature `etch(address,bytes)` and selector `0x61106001`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "etch", abi = "etch(address,bytes)")]
    pub struct EtchCall(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,bytes)` and selector `0xc244877f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,bytes)")]
    pub struct ExpectCall0Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,bytes,uint64)` and selector `0x35ce3631`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,bytes,uint64)")]
    pub struct ExpectCall1Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Bytes,
        pub u64,
    );
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,bytes)` and selector `0x5a1f47d6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,bytes)")]
    pub struct ExpectCall2Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,bytes,uint64)` and selector `0xde6e6e54`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,bytes,uint64)")]
    pub struct ExpectCall3Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub ::corebc_core::types::Bytes,
        pub u64,
    );
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,uint64,bytes)` and selector `0x536671ce`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectCall", abi = "expectCall(address,uint256,uint64,bytes)")]
    pub struct ExpectCall4Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub u64,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `expectCall` function with signature `expectCall(address,uint256,uint64,bytes,uint64)` and selector `0x66ca6367`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "expectCall",
        abi = "expectCall(address,uint256,uint64,bytes,uint64)"
    )]
    pub struct ExpectCall5Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub u64,
        pub ::corebc_core::types::Bytes,
        pub u64,
    );
    ///Container type for all input parameters for the `expectCallMinGas` function with signature `expectCallMinGas(address,uint256,uint64,bytes)` and selector `0x782f8cdb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "expectCallMinGas",
        abi = "expectCallMinGas(address,uint256,uint64,bytes)"
    )]
    pub struct ExpectCallMinGas0Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub u64,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `expectCallMinGas` function with signature `expectCallMinGas(address,uint256,uint64,bytes,uint64)` and selector `0x7f5de23e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "expectCallMinGas",
        abi = "expectCallMinGas(address,uint256,uint64,bytes,uint64)"
    )]
    pub struct ExpectCallMinGas1Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub u64,
        pub ::corebc_core::types::Bytes,
        pub u64,
    );
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit()` and selector `0x51bd56ad`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectEmit", abi = "expectEmit()")]
    pub struct ExpectEmit0Call;
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit(address)` and selector `0xbff0146e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectEmit", abi = "expectEmit(address)")]
    pub struct ExpectEmit1Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit(bool,bool,bool,bool)` and selector `0xe2e01dd4`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectEmit", abi = "expectEmit(bool,bool,bool,bool)")]
    pub struct ExpectEmit2Call(pub bool, pub bool, pub bool, pub bool);
    ///Container type for all input parameters for the `expectEmit` function with signature `expectEmit(bool,bool,bool,bool,address)` and selector `0xdb3d4fd8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectEmit", abi = "expectEmit(bool,bool,bool,bool,address)")]
    pub struct ExpectEmit3Call(
        pub bool,
        pub bool,
        pub bool,
        pub bool,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `expectRevert` function with signature `expectRevert()` and selector `0x10403acb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectRevert", abi = "expectRevert()")]
    pub struct ExpectRevert0Call;
    ///Container type for all input parameters for the `expectRevert` function with signature `expectRevert(bytes)` and selector `0xe37ab4ed`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectRevert", abi = "expectRevert(bytes)")]
    pub struct ExpectRevert1Call(pub ::corebc_core::types::Bytes);
    ///Container type for all input parameters for the `expectRevert` function with signature `expectRevert(bytes4)` and selector `0x4e7614bc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectRevert", abi = "expectRevert(bytes4)")]
    pub struct ExpectRevert2Call(pub [u8; 4]);
    ///Container type for all input parameters for the `expectSafeMemory` function with signature `expectSafeMemory(uint64,uint64)` and selector `0x46bb5493`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "expectSafeMemory", abi = "expectSafeMemory(uint64,uint64)")]
    pub struct ExpectSafeMemoryCall(pub u64, pub u64);
    ///Container type for all input parameters for the `expectSafeMemoryCall` function with signature `expectSafeMemoryCall(uint64,uint64)` and selector `0x9b61c661`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "expectSafeMemoryCall",
        abi = "expectSafeMemoryCall(uint64,uint64)"
    )]
    pub struct ExpectSafeMemoryCallCall(pub u64, pub u64);
    ///Container type for all input parameters for the `fee` function with signature `fee(uint256)` and selector `0xaa32efc1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fee", abi = "fee(uint256)")]
    pub struct FeeCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `ffi` function with signature `ffi(string[])` and selector `0x64ba2b32`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "ffi", abi = "ffi(string[])")]
    pub struct FfiCall(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all input parameters for the `fsMetadata` function with signature `fsMetadata(string)` and selector `0x895b0357`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "fsMetadata", abi = "fsMetadata(string)")]
    pub struct FsMetadataCall(pub ::std::string::String);
    ///Container type for all input parameters for the `getCode` function with signature `getCode(string)` and selector `0xf4311698`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getCode", abi = "getCode(string)")]
    pub struct GetCodeCall(pub ::std::string::String);
    ///Container type for all input parameters for the `getDeployedCode` function with signature `getDeployedCode(string)` and selector `0xe04cfba1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getDeployedCode", abi = "getDeployedCode(string)")]
    pub struct GetDeployedCodeCall(pub ::std::string::String);
    ///Container type for all input parameters for the `getLabel` function with signature `getLabel(address)` and selector `0x56c8fc59`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getLabel", abi = "getLabel(address)")]
    pub struct GetLabelCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `getMappingKeyAndParentOf` function with signature `getMappingKeyAndParentOf(address,bytes32)` and selector `0x1b402ced`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getMappingKeyAndParentOf",
        abi = "getMappingKeyAndParentOf(address,bytes32)"
    )]
    pub struct GetMappingKeyAndParentOfCall(
        pub ::corebc_core::types::Address,
        pub [u8; 32],
    );
    ///Container type for all input parameters for the `getMappingLength` function with signature `getMappingLength(address,bytes32)` and selector `0x5e0a6a20`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getMappingLength", abi = "getMappingLength(address,bytes32)")]
    pub struct GetMappingLengthCall(pub ::corebc_core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `getMappingSlotAt` function with signature `getMappingSlotAt(address,bytes32,uint256)` and selector `0x6028eb0c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "getMappingSlotAt",
        abi = "getMappingSlotAt(address,bytes32,uint256)"
    )]
    pub struct GetMappingSlotAtCall(
        pub ::corebc_core::types::Address,
        pub [u8; 32],
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `getNonce` function with signature `getNonce(address)` and selector `0x7ee608d7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getNonce", abi = "getNonce(address)")]
    pub struct GetNonceCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `getRecordedLogs` function with signature `getRecordedLogs()` and selector `0x6aa9ac15`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "getRecordedLogs", abi = "getRecordedLogs()")]
    pub struct GetRecordedLogsCall;
    ///Container type for all input parameters for the `isPersistent` function with signature `isPersistent(address)` and selector `0x27051a9d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isPersistent", abi = "isPersistent(address)")]
    pub struct IsPersistentCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `label` function with signature `label(address,string)` and selector `0xb167f0ca`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "label", abi = "label(address,string)")]
    pub struct LabelCall(pub ::corebc_core::types::Address, pub ::std::string::String);
    ///Container type for all input parameters for the `load` function with signature `load(address,bytes32)` and selector `0xfac88623`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "load", abi = "load(address,bytes32)")]
    pub struct LoadCall(pub ::corebc_core::types::Address, pub [u8; 32]);
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address)` and selector `0x9ff5ebb8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "makePersistent", abi = "makePersistent(address)")]
    pub struct MakePersistent0Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address,address)` and selector `0x05bd09f0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "makePersistent", abi = "makePersistent(address,address)")]
    pub struct MakePersistent2Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address,address,address)` and selector `0xd0f24610`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "makePersistent", abi = "makePersistent(address,address,address)")]
    pub struct MakePersistent3Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `makePersistent` function with signature `makePersistent(address[])` and selector `0xa6180f41`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "makePersistent", abi = "makePersistent(address[])")]
    pub struct MakePersistent1Call(pub ::std::vec::Vec<::corebc_core::types::Address>);
    ///Container type for all input parameters for the `mockCall` function with signature `mockCall(address,bytes,bytes)` and selector `0x4b25c120`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mockCall", abi = "mockCall(address,bytes,bytes)")]
    pub struct MockCall0Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Bytes,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `mockCall` function with signature `mockCall(address,uint256,bytes,bytes)` and selector `0xe05abf37`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mockCall", abi = "mockCall(address,uint256,bytes,bytes)")]
    pub struct MockCall1Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub ::corebc_core::types::Bytes,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `mockCallRevert` function with signature `mockCallRevert(address,bytes,bytes)` and selector `0xe9bb00fb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "mockCallRevert", abi = "mockCallRevert(address,bytes,bytes)")]
    pub struct MockCallRevert0Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Bytes,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `mockCallRevert` function with signature `mockCallRevert(address,uint256,bytes,bytes)` and selector `0xd216223d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "mockCallRevert",
        abi = "mockCallRevert(address,uint256,bytes,bytes)"
    )]
    pub struct MockCallRevert1Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::U256,
        pub ::corebc_core::types::Bytes,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `openFile` function with signature `openFile(string)` and selector `0x87badc3e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "openFile", abi = "openFile(string)")]
    pub struct OpenFileCall(pub ::std::string::String);
    ///Container type for all input parameters for the `parseAddress` function with signature `parseAddress(string)` and selector `0xbf03d4f6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseAddress", abi = "parseAddress(string)")]
    pub struct ParseAddressCall(pub ::std::string::String);
    ///Container type for all input parameters for the `parseBool` function with signature `parseBool(string)` and selector `0xc711f34c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseBool", abi = "parseBool(string)")]
    pub struct ParseBoolCall(pub ::std::string::String);
    ///Container type for all input parameters for the `parseBytes` function with signature `parseBytes(string)` and selector `0x3dc7ecf1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseBytes", abi = "parseBytes(string)")]
    pub struct ParseBytesCall(pub ::std::string::String);
    ///Container type for all input parameters for the `parseBytes32` function with signature `parseBytes32(string)` and selector `0x6ea2d6ed`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseBytes32", abi = "parseBytes32(string)")]
    pub struct ParseBytes32Call(pub ::std::string::String);
    ///Container type for all input parameters for the `parseInt` function with signature `parseInt(string)` and selector `0xcacaea8a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseInt", abi = "parseInt(string)")]
    pub struct ParseIntCall(pub ::std::string::String);
    ///Container type for all input parameters for the `parseJson` function with signature `parseJson(string)` and selector `0x1b14d0fa`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJson", abi = "parseJson(string)")]
    pub struct ParseJson0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `parseJson` function with signature `parseJson(string,string)` and selector `0x9ed12636`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJson", abi = "parseJson(string,string)")]
    pub struct ParseJson1Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `parseJsonAddress` function with signature `parseJsonAddress(string,string)` and selector `0x6d037439`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonAddress", abi = "parseJsonAddress(string,string)")]
    pub struct ParseJsonAddressCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonAddressArray` function with signature `parseJsonAddressArray(string,string)` and selector `0x73ae0e2b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "parseJsonAddressArray",
        abi = "parseJsonAddressArray(string,string)"
    )]
    pub struct ParseJsonAddressArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonBool` function with signature `parseJsonBool(string,string)` and selector `0x1ddc1e05`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonBool", abi = "parseJsonBool(string,string)")]
    pub struct ParseJsonBoolCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `parseJsonBoolArray` function with signature `parseJsonBoolArray(string,string)` and selector `0x1b7a2c28`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonBoolArray", abi = "parseJsonBoolArray(string,string)")]
    pub struct ParseJsonBoolArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonBytes` function with signature `parseJsonBytes(string,string)` and selector `0x4ae7c4b9`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonBytes", abi = "parseJsonBytes(string,string)")]
    pub struct ParseJsonBytesCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `parseJsonBytes32` function with signature `parseJsonBytes32(string,string)` and selector `0x2088d639`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonBytes32", abi = "parseJsonBytes32(string,string)")]
    pub struct ParseJsonBytes32Call(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonBytes32Array` function with signature `parseJsonBytes32Array(string,string)` and selector `0x6ed6bf6d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "parseJsonBytes32Array",
        abi = "parseJsonBytes32Array(string,string)"
    )]
    pub struct ParseJsonBytes32ArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonBytesArray` function with signature `parseJsonBytesArray(string,string)` and selector `0xc7f2b390`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonBytesArray", abi = "parseJsonBytesArray(string,string)")]
    pub struct ParseJsonBytesArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonInt` function with signature `parseJsonInt(string,string)` and selector `0x7e3d173a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonInt", abi = "parseJsonInt(string,string)")]
    pub struct ParseJsonIntCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `parseJsonIntArray` function with signature `parseJsonIntArray(string,string)` and selector `0xc37c99a1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonIntArray", abi = "parseJsonIntArray(string,string)")]
    pub struct ParseJsonIntArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonString` function with signature `parseJsonString(string,string)` and selector `0x9d9b1273`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonString", abi = "parseJsonString(string,string)")]
    pub struct ParseJsonStringCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `parseJsonStringArray` function with signature `parseJsonStringArray(string,string)` and selector `0x6499a16c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "parseJsonStringArray",
        abi = "parseJsonStringArray(string,string)"
    )]
    pub struct ParseJsonStringArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseJsonUint` function with signature `parseJsonUint(string,string)` and selector `0x7aa5f1ff`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonUint", abi = "parseJsonUint(string,string)")]
    pub struct ParseJsonUintCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `parseJsonUintArray` function with signature `parseJsonUintArray(string,string)` and selector `0x3118994d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseJsonUintArray", abi = "parseJsonUintArray(string,string)")]
    pub struct ParseJsonUintArrayCall(
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `parseUint` function with signature `parseUint(string)` and selector `0xd3fdc787`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "parseUint", abi = "parseUint(string)")]
    pub struct ParseUintCall(pub ::std::string::String);
    ///Container type for all input parameters for the `pauseGasMetering` function with signature `pauseGasMetering()` and selector `0x48b8fe9f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "pauseGasMetering", abi = "pauseGasMetering()")]
    pub struct PauseGasMeteringCall;
    ///Container type for all input parameters for the `prank` function with signature `prank(address)` and selector `0x10d64053`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "prank", abi = "prank(address)")]
    pub struct Prank0Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `prank` function with signature `prank(address,address)` and selector `0x5c3c5387`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "prank", abi = "prank(address,address)")]
    pub struct Prank1Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `projectRoot` function with signature `projectRoot()` and selector `0x6e197bfb`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "projectRoot", abi = "projectRoot()")]
    pub struct ProjectRootCall;
    ///Container type for all input parameters for the `readCallers` function with signature `readCallers()` and selector `0xc89ced7f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readCallers", abi = "readCallers()")]
    pub struct ReadCallersCall;
    ///Container type for all input parameters for the `readDir` function with signature `readDir(string)` and selector `0xf108a04c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readDir", abi = "readDir(string)")]
    pub struct ReadDir0Call(pub ::std::string::String);
    ///Container type for all input parameters for the `readDir` function with signature `readDir(string,uint64)` and selector `0xb2f4318c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readDir", abi = "readDir(string,uint64)")]
    pub struct ReadDir1Call(pub ::std::string::String, pub u64);
    ///Container type for all input parameters for the `readDir` function with signature `readDir(string,uint64,bool)` and selector `0xc87aafac`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readDir", abi = "readDir(string,uint64,bool)")]
    pub struct ReadDir2Call(pub ::std::string::String, pub u64, pub bool);
    ///Container type for all input parameters for the `readFile` function with signature `readFile(string)` and selector `0x875a759d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readFile", abi = "readFile(string)")]
    pub struct ReadFileCall(pub ::std::string::String);
    ///Container type for all input parameters for the `readFileBinary` function with signature `readFileBinary(string)` and selector `0x06623015`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readFileBinary", abi = "readFileBinary(string)")]
    pub struct ReadFileBinaryCall(pub ::std::string::String);
    ///Container type for all input parameters for the `readLine` function with signature `readLine(string)` and selector `0xadd52189`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readLine", abi = "readLine(string)")]
    pub struct ReadLineCall(pub ::std::string::String);
    ///Container type for all input parameters for the `readLink` function with signature `readLink(string)` and selector `0xbb3ef03a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "readLink", abi = "readLink(string)")]
    pub struct ReadLinkCall(pub ::std::string::String);
    ///Container type for all input parameters for the `record` function with signature `record()` and selector `0x620bef66`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "record", abi = "record()")]
    pub struct RecordCall;
    ///Container type for all input parameters for the `recordLogs` function with signature `recordLogs()` and selector `0x9abda091`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "recordLogs", abi = "recordLogs()")]
    pub struct RecordLogsCall;
    ///Container type for all input parameters for the `rememberKey` function with signature `rememberKey(string)` and selector `0x40eac48c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rememberKey", abi = "rememberKey(string)")]
    pub struct RememberKeyCall(pub ::std::string::String);
    ///Container type for all input parameters for the `removeDir` function with signature `removeDir(string,bool)` and selector `0x45febd7d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeDir", abi = "removeDir(string,bool)")]
    pub struct RemoveDirCall(pub ::std::string::String, pub bool);
    ///Container type for all input parameters for the `removeFile` function with signature `removeFile(string)` and selector `0x32d0a050`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "removeFile", abi = "removeFile(string)")]
    pub struct RemoveFileCall(pub ::std::string::String);
    ///Container type for all input parameters for the `resetNonce` function with signature `resetNonce(address)` and selector `0x3a1289c1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "resetNonce", abi = "resetNonce(address)")]
    pub struct ResetNonceCall(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `resumeGasMetering` function with signature `resumeGasMetering()` and selector `0x626bfc65`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "resumeGasMetering", abi = "resumeGasMetering()")]
    pub struct ResumeGasMeteringCall;
    ///Container type for all input parameters for the `revertTo` function with signature `revertTo(uint256)` and selector `0x255a8c36`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revertTo", abi = "revertTo(uint256)")]
    pub struct RevertToCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `revokePersistent` function with signature `revokePersistent(address)` and selector `0x135c6569`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokePersistent", abi = "revokePersistent(address)")]
    pub struct RevokePersistent0Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `revokePersistent` function with signature `revokePersistent(address[])` and selector `0x9bf340e8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "revokePersistent", abi = "revokePersistent(address[])")]
    pub struct RevokePersistent1Call(pub ::std::vec::Vec<::corebc_core::types::Address>);
    ///Container type for all input parameters for the `roll` function with signature `roll(uint256)` and selector `0xe8fcf06b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "roll", abi = "roll(uint256)")]
    pub struct RollCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(uint256)` and selector `0xe3666077`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollFork", abi = "rollFork(uint256)")]
    pub struct RollFork0Call(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(bytes32)` and selector `0x0d538c69`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollFork", abi = "rollFork(bytes32)")]
    pub struct RollFork1Call(pub [u8; 32]);
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(uint256,uint256)` and selector `0x439cddd8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollFork", abi = "rollFork(uint256,uint256)")]
    pub struct RollFork2Call(
        pub ::corebc_core::types::U256,
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `rollFork` function with signature `rollFork(uint256,bytes32)` and selector `0x74340412`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rollFork", abi = "rollFork(uint256,bytes32)")]
    pub struct RollFork3Call(pub ::corebc_core::types::U256, pub [u8; 32]);
    ///Container type for all input parameters for the `rpcUrl` function with signature `rpcUrl(string)` and selector `0x931ea701`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rpcUrl", abi = "rpcUrl(string)")]
    pub struct RpcUrlCall(pub ::std::string::String);
    ///Container type for all input parameters for the `rpcUrlStructs` function with signature `rpcUrlStructs()` and selector `0x725410b7`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rpcUrlStructs", abi = "rpcUrlStructs()")]
    pub struct RpcUrlStructsCall;
    ///Container type for all input parameters for the `rpcUrls` function with signature `rpcUrls()` and selector `0x266095bf`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "rpcUrls", abi = "rpcUrls()")]
    pub struct RpcUrlsCall;
    ///Container type for all input parameters for the `selectFork` function with signature `selectFork(uint256)` and selector `0x4669f876`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "selectFork", abi = "selectFork(uint256)")]
    pub struct SelectForkCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `serializeAddress` function with signature `serializeAddress(string,string,address)` and selector `0x1b9dc788`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "serializeAddress",
        abi = "serializeAddress(string,string,address)"
    )]
    pub struct SerializeAddress0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `serializeAddress` function with signature `serializeAddress(string,string,address[])` and selector `0x9ef1ff0b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "serializeAddress",
        abi = "serializeAddress(string,string,address[])"
    )]
    pub struct SerializeAddress1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::Address>,
    );
    ///Container type for all input parameters for the `serializeBool` function with signature `serializeBool(string,string,bool)` and selector `0x06c97128`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeBool", abi = "serializeBool(string,string,bool)")]
    pub struct SerializeBool0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub bool,
    );
    ///Container type for all input parameters for the `serializeBool` function with signature `serializeBool(string,string,bool[])` and selector `0xc40c3d45`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeBool", abi = "serializeBool(string,string,bool[])")]
    pub struct SerializeBool1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<bool>,
    );
    ///Container type for all input parameters for the `serializeBytes` function with signature `serializeBytes(string,string,bytes)` and selector `0xd2d2b28a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeBytes", abi = "serializeBytes(string,string,bytes)")]
    pub struct SerializeBytes0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `serializeBytes` function with signature `serializeBytes(string,string,bytes[])` and selector `0x0af30cb5`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeBytes", abi = "serializeBytes(string,string,bytes[])")]
    pub struct SerializeBytes1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::Bytes>,
    );
    ///Container type for all input parameters for the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32)` and selector `0x271a3830`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "serializeBytes32",
        abi = "serializeBytes32(string,string,bytes32)"
    )]
    pub struct SerializeBytes320Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub [u8; 32],
    );
    ///Container type for all input parameters for the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32[])` and selector `0x6324bb27`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(
        name = "serializeBytes32",
        abi = "serializeBytes32(string,string,bytes32[])"
    )]
    pub struct SerializeBytes321Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<[u8; 32]>,
    );
    ///Container type for all input parameters for the `serializeInt` function with signature `serializeInt(string,string,int256)` and selector `0x38b12609`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeInt", abi = "serializeInt(string,string,int256)")]
    pub struct SerializeInt0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::corebc_core::types::I256,
    );
    ///Container type for all input parameters for the `serializeInt` function with signature `serializeInt(string,string,int256[])` and selector `0xb2decdb3`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeInt", abi = "serializeInt(string,string,int256[])")]
    pub struct SerializeInt1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::I256>,
    );
    ///Container type for all input parameters for the `serializeString` function with signature `serializeString(string,string,string)` and selector `0x5c7ccb97`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeString", abi = "serializeString(string,string,string)")]
    pub struct SerializeString0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `serializeString` function with signature `serializeString(string,string,string[])` and selector `0xa5df3cd6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeString", abi = "serializeString(string,string,string[])")]
    pub struct SerializeString1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::std::string::String>,
    );
    ///Container type for all input parameters for the `serializeUint` function with signature `serializeUint(string,string,uint256)` and selector `0xa11b741a`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeUint", abi = "serializeUint(string,string,uint256)")]
    pub struct SerializeUint0Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::corebc_core::types::U256,
    );
    ///Container type for all input parameters for the `serializeUint` function with signature `serializeUint(string,string,uint256[])` and selector `0x6d152d8f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "serializeUint", abi = "serializeUint(string,string,uint256[])")]
    pub struct SerializeUint1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::vec::Vec<::corebc_core::types::U256>,
    );
    ///Container type for all input parameters for the `setEnv` function with signature `setEnv(string,string)` and selector `0xd7347a08`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setEnv", abi = "setEnv(string,string)")]
    pub struct SetEnvCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `setNonce` function with signature `setNonce(address,uint64)` and selector `0x38831543`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setNonce", abi = "setNonce(address,uint64)")]
    pub struct SetNonceCall(pub ::corebc_core::types::Address, pub u64);
    ///Container type for all input parameters for the `setNonceUnsafe` function with signature `setNonceUnsafe(address,uint64)` and selector `0xdc395990`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "setNonceUnsafe", abi = "setNonceUnsafe(address,uint64)")]
    pub struct SetNonceUnsafeCall(pub ::corebc_core::types::Address, pub u64);
    ///Container type for all input parameters for the `sign` function with signature `sign(string,bytes32)` and selector `0xd0c67f5f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "sign", abi = "sign(string,bytes32)")]
    pub struct SignCall(pub ::std::string::String, pub [u8; 32]);
    ///Container type for all input parameters for the `skip` function with signature `skip(bool)` and selector `0xe6365bdc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "skip", abi = "skip(bool)")]
    pub struct SkipCall(pub bool);
    ///Container type for all input parameters for the `snapshot` function with signature `snapshot()` and selector `0x843c63f8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "snapshot", abi = "snapshot()")]
    pub struct SnapshotCall;
    ///Container type for all input parameters for the `startBroadcast` function with signature `startBroadcast()` and selector `0x423144a6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "startBroadcast", abi = "startBroadcast()")]
    pub struct StartBroadcast0Call;
    ///Container type for all input parameters for the `startBroadcast` function with signature `startBroadcast(address)` and selector `0x14d2714b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "startBroadcast", abi = "startBroadcast(address)")]
    pub struct StartBroadcast1Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `startBroadcast` function with signature `startBroadcast(string)` and selector `0x09b04225`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "startBroadcast", abi = "startBroadcast(string)")]
    pub struct StartBroadcast2Call(pub ::std::string::String);
    ///Container type for all input parameters for the `startMappingRecording` function with signature `startMappingRecording()` and selector `0x01b71662`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "startMappingRecording", abi = "startMappingRecording()")]
    pub struct StartMappingRecordingCall;
    ///Container type for all input parameters for the `startPrank` function with signature `startPrank(address)` and selector `0xf1236596`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "startPrank", abi = "startPrank(address)")]
    pub struct StartPrank0Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `startPrank` function with signature `startPrank(address,address)` and selector `0x6e46fc4f`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "startPrank", abi = "startPrank(address,address)")]
    pub struct StartPrank1Call(
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all input parameters for the `stopBroadcast` function with signature `stopBroadcast()` and selector `0x741490df`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stopBroadcast", abi = "stopBroadcast()")]
    pub struct StopBroadcastCall;
    ///Container type for all input parameters for the `stopMappingRecording` function with signature `stopMappingRecording()` and selector `0x3c3d719c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stopMappingRecording", abi = "stopMappingRecording()")]
    pub struct StopMappingRecordingCall;
    ///Container type for all input parameters for the `stopPrank` function with signature `stopPrank()` and selector `0xad743cce`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "stopPrank", abi = "stopPrank()")]
    pub struct StopPrankCall;
    ///Container type for all input parameters for the `store` function with signature `store(address,bytes32,bytes32)` and selector `0x404ab67b`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "store", abi = "store(address,bytes32,bytes32)")]
    pub struct StoreCall(pub ::corebc_core::types::Address, pub [u8; 32], pub [u8; 32]);
    ///Container type for all input parameters for the `toString` function with signature `toString(bytes)` and selector `0xe1c5ec89`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toString", abi = "toString(bytes)")]
    pub struct ToString0Call(pub ::corebc_core::types::Bytes);
    ///Container type for all input parameters for the `toString` function with signature `toString(address)` and selector `0xa79a7df1`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toString", abi = "toString(address)")]
    pub struct ToString1Call(pub ::corebc_core::types::Address);
    ///Container type for all input parameters for the `toString` function with signature `toString(uint256)` and selector `0x26e977fd`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toString", abi = "toString(uint256)")]
    pub struct ToString2Call(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `toString` function with signature `toString(int256)` and selector `0xf5d91aec`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toString", abi = "toString(int256)")]
    pub struct ToString3Call(pub ::corebc_core::types::I256);
    ///Container type for all input parameters for the `toString` function with signature `toString(bytes32)` and selector `0xe9dcfb8e`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toString", abi = "toString(bytes32)")]
    pub struct ToString4Call(pub [u8; 32]);
    ///Container type for all input parameters for the `toString` function with signature `toString(bool)` and selector `0x359df6f8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "toString", abi = "toString(bool)")]
    pub struct ToString5Call(pub bool);
    ///Container type for all input parameters for the `transact` function with signature `transact(bytes32)` and selector `0x92cd3fce`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transact", abi = "transact(bytes32)")]
    pub struct Transact0Call(pub [u8; 32]);
    ///Container type for all input parameters for the `transact` function with signature `transact(uint256,bytes32)` and selector `0x04f2e146`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "transact", abi = "transact(uint256,bytes32)")]
    pub struct Transact1Call(pub ::corebc_core::types::U256, pub [u8; 32]);
    ///Container type for all input parameters for the `txGasPrice` function with signature `txGasPrice(uint256)` and selector `0xc69545f8`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "txGasPrice", abi = "txGasPrice(uint256)")]
    pub struct TxGasPriceCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `warp` function with signature `warp(uint256)` and selector `0x037a7f89`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "warp", abi = "warp(uint256)")]
    pub struct WarpCall(pub ::corebc_core::types::U256);
    ///Container type for all input parameters for the `writeFile` function with signature `writeFile(string,string)` and selector `0x6b52a3c0`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "writeFile", abi = "writeFile(string,string)")]
    pub struct WriteFileCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `writeFileBinary` function with signature `writeFileBinary(string,bytes)` and selector `0x103030bc`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "writeFileBinary", abi = "writeFileBinary(string,bytes)")]
    pub struct WriteFileBinaryCall(
        pub ::std::string::String,
        pub ::corebc_core::types::Bytes,
    );
    ///Container type for all input parameters for the `writeJson` function with signature `writeJson(string,string)` and selector `0xb5e0aa0c`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "writeJson", abi = "writeJson(string,string)")]
    pub struct WriteJson0Call(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all input parameters for the `writeJson` function with signature `writeJson(string,string,string)` and selector `0xa57e5e2d`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "writeJson", abi = "writeJson(string,string,string)")]
    pub struct WriteJson1Call(
        pub ::std::string::String,
        pub ::std::string::String,
        pub ::std::string::String,
    );
    ///Container type for all input parameters for the `writeLine` function with signature `writeLine(string,string)` and selector `0x592b1bb6`
    #[derive(
        Clone,
        ::corebc_contract::EthCall,
        ::corebc_contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "writeLine", abi = "writeLine(string,string)")]
    pub struct WriteLineCall(pub ::std::string::String, pub ::std::string::String);
    ///Container type for all of the contract's call
    #[derive(Clone, ::corebc_contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum HEVMCalls {
        Accesses(AccessesCall),
        ActiveFork(ActiveForkCall),
        Addr(AddrCall),
        AllowCheatcodes(AllowCheatcodesCall),
        Assume(AssumeCall),
        Breakpoint0(Breakpoint0Call),
        Breakpoint1(Breakpoint1Call),
        Broadcast0(Broadcast0Call),
        Broadcast1(Broadcast1Call),
        Broadcast2(Broadcast2Call),
        ChainId(ChainIdCall),
        ClearMockedCalls(ClearMockedCallsCall),
        CloseFile(CloseFileCall),
        Coinbase(CoinbaseCall),
        ComputeCreate2Address1(ComputeCreate2Address1Call),
        ComputeCreate2Address0(ComputeCreate2Address0Call),
        ComputeCreateAddress(ComputeCreateAddressCall),
        CreateDir(CreateDirCall),
        CreateFork1(CreateFork1Call),
        CreateFork2(CreateFork2Call),
        CreateFork0(CreateFork0Call),
        CreateSelectFork1(CreateSelectFork1Call),
        CreateSelectFork2(CreateSelectFork2Call),
        CreateSelectFork0(CreateSelectFork0Call),
        Deal(DealCall),
        DeriveKey0(DeriveKey0Call),
        DeriveKey1(DeriveKey1Call),
        DeriveKey2(DeriveKey2Call),
        DeriveKey3(DeriveKey3Call),
        Difficulty(DifficultyCall),
        EnvAddress0(EnvAddress0Call),
        EnvAddress1(EnvAddress1Call),
        EnvBool0(EnvBool0Call),
        EnvBool1(EnvBool1Call),
        EnvBytes0(EnvBytes0Call),
        EnvBytes1(EnvBytes1Call),
        EnvBytes320(EnvBytes320Call),
        EnvBytes321(EnvBytes321Call),
        EnvInt0(EnvInt0Call),
        EnvInt1(EnvInt1Call),
        EnvOr0(EnvOr0Call),
        EnvOr1(EnvOr1Call),
        EnvOr2(EnvOr2Call),
        EnvOr3(EnvOr3Call),
        EnvOr4(EnvOr4Call),
        EnvOr5(EnvOr5Call),
        EnvOr6(EnvOr6Call),
        EnvOr7(EnvOr7Call),
        EnvOr8(EnvOr8Call),
        EnvOr9(EnvOr9Call),
        EnvOr10(EnvOr10Call),
        EnvOr11(EnvOr11Call),
        EnvOr12(EnvOr12Call),
        EnvOr13(EnvOr13Call),
        EnvString0(EnvString0Call),
        EnvString1(EnvString1Call),
        EnvUint0(EnvUint0Call),
        EnvUint1(EnvUint1Call),
        Etch(EtchCall),
        ExpectCall0(ExpectCall0Call),
        ExpectCall1(ExpectCall1Call),
        ExpectCall2(ExpectCall2Call),
        ExpectCall3(ExpectCall3Call),
        ExpectCall4(ExpectCall4Call),
        ExpectCall5(ExpectCall5Call),
        ExpectCallMinGas0(ExpectCallMinGas0Call),
        ExpectCallMinGas1(ExpectCallMinGas1Call),
        ExpectEmit0(ExpectEmit0Call),
        ExpectEmit1(ExpectEmit1Call),
        ExpectEmit2(ExpectEmit2Call),
        ExpectEmit3(ExpectEmit3Call),
        ExpectRevert0(ExpectRevert0Call),
        ExpectRevert1(ExpectRevert1Call),
        ExpectRevert2(ExpectRevert2Call),
        ExpectSafeMemory(ExpectSafeMemoryCall),
        ExpectSafeMemoryCall(ExpectSafeMemoryCallCall),
        Fee(FeeCall),
        Ffi(FfiCall),
        FsMetadata(FsMetadataCall),
        GetCode(GetCodeCall),
        GetDeployedCode(GetDeployedCodeCall),
        GetLabel(GetLabelCall),
        GetMappingKeyAndParentOf(GetMappingKeyAndParentOfCall),
        GetMappingLength(GetMappingLengthCall),
        GetMappingSlotAt(GetMappingSlotAtCall),
        GetNonce(GetNonceCall),
        GetRecordedLogs(GetRecordedLogsCall),
        IsPersistent(IsPersistentCall),
        Label(LabelCall),
        Load(LoadCall),
        MakePersistent0(MakePersistent0Call),
        MakePersistent2(MakePersistent2Call),
        MakePersistent3(MakePersistent3Call),
        MakePersistent1(MakePersistent1Call),
        MockCall0(MockCall0Call),
        MockCall1(MockCall1Call),
        MockCallRevert0(MockCallRevert0Call),
        MockCallRevert1(MockCallRevert1Call),
        OpenFile(OpenFileCall),
        ParseAddress(ParseAddressCall),
        ParseBool(ParseBoolCall),
        ParseBytes(ParseBytesCall),
        ParseBytes32(ParseBytes32Call),
        ParseInt(ParseIntCall),
        ParseJson0(ParseJson0Call),
        ParseJson1(ParseJson1Call),
        ParseJsonAddress(ParseJsonAddressCall),
        ParseJsonAddressArray(ParseJsonAddressArrayCall),
        ParseJsonBool(ParseJsonBoolCall),
        ParseJsonBoolArray(ParseJsonBoolArrayCall),
        ParseJsonBytes(ParseJsonBytesCall),
        ParseJsonBytes32(ParseJsonBytes32Call),
        ParseJsonBytes32Array(ParseJsonBytes32ArrayCall),
        ParseJsonBytesArray(ParseJsonBytesArrayCall),
        ParseJsonInt(ParseJsonIntCall),
        ParseJsonIntArray(ParseJsonIntArrayCall),
        ParseJsonString(ParseJsonStringCall),
        ParseJsonStringArray(ParseJsonStringArrayCall),
        ParseJsonUint(ParseJsonUintCall),
        ParseJsonUintArray(ParseJsonUintArrayCall),
        ParseUint(ParseUintCall),
        PauseGasMetering(PauseGasMeteringCall),
        Prank0(Prank0Call),
        Prank1(Prank1Call),
        ProjectRoot(ProjectRootCall),
        ReadCallers(ReadCallersCall),
        ReadDir0(ReadDir0Call),
        ReadDir1(ReadDir1Call),
        ReadDir2(ReadDir2Call),
        ReadFile(ReadFileCall),
        ReadFileBinary(ReadFileBinaryCall),
        ReadLine(ReadLineCall),
        ReadLink(ReadLinkCall),
        Record(RecordCall),
        RecordLogs(RecordLogsCall),
        RememberKey(RememberKeyCall),
        RemoveDir(RemoveDirCall),
        RemoveFile(RemoveFileCall),
        ResetNonce(ResetNonceCall),
        ResumeGasMetering(ResumeGasMeteringCall),
        RevertTo(RevertToCall),
        RevokePersistent0(RevokePersistent0Call),
        RevokePersistent1(RevokePersistent1Call),
        Roll(RollCall),
        RollFork0(RollFork0Call),
        RollFork1(RollFork1Call),
        RollFork2(RollFork2Call),
        RollFork3(RollFork3Call),
        RpcUrl(RpcUrlCall),
        RpcUrlStructs(RpcUrlStructsCall),
        RpcUrls(RpcUrlsCall),
        SelectFork(SelectForkCall),
        SerializeAddress0(SerializeAddress0Call),
        SerializeAddress1(SerializeAddress1Call),
        SerializeBool0(SerializeBool0Call),
        SerializeBool1(SerializeBool1Call),
        SerializeBytes0(SerializeBytes0Call),
        SerializeBytes1(SerializeBytes1Call),
        SerializeBytes320(SerializeBytes320Call),
        SerializeBytes321(SerializeBytes321Call),
        SerializeInt0(SerializeInt0Call),
        SerializeInt1(SerializeInt1Call),
        SerializeString0(SerializeString0Call),
        SerializeString1(SerializeString1Call),
        SerializeUint0(SerializeUint0Call),
        SerializeUint1(SerializeUint1Call),
        SetEnv(SetEnvCall),
        SetNonce(SetNonceCall),
        SetNonceUnsafe(SetNonceUnsafeCall),
        Sign(SignCall),
        Skip(SkipCall),
        Snapshot(SnapshotCall),
        StartBroadcast0(StartBroadcast0Call),
        StartBroadcast1(StartBroadcast1Call),
        StartBroadcast2(StartBroadcast2Call),
        StartMappingRecording(StartMappingRecordingCall),
        StartPrank0(StartPrank0Call),
        StartPrank1(StartPrank1Call),
        StopBroadcast(StopBroadcastCall),
        StopMappingRecording(StopMappingRecordingCall),
        StopPrank(StopPrankCall),
        Store(StoreCall),
        ToString0(ToString0Call),
        ToString1(ToString1Call),
        ToString2(ToString2Call),
        ToString3(ToString3Call),
        ToString4(ToString4Call),
        ToString5(ToString5Call),
        Transact0(Transact0Call),
        Transact1(Transact1Call),
        TxGasPrice(TxGasPriceCall),
        Warp(WarpCall),
        WriteFile(WriteFileCall),
        WriteFileBinary(WriteFileBinaryCall),
        WriteJson0(WriteJson0Call),
        WriteJson1(WriteJson1Call),
        WriteLine(WriteLineCall),
    }
    impl ::corebc_core::abi::AbiDecode for HEVMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::corebc_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded) = <AccessesCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Accesses(decoded));
            }
            if let Ok(decoded) = <ActiveForkCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ActiveFork(decoded));
            }
            if let Ok(decoded) = <AddrCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Addr(decoded));
            }
            if let Ok(decoded) = <AllowCheatcodesCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::AllowCheatcodes(decoded));
            }
            if let Ok(decoded) = <AssumeCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Assume(decoded));
            }
            if let Ok(decoded) = <Breakpoint0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Breakpoint0(decoded));
            }
            if let Ok(decoded) = <Breakpoint1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Breakpoint1(decoded));
            }
            if let Ok(decoded) = <Broadcast0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast0(decoded));
            }
            if let Ok(decoded) = <Broadcast1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast1(decoded));
            }
            if let Ok(decoded) = <Broadcast2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Broadcast2(decoded));
            }
            if let Ok(decoded) = <ChainIdCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ChainId(decoded));
            }
            if let Ok(decoded) = <ClearMockedCallsCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ClearMockedCalls(decoded));
            }
            if let Ok(decoded) = <CloseFileCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CloseFile(decoded));
            }
            if let Ok(decoded) = <CoinbaseCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Coinbase(decoded));
            }
            if let Ok(decoded) = <ComputeCreate2Address1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate2Address1(decoded));
            }
            if let Ok(decoded) = <ComputeCreate2Address0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreate2Address0(decoded));
            }
            if let Ok(decoded) = <ComputeCreateAddressCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ComputeCreateAddress(decoded));
            }
            if let Ok(decoded) = <CreateDirCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateDir(decoded));
            }
            if let Ok(decoded) = <CreateFork1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork1(decoded));
            }
            if let Ok(decoded) = <CreateFork2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork2(decoded));
            }
            if let Ok(decoded) = <CreateFork0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateFork0(decoded));
            }
            if let Ok(decoded) = <CreateSelectFork1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork1(decoded));
            }
            if let Ok(decoded) = <CreateSelectFork2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork2(decoded));
            }
            if let Ok(decoded) = <CreateSelectFork0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::CreateSelectFork0(decoded));
            }
            if let Ok(decoded) = <DealCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Deal(decoded));
            }
            if let Ok(decoded) = <DeriveKey0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey0(decoded));
            }
            if let Ok(decoded) = <DeriveKey1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey1(decoded));
            }
            if let Ok(decoded) = <DeriveKey2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey2(decoded));
            }
            if let Ok(decoded) = <DeriveKey3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::DeriveKey3(decoded));
            }
            if let Ok(decoded) = <DifficultyCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Difficulty(decoded));
            }
            if let Ok(decoded) = <EnvAddress0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvAddress0(decoded));
            }
            if let Ok(decoded) = <EnvAddress1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvAddress1(decoded));
            }
            if let Ok(decoded) = <EnvBool0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBool0(decoded));
            }
            if let Ok(decoded) = <EnvBool1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBool1(decoded));
            }
            if let Ok(decoded) = <EnvBytes0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes0(decoded));
            }
            if let Ok(decoded) = <EnvBytes1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes1(decoded));
            }
            if let Ok(decoded) = <EnvBytes320Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes320(decoded));
            }
            if let Ok(decoded) = <EnvBytes321Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvBytes321(decoded));
            }
            if let Ok(decoded) = <EnvInt0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvInt0(decoded));
            }
            if let Ok(decoded) = <EnvInt1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvInt1(decoded));
            }
            if let Ok(decoded) = <EnvOr0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr0(decoded));
            }
            if let Ok(decoded) = <EnvOr1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr1(decoded));
            }
            if let Ok(decoded) = <EnvOr2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr2(decoded));
            }
            if let Ok(decoded) = <EnvOr3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr3(decoded));
            }
            if let Ok(decoded) = <EnvOr4Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr4(decoded));
            }
            if let Ok(decoded) = <EnvOr5Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr5(decoded));
            }
            if let Ok(decoded) = <EnvOr6Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr6(decoded));
            }
            if let Ok(decoded) = <EnvOr7Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr7(decoded));
            }
            if let Ok(decoded) = <EnvOr8Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr8(decoded));
            }
            if let Ok(decoded) = <EnvOr9Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr9(decoded));
            }
            if let Ok(decoded) = <EnvOr10Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr10(decoded));
            }
            if let Ok(decoded) = <EnvOr11Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr11(decoded));
            }
            if let Ok(decoded) = <EnvOr12Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr12(decoded));
            }
            if let Ok(decoded) = <EnvOr13Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvOr13(decoded));
            }
            if let Ok(decoded) = <EnvString0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvString0(decoded));
            }
            if let Ok(decoded) = <EnvString1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvString1(decoded));
            }
            if let Ok(decoded) = <EnvUint0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvUint0(decoded));
            }
            if let Ok(decoded) = <EnvUint1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::EnvUint1(decoded));
            }
            if let Ok(decoded) = <EtchCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Etch(decoded));
            }
            if let Ok(decoded) = <ExpectCall0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall0(decoded));
            }
            if let Ok(decoded) = <ExpectCall1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall1(decoded));
            }
            if let Ok(decoded) = <ExpectCall2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall2(decoded));
            }
            if let Ok(decoded) = <ExpectCall3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall3(decoded));
            }
            if let Ok(decoded) = <ExpectCall4Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall4(decoded));
            }
            if let Ok(decoded) = <ExpectCall5Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCall5(decoded));
            }
            if let Ok(decoded) = <ExpectCallMinGas0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCallMinGas0(decoded));
            }
            if let Ok(decoded) = <ExpectCallMinGas1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectCallMinGas1(decoded));
            }
            if let Ok(decoded) = <ExpectEmit0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit0(decoded));
            }
            if let Ok(decoded) = <ExpectEmit1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit1(decoded));
            }
            if let Ok(decoded) = <ExpectEmit2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit2(decoded));
            }
            if let Ok(decoded) = <ExpectEmit3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectEmit3(decoded));
            }
            if let Ok(decoded) = <ExpectRevert0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectRevert0(decoded));
            }
            if let Ok(decoded) = <ExpectRevert1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectRevert1(decoded));
            }
            if let Ok(decoded) = <ExpectRevert2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectRevert2(decoded));
            }
            if let Ok(decoded) = <ExpectSafeMemoryCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectSafeMemory(decoded));
            }
            if let Ok(decoded) = <ExpectSafeMemoryCallCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ExpectSafeMemoryCall(decoded));
            }
            if let Ok(decoded) = <FeeCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded) = <FfiCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Ffi(decoded));
            }
            if let Ok(decoded) = <FsMetadataCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::FsMetadata(decoded));
            }
            if let Ok(decoded) = <GetCodeCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetCode(decoded));
            }
            if let Ok(decoded) = <GetDeployedCodeCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetDeployedCode(decoded));
            }
            if let Ok(decoded) = <GetLabelCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetLabel(decoded));
            }
            if let Ok(decoded) = <GetMappingKeyAndParentOfCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMappingKeyAndParentOf(decoded));
            }
            if let Ok(decoded) = <GetMappingLengthCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMappingLength(decoded));
            }
            if let Ok(decoded) = <GetMappingSlotAtCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetMappingSlotAt(decoded));
            }
            if let Ok(decoded) = <GetNonceCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetNonce(decoded));
            }
            if let Ok(decoded) = <GetRecordedLogsCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::GetRecordedLogs(decoded));
            }
            if let Ok(decoded) = <IsPersistentCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::IsPersistent(decoded));
            }
            if let Ok(decoded) = <LabelCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Label(decoded));
            }
            if let Ok(decoded) = <LoadCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Load(decoded));
            }
            if let Ok(decoded) = <MakePersistent0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent0(decoded));
            }
            if let Ok(decoded) = <MakePersistent2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent2(decoded));
            }
            if let Ok(decoded) = <MakePersistent3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent3(decoded));
            }
            if let Ok(decoded) = <MakePersistent1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MakePersistent1(decoded));
            }
            if let Ok(decoded) = <MockCall0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCall0(decoded));
            }
            if let Ok(decoded) = <MockCall1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCall1(decoded));
            }
            if let Ok(decoded) = <MockCallRevert0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCallRevert0(decoded));
            }
            if let Ok(decoded) = <MockCallRevert1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::MockCallRevert1(decoded));
            }
            if let Ok(decoded) = <OpenFileCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::OpenFile(decoded));
            }
            if let Ok(decoded) = <ParseAddressCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseAddress(decoded));
            }
            if let Ok(decoded) = <ParseBoolCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseBool(decoded));
            }
            if let Ok(decoded) = <ParseBytesCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseBytes(decoded));
            }
            if let Ok(decoded) = <ParseBytes32Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseBytes32(decoded));
            }
            if let Ok(decoded) = <ParseIntCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseInt(decoded));
            }
            if let Ok(decoded) = <ParseJson0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJson0(decoded));
            }
            if let Ok(decoded) = <ParseJson1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJson1(decoded));
            }
            if let Ok(decoded) = <ParseJsonAddressCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonAddress(decoded));
            }
            if let Ok(decoded) = <ParseJsonAddressArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonAddressArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonBoolCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBool(decoded));
            }
            if let Ok(decoded) = <ParseJsonBoolArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBoolArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytesCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytes(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytes32Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytes32(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytes32ArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytes32Array(decoded));
            }
            if let Ok(decoded) = <ParseJsonBytesArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonBytesArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonIntCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonInt(decoded));
            }
            if let Ok(decoded) = <ParseJsonIntArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonIntArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonStringCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonString(decoded));
            }
            if let Ok(decoded) = <ParseJsonStringArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonStringArray(decoded));
            }
            if let Ok(decoded) = <ParseJsonUintCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonUint(decoded));
            }
            if let Ok(decoded) = <ParseJsonUintArrayCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseJsonUintArray(decoded));
            }
            if let Ok(decoded) = <ParseUintCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ParseUint(decoded));
            }
            if let Ok(decoded) = <PauseGasMeteringCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::PauseGasMetering(decoded));
            }
            if let Ok(decoded) = <Prank0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prank0(decoded));
            }
            if let Ok(decoded) = <Prank1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Prank1(decoded));
            }
            if let Ok(decoded) = <ProjectRootCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ProjectRoot(decoded));
            }
            if let Ok(decoded) = <ReadCallersCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadCallers(decoded));
            }
            if let Ok(decoded) = <ReadDir0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadDir0(decoded));
            }
            if let Ok(decoded) = <ReadDir1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadDir1(decoded));
            }
            if let Ok(decoded) = <ReadDir2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadDir2(decoded));
            }
            if let Ok(decoded) = <ReadFileCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadFile(decoded));
            }
            if let Ok(decoded) = <ReadFileBinaryCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadFileBinary(decoded));
            }
            if let Ok(decoded) = <ReadLineCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadLine(decoded));
            }
            if let Ok(decoded) = <ReadLinkCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ReadLink(decoded));
            }
            if let Ok(decoded) = <RecordCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Record(decoded));
            }
            if let Ok(decoded) = <RecordLogsCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RecordLogs(decoded));
            }
            if let Ok(decoded) = <RememberKeyCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RememberKey(decoded));
            }
            if let Ok(decoded) = <RemoveDirCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveDir(decoded));
            }
            if let Ok(decoded) = <RemoveFileCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RemoveFile(decoded));
            }
            if let Ok(decoded) = <ResetNonceCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResetNonce(decoded));
            }
            if let Ok(decoded) = <ResumeGasMeteringCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ResumeGasMetering(decoded));
            }
            if let Ok(decoded) = <RevertToCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevertTo(decoded));
            }
            if let Ok(decoded) = <RevokePersistent0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePersistent0(decoded));
            }
            if let Ok(decoded) = <RevokePersistent1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RevokePersistent1(decoded));
            }
            if let Ok(decoded) = <RollCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Roll(decoded));
            }
            if let Ok(decoded) = <RollFork0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork0(decoded));
            }
            if let Ok(decoded) = <RollFork1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork1(decoded));
            }
            if let Ok(decoded) = <RollFork2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork2(decoded));
            }
            if let Ok(decoded) = <RollFork3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RollFork3(decoded));
            }
            if let Ok(decoded) = <RpcUrlCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RpcUrl(decoded));
            }
            if let Ok(decoded) = <RpcUrlStructsCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RpcUrlStructs(decoded));
            }
            if let Ok(decoded) = <RpcUrlsCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::RpcUrls(decoded));
            }
            if let Ok(decoded) = <SelectForkCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SelectFork(decoded));
            }
            if let Ok(decoded) = <SerializeAddress0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeAddress0(decoded));
            }
            if let Ok(decoded) = <SerializeAddress1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeAddress1(decoded));
            }
            if let Ok(decoded) = <SerializeBool0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBool0(decoded));
            }
            if let Ok(decoded) = <SerializeBool1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBool1(decoded));
            }
            if let Ok(decoded) = <SerializeBytes0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes0(decoded));
            }
            if let Ok(decoded) = <SerializeBytes1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes1(decoded));
            }
            if let Ok(decoded) = <SerializeBytes320Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes320(decoded));
            }
            if let Ok(decoded) = <SerializeBytes321Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeBytes321(decoded));
            }
            if let Ok(decoded) = <SerializeInt0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeInt0(decoded));
            }
            if let Ok(decoded) = <SerializeInt1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeInt1(decoded));
            }
            if let Ok(decoded) = <SerializeString0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeString0(decoded));
            }
            if let Ok(decoded) = <SerializeString1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeString1(decoded));
            }
            if let Ok(decoded) = <SerializeUint0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeUint0(decoded));
            }
            if let Ok(decoded) = <SerializeUint1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SerializeUint1(decoded));
            }
            if let Ok(decoded) = <SetEnvCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetEnv(decoded));
            }
            if let Ok(decoded) = <SetNonceCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNonce(decoded));
            }
            if let Ok(decoded) = <SetNonceUnsafeCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::SetNonceUnsafe(decoded));
            }
            if let Ok(decoded) = <SignCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Sign(decoded));
            }
            if let Ok(decoded) = <SkipCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Skip(decoded));
            }
            if let Ok(decoded) = <SnapshotCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Snapshot(decoded));
            }
            if let Ok(decoded) = <StartBroadcast0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast0(decoded));
            }
            if let Ok(decoded) = <StartBroadcast1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast1(decoded));
            }
            if let Ok(decoded) = <StartBroadcast2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartBroadcast2(decoded));
            }
            if let Ok(decoded) = <StartMappingRecordingCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartMappingRecording(decoded));
            }
            if let Ok(decoded) = <StartPrank0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartPrank0(decoded));
            }
            if let Ok(decoded) = <StartPrank1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StartPrank1(decoded));
            }
            if let Ok(decoded) = <StopBroadcastCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopBroadcast(decoded));
            }
            if let Ok(decoded) = <StopMappingRecordingCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopMappingRecording(decoded));
            }
            if let Ok(decoded) = <StopPrankCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::StopPrank(decoded));
            }
            if let Ok(decoded) = <StoreCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Store(decoded));
            }
            if let Ok(decoded) = <ToString0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString0(decoded));
            }
            if let Ok(decoded) = <ToString1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString1(decoded));
            }
            if let Ok(decoded) = <ToString2Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString2(decoded));
            }
            if let Ok(decoded) = <ToString3Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString3(decoded));
            }
            if let Ok(decoded) = <ToString4Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString4(decoded));
            }
            if let Ok(decoded) = <ToString5Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::ToString5(decoded));
            }
            if let Ok(decoded) = <Transact0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transact0(decoded));
            }
            if let Ok(decoded) = <Transact1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Transact1(decoded));
            }
            if let Ok(decoded) = <TxGasPriceCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::TxGasPrice(decoded));
            }
            if let Ok(decoded) = <WarpCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::Warp(decoded));
            }
            if let Ok(decoded) = <WriteFileCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteFile(decoded));
            }
            if let Ok(decoded) = <WriteFileBinaryCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteFileBinary(decoded));
            }
            if let Ok(decoded) = <WriteJson0Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteJson0(decoded));
            }
            if let Ok(decoded) = <WriteJson1Call as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteJson1(decoded));
            }
            if let Ok(decoded) = <WriteLineCall as ::corebc_core::abi::AbiDecode>::decode(
                data,
            ) {
                return Ok(Self::WriteLine(decoded));
            }
            Err(::corebc_core::abi::Error::InvalidData.into())
        }
    }
    impl ::corebc_core::abi::AbiEncode for HEVMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Accesses(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ActiveFork(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Addr(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::AllowCheatcodes(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Assume(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Breakpoint0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Breakpoint1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Broadcast0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Broadcast1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Broadcast2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ChainId(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ClearMockedCalls(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CloseFile(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Coinbase(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ComputeCreate2Address1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreate2Address0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ComputeCreateAddress(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateDir(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateFork1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateFork2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateFork0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::CreateSelectFork0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Deal(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::DeriveKey0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::DeriveKey3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Difficulty(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvAddress0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvAddress1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvBool0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvBool1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvBytes0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes320(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvBytes321(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvInt0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvInt1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr2(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr3(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr4(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr5(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr6(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr7(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr8(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr9(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr10(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr11(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr12(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvOr13(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvString0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvString1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::EnvUint0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::EnvUint1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Etch(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ExpectCall0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall4(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCall5(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCallMinGas0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectCallMinGas1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectEmit3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectRevert0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectRevert1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectRevert2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectSafeMemory(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ExpectSafeMemoryCall(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Fee(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Ffi(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::FsMetadata(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::GetCode(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::GetDeployedCode(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::GetLabel(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::GetMappingKeyAndParentOf(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::GetMappingLength(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::GetMappingSlotAt(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::GetNonce(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::GetRecordedLogs(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::IsPersistent(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Label(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Load(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::MakePersistent0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MakePersistent1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MockCall0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MockCall1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MockCallRevert0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::MockCallRevert1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::OpenFile(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ParseAddress(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseBool(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseBytes(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseBytes32(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseInt(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ParseJson0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJson1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonAddress(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonAddressArray(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBool(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBoolArray(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytes(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytes32(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytes32Array(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonBytesArray(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonInt(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonIntArray(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonString(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonStringArray(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonUint(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseJsonUintArray(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ParseUint(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::PauseGasMetering(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Prank0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Prank1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ProjectRoot(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ReadCallers(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ReadDir0(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ReadDir1(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ReadDir2(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ReadFile(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ReadFileBinary(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ReadLine(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ReadLink(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Record(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::RecordLogs(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RememberKey(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RemoveDir(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RemoveFile(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ResetNonce(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ResumeGasMetering(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RevertTo(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::RevokePersistent0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RevokePersistent1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Roll(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::RollFork0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RollFork1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RollFork2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RollFork3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RpcUrl(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::RpcUrlStructs(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::RpcUrls(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::SelectFork(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeAddress0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeAddress1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBool0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBool1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes320(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeBytes321(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeInt0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeInt1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeString0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeString1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeUint0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SerializeUint1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::SetEnv(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::SetNonce(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::SetNonceUnsafe(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Sign(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Skip(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::Snapshot(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::StartBroadcast0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StartBroadcast2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StartMappingRecording(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StartPrank0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StartPrank1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StopBroadcast(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StopMappingRecording(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::StopPrank(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Store(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::ToString0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ToString1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ToString2(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ToString3(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ToString4(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::ToString5(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Transact0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Transact1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::TxGasPrice(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::Warp(element) => ::corebc_core::abi::AbiEncode::encode(element),
                Self::WriteFile(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::WriteFileBinary(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::WriteJson0(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::WriteJson1(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
                Self::WriteLine(element) => {
                    ::corebc_core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for HEVMCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Accesses(element) => ::core::fmt::Display::fmt(element, f),
                Self::ActiveFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::Addr(element) => ::core::fmt::Display::fmt(element, f),
                Self::AllowCheatcodes(element) => ::core::fmt::Display::fmt(element, f),
                Self::Assume(element) => ::core::fmt::Display::fmt(element, f),
                Self::Breakpoint0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Breakpoint1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Broadcast2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChainId(element) => ::core::fmt::Display::fmt(element, f),
                Self::ClearMockedCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::CloseFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::Coinbase(element) => ::core::fmt::Display::fmt(element, f),
                Self::ComputeCreate2Address1(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreate2Address0(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ComputeCreateAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CreateDir(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateSelectFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateSelectFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::CreateSelectFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Deal(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey0(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey1(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey2(element) => ::core::fmt::Display::fmt(element, f),
                Self::DeriveKey3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Difficulty(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvAddress0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvAddress1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBool0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBool1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes320(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvBytes321(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvInt0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvInt1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr2(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr3(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr4(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr5(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr6(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr7(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr8(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr9(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr10(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr11(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr12(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvOr13(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvUint0(element) => ::core::fmt::Display::fmt(element, f),
                Self::EnvUint1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Etch(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall4(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCall5(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCallMinGas0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectCallMinGas1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectEmit3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectRevert2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectSafeMemory(element) => ::core::fmt::Display::fmt(element, f),
                Self::ExpectSafeMemoryCall(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ffi(element) => ::core::fmt::Display::fmt(element, f),
                Self::FsMetadata(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetDeployedCode(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetLabel(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMappingKeyAndParentOf(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::GetMappingLength(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetMappingSlotAt(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetRecordedLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPersistent(element) => ::core::fmt::Display::fmt(element, f),
                Self::Label(element) => ::core::fmt::Display::fmt(element, f),
                Self::Load(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent0(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent2(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent3(element) => ::core::fmt::Display::fmt(element, f),
                Self::MakePersistent1(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCall0(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCall1(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCallRevert0(element) => ::core::fmt::Display::fmt(element, f),
                Self::MockCallRevert1(element) => ::core::fmt::Display::fmt(element, f),
                Self::OpenFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJson1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonAddressArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonBoolArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonBytes32Array(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonBytesArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonIntArray(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonString(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonStringArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseJsonUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::ParseJsonUintArray(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ParseUint(element) => ::core::fmt::Display::fmt(element, f),
                Self::PauseGasMetering(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prank0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Prank1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProjectRoot(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadCallers(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadDir0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadDir1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadDir2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadLine(element) => ::core::fmt::Display::fmt(element, f),
                Self::ReadLink(element) => ::core::fmt::Display::fmt(element, f),
                Self::Record(element) => ::core::fmt::Display::fmt(element, f),
                Self::RecordLogs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RememberKey(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveDir(element) => ::core::fmt::Display::fmt(element, f),
                Self::RemoveFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::ResumeGasMetering(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertTo(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePersistent0(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevokePersistent1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Roll(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork0(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork1(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork2(element) => ::core::fmt::Display::fmt(element, f),
                Self::RollFork3(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrl(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrlStructs(element) => ::core::fmt::Display::fmt(element, f),
                Self::RpcUrls(element) => ::core::fmt::Display::fmt(element, f),
                Self::SelectFork(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeAddress0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeAddress1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBool0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBool1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes320(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeBytes321(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeInt0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeInt1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeUint0(element) => ::core::fmt::Display::fmt(element, f),
                Self::SerializeUint1(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetEnv(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNonce(element) => ::core::fmt::Display::fmt(element, f),
                Self::SetNonceUnsafe(element) => ::core::fmt::Display::fmt(element, f),
                Self::Sign(element) => ::core::fmt::Display::fmt(element, f),
                Self::Skip(element) => ::core::fmt::Display::fmt(element, f),
                Self::Snapshot(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast0(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast1(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartBroadcast2(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartMappingRecording(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StartPrank0(element) => ::core::fmt::Display::fmt(element, f),
                Self::StartPrank1(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopBroadcast(element) => ::core::fmt::Display::fmt(element, f),
                Self::StopMappingRecording(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::StopPrank(element) => ::core::fmt::Display::fmt(element, f),
                Self::Store(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString0(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString1(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString2(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString3(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString4(element) => ::core::fmt::Display::fmt(element, f),
                Self::ToString5(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transact0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Transact1(element) => ::core::fmt::Display::fmt(element, f),
                Self::TxGasPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Warp(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFile(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteFileBinary(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJson0(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteJson1(element) => ::core::fmt::Display::fmt(element, f),
                Self::WriteLine(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AccessesCall> for HEVMCalls {
        fn from(value: AccessesCall) -> Self {
            Self::Accesses(value)
        }
    }
    impl ::core::convert::From<ActiveForkCall> for HEVMCalls {
        fn from(value: ActiveForkCall) -> Self {
            Self::ActiveFork(value)
        }
    }
    impl ::core::convert::From<AddrCall> for HEVMCalls {
        fn from(value: AddrCall) -> Self {
            Self::Addr(value)
        }
    }
    impl ::core::convert::From<AllowCheatcodesCall> for HEVMCalls {
        fn from(value: AllowCheatcodesCall) -> Self {
            Self::AllowCheatcodes(value)
        }
    }
    impl ::core::convert::From<AssumeCall> for HEVMCalls {
        fn from(value: AssumeCall) -> Self {
            Self::Assume(value)
        }
    }
    impl ::core::convert::From<Breakpoint0Call> for HEVMCalls {
        fn from(value: Breakpoint0Call) -> Self {
            Self::Breakpoint0(value)
        }
    }
    impl ::core::convert::From<Breakpoint1Call> for HEVMCalls {
        fn from(value: Breakpoint1Call) -> Self {
            Self::Breakpoint1(value)
        }
    }
    impl ::core::convert::From<Broadcast0Call> for HEVMCalls {
        fn from(value: Broadcast0Call) -> Self {
            Self::Broadcast0(value)
        }
    }
    impl ::core::convert::From<Broadcast1Call> for HEVMCalls {
        fn from(value: Broadcast1Call) -> Self {
            Self::Broadcast1(value)
        }
    }
    impl ::core::convert::From<Broadcast2Call> for HEVMCalls {
        fn from(value: Broadcast2Call) -> Self {
            Self::Broadcast2(value)
        }
    }
    impl ::core::convert::From<ChainIdCall> for HEVMCalls {
        fn from(value: ChainIdCall) -> Self {
            Self::ChainId(value)
        }
    }
    impl ::core::convert::From<ClearMockedCallsCall> for HEVMCalls {
        fn from(value: ClearMockedCallsCall) -> Self {
            Self::ClearMockedCalls(value)
        }
    }
    impl ::core::convert::From<CloseFileCall> for HEVMCalls {
        fn from(value: CloseFileCall) -> Self {
            Self::CloseFile(value)
        }
    }
    impl ::core::convert::From<CoinbaseCall> for HEVMCalls {
        fn from(value: CoinbaseCall) -> Self {
            Self::Coinbase(value)
        }
    }
    impl ::core::convert::From<ComputeCreate2Address1Call> for HEVMCalls {
        fn from(value: ComputeCreate2Address1Call) -> Self {
            Self::ComputeCreate2Address1(value)
        }
    }
    impl ::core::convert::From<ComputeCreate2Address0Call> for HEVMCalls {
        fn from(value: ComputeCreate2Address0Call) -> Self {
            Self::ComputeCreate2Address0(value)
        }
    }
    impl ::core::convert::From<ComputeCreateAddressCall> for HEVMCalls {
        fn from(value: ComputeCreateAddressCall) -> Self {
            Self::ComputeCreateAddress(value)
        }
    }
    impl ::core::convert::From<CreateDirCall> for HEVMCalls {
        fn from(value: CreateDirCall) -> Self {
            Self::CreateDir(value)
        }
    }
    impl ::core::convert::From<CreateFork1Call> for HEVMCalls {
        fn from(value: CreateFork1Call) -> Self {
            Self::CreateFork1(value)
        }
    }
    impl ::core::convert::From<CreateFork2Call> for HEVMCalls {
        fn from(value: CreateFork2Call) -> Self {
            Self::CreateFork2(value)
        }
    }
    impl ::core::convert::From<CreateFork0Call> for HEVMCalls {
        fn from(value: CreateFork0Call) -> Self {
            Self::CreateFork0(value)
        }
    }
    impl ::core::convert::From<CreateSelectFork1Call> for HEVMCalls {
        fn from(value: CreateSelectFork1Call) -> Self {
            Self::CreateSelectFork1(value)
        }
    }
    impl ::core::convert::From<CreateSelectFork2Call> for HEVMCalls {
        fn from(value: CreateSelectFork2Call) -> Self {
            Self::CreateSelectFork2(value)
        }
    }
    impl ::core::convert::From<CreateSelectFork0Call> for HEVMCalls {
        fn from(value: CreateSelectFork0Call) -> Self {
            Self::CreateSelectFork0(value)
        }
    }
    impl ::core::convert::From<DealCall> for HEVMCalls {
        fn from(value: DealCall) -> Self {
            Self::Deal(value)
        }
    }
    impl ::core::convert::From<DeriveKey0Call> for HEVMCalls {
        fn from(value: DeriveKey0Call) -> Self {
            Self::DeriveKey0(value)
        }
    }
    impl ::core::convert::From<DeriveKey1Call> for HEVMCalls {
        fn from(value: DeriveKey1Call) -> Self {
            Self::DeriveKey1(value)
        }
    }
    impl ::core::convert::From<DeriveKey2Call> for HEVMCalls {
        fn from(value: DeriveKey2Call) -> Self {
            Self::DeriveKey2(value)
        }
    }
    impl ::core::convert::From<DeriveKey3Call> for HEVMCalls {
        fn from(value: DeriveKey3Call) -> Self {
            Self::DeriveKey3(value)
        }
    }
    impl ::core::convert::From<DifficultyCall> for HEVMCalls {
        fn from(value: DifficultyCall) -> Self {
            Self::Difficulty(value)
        }
    }
    impl ::core::convert::From<EnvAddress0Call> for HEVMCalls {
        fn from(value: EnvAddress0Call) -> Self {
            Self::EnvAddress0(value)
        }
    }
    impl ::core::convert::From<EnvAddress1Call> for HEVMCalls {
        fn from(value: EnvAddress1Call) -> Self {
            Self::EnvAddress1(value)
        }
    }
    impl ::core::convert::From<EnvBool0Call> for HEVMCalls {
        fn from(value: EnvBool0Call) -> Self {
            Self::EnvBool0(value)
        }
    }
    impl ::core::convert::From<EnvBool1Call> for HEVMCalls {
        fn from(value: EnvBool1Call) -> Self {
            Self::EnvBool1(value)
        }
    }
    impl ::core::convert::From<EnvBytes0Call> for HEVMCalls {
        fn from(value: EnvBytes0Call) -> Self {
            Self::EnvBytes0(value)
        }
    }
    impl ::core::convert::From<EnvBytes1Call> for HEVMCalls {
        fn from(value: EnvBytes1Call) -> Self {
            Self::EnvBytes1(value)
        }
    }
    impl ::core::convert::From<EnvBytes320Call> for HEVMCalls {
        fn from(value: EnvBytes320Call) -> Self {
            Self::EnvBytes320(value)
        }
    }
    impl ::core::convert::From<EnvBytes321Call> for HEVMCalls {
        fn from(value: EnvBytes321Call) -> Self {
            Self::EnvBytes321(value)
        }
    }
    impl ::core::convert::From<EnvInt0Call> for HEVMCalls {
        fn from(value: EnvInt0Call) -> Self {
            Self::EnvInt0(value)
        }
    }
    impl ::core::convert::From<EnvInt1Call> for HEVMCalls {
        fn from(value: EnvInt1Call) -> Self {
            Self::EnvInt1(value)
        }
    }
    impl ::core::convert::From<EnvOr0Call> for HEVMCalls {
        fn from(value: EnvOr0Call) -> Self {
            Self::EnvOr0(value)
        }
    }
    impl ::core::convert::From<EnvOr1Call> for HEVMCalls {
        fn from(value: EnvOr1Call) -> Self {
            Self::EnvOr1(value)
        }
    }
    impl ::core::convert::From<EnvOr2Call> for HEVMCalls {
        fn from(value: EnvOr2Call) -> Self {
            Self::EnvOr2(value)
        }
    }
    impl ::core::convert::From<EnvOr3Call> for HEVMCalls {
        fn from(value: EnvOr3Call) -> Self {
            Self::EnvOr3(value)
        }
    }
    impl ::core::convert::From<EnvOr4Call> for HEVMCalls {
        fn from(value: EnvOr4Call) -> Self {
            Self::EnvOr4(value)
        }
    }
    impl ::core::convert::From<EnvOr5Call> for HEVMCalls {
        fn from(value: EnvOr5Call) -> Self {
            Self::EnvOr5(value)
        }
    }
    impl ::core::convert::From<EnvOr6Call> for HEVMCalls {
        fn from(value: EnvOr6Call) -> Self {
            Self::EnvOr6(value)
        }
    }
    impl ::core::convert::From<EnvOr7Call> for HEVMCalls {
        fn from(value: EnvOr7Call) -> Self {
            Self::EnvOr7(value)
        }
    }
    impl ::core::convert::From<EnvOr8Call> for HEVMCalls {
        fn from(value: EnvOr8Call) -> Self {
            Self::EnvOr8(value)
        }
    }
    impl ::core::convert::From<EnvOr9Call> for HEVMCalls {
        fn from(value: EnvOr9Call) -> Self {
            Self::EnvOr9(value)
        }
    }
    impl ::core::convert::From<EnvOr10Call> for HEVMCalls {
        fn from(value: EnvOr10Call) -> Self {
            Self::EnvOr10(value)
        }
    }
    impl ::core::convert::From<EnvOr11Call> for HEVMCalls {
        fn from(value: EnvOr11Call) -> Self {
            Self::EnvOr11(value)
        }
    }
    impl ::core::convert::From<EnvOr12Call> for HEVMCalls {
        fn from(value: EnvOr12Call) -> Self {
            Self::EnvOr12(value)
        }
    }
    impl ::core::convert::From<EnvOr13Call> for HEVMCalls {
        fn from(value: EnvOr13Call) -> Self {
            Self::EnvOr13(value)
        }
    }
    impl ::core::convert::From<EnvString0Call> for HEVMCalls {
        fn from(value: EnvString0Call) -> Self {
            Self::EnvString0(value)
        }
    }
    impl ::core::convert::From<EnvString1Call> for HEVMCalls {
        fn from(value: EnvString1Call) -> Self {
            Self::EnvString1(value)
        }
    }
    impl ::core::convert::From<EnvUint0Call> for HEVMCalls {
        fn from(value: EnvUint0Call) -> Self {
            Self::EnvUint0(value)
        }
    }
    impl ::core::convert::From<EnvUint1Call> for HEVMCalls {
        fn from(value: EnvUint1Call) -> Self {
            Self::EnvUint1(value)
        }
    }
    impl ::core::convert::From<EtchCall> for HEVMCalls {
        fn from(value: EtchCall) -> Self {
            Self::Etch(value)
        }
    }
    impl ::core::convert::From<ExpectCall0Call> for HEVMCalls {
        fn from(value: ExpectCall0Call) -> Self {
            Self::ExpectCall0(value)
        }
    }
    impl ::core::convert::From<ExpectCall1Call> for HEVMCalls {
        fn from(value: ExpectCall1Call) -> Self {
            Self::ExpectCall1(value)
        }
    }
    impl ::core::convert::From<ExpectCall2Call> for HEVMCalls {
        fn from(value: ExpectCall2Call) -> Self {
            Self::ExpectCall2(value)
        }
    }
    impl ::core::convert::From<ExpectCall3Call> for HEVMCalls {
        fn from(value: ExpectCall3Call) -> Self {
            Self::ExpectCall3(value)
        }
    }
    impl ::core::convert::From<ExpectCall4Call> for HEVMCalls {
        fn from(value: ExpectCall4Call) -> Self {
            Self::ExpectCall4(value)
        }
    }
    impl ::core::convert::From<ExpectCall5Call> for HEVMCalls {
        fn from(value: ExpectCall5Call) -> Self {
            Self::ExpectCall5(value)
        }
    }
    impl ::core::convert::From<ExpectCallMinGas0Call> for HEVMCalls {
        fn from(value: ExpectCallMinGas0Call) -> Self {
            Self::ExpectCallMinGas0(value)
        }
    }
    impl ::core::convert::From<ExpectCallMinGas1Call> for HEVMCalls {
        fn from(value: ExpectCallMinGas1Call) -> Self {
            Self::ExpectCallMinGas1(value)
        }
    }
    impl ::core::convert::From<ExpectEmit0Call> for HEVMCalls {
        fn from(value: ExpectEmit0Call) -> Self {
            Self::ExpectEmit0(value)
        }
    }
    impl ::core::convert::From<ExpectEmit1Call> for HEVMCalls {
        fn from(value: ExpectEmit1Call) -> Self {
            Self::ExpectEmit1(value)
        }
    }
    impl ::core::convert::From<ExpectEmit2Call> for HEVMCalls {
        fn from(value: ExpectEmit2Call) -> Self {
            Self::ExpectEmit2(value)
        }
    }
    impl ::core::convert::From<ExpectEmit3Call> for HEVMCalls {
        fn from(value: ExpectEmit3Call) -> Self {
            Self::ExpectEmit3(value)
        }
    }
    impl ::core::convert::From<ExpectRevert0Call> for HEVMCalls {
        fn from(value: ExpectRevert0Call) -> Self {
            Self::ExpectRevert0(value)
        }
    }
    impl ::core::convert::From<ExpectRevert1Call> for HEVMCalls {
        fn from(value: ExpectRevert1Call) -> Self {
            Self::ExpectRevert1(value)
        }
    }
    impl ::core::convert::From<ExpectRevert2Call> for HEVMCalls {
        fn from(value: ExpectRevert2Call) -> Self {
            Self::ExpectRevert2(value)
        }
    }
    impl ::core::convert::From<ExpectSafeMemoryCall> for HEVMCalls {
        fn from(value: ExpectSafeMemoryCall) -> Self {
            Self::ExpectSafeMemory(value)
        }
    }
    impl ::core::convert::From<ExpectSafeMemoryCallCall> for HEVMCalls {
        fn from(value: ExpectSafeMemoryCallCall) -> Self {
            Self::ExpectSafeMemoryCall(value)
        }
    }
    impl ::core::convert::From<FeeCall> for HEVMCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<FfiCall> for HEVMCalls {
        fn from(value: FfiCall) -> Self {
            Self::Ffi(value)
        }
    }
    impl ::core::convert::From<FsMetadataCall> for HEVMCalls {
        fn from(value: FsMetadataCall) -> Self {
            Self::FsMetadata(value)
        }
    }
    impl ::core::convert::From<GetCodeCall> for HEVMCalls {
        fn from(value: GetCodeCall) -> Self {
            Self::GetCode(value)
        }
    }
    impl ::core::convert::From<GetDeployedCodeCall> for HEVMCalls {
        fn from(value: GetDeployedCodeCall) -> Self {
            Self::GetDeployedCode(value)
        }
    }
    impl ::core::convert::From<GetLabelCall> for HEVMCalls {
        fn from(value: GetLabelCall) -> Self {
            Self::GetLabel(value)
        }
    }
    impl ::core::convert::From<GetMappingKeyAndParentOfCall> for HEVMCalls {
        fn from(value: GetMappingKeyAndParentOfCall) -> Self {
            Self::GetMappingKeyAndParentOf(value)
        }
    }
    impl ::core::convert::From<GetMappingLengthCall> for HEVMCalls {
        fn from(value: GetMappingLengthCall) -> Self {
            Self::GetMappingLength(value)
        }
    }
    impl ::core::convert::From<GetMappingSlotAtCall> for HEVMCalls {
        fn from(value: GetMappingSlotAtCall) -> Self {
            Self::GetMappingSlotAt(value)
        }
    }
    impl ::core::convert::From<GetNonceCall> for HEVMCalls {
        fn from(value: GetNonceCall) -> Self {
            Self::GetNonce(value)
        }
    }
    impl ::core::convert::From<GetRecordedLogsCall> for HEVMCalls {
        fn from(value: GetRecordedLogsCall) -> Self {
            Self::GetRecordedLogs(value)
        }
    }
    impl ::core::convert::From<IsPersistentCall> for HEVMCalls {
        fn from(value: IsPersistentCall) -> Self {
            Self::IsPersistent(value)
        }
    }
    impl ::core::convert::From<LabelCall> for HEVMCalls {
        fn from(value: LabelCall) -> Self {
            Self::Label(value)
        }
    }
    impl ::core::convert::From<LoadCall> for HEVMCalls {
        fn from(value: LoadCall) -> Self {
            Self::Load(value)
        }
    }
    impl ::core::convert::From<MakePersistent0Call> for HEVMCalls {
        fn from(value: MakePersistent0Call) -> Self {
            Self::MakePersistent0(value)
        }
    }
    impl ::core::convert::From<MakePersistent2Call> for HEVMCalls {
        fn from(value: MakePersistent2Call) -> Self {
            Self::MakePersistent2(value)
        }
    }
    impl ::core::convert::From<MakePersistent3Call> for HEVMCalls {
        fn from(value: MakePersistent3Call) -> Self {
            Self::MakePersistent3(value)
        }
    }
    impl ::core::convert::From<MakePersistent1Call> for HEVMCalls {
        fn from(value: MakePersistent1Call) -> Self {
            Self::MakePersistent1(value)
        }
    }
    impl ::core::convert::From<MockCall0Call> for HEVMCalls {
        fn from(value: MockCall0Call) -> Self {
            Self::MockCall0(value)
        }
    }
    impl ::core::convert::From<MockCall1Call> for HEVMCalls {
        fn from(value: MockCall1Call) -> Self {
            Self::MockCall1(value)
        }
    }
    impl ::core::convert::From<MockCallRevert0Call> for HEVMCalls {
        fn from(value: MockCallRevert0Call) -> Self {
            Self::MockCallRevert0(value)
        }
    }
    impl ::core::convert::From<MockCallRevert1Call> for HEVMCalls {
        fn from(value: MockCallRevert1Call) -> Self {
            Self::MockCallRevert1(value)
        }
    }
    impl ::core::convert::From<OpenFileCall> for HEVMCalls {
        fn from(value: OpenFileCall) -> Self {
            Self::OpenFile(value)
        }
    }
    impl ::core::convert::From<ParseAddressCall> for HEVMCalls {
        fn from(value: ParseAddressCall) -> Self {
            Self::ParseAddress(value)
        }
    }
    impl ::core::convert::From<ParseBoolCall> for HEVMCalls {
        fn from(value: ParseBoolCall) -> Self {
            Self::ParseBool(value)
        }
    }
    impl ::core::convert::From<ParseBytesCall> for HEVMCalls {
        fn from(value: ParseBytesCall) -> Self {
            Self::ParseBytes(value)
        }
    }
    impl ::core::convert::From<ParseBytes32Call> for HEVMCalls {
        fn from(value: ParseBytes32Call) -> Self {
            Self::ParseBytes32(value)
        }
    }
    impl ::core::convert::From<ParseIntCall> for HEVMCalls {
        fn from(value: ParseIntCall) -> Self {
            Self::ParseInt(value)
        }
    }
    impl ::core::convert::From<ParseJson0Call> for HEVMCalls {
        fn from(value: ParseJson0Call) -> Self {
            Self::ParseJson0(value)
        }
    }
    impl ::core::convert::From<ParseJson1Call> for HEVMCalls {
        fn from(value: ParseJson1Call) -> Self {
            Self::ParseJson1(value)
        }
    }
    impl ::core::convert::From<ParseJsonAddressCall> for HEVMCalls {
        fn from(value: ParseJsonAddressCall) -> Self {
            Self::ParseJsonAddress(value)
        }
    }
    impl ::core::convert::From<ParseJsonAddressArrayCall> for HEVMCalls {
        fn from(value: ParseJsonAddressArrayCall) -> Self {
            Self::ParseJsonAddressArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonBoolCall> for HEVMCalls {
        fn from(value: ParseJsonBoolCall) -> Self {
            Self::ParseJsonBool(value)
        }
    }
    impl ::core::convert::From<ParseJsonBoolArrayCall> for HEVMCalls {
        fn from(value: ParseJsonBoolArrayCall) -> Self {
            Self::ParseJsonBoolArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytesCall> for HEVMCalls {
        fn from(value: ParseJsonBytesCall) -> Self {
            Self::ParseJsonBytes(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytes32Call> for HEVMCalls {
        fn from(value: ParseJsonBytes32Call) -> Self {
            Self::ParseJsonBytes32(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytes32ArrayCall> for HEVMCalls {
        fn from(value: ParseJsonBytes32ArrayCall) -> Self {
            Self::ParseJsonBytes32Array(value)
        }
    }
    impl ::core::convert::From<ParseJsonBytesArrayCall> for HEVMCalls {
        fn from(value: ParseJsonBytesArrayCall) -> Self {
            Self::ParseJsonBytesArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonIntCall> for HEVMCalls {
        fn from(value: ParseJsonIntCall) -> Self {
            Self::ParseJsonInt(value)
        }
    }
    impl ::core::convert::From<ParseJsonIntArrayCall> for HEVMCalls {
        fn from(value: ParseJsonIntArrayCall) -> Self {
            Self::ParseJsonIntArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonStringCall> for HEVMCalls {
        fn from(value: ParseJsonStringCall) -> Self {
            Self::ParseJsonString(value)
        }
    }
    impl ::core::convert::From<ParseJsonStringArrayCall> for HEVMCalls {
        fn from(value: ParseJsonStringArrayCall) -> Self {
            Self::ParseJsonStringArray(value)
        }
    }
    impl ::core::convert::From<ParseJsonUintCall> for HEVMCalls {
        fn from(value: ParseJsonUintCall) -> Self {
            Self::ParseJsonUint(value)
        }
    }
    impl ::core::convert::From<ParseJsonUintArrayCall> for HEVMCalls {
        fn from(value: ParseJsonUintArrayCall) -> Self {
            Self::ParseJsonUintArray(value)
        }
    }
    impl ::core::convert::From<ParseUintCall> for HEVMCalls {
        fn from(value: ParseUintCall) -> Self {
            Self::ParseUint(value)
        }
    }
    impl ::core::convert::From<PauseGasMeteringCall> for HEVMCalls {
        fn from(value: PauseGasMeteringCall) -> Self {
            Self::PauseGasMetering(value)
        }
    }
    impl ::core::convert::From<Prank0Call> for HEVMCalls {
        fn from(value: Prank0Call) -> Self {
            Self::Prank0(value)
        }
    }
    impl ::core::convert::From<Prank1Call> for HEVMCalls {
        fn from(value: Prank1Call) -> Self {
            Self::Prank1(value)
        }
    }
    impl ::core::convert::From<ProjectRootCall> for HEVMCalls {
        fn from(value: ProjectRootCall) -> Self {
            Self::ProjectRoot(value)
        }
    }
    impl ::core::convert::From<ReadCallersCall> for HEVMCalls {
        fn from(value: ReadCallersCall) -> Self {
            Self::ReadCallers(value)
        }
    }
    impl ::core::convert::From<ReadDir0Call> for HEVMCalls {
        fn from(value: ReadDir0Call) -> Self {
            Self::ReadDir0(value)
        }
    }
    impl ::core::convert::From<ReadDir1Call> for HEVMCalls {
        fn from(value: ReadDir1Call) -> Self {
            Self::ReadDir1(value)
        }
    }
    impl ::core::convert::From<ReadDir2Call> for HEVMCalls {
        fn from(value: ReadDir2Call) -> Self {
            Self::ReadDir2(value)
        }
    }
    impl ::core::convert::From<ReadFileCall> for HEVMCalls {
        fn from(value: ReadFileCall) -> Self {
            Self::ReadFile(value)
        }
    }
    impl ::core::convert::From<ReadFileBinaryCall> for HEVMCalls {
        fn from(value: ReadFileBinaryCall) -> Self {
            Self::ReadFileBinary(value)
        }
    }
    impl ::core::convert::From<ReadLineCall> for HEVMCalls {
        fn from(value: ReadLineCall) -> Self {
            Self::ReadLine(value)
        }
    }
    impl ::core::convert::From<ReadLinkCall> for HEVMCalls {
        fn from(value: ReadLinkCall) -> Self {
            Self::ReadLink(value)
        }
    }
    impl ::core::convert::From<RecordCall> for HEVMCalls {
        fn from(value: RecordCall) -> Self {
            Self::Record(value)
        }
    }
    impl ::core::convert::From<RecordLogsCall> for HEVMCalls {
        fn from(value: RecordLogsCall) -> Self {
            Self::RecordLogs(value)
        }
    }
    impl ::core::convert::From<RememberKeyCall> for HEVMCalls {
        fn from(value: RememberKeyCall) -> Self {
            Self::RememberKey(value)
        }
    }
    impl ::core::convert::From<RemoveDirCall> for HEVMCalls {
        fn from(value: RemoveDirCall) -> Self {
            Self::RemoveDir(value)
        }
    }
    impl ::core::convert::From<RemoveFileCall> for HEVMCalls {
        fn from(value: RemoveFileCall) -> Self {
            Self::RemoveFile(value)
        }
    }
    impl ::core::convert::From<ResetNonceCall> for HEVMCalls {
        fn from(value: ResetNonceCall) -> Self {
            Self::ResetNonce(value)
        }
    }
    impl ::core::convert::From<ResumeGasMeteringCall> for HEVMCalls {
        fn from(value: ResumeGasMeteringCall) -> Self {
            Self::ResumeGasMetering(value)
        }
    }
    impl ::core::convert::From<RevertToCall> for HEVMCalls {
        fn from(value: RevertToCall) -> Self {
            Self::RevertTo(value)
        }
    }
    impl ::core::convert::From<RevokePersistent0Call> for HEVMCalls {
        fn from(value: RevokePersistent0Call) -> Self {
            Self::RevokePersistent0(value)
        }
    }
    impl ::core::convert::From<RevokePersistent1Call> for HEVMCalls {
        fn from(value: RevokePersistent1Call) -> Self {
            Self::RevokePersistent1(value)
        }
    }
    impl ::core::convert::From<RollCall> for HEVMCalls {
        fn from(value: RollCall) -> Self {
            Self::Roll(value)
        }
    }
    impl ::core::convert::From<RollFork0Call> for HEVMCalls {
        fn from(value: RollFork0Call) -> Self {
            Self::RollFork0(value)
        }
    }
    impl ::core::convert::From<RollFork1Call> for HEVMCalls {
        fn from(value: RollFork1Call) -> Self {
            Self::RollFork1(value)
        }
    }
    impl ::core::convert::From<RollFork2Call> for HEVMCalls {
        fn from(value: RollFork2Call) -> Self {
            Self::RollFork2(value)
        }
    }
    impl ::core::convert::From<RollFork3Call> for HEVMCalls {
        fn from(value: RollFork3Call) -> Self {
            Self::RollFork3(value)
        }
    }
    impl ::core::convert::From<RpcUrlCall> for HEVMCalls {
        fn from(value: RpcUrlCall) -> Self {
            Self::RpcUrl(value)
        }
    }
    impl ::core::convert::From<RpcUrlStructsCall> for HEVMCalls {
        fn from(value: RpcUrlStructsCall) -> Self {
            Self::RpcUrlStructs(value)
        }
    }
    impl ::core::convert::From<RpcUrlsCall> for HEVMCalls {
        fn from(value: RpcUrlsCall) -> Self {
            Self::RpcUrls(value)
        }
    }
    impl ::core::convert::From<SelectForkCall> for HEVMCalls {
        fn from(value: SelectForkCall) -> Self {
            Self::SelectFork(value)
        }
    }
    impl ::core::convert::From<SerializeAddress0Call> for HEVMCalls {
        fn from(value: SerializeAddress0Call) -> Self {
            Self::SerializeAddress0(value)
        }
    }
    impl ::core::convert::From<SerializeAddress1Call> for HEVMCalls {
        fn from(value: SerializeAddress1Call) -> Self {
            Self::SerializeAddress1(value)
        }
    }
    impl ::core::convert::From<SerializeBool0Call> for HEVMCalls {
        fn from(value: SerializeBool0Call) -> Self {
            Self::SerializeBool0(value)
        }
    }
    impl ::core::convert::From<SerializeBool1Call> for HEVMCalls {
        fn from(value: SerializeBool1Call) -> Self {
            Self::SerializeBool1(value)
        }
    }
    impl ::core::convert::From<SerializeBytes0Call> for HEVMCalls {
        fn from(value: SerializeBytes0Call) -> Self {
            Self::SerializeBytes0(value)
        }
    }
    impl ::core::convert::From<SerializeBytes1Call> for HEVMCalls {
        fn from(value: SerializeBytes1Call) -> Self {
            Self::SerializeBytes1(value)
        }
    }
    impl ::core::convert::From<SerializeBytes320Call> for HEVMCalls {
        fn from(value: SerializeBytes320Call) -> Self {
            Self::SerializeBytes320(value)
        }
    }
    impl ::core::convert::From<SerializeBytes321Call> for HEVMCalls {
        fn from(value: SerializeBytes321Call) -> Self {
            Self::SerializeBytes321(value)
        }
    }
    impl ::core::convert::From<SerializeInt0Call> for HEVMCalls {
        fn from(value: SerializeInt0Call) -> Self {
            Self::SerializeInt0(value)
        }
    }
    impl ::core::convert::From<SerializeInt1Call> for HEVMCalls {
        fn from(value: SerializeInt1Call) -> Self {
            Self::SerializeInt1(value)
        }
    }
    impl ::core::convert::From<SerializeString0Call> for HEVMCalls {
        fn from(value: SerializeString0Call) -> Self {
            Self::SerializeString0(value)
        }
    }
    impl ::core::convert::From<SerializeString1Call> for HEVMCalls {
        fn from(value: SerializeString1Call) -> Self {
            Self::SerializeString1(value)
        }
    }
    impl ::core::convert::From<SerializeUint0Call> for HEVMCalls {
        fn from(value: SerializeUint0Call) -> Self {
            Self::SerializeUint0(value)
        }
    }
    impl ::core::convert::From<SerializeUint1Call> for HEVMCalls {
        fn from(value: SerializeUint1Call) -> Self {
            Self::SerializeUint1(value)
        }
    }
    impl ::core::convert::From<SetEnvCall> for HEVMCalls {
        fn from(value: SetEnvCall) -> Self {
            Self::SetEnv(value)
        }
    }
    impl ::core::convert::From<SetNonceCall> for HEVMCalls {
        fn from(value: SetNonceCall) -> Self {
            Self::SetNonce(value)
        }
    }
    impl ::core::convert::From<SetNonceUnsafeCall> for HEVMCalls {
        fn from(value: SetNonceUnsafeCall) -> Self {
            Self::SetNonceUnsafe(value)
        }
    }
    impl ::core::convert::From<SignCall> for HEVMCalls {
        fn from(value: SignCall) -> Self {
            Self::Sign(value)
        }
    }
    impl ::core::convert::From<SkipCall> for HEVMCalls {
        fn from(value: SkipCall) -> Self {
            Self::Skip(value)
        }
    }
    impl ::core::convert::From<SnapshotCall> for HEVMCalls {
        fn from(value: SnapshotCall) -> Self {
            Self::Snapshot(value)
        }
    }
    impl ::core::convert::From<StartBroadcast0Call> for HEVMCalls {
        fn from(value: StartBroadcast0Call) -> Self {
            Self::StartBroadcast0(value)
        }
    }
    impl ::core::convert::From<StartBroadcast1Call> for HEVMCalls {
        fn from(value: StartBroadcast1Call) -> Self {
            Self::StartBroadcast1(value)
        }
    }
    impl ::core::convert::From<StartBroadcast2Call> for HEVMCalls {
        fn from(value: StartBroadcast2Call) -> Self {
            Self::StartBroadcast2(value)
        }
    }
    impl ::core::convert::From<StartMappingRecordingCall> for HEVMCalls {
        fn from(value: StartMappingRecordingCall) -> Self {
            Self::StartMappingRecording(value)
        }
    }
    impl ::core::convert::From<StartPrank0Call> for HEVMCalls {
        fn from(value: StartPrank0Call) -> Self {
            Self::StartPrank0(value)
        }
    }
    impl ::core::convert::From<StartPrank1Call> for HEVMCalls {
        fn from(value: StartPrank1Call) -> Self {
            Self::StartPrank1(value)
        }
    }
    impl ::core::convert::From<StopBroadcastCall> for HEVMCalls {
        fn from(value: StopBroadcastCall) -> Self {
            Self::StopBroadcast(value)
        }
    }
    impl ::core::convert::From<StopMappingRecordingCall> for HEVMCalls {
        fn from(value: StopMappingRecordingCall) -> Self {
            Self::StopMappingRecording(value)
        }
    }
    impl ::core::convert::From<StopPrankCall> for HEVMCalls {
        fn from(value: StopPrankCall) -> Self {
            Self::StopPrank(value)
        }
    }
    impl ::core::convert::From<StoreCall> for HEVMCalls {
        fn from(value: StoreCall) -> Self {
            Self::Store(value)
        }
    }
    impl ::core::convert::From<ToString0Call> for HEVMCalls {
        fn from(value: ToString0Call) -> Self {
            Self::ToString0(value)
        }
    }
    impl ::core::convert::From<ToString1Call> for HEVMCalls {
        fn from(value: ToString1Call) -> Self {
            Self::ToString1(value)
        }
    }
    impl ::core::convert::From<ToString2Call> for HEVMCalls {
        fn from(value: ToString2Call) -> Self {
            Self::ToString2(value)
        }
    }
    impl ::core::convert::From<ToString3Call> for HEVMCalls {
        fn from(value: ToString3Call) -> Self {
            Self::ToString3(value)
        }
    }
    impl ::core::convert::From<ToString4Call> for HEVMCalls {
        fn from(value: ToString4Call) -> Self {
            Self::ToString4(value)
        }
    }
    impl ::core::convert::From<ToString5Call> for HEVMCalls {
        fn from(value: ToString5Call) -> Self {
            Self::ToString5(value)
        }
    }
    impl ::core::convert::From<Transact0Call> for HEVMCalls {
        fn from(value: Transact0Call) -> Self {
            Self::Transact0(value)
        }
    }
    impl ::core::convert::From<Transact1Call> for HEVMCalls {
        fn from(value: Transact1Call) -> Self {
            Self::Transact1(value)
        }
    }
    impl ::core::convert::From<TxGasPriceCall> for HEVMCalls {
        fn from(value: TxGasPriceCall) -> Self {
            Self::TxGasPrice(value)
        }
    }
    impl ::core::convert::From<WarpCall> for HEVMCalls {
        fn from(value: WarpCall) -> Self {
            Self::Warp(value)
        }
    }
    impl ::core::convert::From<WriteFileCall> for HEVMCalls {
        fn from(value: WriteFileCall) -> Self {
            Self::WriteFile(value)
        }
    }
    impl ::core::convert::From<WriteFileBinaryCall> for HEVMCalls {
        fn from(value: WriteFileBinaryCall) -> Self {
            Self::WriteFileBinary(value)
        }
    }
    impl ::core::convert::From<WriteJson0Call> for HEVMCalls {
        fn from(value: WriteJson0Call) -> Self {
            Self::WriteJson0(value)
        }
    }
    impl ::core::convert::From<WriteJson1Call> for HEVMCalls {
        fn from(value: WriteJson1Call) -> Self {
            Self::WriteJson1(value)
        }
    }
    impl ::core::convert::From<WriteLineCall> for HEVMCalls {
        fn from(value: WriteLineCall) -> Self {
            Self::WriteLine(value)
        }
    }
    ///Container type for all return fields from the `accesses` function with signature `accesses(address)` and selector `0x6e606627`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AccessesReturn(
        pub ::std::vec::Vec<[u8; 32]>,
        pub ::std::vec::Vec<[u8; 32]>,
    );
    ///Container type for all return fields from the `activeFork` function with signature `activeFork()` and selector `0x2a978c58`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ActiveForkReturn(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `addr` function with signature `addr(string)` and selector `0xf91daf04`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct AddrReturn(pub ::corebc_core::types::Address);
    ///Container type for all return fields from the `createFork` function with signature `createFork(string,uint256)` and selector `0x27dc0128`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateFork1Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `createFork` function with signature `createFork(string,bytes32)` and selector `0x472ee61e`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateFork2Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `createFork` function with signature `createFork(string)` and selector `0xc8282be0`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateFork0Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `createSelectFork` function with signature `createSelectFork(string,uint256)` and selector `0xcc15574b`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateSelectFork1Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `createSelectFork` function with signature `createSelectFork(string,bytes32)` and selector `0x47cbbd7a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateSelectFork2Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `createSelectFork` function with signature `createSelectFork(string)` and selector `0x6bcf2c60`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CreateSelectFork0Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,uint32)` and selector `0xf6f7e9e1`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeriveKey0Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,string,uint32)` and selector `0xd5a72210`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeriveKey1Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,uint32,string)` and selector `0xee602b67`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeriveKey2Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `deriveKey` function with signature `deriveKey(string,string,uint32,string)` and selector `0xcc819384`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DeriveKey3Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `envAddress` function with signature `envAddress(string)` and selector `0x5992c5c6`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvAddress0Return(pub ::corebc_core::types::Address);
    ///Container type for all return fields from the `envAddress` function with signature `envAddress(string,string)` and selector `0xdd088083`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvAddress1Return(pub ::std::vec::Vec<::corebc_core::types::Address>);
    ///Container type for all return fields from the `envBool` function with signature `envBool(string)` and selector `0xfd40bd11`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvBool0Return(pub bool);
    ///Container type for all return fields from the `envBool` function with signature `envBool(string,string)` and selector `0xdf739509`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvBool1Return(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `envBytes` function with signature `envBytes(string)` and selector `0xb1676ba0`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvBytes0Return(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `envBytes` function with signature `envBytes(string,string)` and selector `0xd9577bb1`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvBytes1Return(pub ::std::vec::Vec<::corebc_core::types::Bytes>);
    ///Container type for all return fields from the `envBytes32` function with signature `envBytes32(string)` and selector `0x2d261418`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvBytes320Return(pub [u8; 32]);
    ///Container type for all return fields from the `envBytes32` function with signature `envBytes32(string,string)` and selector `0x5c1826d3`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvBytes321Return(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `envInt` function with signature `envInt(string)` and selector `0xdba2cad8`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvInt0Return(pub ::corebc_core::types::I256);
    ///Container type for all return fields from the `envInt` function with signature `envInt(string,string)` and selector `0x347968b0`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvInt1Return(pub ::std::vec::Vec<::corebc_core::types::I256>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,bool)` and selector `0x8efd04a9`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr0Return(pub bool);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,uint256)` and selector `0x1c6db43b`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr1Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,int256)` and selector `0x35711e3f`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr2Return(pub ::corebc_core::types::I256);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,address)` and selector `0x18745f8c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr3Return(pub ::corebc_core::types::Address);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,bytes32)` and selector `0xcb48937a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr4Return(pub [u8; 32]);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string)` and selector `0x42d1d0ef`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr5Return(pub ::std::string::String);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,bytes)` and selector `0x2dd17959`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr6Return(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,bool[])` and selector `0x9430ce1e`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr7Return(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,uint256[])` and selector `0xbf6e43ef`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr8Return(pub ::std::vec::Vec<::corebc_core::types::U256>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,int256[])` and selector `0x60a8eb5e`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr9Return(pub ::std::vec::Vec<::corebc_core::types::I256>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,address[])` and selector `0x0dbd12fc`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr10Return(pub ::std::vec::Vec<::corebc_core::types::Address>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,bytes32[])` and selector `0x864e3a3c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr11Return(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,string[])` and selector `0x9eb2a8c4`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr12Return(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `envOr` function with signature `envOr(string,string,bytes[])` and selector `0x206f7cdd`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvOr13Return(pub ::std::vec::Vec<::corebc_core::types::Bytes>);
    ///Container type for all return fields from the `envString` function with signature `envString(string)` and selector `0xe613c3f6`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvString0Return(pub ::std::string::String);
    ///Container type for all return fields from the `envString` function with signature `envString(string,string)` and selector `0x535a86f8`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvString1Return(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `envUint` function with signature `envUint(string)` and selector `0x22b1fcf4`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvUint0Return(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `envUint` function with signature `envUint(string,string)` and selector `0x09437722`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct EnvUint1Return(pub ::std::vec::Vec<::corebc_core::types::U256>);
    ///Container type for all return fields from the `ffi` function with signature `ffi(string[])` and selector `0x64ba2b32`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FfiReturn(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `fsMetadata` function with signature `fsMetadata(string)` and selector `0x895b0357`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FsMetadataReturn(
        pub (
            bool,
            bool,
            ::corebc_core::types::U256,
            bool,
            ::corebc_core::types::U256,
            ::corebc_core::types::U256,
            ::corebc_core::types::U256,
        ),
    );
    ///Container type for all return fields from the `getLabel` function with signature `getLabel(address)` and selector `0x56c8fc59`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetLabelReturn(pub ::std::string::String);
    ///Container type for all return fields from the `getRecordedLogs` function with signature `getRecordedLogs()` and selector `0x6aa9ac15`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct GetRecordedLogsReturn(
        pub ::std::vec::Vec<(::std::vec::Vec<[u8; 32]>, ::corebc_core::types::Bytes)>,
    );
    ///Container type for all return fields from the `isPersistent` function with signature `isPersistent(address)` and selector `0x27051a9d`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsPersistentReturn(pub bool);
    ///Container type for all return fields from the `load` function with signature `load(address,bytes32)` and selector `0xfac88623`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct LoadReturn(pub [u8; 32]);
    ///Container type for all return fields from the `parseAddress` function with signature `parseAddress(string)` and selector `0xbf03d4f6`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseAddressReturn(pub ::corebc_core::types::Address);
    ///Container type for all return fields from the `parseBool` function with signature `parseBool(string)` and selector `0xc711f34c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseBoolReturn(pub bool);
    ///Container type for all return fields from the `parseBytes` function with signature `parseBytes(string)` and selector `0x3dc7ecf1`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseBytesReturn(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `parseBytes32` function with signature `parseBytes32(string)` and selector `0x6ea2d6ed`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `parseInt` function with signature `parseInt(string)` and selector `0xcacaea8a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseIntReturn(pub ::corebc_core::types::I256);
    ///Container type for all return fields from the `parseJson` function with signature `parseJson(string)` and selector `0x1b14d0fa`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJson0Return(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `parseJson` function with signature `parseJson(string,string)` and selector `0x9ed12636`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJson1Return(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `parseJsonAddress` function with signature `parseJsonAddress(string,string)` and selector `0x6d037439`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonAddressReturn(pub ::corebc_core::types::Address);
    ///Container type for all return fields from the `parseJsonAddressArray` function with signature `parseJsonAddressArray(string,string)` and selector `0x73ae0e2b`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonAddressArrayReturn(
        pub ::std::vec::Vec<::corebc_core::types::Address>,
    );
    ///Container type for all return fields from the `parseJsonBool` function with signature `parseJsonBool(string,string)` and selector `0x1ddc1e05`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonBoolReturn(pub bool);
    ///Container type for all return fields from the `parseJsonBoolArray` function with signature `parseJsonBoolArray(string,string)` and selector `0x1b7a2c28`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonBoolArrayReturn(pub ::std::vec::Vec<bool>);
    ///Container type for all return fields from the `parseJsonBytes` function with signature `parseJsonBytes(string,string)` and selector `0x4ae7c4b9`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonBytesReturn(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `parseJsonBytes32` function with signature `parseJsonBytes32(string,string)` and selector `0x2088d639`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonBytes32Return(pub [u8; 32]);
    ///Container type for all return fields from the `parseJsonBytes32Array` function with signature `parseJsonBytes32Array(string,string)` and selector `0x6ed6bf6d`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonBytes32ArrayReturn(pub ::std::vec::Vec<[u8; 32]>);
    ///Container type for all return fields from the `parseJsonBytesArray` function with signature `parseJsonBytesArray(string,string)` and selector `0xc7f2b390`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonBytesArrayReturn(
        pub ::std::vec::Vec<::corebc_core::types::Bytes>,
    );
    ///Container type for all return fields from the `parseJsonInt` function with signature `parseJsonInt(string,string)` and selector `0x7e3d173a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonIntReturn(pub ::corebc_core::types::I256);
    ///Container type for all return fields from the `parseJsonIntArray` function with signature `parseJsonIntArray(string,string)` and selector `0xc37c99a1`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonIntArrayReturn(pub ::std::vec::Vec<::corebc_core::types::I256>);
    ///Container type for all return fields from the `parseJsonString` function with signature `parseJsonString(string,string)` and selector `0x9d9b1273`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonStringReturn(pub ::std::string::String);
    ///Container type for all return fields from the `parseJsonStringArray` function with signature `parseJsonStringArray(string,string)` and selector `0x6499a16c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonStringArrayReturn(pub ::std::vec::Vec<::std::string::String>);
    ///Container type for all return fields from the `parseJsonUint` function with signature `parseJsonUint(string,string)` and selector `0x7aa5f1ff`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonUintReturn(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `parseJsonUintArray` function with signature `parseJsonUintArray(string,string)` and selector `0x3118994d`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseJsonUintArrayReturn(pub ::std::vec::Vec<::corebc_core::types::U256>);
    ///Container type for all return fields from the `parseUint` function with signature `parseUint(string)` and selector `0xd3fdc787`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ParseUintReturn(pub ::corebc_core::types::U256);
    ///Container type for all return fields from the `projectRoot` function with signature `projectRoot()` and selector `0x6e197bfb`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProjectRootReturn(pub ::std::string::String);
    ///Container type for all return fields from the `readCallers` function with signature `readCallers()` and selector `0xc89ced7f`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadCallersReturn(
        pub ::corebc_core::types::U256,
        pub ::corebc_core::types::Address,
        pub ::corebc_core::types::Address,
    );
    ///Container type for all return fields from the `readDir` function with signature `readDir(string)` and selector `0xf108a04c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadDir0Return(
        pub ::std::vec::Vec<
            (::std::string::String, ::std::string::String, u64, bool, bool),
        >,
    );
    ///Container type for all return fields from the `readDir` function with signature `readDir(string,uint64)` and selector `0xb2f4318c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadDir1Return(
        pub ::std::vec::Vec<
            (::std::string::String, ::std::string::String, u64, bool, bool),
        >,
    );
    ///Container type for all return fields from the `readDir` function with signature `readDir(string,uint64,bool)` and selector `0xc87aafac`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadDir2Return(
        pub ::std::vec::Vec<
            (::std::string::String, ::std::string::String, u64, bool, bool),
        >,
    );
    ///Container type for all return fields from the `readFile` function with signature `readFile(string)` and selector `0x875a759d`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadFileReturn(pub ::std::string::String);
    ///Container type for all return fields from the `readFileBinary` function with signature `readFileBinary(string)` and selector `0x06623015`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadFileBinaryReturn(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `readLine` function with signature `readLine(string)` and selector `0xadd52189`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadLineReturn(pub ::std::string::String);
    ///Container type for all return fields from the `readLink` function with signature `readLink(string)` and selector `0xbb3ef03a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ReadLinkReturn(pub ::std::string::String);
    ///Container type for all return fields from the `rememberKey` function with signature `rememberKey(string)` and selector `0x40eac48c`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RememberKeyReturn(pub ::corebc_core::types::Address);
    ///Container type for all return fields from the `revertTo` function with signature `revertTo(uint256)` and selector `0x255a8c36`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RevertToReturn(pub bool);
    ///Container type for all return fields from the `rpcUrl` function with signature `rpcUrl(string)` and selector `0x931ea701`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RpcUrlReturn(pub ::std::string::String);
    ///Container type for all return fields from the `rpcUrlStructs` function with signature `rpcUrlStructs()` and selector `0x725410b7`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RpcUrlStructsReturn(
        pub ::std::vec::Vec<(::std::string::String, ::std::string::String)>,
    );
    ///Container type for all return fields from the `rpcUrls` function with signature `rpcUrls()` and selector `0x266095bf`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RpcUrlsReturn(pub ::std::vec::Vec<[::std::string::String; 2]>);
    ///Container type for all return fields from the `serializeAddress` function with signature `serializeAddress(string,string,address)` and selector `0x1b9dc788`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeAddress0Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeAddress` function with signature `serializeAddress(string,string,address[])` and selector `0x9ef1ff0b`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeAddress1Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeBool` function with signature `serializeBool(string,string,bool)` and selector `0x06c97128`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeBool0Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeBool` function with signature `serializeBool(string,string,bool[])` and selector `0xc40c3d45`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeBool1Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeBytes` function with signature `serializeBytes(string,string,bytes)` and selector `0xd2d2b28a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeBytes0Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeBytes` function with signature `serializeBytes(string,string,bytes[])` and selector `0x0af30cb5`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeBytes1Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32)` and selector `0x271a3830`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeBytes320Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeBytes32` function with signature `serializeBytes32(string,string,bytes32[])` and selector `0x6324bb27`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeBytes321Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeInt` function with signature `serializeInt(string,string,int256)` and selector `0x38b12609`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeInt0Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeInt` function with signature `serializeInt(string,string,int256[])` and selector `0xb2decdb3`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeInt1Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeString` function with signature `serializeString(string,string,string)` and selector `0x5c7ccb97`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeString0Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeString` function with signature `serializeString(string,string,string[])` and selector `0xa5df3cd6`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeString1Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeUint` function with signature `serializeUint(string,string,uint256)` and selector `0xa11b741a`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeUint0Return(pub ::std::string::String);
    ///Container type for all return fields from the `serializeUint` function with signature `serializeUint(string,string,uint256[])` and selector `0x6d152d8f`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SerializeUint1Return(pub ::std::string::String);
    ///Container type for all return fields from the `sign` function with signature `sign(string,bytes32)` and selector `0xd0c67f5f`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SignReturn(pub ::corebc_core::types::Bytes);
    ///Container type for all return fields from the `snapshot` function with signature `snapshot()` and selector `0x843c63f8`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct SnapshotReturn(pub ::corebc_core::types::U256);
    ///`DirEntry(string,string,uint64,bool,bool)`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct DirEntry {
        pub error_message: ::std::string::String,
        pub path: ::std::string::String,
        pub depth: u64,
        pub is_dir: bool,
        pub is_symlink: bool,
    }
    ///`FsMetadata(bool,bool,uint256,bool,uint256,uint256,uint256)`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct FsMetadata {
        pub is_dir: bool,
        pub is_symlink: bool,
        pub length: ::corebc_core::types::U256,
        pub read_only: bool,
        pub modified: ::corebc_core::types::U256,
        pub accessed: ::corebc_core::types::U256,
        pub created: ::corebc_core::types::U256,
    }
    ///`Log(bytes32[],bytes)`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Log {
        pub topics: ::std::vec::Vec<[u8; 32]>,
        pub data: ::corebc_core::types::Bytes,
    }
    ///`Rpc(string,string)`
    #[derive(
        Clone,
        ::corebc_contract::EthAbiType,
        ::corebc_contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct Rpc {
        pub name: ::std::string::String,
        pub url: ::std::string::String,
    }
}
