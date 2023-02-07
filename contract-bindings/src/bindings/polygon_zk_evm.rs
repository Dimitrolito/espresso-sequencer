pub use polygon_zk_evm::*;
#[allow(clippy::too_many_arguments, non_camel_case_types)]
pub mod polygon_zk_evm {
    #![allow(clippy::enum_variant_names)]
    #![allow(dead_code)]
    #![allow(clippy::type_complexity)]
    #![allow(unused_imports)]
    pub use super::super::shared_types::*;
    use ethers::contract::{
        builders::{ContractCall, Event},
        Contract, Lazy,
    };
    use ethers::core::{
        abi::{Abi, Detokenize, InvalidOutputType, Token, Tokenizable},
        types::*,
    };
    use ethers::providers::Middleware;
    #[doc = "PolygonZkEVM was auto-generated with ethers-rs Abigen. More information at: https://github.com/gakonst/ethers-rs"]
    use std::sync::Arc;
    # [rustfmt :: skip] const __ABI : & str = "[{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"ConsolidatePendingState\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateActivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"EmergencyStateDeactivated\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"forceBatchNum\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"lastGlobalExitRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"sequencer\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ForceBatch\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"version\",\"type\":\"uint8\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"Initialized\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"aggregator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OverridePendingState\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"previousOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"storedStateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"bytes32\",\"name\":\"provedStateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"ProveNonDeterministicPendingState\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SequenceBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"SequenceForceBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetAdmin\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"newForceBatchAllowed\",\"type\":\"bool\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetForceBatchAllowed\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"newMultiplierBatchFee\",\"type\":\"uint16\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetMultiplierBatchFee\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newPendingStateTimeout\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetPendingStateTimeout\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedAggregator\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedAggregator\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newTrustedAggregatorTimeout\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedAggregatorTimeout\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedSequencer\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedSequencer\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newTrustedSequencerURL\",\"type\":\"string\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetTrustedSequencerURL\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newVeryBatchTimeTarget\",\"type\":\"uint64\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SetVeryBatchTimeTarget\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"aggregator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"TrustedVerifyBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"numBatch\",\"type\":\"uint64\",\"components\":[],\"indexed\":true},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"aggregator\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"VerifyBatches\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"FORCE_BATCH_TIMEOUT\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"HALT_AGGREGATION_TIMEOUT\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_BATCH_MULTIPLIER\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_TRANSACTIONS_BYTE_LENGTH\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"MAX_VERIFY_BATCHES\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"sequencedBatchNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"activateEmergencyState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"admin\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"batchFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"batchNumToStateRoot\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"bridgeAddress\",\"outputs\":[{\"internalType\":\"contract IPolygonZkEVMBridge\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"calculateRewardPerBatch\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"chainID\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"consolidatePendingState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"deactivateEmergencyState\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maticAmount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"forceBatch\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"forceBatchAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"forcedBatches\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCurrentBatchFee\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"oldStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getInputSnarkBytes\",\"outputs\":[{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getLastVerifiedBatch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"globalExitRootManager\",\"outputs\":[{\"internalType\":\"contract IPolygonZkEVMGlobalExitRoot\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract IPolygonZkEVMGlobalExitRoot\",\"name\":\"_globalExitRootManager\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IERC20Upgradeable\",\"name\":\"_matic\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IVerifierRollup\",\"name\":\"_rollupVerifier\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"contract IPolygonZkEVMBridge\",\"name\":\"_bridgeAddress\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct PolygonZkEVM.InitializePackedParameters\",\"name\":\"initializePackedParameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"admin\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"chainID\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trustedSequencer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"pendingStateTimeout\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"forceBatchAllowed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"trustedAggregator\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"trustedAggregatorTimeout\",\"type\":\"uint64\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"genesisRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_trustedSequencerURL\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"string\",\"name\":\"_networkName\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isEmergencyState\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPendingStateConsolidable\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastBatchSequenced\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastForceBatch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastForceBatchSequenced\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastPendingState\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastPendingStateConsolidated\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastTimestamp\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"lastVerifiedBatch\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"matic\",\"outputs\":[{\"internalType\":\"contract IERC20Upgradeable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"multiplierBatchFee\",\"outputs\":[{\"internalType\":\"uint16\",\"name\":\"\",\"type\":\"uint16\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"networkName\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"initPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"overridePendingState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingStateTimeout\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"pendingStateTransitions\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"lastVerifiedBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"exitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"stateRoot\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"initPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalPendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"proveNonDeterministicPendingState\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"renounceOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"rollupVerifier\",\"outputs\":[{\"internalType\":\"contract IVerifierRollup\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct PolygonZkEVM.BatchData[]\",\"name\":\"batches\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"timestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"minForcedTimestamp\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sequenceBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"struct PolygonZkEVM.ForcedBatchData[]\",\"name\":\"batches\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"bytes\",\"name\":\"transactions\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"globalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"minForcedTimestamp\",\"type\":\"uint64\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"sequenceForceBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"sequencedBatches\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"accInputHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"sequencedTimestamp\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"previousLastBatchSequenced\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newAdmin\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setAdmin\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"newForceBatchAllowed\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setForceBatchAllowed\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint16\",\"name\":\"newMultiplierBatchFee\",\"type\":\"uint16\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setMultiplierBatchFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newPendingStateTimeout\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setPendingStateTimeout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedAggregator\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedAggregator\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newTrustedAggregatorTimeout\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedAggregatorTimeout\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newTrustedSequencer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedSequencer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"newTrustedSequencerURL\",\"type\":\"string\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setTrustedSequencerURL\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"newVeryBatchTimeTarget\",\"type\":\"uint64\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"setVeryBatchTimeTarget\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedAggregator\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedAggregatorTimeout\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedSequencer\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"trustedSequencerURL\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"trustedVerifyBatches\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint64\",\"name\":\"pendingStateNum\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"initNumBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"uint64\",\"name\":\"finalNewBatch\",\"type\":\"uint64\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newLocalExitRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"newStateRoot\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofA\",\"type\":\"uint256[2]\",\"components\":[]},{\"internalType\":\"uint256[2][2]\",\"name\":\"proofB\",\"type\":\"uint256[2][2]\",\"components\":[]},{\"internalType\":\"uint256[2]\",\"name\":\"proofC\",\"type\":\"uint256[2]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"verifyBatches\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"veryBatchTimeTarget\",\"outputs\":[{\"internalType\":\"uint64\",\"name\":\"\",\"type\":\"uint64\",\"components\":[]}]}]" ;
    #[doc = r" The parsed JSON-ABI of the contract."]
    pub static POLYGONZKEVM_ABI: ethers::contract::Lazy<ethers::core::abi::Abi> =
        ethers::contract::Lazy::new(|| {
            ethers::core::utils::__serde_json::from_str(__ABI).expect("invalid abi")
        });
    pub struct PolygonZkEVM<M>(ethers::contract::Contract<M>);
    impl<M> Clone for PolygonZkEVM<M> {
        fn clone(&self) -> Self {
            PolygonZkEVM(self.0.clone())
        }
    }
    impl<M> std::ops::Deref for PolygonZkEVM<M> {
        type Target = ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> std::fmt::Debug for PolygonZkEVM<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.debug_tuple(stringify!(PolygonZkEVM))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ethers::providers::Middleware> PolygonZkEVM<M> {
        #[doc = r" Creates a new contract instance with the specified `ethers`"]
        #[doc = r" client at the given `Address`. The contract derefs to a `ethers::Contract`"]
        #[doc = r" object"]
        pub fn new<T: Into<ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            ethers::contract::Contract::new(address.into(), POLYGONZKEVM_ABI.clone(), client).into()
        }
        #[doc = "Calls the contract's `FORCE_BATCH_TIMEOUT` (0xab9fc5ef) function"]
        pub fn force_batch_timeout(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([171, 159, 197, 239], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `HALT_AGGREGATION_TIMEOUT` (0x8b48931e) function"]
        pub fn halt_aggregation_timeout(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([139, 72, 147, 30], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_BATCH_MULTIPLIER` (0x9eb831b9) function"]
        pub fn max_batch_multiplier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([158, 184, 49, 185], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_TRANSACTIONS_BYTE_LENGTH` (0x2d0889d3) function"]
        pub fn max_transactions_byte_length(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([45, 8, 137, 211], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `MAX_VERIFY_BATCHES` (0xe217cfd6) function"]
        pub fn max_verify_batches(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([226, 23, 207, 214], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `activateEmergencyState` (0x7215541a) function"]
        pub fn activate_emergency_state(
            &self,
            sequenced_batch_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([114, 21, 84, 26], sequenced_batch_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `admin` (0xf851a440) function"]
        pub fn admin(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([248, 81, 164, 64], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchFee` (0xf8b823e4) function"]
        pub fn batch_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([248, 184, 35, 228], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `batchNumToStateRoot` (0x5392c5e0) function"]
        pub fn batch_num_to_state_root(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([83, 146, 197, 224], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `bridgeAddress` (0xa3c573eb) function"]
        pub fn bridge_address(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([163, 197, 115, 235], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `calculateRewardPerBatch` (0x99f5634e) function"]
        pub fn calculate_reward_per_batch(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([153, 245, 99, 78], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `chainID` (0xadc879e9) function"]
        pub fn chain_id(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([173, 200, 121, 233], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `consolidatePendingState` (0x4a910e6a) function"]
        pub fn consolidate_pending_state(
            &self,
            pending_state_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 145, 14, 106], pending_state_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `deactivateEmergencyState` (0xdbc16976) function"]
        pub fn deactivate_emergency_state(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 193, 105, 118], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forceBatch` (0xeaeb077b) function"]
        pub fn force_batch(
            &self,
            transactions: ethers::core::types::Bytes,
            matic_amount: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([234, 235, 7, 123], (transactions, matic_amount))
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forceBatchAllowed` (0xd8f54db0) function"]
        pub fn force_batch_allowed(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([216, 245, 77, 176], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `forcedBatches` (0x6b8616ce) function"]
        pub fn forced_batches(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([107, 134, 22, 206], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getCurrentBatchFee` (0x9f0d039d) function"]
        pub fn get_current_batch_fee(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::U256> {
            self.0
                .method_hash([159, 13, 3, 157], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getInputSnarkBytes` (0x220d7899) function"]
        pub fn get_input_snark_bytes(
            &self,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            old_state_root: [u8; 32],
            new_state_root: [u8; 32],
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Bytes> {
            self.0
                .method_hash(
                    [34, 13, 120, 153],
                    (
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        old_state_root,
                        new_state_root,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `getLastVerifiedBatch` (0xc0ed84e0) function"]
        pub fn get_last_verified_batch(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([192, 237, 132, 224], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `globalExitRootManager` (0xd02103ca) function"]
        pub fn global_exit_root_manager(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([208, 33, 3, 202], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `initialize` (0x60943d6a) function"]
        pub fn initialize(
            &self,
            global_exit_root_manager: ethers::core::types::Address,
            matic: ethers::core::types::Address,
            rollup_verifier: ethers::core::types::Address,
            bridge_address: ethers::core::types::Address,
            initialize_packed_parameters: InitializePackedParameters,
            genesis_root: [u8; 32],
            trusted_sequencer_url: String,
            network_name: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [96, 148, 61, 106],
                    (
                        global_exit_root_manager,
                        matic,
                        rollup_verifier,
                        bridge_address,
                        initialize_packed_parameters,
                        genesis_root,
                        trusted_sequencer_url,
                        network_name,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isEmergencyState` (0x15064c96) function"]
        pub fn is_emergency_state(&self) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([21, 6, 76, 150], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `isPendingStateConsolidable` (0x383b3be8) function"]
        pub fn is_pending_state_consolidable(
            &self,
            pending_state_num: u64,
        ) -> ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([56, 59, 59, 232], pending_state_num)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastBatchSequenced` (0x423fa856) function"]
        pub fn last_batch_sequenced(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([66, 63, 168, 86], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastForceBatch` (0xe7a7ed02) function"]
        pub fn last_force_batch(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([231, 167, 237, 2], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastForceBatchSequenced` (0x45605267) function"]
        pub fn last_force_batch_sequenced(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([69, 96, 82, 103], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastPendingState` (0x458c0477) function"]
        pub fn last_pending_state(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([69, 140, 4, 119], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastPendingStateConsolidated` (0x4a1a89a7) function"]
        pub fn last_pending_state_consolidated(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([74, 26, 137, 167], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastTimestamp` (0x19d8ac61) function"]
        pub fn last_timestamp(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([25, 216, 172, 97], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `lastVerifiedBatch` (0x7fcb3653) function"]
        pub fn last_verified_batch(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([127, 203, 54, 83], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `matic` (0xb6b0b097) function"]
        pub fn matic(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([182, 176, 176, 151], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `multiplierBatchFee` (0xafd23cbe) function"]
        pub fn multiplier_batch_fee(&self) -> ethers::contract::builders::ContractCall<M, u16> {
            self.0
                .method_hash([175, 210, 60, 190], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `networkName` (0x107bf28c) function"]
        pub fn network_name(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([16, 123, 242, 140], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `overridePendingState` (0xe11f3f18) function"]
        pub fn override_pending_state(
            &self,
            init_pending_state_num: u64,
            final_pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [225, 31, 63, 24],
                    (
                        init_pending_state_num,
                        final_pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `owner` (0x8da5cb5b) function"]
        pub fn owner(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingStateTimeout` (0xd939b315) function"]
        pub fn pending_state_timeout(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([217, 57, 179, 21], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `pendingStateTransitions` (0x837a4738) function"]
        pub fn pending_state_transitions(
            &self,
            p0: ethers::core::types::U256,
        ) -> ethers::contract::builders::ContractCall<M, (u64, u64, [u8; 32], [u8; 32])> {
            self.0
                .method_hash([131, 122, 71, 56], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `proveNonDeterministicPendingState` (0x75c508b3) function"]
        pub fn prove_non_deterministic_pending_state(
            &self,
            init_pending_state_num: u64,
            final_pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [117, 197, 8, 179],
                    (
                        init_pending_state_num,
                        final_pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `renounceOwnership` (0x715018a6) function"]
        pub fn renounce_ownership(&self) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 80, 24, 166], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `rollupVerifier` (0xe8bf92ed) function"]
        pub fn rollup_verifier(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([232, 191, 146, 237], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequenceBatches` (0x3c158267) function"]
        pub fn sequence_batches(
            &self,
            batches: ::std::vec::Vec<BatchData>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 21, 130, 103], batches)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequenceForceBatches` (0xd8d1091b) function"]
        pub fn sequence_force_batches(
            &self,
            batches: ::std::vec::Vec<ForcedBatchData>,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 209, 9, 27], batches)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `sequencedBatches` (0xb4d63f58) function"]
        pub fn sequenced_batches(
            &self,
            p0: u64,
        ) -> ethers::contract::builders::ContractCall<M, ([u8; 32], u64, u64)> {
            self.0
                .method_hash([180, 214, 63, 88], p0)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setAdmin` (0x704b6c02) function"]
        pub fn set_admin(
            &self,
            new_admin: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([112, 75, 108, 2], new_admin)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setForceBatchAllowed` (0x8c4a0af7) function"]
        pub fn set_force_batch_allowed(
            &self,
            new_force_batch_allowed: bool,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 74, 10, 247], new_force_batch_allowed)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setMultiplierBatchFee` (0x1816b7e5) function"]
        pub fn set_multiplier_batch_fee(
            &self,
            new_multiplier_batch_fee: u16,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 22, 183, 229], new_multiplier_batch_fee)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setPendingStateTimeout` (0x9c9f3dfe) function"]
        pub fn set_pending_state_timeout(
            &self,
            new_pending_state_timeout: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 159, 61, 254], new_pending_state_timeout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedAggregator` (0xf14916d6) function"]
        pub fn set_trusted_aggregator(
            &self,
            new_trusted_aggregator: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 73, 22, 214], new_trusted_aggregator)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedAggregatorTimeout` (0x394218e9) function"]
        pub fn set_trusted_aggregator_timeout(
            &self,
            new_trusted_aggregator_timeout: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 66, 24, 233], new_trusted_aggregator_timeout)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedSequencer` (0x6ff512cc) function"]
        pub fn set_trusted_sequencer(
            &self,
            new_trusted_sequencer: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 245, 18, 204], new_trusted_sequencer)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setTrustedSequencerURL` (0xc89e42df) function"]
        pub fn set_trusted_sequencer_url(
            &self,
            new_trusted_sequencer_url: String,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([200, 158, 66, 223], new_trusted_sequencer_url)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `setVeryBatchTimeTarget` (0xcf136306) function"]
        pub fn set_very_batch_time_target(
            &self,
            new_very_batch_time_target: u64,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 19, 99, 6], new_very_batch_time_target)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `transferOwnership` (0xf2fde38b) function"]
        pub fn transfer_ownership(
            &self,
            new_owner: ethers::core::types::Address,
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedAggregator` (0x29878983) function"]
        pub fn trusted_aggregator(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([41, 135, 137, 131], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedAggregatorTimeout` (0x841b24d7) function"]
        pub fn trusted_aggregator_timeout(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([132, 27, 36, 215], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedSequencer` (0xcfa8ed47) function"]
        pub fn trusted_sequencer(
            &self,
        ) -> ethers::contract::builders::ContractCall<M, ethers::core::types::Address> {
            self.0
                .method_hash([207, 168, 237, 71], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedSequencerURL` (0x542028d5) function"]
        pub fn trusted_sequencer_url(&self) -> ethers::contract::builders::ContractCall<M, String> {
            self.0
                .method_hash([84, 32, 40, 213], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `trustedVerifyBatches` (0xedc41121) function"]
        pub fn trusted_verify_batches(
            &self,
            pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [237, 196, 17, 33],
                    (
                        pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `verifyBatches` (0x4834a343) function"]
        pub fn verify_batches(
            &self,
            pending_state_num: u64,
            init_num_batch: u64,
            final_new_batch: u64,
            new_local_exit_root: [u8; 32],
            new_state_root: [u8; 32],
            proof_a: [ethers::core::types::U256; 2usize],
            proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
            proof_c: [ethers::core::types::U256; 2usize],
        ) -> ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [72, 52, 163, 67],
                    (
                        pending_state_num,
                        init_num_batch,
                        final_new_batch,
                        new_local_exit_root,
                        new_state_root,
                        proof_a,
                        proof_b,
                        proof_c,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        #[doc = "Calls the contract's `veryBatchTimeTarget` (0xaa58bad6) function"]
        pub fn very_batch_time_target(&self) -> ethers::contract::builders::ContractCall<M, u64> {
            self.0
                .method_hash([170, 88, 186, 214], ())
                .expect("method not found (this should never happen)")
        }
        #[doc = "Gets the contract's `ConsolidatePendingState` event"]
        pub fn consolidate_pending_state_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ConsolidatePendingStateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EmergencyStateActivated` event"]
        pub fn emergency_state_activated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EmergencyStateActivatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `EmergencyStateDeactivated` event"]
        pub fn emergency_state_deactivated_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, EmergencyStateDeactivatedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ForceBatch` event"]
        pub fn force_batch_filter(&self) -> ethers::contract::builders::Event<M, ForceBatchFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `Initialized` event"]
        pub fn initialized_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, InitializedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OverridePendingState` event"]
        pub fn override_pending_state_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OverridePendingStateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `OwnershipTransferred` event"]
        pub fn ownership_transferred_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, OwnershipTransferredFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `ProveNonDeterministicPendingState` event"]
        pub fn prove_non_deterministic_pending_state_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, ProveNonDeterministicPendingStateFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SequenceBatches` event"]
        pub fn sequence_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SequenceBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SequenceForceBatches` event"]
        pub fn sequence_force_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SequenceForceBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetAdmin` event"]
        pub fn set_admin_filter(&self) -> ethers::contract::builders::Event<M, SetAdminFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetForceBatchAllowed` event"]
        pub fn set_force_batch_allowed_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetForceBatchAllowedFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetMultiplierBatchFee` event"]
        pub fn set_multiplier_batch_fee_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetMultiplierBatchFeeFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetPendingStateTimeout` event"]
        pub fn set_pending_state_timeout_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetPendingStateTimeoutFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedAggregator` event"]
        pub fn set_trusted_aggregator_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedAggregatorFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedAggregatorTimeout` event"]
        pub fn set_trusted_aggregator_timeout_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedAggregatorTimeoutFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedSequencer` event"]
        pub fn set_trusted_sequencer_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedSequencerFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetTrustedSequencerURL` event"]
        pub fn set_trusted_sequencer_url_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetTrustedSequencerURLFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `SetVeryBatchTimeTarget` event"]
        pub fn set_very_batch_time_target_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, SetVeryBatchTimeTargetFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `TrustedVerifyBatches` event"]
        pub fn trusted_verify_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, TrustedVerifyBatchesFilter> {
            self.0.event()
        }
        #[doc = "Gets the contract's `VerifyBatches` event"]
        pub fn verify_batches_filter(
            &self,
        ) -> ethers::contract::builders::Event<M, VerifyBatchesFilter> {
            self.0.event()
        }
        #[doc = r" Returns an [`Event`](#ethers_contract::builders::Event) builder for all events of this contract"]
        pub fn events(&self) -> ethers::contract::builders::Event<M, PolygonZkEVMEvents> {
            self.0.event_with_filter(Default::default())
        }
    }
    impl<M: ethers::providers::Middleware> From<ethers::contract::Contract<M>> for PolygonZkEVM<M> {
        fn from(contract: ethers::contract::Contract<M>) -> Self {
            Self(contract)
        }
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ConsolidatePendingState",
        abi = "ConsolidatePendingState(uint64,bytes32,uint64)"
    )]
    pub struct ConsolidatePendingStateFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub pending_state_num: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "EmergencyStateActivated", abi = "EmergencyStateActivated()")]
    pub struct EmergencyStateActivatedFilter();
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "EmergencyStateDeactivated",
        abi = "EmergencyStateDeactivated()"
    )]
    pub struct EmergencyStateDeactivatedFilter();
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "ForceBatch", abi = "ForceBatch(uint64,bytes32,address,bytes)")]
    pub struct ForceBatchFilter {
        #[ethevent(indexed)]
        pub force_batch_num: u64,
        pub last_global_exit_root: [u8; 32],
        pub sequencer: ethers::core::types::Address,
        pub transactions: ethers::core::types::Bytes,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "Initialized", abi = "Initialized(uint8)")]
    pub struct InitializedFilter {
        pub version: u8,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OverridePendingState",
        abi = "OverridePendingState(uint64,bytes32,address)"
    )]
    pub struct OverridePendingStateFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "OwnershipTransferred",
        abi = "OwnershipTransferred(address,address)"
    )]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub previous_owner: ethers::core::types::Address,
        #[ethevent(indexed)]
        pub new_owner: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "ProveNonDeterministicPendingState",
        abi = "ProveNonDeterministicPendingState(bytes32,bytes32)"
    )]
    pub struct ProveNonDeterministicPendingStateFilter {
        pub stored_state_root: [u8; 32],
        pub proved_state_root: [u8; 32],
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SequenceBatches", abi = "SequenceBatches(uint64)")]
    pub struct SequenceBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SequenceForceBatches", abi = "SequenceForceBatches(uint64)")]
    pub struct SequenceForceBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetAdmin", abi = "SetAdmin(address)")]
    pub struct SetAdminFilter {
        pub new_admin: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetForceBatchAllowed", abi = "SetForceBatchAllowed(bool)")]
    pub struct SetForceBatchAllowedFilter {
        pub new_force_batch_allowed: bool,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetMultiplierBatchFee", abi = "SetMultiplierBatchFee(uint16)")]
    pub struct SetMultiplierBatchFeeFilter {
        pub new_multiplier_batch_fee: u16,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetPendingStateTimeout",
        abi = "SetPendingStateTimeout(uint64)"
    )]
    pub struct SetPendingStateTimeoutFilter {
        pub new_pending_state_timeout: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetTrustedAggregator", abi = "SetTrustedAggregator(address)")]
    pub struct SetTrustedAggregatorFilter {
        pub new_trusted_aggregator: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetTrustedAggregatorTimeout",
        abi = "SetTrustedAggregatorTimeout(uint64)"
    )]
    pub struct SetTrustedAggregatorTimeoutFilter {
        pub new_trusted_aggregator_timeout: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "SetTrustedSequencer", abi = "SetTrustedSequencer(address)")]
    pub struct SetTrustedSequencerFilter {
        pub new_trusted_sequencer: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetTrustedSequencerURL",
        abi = "SetTrustedSequencerURL(string)"
    )]
    pub struct SetTrustedSequencerURLFilter {
        pub new_trusted_sequencer_url: String,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "SetVeryBatchTimeTarget",
        abi = "SetVeryBatchTimeTarget(uint64)"
    )]
    pub struct SetVeryBatchTimeTargetFilter {
        pub new_very_batch_time_target: u64,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(
        name = "TrustedVerifyBatches",
        abi = "TrustedVerifyBatches(uint64,bytes32,address)"
    )]
    pub struct TrustedVerifyBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
    }
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthEvent,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethevent(name = "VerifyBatches", abi = "VerifyBatches(uint64,bytes32,address)")]
    pub struct VerifyBatchesFilter {
        #[ethevent(indexed)]
        pub num_batch: u64,
        pub state_root: [u8; 32],
        #[ethevent(indexed)]
        pub aggregator: ethers::core::types::Address,
    }
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMEvents {
        ConsolidatePendingStateFilter(ConsolidatePendingStateFilter),
        EmergencyStateActivatedFilter(EmergencyStateActivatedFilter),
        EmergencyStateDeactivatedFilter(EmergencyStateDeactivatedFilter),
        ForceBatchFilter(ForceBatchFilter),
        InitializedFilter(InitializedFilter),
        OverridePendingStateFilter(OverridePendingStateFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        ProveNonDeterministicPendingStateFilter(ProveNonDeterministicPendingStateFilter),
        SequenceBatchesFilter(SequenceBatchesFilter),
        SequenceForceBatchesFilter(SequenceForceBatchesFilter),
        SetAdminFilter(SetAdminFilter),
        SetForceBatchAllowedFilter(SetForceBatchAllowedFilter),
        SetMultiplierBatchFeeFilter(SetMultiplierBatchFeeFilter),
        SetPendingStateTimeoutFilter(SetPendingStateTimeoutFilter),
        SetTrustedAggregatorFilter(SetTrustedAggregatorFilter),
        SetTrustedAggregatorTimeoutFilter(SetTrustedAggregatorTimeoutFilter),
        SetTrustedSequencerFilter(SetTrustedSequencerFilter),
        SetTrustedSequencerURLFilter(SetTrustedSequencerURLFilter),
        SetVeryBatchTimeTargetFilter(SetVeryBatchTimeTargetFilter),
        TrustedVerifyBatchesFilter(TrustedVerifyBatchesFilter),
        VerifyBatchesFilter(VerifyBatchesFilter),
    }
    impl ethers::contract::EthLogDecode for PolygonZkEVMEvents {
        fn decode_log(
            log: &ethers::core::abi::RawLog,
        ) -> ::std::result::Result<Self, ethers::core::abi::Error>
        where
            Self: Sized,
        {
            if let Ok(decoded) = ConsolidatePendingStateFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::ConsolidatePendingStateFilter(decoded));
            }
            if let Ok(decoded) = EmergencyStateActivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::EmergencyStateActivatedFilter(decoded));
            }
            if let Ok(decoded) = EmergencyStateDeactivatedFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::EmergencyStateDeactivatedFilter(decoded));
            }
            if let Ok(decoded) = ForceBatchFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::ForceBatchFilter(decoded));
            }
            if let Ok(decoded) = InitializedFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::InitializedFilter(decoded));
            }
            if let Ok(decoded) = OverridePendingStateFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::OverridePendingStateFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = ProveNonDeterministicPendingStateFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::ProveNonDeterministicPendingStateFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SequenceBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SequenceBatchesFilter(decoded));
            }
            if let Ok(decoded) = SequenceForceBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SequenceForceBatchesFilter(decoded));
            }
            if let Ok(decoded) = SetAdminFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetAdminFilter(decoded));
            }
            if let Ok(decoded) = SetForceBatchAllowedFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetForceBatchAllowedFilter(decoded));
            }
            if let Ok(decoded) = SetMultiplierBatchFeeFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetMultiplierBatchFeeFilter(decoded));
            }
            if let Ok(decoded) = SetPendingStateTimeoutFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetPendingStateTimeoutFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedAggregatorFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetTrustedAggregatorFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedAggregatorTimeoutFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetTrustedAggregatorTimeoutFilter(
                    decoded,
                ));
            }
            if let Ok(decoded) = SetTrustedSequencerFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetTrustedSequencerFilter(decoded));
            }
            if let Ok(decoded) = SetTrustedSequencerURLFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetTrustedSequencerURLFilter(decoded));
            }
            if let Ok(decoded) = SetVeryBatchTimeTargetFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::SetVeryBatchTimeTargetFilter(decoded));
            }
            if let Ok(decoded) = TrustedVerifyBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::TrustedVerifyBatchesFilter(decoded));
            }
            if let Ok(decoded) = VerifyBatchesFilter::decode_log(log) {
                return Ok(PolygonZkEVMEvents::VerifyBatchesFilter(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMEvents {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMEvents::ConsolidatePendingStateFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::EmergencyStateActivatedFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::EmergencyStateDeactivatedFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::ForceBatchFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::InitializedFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::OverridePendingStateFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::OwnershipTransferredFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::ProveNonDeterministicPendingStateFilter(element) => {
                    element.fmt(f)
                }
                PolygonZkEVMEvents::SequenceBatchesFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SequenceForceBatchesFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetAdminFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetForceBatchAllowedFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetMultiplierBatchFeeFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetPendingStateTimeoutFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetTrustedAggregatorFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetTrustedAggregatorTimeoutFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetTrustedSequencerFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetTrustedSequencerURLFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::SetVeryBatchTimeTargetFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::TrustedVerifyBatchesFilter(element) => element.fmt(f),
                PolygonZkEVMEvents::VerifyBatchesFilter(element) => element.fmt(f),
            }
        }
    }
    #[doc = "Container type for all input parameters for the `FORCE_BATCH_TIMEOUT` function with signature `FORCE_BATCH_TIMEOUT()` and selector `[171, 159, 197, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "FORCE_BATCH_TIMEOUT", abi = "FORCE_BATCH_TIMEOUT()")]
    pub struct ForceBatchTimeoutCall;
    #[doc = "Container type for all input parameters for the `HALT_AGGREGATION_TIMEOUT` function with signature `HALT_AGGREGATION_TIMEOUT()` and selector `[139, 72, 147, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "HALT_AGGREGATION_TIMEOUT", abi = "HALT_AGGREGATION_TIMEOUT()")]
    pub struct HaltAggregationTimeoutCall;
    #[doc = "Container type for all input parameters for the `MAX_BATCH_MULTIPLIER` function with signature `MAX_BATCH_MULTIPLIER()` and selector `[158, 184, 49, 185]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_BATCH_MULTIPLIER", abi = "MAX_BATCH_MULTIPLIER()")]
    pub struct MaxBatchMultiplierCall;
    #[doc = "Container type for all input parameters for the `MAX_TRANSACTIONS_BYTE_LENGTH` function with signature `MAX_TRANSACTIONS_BYTE_LENGTH()` and selector `[45, 8, 137, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "MAX_TRANSACTIONS_BYTE_LENGTH",
        abi = "MAX_TRANSACTIONS_BYTE_LENGTH()"
    )]
    pub struct MaxTransactionsByteLengthCall;
    #[doc = "Container type for all input parameters for the `MAX_VERIFY_BATCHES` function with signature `MAX_VERIFY_BATCHES()` and selector `[226, 23, 207, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "MAX_VERIFY_BATCHES", abi = "MAX_VERIFY_BATCHES()")]
    pub struct MaxVerifyBatchesCall;
    #[doc = "Container type for all input parameters for the `activateEmergencyState` function with signature `activateEmergencyState(uint64)` and selector `[114, 21, 84, 26]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "activateEmergencyState",
        abi = "activateEmergencyState(uint64)"
    )]
    pub struct ActivateEmergencyStateCall {
        pub sequenced_batch_num: u64,
    }
    #[doc = "Container type for all input parameters for the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "admin", abi = "admin()")]
    pub struct AdminCall;
    #[doc = "Container type for all input parameters for the `batchFee` function with signature `batchFee()` and selector `[248, 184, 35, 228]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "batchFee", abi = "batchFee()")]
    pub struct BatchFeeCall;
    #[doc = "Container type for all input parameters for the `batchNumToStateRoot` function with signature `batchNumToStateRoot(uint64)` and selector `[83, 146, 197, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "batchNumToStateRoot", abi = "batchNumToStateRoot(uint64)")]
    pub struct BatchNumToStateRootCall(pub u64);
    #[doc = "Container type for all input parameters for the `bridgeAddress` function with signature `bridgeAddress()` and selector `[163, 197, 115, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "bridgeAddress", abi = "bridgeAddress()")]
    pub struct BridgeAddressCall;
    #[doc = "Container type for all input parameters for the `calculateRewardPerBatch` function with signature `calculateRewardPerBatch()` and selector `[153, 245, 99, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "calculateRewardPerBatch", abi = "calculateRewardPerBatch()")]
    pub struct CalculateRewardPerBatchCall;
    #[doc = "Container type for all input parameters for the `chainID` function with signature `chainID()` and selector `[173, 200, 121, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "chainID", abi = "chainID()")]
    pub struct ChainIDCall;
    #[doc = "Container type for all input parameters for the `consolidatePendingState` function with signature `consolidatePendingState(uint64)` and selector `[74, 145, 14, 106]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "consolidatePendingState",
        abi = "consolidatePendingState(uint64)"
    )]
    pub struct ConsolidatePendingStateCall {
        pub pending_state_num: u64,
    }
    #[doc = "Container type for all input parameters for the `deactivateEmergencyState` function with signature `deactivateEmergencyState()` and selector `[219, 193, 105, 118]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "deactivateEmergencyState", abi = "deactivateEmergencyState()")]
    pub struct DeactivateEmergencyStateCall;
    #[doc = "Container type for all input parameters for the `forceBatch` function with signature `forceBatch(bytes,uint256)` and selector `[234, 235, 7, 123]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forceBatch", abi = "forceBatch(bytes,uint256)")]
    pub struct ForceBatchCall {
        pub transactions: ethers::core::types::Bytes,
        pub matic_amount: ethers::core::types::U256,
    }
    #[doc = "Container type for all input parameters for the `forceBatchAllowed` function with signature `forceBatchAllowed()` and selector `[216, 245, 77, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forceBatchAllowed", abi = "forceBatchAllowed()")]
    pub struct ForceBatchAllowedCall;
    #[doc = "Container type for all input parameters for the `forcedBatches` function with signature `forcedBatches(uint64)` and selector `[107, 134, 22, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "forcedBatches", abi = "forcedBatches(uint64)")]
    pub struct ForcedBatchesCall(pub u64);
    #[doc = "Container type for all input parameters for the `getCurrentBatchFee` function with signature `getCurrentBatchFee()` and selector `[159, 13, 3, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getCurrentBatchFee", abi = "getCurrentBatchFee()")]
    pub struct GetCurrentBatchFeeCall;
    #[doc = "Container type for all input parameters for the `getInputSnarkBytes` function with signature `getInputSnarkBytes(uint64,uint64,bytes32,bytes32,bytes32)` and selector `[34, 13, 120, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "getInputSnarkBytes",
        abi = "getInputSnarkBytes(uint64,uint64,bytes32,bytes32,bytes32)"
    )]
    pub struct GetInputSnarkBytesCall {
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub old_state_root: [u8; 32],
        pub new_state_root: [u8; 32],
    }
    #[doc = "Container type for all input parameters for the `getLastVerifiedBatch` function with signature `getLastVerifiedBatch()` and selector `[192, 237, 132, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "getLastVerifiedBatch", abi = "getLastVerifiedBatch()")]
    pub struct GetLastVerifiedBatchCall;
    #[doc = "Container type for all input parameters for the `globalExitRootManager` function with signature `globalExitRootManager()` and selector `[208, 33, 3, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "globalExitRootManager", abi = "globalExitRootManager()")]
    pub struct GlobalExitRootManagerCall;
    #[doc = "Container type for all input parameters for the `initialize` function with signature `initialize(address,address,address,address,(address,uint64,address,uint64,bool,address,uint64),bytes32,string,string)` and selector `[96, 148, 61, 106]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "initialize",
        abi = "initialize(address,address,address,address,(address,uint64,address,uint64,bool,address,uint64),bytes32,string,string)"
    )]
    pub struct InitializeCall {
        pub global_exit_root_manager: ethers::core::types::Address,
        pub matic: ethers::core::types::Address,
        pub rollup_verifier: ethers::core::types::Address,
        pub bridge_address: ethers::core::types::Address,
        pub initialize_packed_parameters: InitializePackedParameters,
        pub genesis_root: [u8; 32],
        pub trusted_sequencer_url: String,
        pub network_name: String,
    }
    #[doc = "Container type for all input parameters for the `isEmergencyState` function with signature `isEmergencyState()` and selector `[21, 6, 76, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "isEmergencyState", abi = "isEmergencyState()")]
    pub struct IsEmergencyStateCall;
    #[doc = "Container type for all input parameters for the `isPendingStateConsolidable` function with signature `isPendingStateConsolidable(uint64)` and selector `[56, 59, 59, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "isPendingStateConsolidable",
        abi = "isPendingStateConsolidable(uint64)"
    )]
    pub struct IsPendingStateConsolidableCall {
        pub pending_state_num: u64,
    }
    #[doc = "Container type for all input parameters for the `lastBatchSequenced` function with signature `lastBatchSequenced()` and selector `[66, 63, 168, 86]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastBatchSequenced", abi = "lastBatchSequenced()")]
    pub struct LastBatchSequencedCall;
    #[doc = "Container type for all input parameters for the `lastForceBatch` function with signature `lastForceBatch()` and selector `[231, 167, 237, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastForceBatch", abi = "lastForceBatch()")]
    pub struct LastForceBatchCall;
    #[doc = "Container type for all input parameters for the `lastForceBatchSequenced` function with signature `lastForceBatchSequenced()` and selector `[69, 96, 82, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastForceBatchSequenced", abi = "lastForceBatchSequenced()")]
    pub struct LastForceBatchSequencedCall;
    #[doc = "Container type for all input parameters for the `lastPendingState` function with signature `lastPendingState()` and selector `[69, 140, 4, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastPendingState", abi = "lastPendingState()")]
    pub struct LastPendingStateCall;
    #[doc = "Container type for all input parameters for the `lastPendingStateConsolidated` function with signature `lastPendingStateConsolidated()` and selector `[74, 26, 137, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "lastPendingStateConsolidated",
        abi = "lastPendingStateConsolidated()"
    )]
    pub struct LastPendingStateConsolidatedCall;
    #[doc = "Container type for all input parameters for the `lastTimestamp` function with signature `lastTimestamp()` and selector `[25, 216, 172, 97]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastTimestamp", abi = "lastTimestamp()")]
    pub struct LastTimestampCall;
    #[doc = "Container type for all input parameters for the `lastVerifiedBatch` function with signature `lastVerifiedBatch()` and selector `[127, 203, 54, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "lastVerifiedBatch", abi = "lastVerifiedBatch()")]
    pub struct LastVerifiedBatchCall;
    #[doc = "Container type for all input parameters for the `matic` function with signature `matic()` and selector `[182, 176, 176, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "matic", abi = "matic()")]
    pub struct MaticCall;
    #[doc = "Container type for all input parameters for the `multiplierBatchFee` function with signature `multiplierBatchFee()` and selector `[175, 210, 60, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "multiplierBatchFee", abi = "multiplierBatchFee()")]
    pub struct MultiplierBatchFeeCall;
    #[doc = "Container type for all input parameters for the `networkName` function with signature `networkName()` and selector `[16, 123, 242, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "networkName", abi = "networkName()")]
    pub struct NetworkNameCall;
    #[doc = "Container type for all input parameters for the `overridePendingState` function with signature `overridePendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[225, 31, 63, 24]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "overridePendingState",
        abi = "overridePendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct OverridePendingStateCall {
        pub init_pending_state_num: u64,
        pub final_pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    #[doc = "Container type for all input parameters for the `pendingStateTimeout` function with signature `pendingStateTimeout()` and selector `[217, 57, 179, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "pendingStateTimeout", abi = "pendingStateTimeout()")]
    pub struct PendingStateTimeoutCall;
    #[doc = "Container type for all input parameters for the `pendingStateTransitions` function with signature `pendingStateTransitions(uint256)` and selector `[131, 122, 71, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "pendingStateTransitions",
        abi = "pendingStateTransitions(uint256)"
    )]
    pub struct PendingStateTransitionsCall(pub ethers::core::types::U256);
    #[doc = "Container type for all input parameters for the `proveNonDeterministicPendingState` function with signature `proveNonDeterministicPendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[117, 197, 8, 179]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "proveNonDeterministicPendingState",
        abi = "proveNonDeterministicPendingState(uint64,uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct ProveNonDeterministicPendingStateCall {
        pub init_pending_state_num: u64,
        pub final_pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `renounceOwnership` function with signature `renounceOwnership()` and selector `[113, 80, 24, 166]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "renounceOwnership", abi = "renounceOwnership()")]
    pub struct RenounceOwnershipCall;
    #[doc = "Container type for all input parameters for the `rollupVerifier` function with signature `rollupVerifier()` and selector `[232, 191, 146, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "rollupVerifier", abi = "rollupVerifier()")]
    pub struct RollupVerifierCall;
    #[doc = "Container type for all input parameters for the `sequenceBatches` function with signature `sequenceBatches((bytes,bytes32,uint64,uint64)[])` and selector `[60, 21, 130, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "sequenceBatches",
        abi = "sequenceBatches((bytes,bytes32,uint64,uint64)[])"
    )]
    pub struct SequenceBatchesCall {
        pub batches: ::std::vec::Vec<BatchData>,
    }
    #[doc = "Container type for all input parameters for the `sequenceForceBatches` function with signature `sequenceForceBatches((bytes,bytes32,uint64)[])` and selector `[216, 209, 9, 27]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "sequenceForceBatches",
        abi = "sequenceForceBatches((bytes,bytes32,uint64)[])"
    )]
    pub struct SequenceForceBatchesCall {
        pub batches: ::std::vec::Vec<ForcedBatchData>,
    }
    #[doc = "Container type for all input parameters for the `sequencedBatches` function with signature `sequencedBatches(uint64)` and selector `[180, 214, 63, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "sequencedBatches", abi = "sequencedBatches(uint64)")]
    pub struct SequencedBatchesCall(pub u64);
    #[doc = "Container type for all input parameters for the `setAdmin` function with signature `setAdmin(address)` and selector `[112, 75, 108, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setAdmin", abi = "setAdmin(address)")]
    pub struct SetAdminCall {
        pub new_admin: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setForceBatchAllowed` function with signature `setForceBatchAllowed(bool)` and selector `[140, 74, 10, 247]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setForceBatchAllowed", abi = "setForceBatchAllowed(bool)")]
    pub struct SetForceBatchAllowedCall {
        pub new_force_batch_allowed: bool,
    }
    #[doc = "Container type for all input parameters for the `setMultiplierBatchFee` function with signature `setMultiplierBatchFee(uint16)` and selector `[24, 22, 183, 229]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setMultiplierBatchFee", abi = "setMultiplierBatchFee(uint16)")]
    pub struct SetMultiplierBatchFeeCall {
        pub new_multiplier_batch_fee: u16,
    }
    #[doc = "Container type for all input parameters for the `setPendingStateTimeout` function with signature `setPendingStateTimeout(uint64)` and selector `[156, 159, 61, 254]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setPendingStateTimeout",
        abi = "setPendingStateTimeout(uint64)"
    )]
    pub struct SetPendingStateTimeoutCall {
        pub new_pending_state_timeout: u64,
    }
    #[doc = "Container type for all input parameters for the `setTrustedAggregator` function with signature `setTrustedAggregator(address)` and selector `[241, 73, 22, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTrustedAggregator", abi = "setTrustedAggregator(address)")]
    pub struct SetTrustedAggregatorCall {
        pub new_trusted_aggregator: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setTrustedAggregatorTimeout` function with signature `setTrustedAggregatorTimeout(uint64)` and selector `[57, 66, 24, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setTrustedAggregatorTimeout",
        abi = "setTrustedAggregatorTimeout(uint64)"
    )]
    pub struct SetTrustedAggregatorTimeoutCall {
        pub new_trusted_aggregator_timeout: u64,
    }
    #[doc = "Container type for all input parameters for the `setTrustedSequencer` function with signature `setTrustedSequencer(address)` and selector `[111, 245, 18, 204]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "setTrustedSequencer", abi = "setTrustedSequencer(address)")]
    pub struct SetTrustedSequencerCall {
        pub new_trusted_sequencer: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `setTrustedSequencerURL` function with signature `setTrustedSequencerURL(string)` and selector `[200, 158, 66, 223]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setTrustedSequencerURL",
        abi = "setTrustedSequencerURL(string)"
    )]
    pub struct SetTrustedSequencerURLCall {
        pub new_trusted_sequencer_url: String,
    }
    #[doc = "Container type for all input parameters for the `setVeryBatchTimeTarget` function with signature `setVeryBatchTimeTarget(uint64)` and selector `[207, 19, 99, 6]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "setVeryBatchTimeTarget",
        abi = "setVeryBatchTimeTarget(uint64)"
    )]
    pub struct SetVeryBatchTimeTargetCall {
        pub new_very_batch_time_target: u64,
    }
    #[doc = "Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `[242, 253, 227, 139]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ethers::core::types::Address,
    }
    #[doc = "Container type for all input parameters for the `trustedAggregator` function with signature `trustedAggregator()` and selector `[41, 135, 137, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedAggregator", abi = "trustedAggregator()")]
    pub struct TrustedAggregatorCall;
    #[doc = "Container type for all input parameters for the `trustedAggregatorTimeout` function with signature `trustedAggregatorTimeout()` and selector `[132, 27, 36, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedAggregatorTimeout", abi = "trustedAggregatorTimeout()")]
    pub struct TrustedAggregatorTimeoutCall;
    #[doc = "Container type for all input parameters for the `trustedSequencer` function with signature `trustedSequencer()` and selector `[207, 168, 237, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedSequencer", abi = "trustedSequencer()")]
    pub struct TrustedSequencerCall;
    #[doc = "Container type for all input parameters for the `trustedSequencerURL` function with signature `trustedSequencerURL()` and selector `[84, 32, 40, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "trustedSequencerURL", abi = "trustedSequencerURL()")]
    pub struct TrustedSequencerURLCall;
    #[doc = "Container type for all input parameters for the `trustedVerifyBatches` function with signature `trustedVerifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[237, 196, 17, 33]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "trustedVerifyBatches",
        abi = "trustedVerifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct TrustedVerifyBatchesCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `verifyBatches` function with signature `verifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])` and selector `[72, 52, 163, 67]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(
        name = "verifyBatches",
        abi = "verifyBatches(uint64,uint64,uint64,bytes32,bytes32,uint256[2],uint256[2][2],uint256[2])"
    )]
    pub struct VerifyBatchesCall {
        pub pending_state_num: u64,
        pub init_num_batch: u64,
        pub final_new_batch: u64,
        pub new_local_exit_root: [u8; 32],
        pub new_state_root: [u8; 32],
        pub proof_a: [ethers::core::types::U256; 2usize],
        pub proof_b: [[ethers::core::types::U256; 2usize]; 2usize],
        pub proof_c: [ethers::core::types::U256; 2usize],
    }
    #[doc = "Container type for all input parameters for the `veryBatchTimeTarget` function with signature `veryBatchTimeTarget()` and selector `[170, 88, 186, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthCall,
        ethers :: contract :: EthDisplay,
        Default,
    )]
    #[ethcall(name = "veryBatchTimeTarget", abi = "veryBatchTimeTarget()")]
    pub struct VeryBatchTimeTargetCall;
    #[derive(Debug, Clone, PartialEq, Eq, ethers :: contract :: EthAbiType)]
    pub enum PolygonZkEVMCalls {
        ForceBatchTimeout(ForceBatchTimeoutCall),
        HaltAggregationTimeout(HaltAggregationTimeoutCall),
        MaxBatchMultiplier(MaxBatchMultiplierCall),
        MaxTransactionsByteLength(MaxTransactionsByteLengthCall),
        MaxVerifyBatches(MaxVerifyBatchesCall),
        ActivateEmergencyState(ActivateEmergencyStateCall),
        Admin(AdminCall),
        BatchFee(BatchFeeCall),
        BatchNumToStateRoot(BatchNumToStateRootCall),
        BridgeAddress(BridgeAddressCall),
        CalculateRewardPerBatch(CalculateRewardPerBatchCall),
        ChainID(ChainIDCall),
        ConsolidatePendingState(ConsolidatePendingStateCall),
        DeactivateEmergencyState(DeactivateEmergencyStateCall),
        ForceBatch(ForceBatchCall),
        ForceBatchAllowed(ForceBatchAllowedCall),
        ForcedBatches(ForcedBatchesCall),
        GetCurrentBatchFee(GetCurrentBatchFeeCall),
        GetInputSnarkBytes(GetInputSnarkBytesCall),
        GetLastVerifiedBatch(GetLastVerifiedBatchCall),
        GlobalExitRootManager(GlobalExitRootManagerCall),
        Initialize(InitializeCall),
        IsEmergencyState(IsEmergencyStateCall),
        IsPendingStateConsolidable(IsPendingStateConsolidableCall),
        LastBatchSequenced(LastBatchSequencedCall),
        LastForceBatch(LastForceBatchCall),
        LastForceBatchSequenced(LastForceBatchSequencedCall),
        LastPendingState(LastPendingStateCall),
        LastPendingStateConsolidated(LastPendingStateConsolidatedCall),
        LastTimestamp(LastTimestampCall),
        LastVerifiedBatch(LastVerifiedBatchCall),
        Matic(MaticCall),
        MultiplierBatchFee(MultiplierBatchFeeCall),
        NetworkName(NetworkNameCall),
        OverridePendingState(OverridePendingStateCall),
        Owner(OwnerCall),
        PendingStateTimeout(PendingStateTimeoutCall),
        PendingStateTransitions(PendingStateTransitionsCall),
        ProveNonDeterministicPendingState(ProveNonDeterministicPendingStateCall),
        RenounceOwnership(RenounceOwnershipCall),
        RollupVerifier(RollupVerifierCall),
        SequenceBatches(SequenceBatchesCall),
        SequenceForceBatches(SequenceForceBatchesCall),
        SequencedBatches(SequencedBatchesCall),
        SetAdmin(SetAdminCall),
        SetForceBatchAllowed(SetForceBatchAllowedCall),
        SetMultiplierBatchFee(SetMultiplierBatchFeeCall),
        SetPendingStateTimeout(SetPendingStateTimeoutCall),
        SetTrustedAggregator(SetTrustedAggregatorCall),
        SetTrustedAggregatorTimeout(SetTrustedAggregatorTimeoutCall),
        SetTrustedSequencer(SetTrustedSequencerCall),
        SetTrustedSequencerURL(SetTrustedSequencerURLCall),
        SetVeryBatchTimeTarget(SetVeryBatchTimeTargetCall),
        TransferOwnership(TransferOwnershipCall),
        TrustedAggregator(TrustedAggregatorCall),
        TrustedAggregatorTimeout(TrustedAggregatorTimeoutCall),
        TrustedSequencer(TrustedSequencerCall),
        TrustedSequencerURL(TrustedSequencerURLCall),
        TrustedVerifyBatches(TrustedVerifyBatchesCall),
        VerifyBatches(VerifyBatchesCall),
        VeryBatchTimeTarget(VeryBatchTimeTargetCall),
    }
    impl ethers::core::abi::AbiDecode for PolygonZkEVMCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::std::result::Result<Self, ethers::core::abi::AbiError> {
            if let Ok(decoded) =
                <ForceBatchTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ForceBatchTimeout(decoded));
            }
            if let Ok(decoded) =
                <HaltAggregationTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::HaltAggregationTimeout(decoded));
            }
            if let Ok(decoded) =
                <MaxBatchMultiplierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::MaxBatchMultiplier(decoded));
            }
            if let Ok(decoded) =
                <MaxTransactionsByteLengthCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::MaxTransactionsByteLength(decoded));
            }
            if let Ok(decoded) =
                <MaxVerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::MaxVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <ActivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ActivateEmergencyState(decoded));
            }
            if let Ok(decoded) = <AdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::Admin(decoded));
            }
            if let Ok(decoded) =
                <BatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::BatchFee(decoded));
            }
            if let Ok(decoded) =
                <BatchNumToStateRootCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::BatchNumToStateRoot(decoded));
            }
            if let Ok(decoded) =
                <BridgeAddressCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::BridgeAddress(decoded));
            }
            if let Ok(decoded) =
                <CalculateRewardPerBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::CalculateRewardPerBatch(decoded));
            }
            if let Ok(decoded) =
                <ChainIDCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ChainID(decoded));
            }
            if let Ok(decoded) =
                <ConsolidatePendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ConsolidatePendingState(decoded));
            }
            if let Ok(decoded) =
                <DeactivateEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::DeactivateEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ForceBatch(decoded));
            }
            if let Ok(decoded) =
                <ForceBatchAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ForceBatchAllowed(decoded));
            }
            if let Ok(decoded) =
                <ForcedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::ForcedBatches(decoded));
            }
            if let Ok(decoded) =
                <GetCurrentBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::GetCurrentBatchFee(decoded));
            }
            if let Ok(decoded) =
                <GetInputSnarkBytesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::GetInputSnarkBytes(decoded));
            }
            if let Ok(decoded) =
                <GetLastVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::GetLastVerifiedBatch(decoded));
            }
            if let Ok(decoded) =
                <GlobalExitRootManagerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::GlobalExitRootManager(decoded));
            }
            if let Ok(decoded) =
                <InitializeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::Initialize(decoded));
            }
            if let Ok(decoded) =
                <IsEmergencyStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::IsEmergencyState(decoded));
            }
            if let Ok(decoded) =
                <IsPendingStateConsolidableCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::IsPendingStateConsolidable(decoded));
            }
            if let Ok(decoded) =
                <LastBatchSequencedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::LastBatchSequenced(decoded));
            }
            if let Ok(decoded) =
                <LastForceBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::LastForceBatch(decoded));
            }
            if let Ok(decoded) =
                <LastForceBatchSequencedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::LastForceBatchSequenced(decoded));
            }
            if let Ok(decoded) =
                <LastPendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::LastPendingState(decoded));
            }
            if let Ok(decoded) =
                <LastPendingStateConsolidatedCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::LastPendingStateConsolidated(decoded));
            }
            if let Ok(decoded) =
                <LastTimestampCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::LastTimestamp(decoded));
            }
            if let Ok(decoded) =
                <LastVerifiedBatchCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::LastVerifiedBatch(decoded));
            }
            if let Ok(decoded) = <MaticCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::Matic(decoded));
            }
            if let Ok(decoded) =
                <MultiplierBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::MultiplierBatchFee(decoded));
            }
            if let Ok(decoded) =
                <NetworkNameCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::NetworkName(decoded));
            }
            if let Ok(decoded) =
                <OverridePendingStateCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::OverridePendingState(decoded));
            }
            if let Ok(decoded) = <OwnerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::Owner(decoded));
            }
            if let Ok(decoded) =
                <PendingStateTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::PendingStateTimeout(decoded));
            }
            if let Ok(decoded) =
                <PendingStateTransitionsCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::PendingStateTransitions(decoded));
            }
            if let Ok(decoded) =
                <ProveNonDeterministicPendingStateCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::ProveNonDeterministicPendingState(
                    decoded,
                ));
            }
            if let Ok(decoded) =
                <RenounceOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::RenounceOwnership(decoded));
            }
            if let Ok(decoded) =
                <RollupVerifierCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::RollupVerifier(decoded));
            }
            if let Ok(decoded) =
                <SequenceBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SequenceBatches(decoded));
            }
            if let Ok(decoded) =
                <SequenceForceBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SequenceForceBatches(decoded));
            }
            if let Ok(decoded) =
                <SequencedBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SequencedBatches(decoded));
            }
            if let Ok(decoded) =
                <SetAdminCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetAdmin(decoded));
            }
            if let Ok(decoded) =
                <SetForceBatchAllowedCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetForceBatchAllowed(decoded));
            }
            if let Ok(decoded) =
                <SetMultiplierBatchFeeCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetMultiplierBatchFee(decoded));
            }
            if let Ok(decoded) =
                <SetPendingStateTimeoutCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetPendingStateTimeout(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetTrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedAggregatorTimeoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::SetTrustedAggregatorTimeout(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetTrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <SetTrustedSequencerURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetTrustedSequencerURL(decoded));
            }
            if let Ok(decoded) =
                <SetVeryBatchTimeTargetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::SetVeryBatchTimeTarget(decoded));
            }
            if let Ok(decoded) =
                <TransferOwnershipCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::TransferOwnership(decoded));
            }
            if let Ok(decoded) =
                <TrustedAggregatorCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::TrustedAggregator(decoded));
            }
            if let Ok(decoded) =
                <TrustedAggregatorTimeoutCall as ethers::core::abi::AbiDecode>::decode(
                    data.as_ref(),
                )
            {
                return Ok(PolygonZkEVMCalls::TrustedAggregatorTimeout(decoded));
            }
            if let Ok(decoded) =
                <TrustedSequencerCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::TrustedSequencer(decoded));
            }
            if let Ok(decoded) =
                <TrustedSequencerURLCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::TrustedSequencerURL(decoded));
            }
            if let Ok(decoded) =
                <TrustedVerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::TrustedVerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <VerifyBatchesCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::VerifyBatches(decoded));
            }
            if let Ok(decoded) =
                <VeryBatchTimeTargetCall as ethers::core::abi::AbiDecode>::decode(data.as_ref())
            {
                return Ok(PolygonZkEVMCalls::VeryBatchTimeTarget(decoded));
            }
            Err(ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ethers::core::abi::AbiEncode for PolygonZkEVMCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                PolygonZkEVMCalls::ForceBatchTimeout(element) => element.encode(),
                PolygonZkEVMCalls::HaltAggregationTimeout(element) => element.encode(),
                PolygonZkEVMCalls::MaxBatchMultiplier(element) => element.encode(),
                PolygonZkEVMCalls::MaxTransactionsByteLength(element) => element.encode(),
                PolygonZkEVMCalls::MaxVerifyBatches(element) => element.encode(),
                PolygonZkEVMCalls::ActivateEmergencyState(element) => element.encode(),
                PolygonZkEVMCalls::Admin(element) => element.encode(),
                PolygonZkEVMCalls::BatchFee(element) => element.encode(),
                PolygonZkEVMCalls::BatchNumToStateRoot(element) => element.encode(),
                PolygonZkEVMCalls::BridgeAddress(element) => element.encode(),
                PolygonZkEVMCalls::CalculateRewardPerBatch(element) => element.encode(),
                PolygonZkEVMCalls::ChainID(element) => element.encode(),
                PolygonZkEVMCalls::ConsolidatePendingState(element) => element.encode(),
                PolygonZkEVMCalls::DeactivateEmergencyState(element) => element.encode(),
                PolygonZkEVMCalls::ForceBatch(element) => element.encode(),
                PolygonZkEVMCalls::ForceBatchAllowed(element) => element.encode(),
                PolygonZkEVMCalls::ForcedBatches(element) => element.encode(),
                PolygonZkEVMCalls::GetCurrentBatchFee(element) => element.encode(),
                PolygonZkEVMCalls::GetInputSnarkBytes(element) => element.encode(),
                PolygonZkEVMCalls::GetLastVerifiedBatch(element) => element.encode(),
                PolygonZkEVMCalls::GlobalExitRootManager(element) => element.encode(),
                PolygonZkEVMCalls::Initialize(element) => element.encode(),
                PolygonZkEVMCalls::IsEmergencyState(element) => element.encode(),
                PolygonZkEVMCalls::IsPendingStateConsolidable(element) => element.encode(),
                PolygonZkEVMCalls::LastBatchSequenced(element) => element.encode(),
                PolygonZkEVMCalls::LastForceBatch(element) => element.encode(),
                PolygonZkEVMCalls::LastForceBatchSequenced(element) => element.encode(),
                PolygonZkEVMCalls::LastPendingState(element) => element.encode(),
                PolygonZkEVMCalls::LastPendingStateConsolidated(element) => element.encode(),
                PolygonZkEVMCalls::LastTimestamp(element) => element.encode(),
                PolygonZkEVMCalls::LastVerifiedBatch(element) => element.encode(),
                PolygonZkEVMCalls::Matic(element) => element.encode(),
                PolygonZkEVMCalls::MultiplierBatchFee(element) => element.encode(),
                PolygonZkEVMCalls::NetworkName(element) => element.encode(),
                PolygonZkEVMCalls::OverridePendingState(element) => element.encode(),
                PolygonZkEVMCalls::Owner(element) => element.encode(),
                PolygonZkEVMCalls::PendingStateTimeout(element) => element.encode(),
                PolygonZkEVMCalls::PendingStateTransitions(element) => element.encode(),
                PolygonZkEVMCalls::ProveNonDeterministicPendingState(element) => element.encode(),
                PolygonZkEVMCalls::RenounceOwnership(element) => element.encode(),
                PolygonZkEVMCalls::RollupVerifier(element) => element.encode(),
                PolygonZkEVMCalls::SequenceBatches(element) => element.encode(),
                PolygonZkEVMCalls::SequenceForceBatches(element) => element.encode(),
                PolygonZkEVMCalls::SequencedBatches(element) => element.encode(),
                PolygonZkEVMCalls::SetAdmin(element) => element.encode(),
                PolygonZkEVMCalls::SetForceBatchAllowed(element) => element.encode(),
                PolygonZkEVMCalls::SetMultiplierBatchFee(element) => element.encode(),
                PolygonZkEVMCalls::SetPendingStateTimeout(element) => element.encode(),
                PolygonZkEVMCalls::SetTrustedAggregator(element) => element.encode(),
                PolygonZkEVMCalls::SetTrustedAggregatorTimeout(element) => element.encode(),
                PolygonZkEVMCalls::SetTrustedSequencer(element) => element.encode(),
                PolygonZkEVMCalls::SetTrustedSequencerURL(element) => element.encode(),
                PolygonZkEVMCalls::SetVeryBatchTimeTarget(element) => element.encode(),
                PolygonZkEVMCalls::TransferOwnership(element) => element.encode(),
                PolygonZkEVMCalls::TrustedAggregator(element) => element.encode(),
                PolygonZkEVMCalls::TrustedAggregatorTimeout(element) => element.encode(),
                PolygonZkEVMCalls::TrustedSequencer(element) => element.encode(),
                PolygonZkEVMCalls::TrustedSequencerURL(element) => element.encode(),
                PolygonZkEVMCalls::TrustedVerifyBatches(element) => element.encode(),
                PolygonZkEVMCalls::VerifyBatches(element) => element.encode(),
                PolygonZkEVMCalls::VeryBatchTimeTarget(element) => element.encode(),
            }
        }
    }
    impl ::std::fmt::Display for PolygonZkEVMCalls {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
            match self {
                PolygonZkEVMCalls::ForceBatchTimeout(element) => element.fmt(f),
                PolygonZkEVMCalls::HaltAggregationTimeout(element) => element.fmt(f),
                PolygonZkEVMCalls::MaxBatchMultiplier(element) => element.fmt(f),
                PolygonZkEVMCalls::MaxTransactionsByteLength(element) => element.fmt(f),
                PolygonZkEVMCalls::MaxVerifyBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::ActivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMCalls::Admin(element) => element.fmt(f),
                PolygonZkEVMCalls::BatchFee(element) => element.fmt(f),
                PolygonZkEVMCalls::BatchNumToStateRoot(element) => element.fmt(f),
                PolygonZkEVMCalls::BridgeAddress(element) => element.fmt(f),
                PolygonZkEVMCalls::CalculateRewardPerBatch(element) => element.fmt(f),
                PolygonZkEVMCalls::ChainID(element) => element.fmt(f),
                PolygonZkEVMCalls::ConsolidatePendingState(element) => element.fmt(f),
                PolygonZkEVMCalls::DeactivateEmergencyState(element) => element.fmt(f),
                PolygonZkEVMCalls::ForceBatch(element) => element.fmt(f),
                PolygonZkEVMCalls::ForceBatchAllowed(element) => element.fmt(f),
                PolygonZkEVMCalls::ForcedBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::GetCurrentBatchFee(element) => element.fmt(f),
                PolygonZkEVMCalls::GetInputSnarkBytes(element) => element.fmt(f),
                PolygonZkEVMCalls::GetLastVerifiedBatch(element) => element.fmt(f),
                PolygonZkEVMCalls::GlobalExitRootManager(element) => element.fmt(f),
                PolygonZkEVMCalls::Initialize(element) => element.fmt(f),
                PolygonZkEVMCalls::IsEmergencyState(element) => element.fmt(f),
                PolygonZkEVMCalls::IsPendingStateConsolidable(element) => element.fmt(f),
                PolygonZkEVMCalls::LastBatchSequenced(element) => element.fmt(f),
                PolygonZkEVMCalls::LastForceBatch(element) => element.fmt(f),
                PolygonZkEVMCalls::LastForceBatchSequenced(element) => element.fmt(f),
                PolygonZkEVMCalls::LastPendingState(element) => element.fmt(f),
                PolygonZkEVMCalls::LastPendingStateConsolidated(element) => element.fmt(f),
                PolygonZkEVMCalls::LastTimestamp(element) => element.fmt(f),
                PolygonZkEVMCalls::LastVerifiedBatch(element) => element.fmt(f),
                PolygonZkEVMCalls::Matic(element) => element.fmt(f),
                PolygonZkEVMCalls::MultiplierBatchFee(element) => element.fmt(f),
                PolygonZkEVMCalls::NetworkName(element) => element.fmt(f),
                PolygonZkEVMCalls::OverridePendingState(element) => element.fmt(f),
                PolygonZkEVMCalls::Owner(element) => element.fmt(f),
                PolygonZkEVMCalls::PendingStateTimeout(element) => element.fmt(f),
                PolygonZkEVMCalls::PendingStateTransitions(element) => element.fmt(f),
                PolygonZkEVMCalls::ProveNonDeterministicPendingState(element) => element.fmt(f),
                PolygonZkEVMCalls::RenounceOwnership(element) => element.fmt(f),
                PolygonZkEVMCalls::RollupVerifier(element) => element.fmt(f),
                PolygonZkEVMCalls::SequenceBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::SequenceForceBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::SequencedBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::SetAdmin(element) => element.fmt(f),
                PolygonZkEVMCalls::SetForceBatchAllowed(element) => element.fmt(f),
                PolygonZkEVMCalls::SetMultiplierBatchFee(element) => element.fmt(f),
                PolygonZkEVMCalls::SetPendingStateTimeout(element) => element.fmt(f),
                PolygonZkEVMCalls::SetTrustedAggregator(element) => element.fmt(f),
                PolygonZkEVMCalls::SetTrustedAggregatorTimeout(element) => element.fmt(f),
                PolygonZkEVMCalls::SetTrustedSequencer(element) => element.fmt(f),
                PolygonZkEVMCalls::SetTrustedSequencerURL(element) => element.fmt(f),
                PolygonZkEVMCalls::SetVeryBatchTimeTarget(element) => element.fmt(f),
                PolygonZkEVMCalls::TransferOwnership(element) => element.fmt(f),
                PolygonZkEVMCalls::TrustedAggregator(element) => element.fmt(f),
                PolygonZkEVMCalls::TrustedAggregatorTimeout(element) => element.fmt(f),
                PolygonZkEVMCalls::TrustedSequencer(element) => element.fmt(f),
                PolygonZkEVMCalls::TrustedSequencerURL(element) => element.fmt(f),
                PolygonZkEVMCalls::TrustedVerifyBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::VerifyBatches(element) => element.fmt(f),
                PolygonZkEVMCalls::VeryBatchTimeTarget(element) => element.fmt(f),
            }
        }
    }
    impl ::std::convert::From<ForceBatchTimeoutCall> for PolygonZkEVMCalls {
        fn from(var: ForceBatchTimeoutCall) -> Self {
            PolygonZkEVMCalls::ForceBatchTimeout(var)
        }
    }
    impl ::std::convert::From<HaltAggregationTimeoutCall> for PolygonZkEVMCalls {
        fn from(var: HaltAggregationTimeoutCall) -> Self {
            PolygonZkEVMCalls::HaltAggregationTimeout(var)
        }
    }
    impl ::std::convert::From<MaxBatchMultiplierCall> for PolygonZkEVMCalls {
        fn from(var: MaxBatchMultiplierCall) -> Self {
            PolygonZkEVMCalls::MaxBatchMultiplier(var)
        }
    }
    impl ::std::convert::From<MaxTransactionsByteLengthCall> for PolygonZkEVMCalls {
        fn from(var: MaxTransactionsByteLengthCall) -> Self {
            PolygonZkEVMCalls::MaxTransactionsByteLength(var)
        }
    }
    impl ::std::convert::From<MaxVerifyBatchesCall> for PolygonZkEVMCalls {
        fn from(var: MaxVerifyBatchesCall) -> Self {
            PolygonZkEVMCalls::MaxVerifyBatches(var)
        }
    }
    impl ::std::convert::From<ActivateEmergencyStateCall> for PolygonZkEVMCalls {
        fn from(var: ActivateEmergencyStateCall) -> Self {
            PolygonZkEVMCalls::ActivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<AdminCall> for PolygonZkEVMCalls {
        fn from(var: AdminCall) -> Self {
            PolygonZkEVMCalls::Admin(var)
        }
    }
    impl ::std::convert::From<BatchFeeCall> for PolygonZkEVMCalls {
        fn from(var: BatchFeeCall) -> Self {
            PolygonZkEVMCalls::BatchFee(var)
        }
    }
    impl ::std::convert::From<BatchNumToStateRootCall> for PolygonZkEVMCalls {
        fn from(var: BatchNumToStateRootCall) -> Self {
            PolygonZkEVMCalls::BatchNumToStateRoot(var)
        }
    }
    impl ::std::convert::From<BridgeAddressCall> for PolygonZkEVMCalls {
        fn from(var: BridgeAddressCall) -> Self {
            PolygonZkEVMCalls::BridgeAddress(var)
        }
    }
    impl ::std::convert::From<CalculateRewardPerBatchCall> for PolygonZkEVMCalls {
        fn from(var: CalculateRewardPerBatchCall) -> Self {
            PolygonZkEVMCalls::CalculateRewardPerBatch(var)
        }
    }
    impl ::std::convert::From<ChainIDCall> for PolygonZkEVMCalls {
        fn from(var: ChainIDCall) -> Self {
            PolygonZkEVMCalls::ChainID(var)
        }
    }
    impl ::std::convert::From<ConsolidatePendingStateCall> for PolygonZkEVMCalls {
        fn from(var: ConsolidatePendingStateCall) -> Self {
            PolygonZkEVMCalls::ConsolidatePendingState(var)
        }
    }
    impl ::std::convert::From<DeactivateEmergencyStateCall> for PolygonZkEVMCalls {
        fn from(var: DeactivateEmergencyStateCall) -> Self {
            PolygonZkEVMCalls::DeactivateEmergencyState(var)
        }
    }
    impl ::std::convert::From<ForceBatchCall> for PolygonZkEVMCalls {
        fn from(var: ForceBatchCall) -> Self {
            PolygonZkEVMCalls::ForceBatch(var)
        }
    }
    impl ::std::convert::From<ForceBatchAllowedCall> for PolygonZkEVMCalls {
        fn from(var: ForceBatchAllowedCall) -> Self {
            PolygonZkEVMCalls::ForceBatchAllowed(var)
        }
    }
    impl ::std::convert::From<ForcedBatchesCall> for PolygonZkEVMCalls {
        fn from(var: ForcedBatchesCall) -> Self {
            PolygonZkEVMCalls::ForcedBatches(var)
        }
    }
    impl ::std::convert::From<GetCurrentBatchFeeCall> for PolygonZkEVMCalls {
        fn from(var: GetCurrentBatchFeeCall) -> Self {
            PolygonZkEVMCalls::GetCurrentBatchFee(var)
        }
    }
    impl ::std::convert::From<GetInputSnarkBytesCall> for PolygonZkEVMCalls {
        fn from(var: GetInputSnarkBytesCall) -> Self {
            PolygonZkEVMCalls::GetInputSnarkBytes(var)
        }
    }
    impl ::std::convert::From<GetLastVerifiedBatchCall> for PolygonZkEVMCalls {
        fn from(var: GetLastVerifiedBatchCall) -> Self {
            PolygonZkEVMCalls::GetLastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<GlobalExitRootManagerCall> for PolygonZkEVMCalls {
        fn from(var: GlobalExitRootManagerCall) -> Self {
            PolygonZkEVMCalls::GlobalExitRootManager(var)
        }
    }
    impl ::std::convert::From<InitializeCall> for PolygonZkEVMCalls {
        fn from(var: InitializeCall) -> Self {
            PolygonZkEVMCalls::Initialize(var)
        }
    }
    impl ::std::convert::From<IsEmergencyStateCall> for PolygonZkEVMCalls {
        fn from(var: IsEmergencyStateCall) -> Self {
            PolygonZkEVMCalls::IsEmergencyState(var)
        }
    }
    impl ::std::convert::From<IsPendingStateConsolidableCall> for PolygonZkEVMCalls {
        fn from(var: IsPendingStateConsolidableCall) -> Self {
            PolygonZkEVMCalls::IsPendingStateConsolidable(var)
        }
    }
    impl ::std::convert::From<LastBatchSequencedCall> for PolygonZkEVMCalls {
        fn from(var: LastBatchSequencedCall) -> Self {
            PolygonZkEVMCalls::LastBatchSequenced(var)
        }
    }
    impl ::std::convert::From<LastForceBatchCall> for PolygonZkEVMCalls {
        fn from(var: LastForceBatchCall) -> Self {
            PolygonZkEVMCalls::LastForceBatch(var)
        }
    }
    impl ::std::convert::From<LastForceBatchSequencedCall> for PolygonZkEVMCalls {
        fn from(var: LastForceBatchSequencedCall) -> Self {
            PolygonZkEVMCalls::LastForceBatchSequenced(var)
        }
    }
    impl ::std::convert::From<LastPendingStateCall> for PolygonZkEVMCalls {
        fn from(var: LastPendingStateCall) -> Self {
            PolygonZkEVMCalls::LastPendingState(var)
        }
    }
    impl ::std::convert::From<LastPendingStateConsolidatedCall> for PolygonZkEVMCalls {
        fn from(var: LastPendingStateConsolidatedCall) -> Self {
            PolygonZkEVMCalls::LastPendingStateConsolidated(var)
        }
    }
    impl ::std::convert::From<LastTimestampCall> for PolygonZkEVMCalls {
        fn from(var: LastTimestampCall) -> Self {
            PolygonZkEVMCalls::LastTimestamp(var)
        }
    }
    impl ::std::convert::From<LastVerifiedBatchCall> for PolygonZkEVMCalls {
        fn from(var: LastVerifiedBatchCall) -> Self {
            PolygonZkEVMCalls::LastVerifiedBatch(var)
        }
    }
    impl ::std::convert::From<MaticCall> for PolygonZkEVMCalls {
        fn from(var: MaticCall) -> Self {
            PolygonZkEVMCalls::Matic(var)
        }
    }
    impl ::std::convert::From<MultiplierBatchFeeCall> for PolygonZkEVMCalls {
        fn from(var: MultiplierBatchFeeCall) -> Self {
            PolygonZkEVMCalls::MultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<NetworkNameCall> for PolygonZkEVMCalls {
        fn from(var: NetworkNameCall) -> Self {
            PolygonZkEVMCalls::NetworkName(var)
        }
    }
    impl ::std::convert::From<OverridePendingStateCall> for PolygonZkEVMCalls {
        fn from(var: OverridePendingStateCall) -> Self {
            PolygonZkEVMCalls::OverridePendingState(var)
        }
    }
    impl ::std::convert::From<OwnerCall> for PolygonZkEVMCalls {
        fn from(var: OwnerCall) -> Self {
            PolygonZkEVMCalls::Owner(var)
        }
    }
    impl ::std::convert::From<PendingStateTimeoutCall> for PolygonZkEVMCalls {
        fn from(var: PendingStateTimeoutCall) -> Self {
            PolygonZkEVMCalls::PendingStateTimeout(var)
        }
    }
    impl ::std::convert::From<PendingStateTransitionsCall> for PolygonZkEVMCalls {
        fn from(var: PendingStateTransitionsCall) -> Self {
            PolygonZkEVMCalls::PendingStateTransitions(var)
        }
    }
    impl ::std::convert::From<ProveNonDeterministicPendingStateCall> for PolygonZkEVMCalls {
        fn from(var: ProveNonDeterministicPendingStateCall) -> Self {
            PolygonZkEVMCalls::ProveNonDeterministicPendingState(var)
        }
    }
    impl ::std::convert::From<RenounceOwnershipCall> for PolygonZkEVMCalls {
        fn from(var: RenounceOwnershipCall) -> Self {
            PolygonZkEVMCalls::RenounceOwnership(var)
        }
    }
    impl ::std::convert::From<RollupVerifierCall> for PolygonZkEVMCalls {
        fn from(var: RollupVerifierCall) -> Self {
            PolygonZkEVMCalls::RollupVerifier(var)
        }
    }
    impl ::std::convert::From<SequenceBatchesCall> for PolygonZkEVMCalls {
        fn from(var: SequenceBatchesCall) -> Self {
            PolygonZkEVMCalls::SequenceBatches(var)
        }
    }
    impl ::std::convert::From<SequenceForceBatchesCall> for PolygonZkEVMCalls {
        fn from(var: SequenceForceBatchesCall) -> Self {
            PolygonZkEVMCalls::SequenceForceBatches(var)
        }
    }
    impl ::std::convert::From<SequencedBatchesCall> for PolygonZkEVMCalls {
        fn from(var: SequencedBatchesCall) -> Self {
            PolygonZkEVMCalls::SequencedBatches(var)
        }
    }
    impl ::std::convert::From<SetAdminCall> for PolygonZkEVMCalls {
        fn from(var: SetAdminCall) -> Self {
            PolygonZkEVMCalls::SetAdmin(var)
        }
    }
    impl ::std::convert::From<SetForceBatchAllowedCall> for PolygonZkEVMCalls {
        fn from(var: SetForceBatchAllowedCall) -> Self {
            PolygonZkEVMCalls::SetForceBatchAllowed(var)
        }
    }
    impl ::std::convert::From<SetMultiplierBatchFeeCall> for PolygonZkEVMCalls {
        fn from(var: SetMultiplierBatchFeeCall) -> Self {
            PolygonZkEVMCalls::SetMultiplierBatchFee(var)
        }
    }
    impl ::std::convert::From<SetPendingStateTimeoutCall> for PolygonZkEVMCalls {
        fn from(var: SetPendingStateTimeoutCall) -> Self {
            PolygonZkEVMCalls::SetPendingStateTimeout(var)
        }
    }
    impl ::std::convert::From<SetTrustedAggregatorCall> for PolygonZkEVMCalls {
        fn from(var: SetTrustedAggregatorCall) -> Self {
            PolygonZkEVMCalls::SetTrustedAggregator(var)
        }
    }
    impl ::std::convert::From<SetTrustedAggregatorTimeoutCall> for PolygonZkEVMCalls {
        fn from(var: SetTrustedAggregatorTimeoutCall) -> Self {
            PolygonZkEVMCalls::SetTrustedAggregatorTimeout(var)
        }
    }
    impl ::std::convert::From<SetTrustedSequencerCall> for PolygonZkEVMCalls {
        fn from(var: SetTrustedSequencerCall) -> Self {
            PolygonZkEVMCalls::SetTrustedSequencer(var)
        }
    }
    impl ::std::convert::From<SetTrustedSequencerURLCall> for PolygonZkEVMCalls {
        fn from(var: SetTrustedSequencerURLCall) -> Self {
            PolygonZkEVMCalls::SetTrustedSequencerURL(var)
        }
    }
    impl ::std::convert::From<SetVeryBatchTimeTargetCall> for PolygonZkEVMCalls {
        fn from(var: SetVeryBatchTimeTargetCall) -> Self {
            PolygonZkEVMCalls::SetVeryBatchTimeTarget(var)
        }
    }
    impl ::std::convert::From<TransferOwnershipCall> for PolygonZkEVMCalls {
        fn from(var: TransferOwnershipCall) -> Self {
            PolygonZkEVMCalls::TransferOwnership(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorCall> for PolygonZkEVMCalls {
        fn from(var: TrustedAggregatorCall) -> Self {
            PolygonZkEVMCalls::TrustedAggregator(var)
        }
    }
    impl ::std::convert::From<TrustedAggregatorTimeoutCall> for PolygonZkEVMCalls {
        fn from(var: TrustedAggregatorTimeoutCall) -> Self {
            PolygonZkEVMCalls::TrustedAggregatorTimeout(var)
        }
    }
    impl ::std::convert::From<TrustedSequencerCall> for PolygonZkEVMCalls {
        fn from(var: TrustedSequencerCall) -> Self {
            PolygonZkEVMCalls::TrustedSequencer(var)
        }
    }
    impl ::std::convert::From<TrustedSequencerURLCall> for PolygonZkEVMCalls {
        fn from(var: TrustedSequencerURLCall) -> Self {
            PolygonZkEVMCalls::TrustedSequencerURL(var)
        }
    }
    impl ::std::convert::From<TrustedVerifyBatchesCall> for PolygonZkEVMCalls {
        fn from(var: TrustedVerifyBatchesCall) -> Self {
            PolygonZkEVMCalls::TrustedVerifyBatches(var)
        }
    }
    impl ::std::convert::From<VerifyBatchesCall> for PolygonZkEVMCalls {
        fn from(var: VerifyBatchesCall) -> Self {
            PolygonZkEVMCalls::VerifyBatches(var)
        }
    }
    impl ::std::convert::From<VeryBatchTimeTargetCall> for PolygonZkEVMCalls {
        fn from(var: VeryBatchTimeTargetCall) -> Self {
            PolygonZkEVMCalls::VeryBatchTimeTarget(var)
        }
    }
    #[doc = "Container type for all return fields from the `FORCE_BATCH_TIMEOUT` function with signature `FORCE_BATCH_TIMEOUT()` and selector `[171, 159, 197, 239]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForceBatchTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `HALT_AGGREGATION_TIMEOUT` function with signature `HALT_AGGREGATION_TIMEOUT()` and selector `[139, 72, 147, 30]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct HaltAggregationTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `MAX_BATCH_MULTIPLIER` function with signature `MAX_BATCH_MULTIPLIER()` and selector `[158, 184, 49, 185]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxBatchMultiplierReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_TRANSACTIONS_BYTE_LENGTH` function with signature `MAX_TRANSACTIONS_BYTE_LENGTH()` and selector `[45, 8, 137, 211]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxTransactionsByteLengthReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `MAX_VERIFY_BATCHES` function with signature `MAX_VERIFY_BATCHES()` and selector `[226, 23, 207, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaxVerifyBatchesReturn(pub u64);
    #[doc = "Container type for all return fields from the `admin` function with signature `admin()` and selector `[248, 81, 164, 64]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct AdminReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `batchFee` function with signature `batchFee()` and selector `[248, 184, 35, 228]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BatchFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `batchNumToStateRoot` function with signature `batchNumToStateRoot(uint64)` and selector `[83, 146, 197, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BatchNumToStateRootReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `bridgeAddress` function with signature `bridgeAddress()` and selector `[163, 197, 115, 235]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct BridgeAddressReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `calculateRewardPerBatch` function with signature `calculateRewardPerBatch()` and selector `[153, 245, 99, 78]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct CalculateRewardPerBatchReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `chainID` function with signature `chainID()` and selector `[173, 200, 121, 233]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ChainIDReturn(pub u64);
    #[doc = "Container type for all return fields from the `forceBatchAllowed` function with signature `forceBatchAllowed()` and selector `[216, 245, 77, 176]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForceBatchAllowedReturn(pub bool);
    #[doc = "Container type for all return fields from the `forcedBatches` function with signature `forcedBatches(uint64)` and selector `[107, 134, 22, 206]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct ForcedBatchesReturn(pub [u8; 32]);
    #[doc = "Container type for all return fields from the `getCurrentBatchFee` function with signature `getCurrentBatchFee()` and selector `[159, 13, 3, 157]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetCurrentBatchFeeReturn(pub ethers::core::types::U256);
    #[doc = "Container type for all return fields from the `getInputSnarkBytes` function with signature `getInputSnarkBytes(uint64,uint64,bytes32,bytes32,bytes32)` and selector `[34, 13, 120, 153]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetInputSnarkBytesReturn(pub ethers::core::types::Bytes);
    #[doc = "Container type for all return fields from the `getLastVerifiedBatch` function with signature `getLastVerifiedBatch()` and selector `[192, 237, 132, 224]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GetLastVerifiedBatchReturn(pub u64);
    #[doc = "Container type for all return fields from the `globalExitRootManager` function with signature `globalExitRootManager()` and selector `[208, 33, 3, 202]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct GlobalExitRootManagerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `isEmergencyState` function with signature `isEmergencyState()` and selector `[21, 6, 76, 150]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsEmergencyStateReturn(pub bool);
    #[doc = "Container type for all return fields from the `isPendingStateConsolidable` function with signature `isPendingStateConsolidable(uint64)` and selector `[56, 59, 59, 232]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct IsPendingStateConsolidableReturn(pub bool);
    #[doc = "Container type for all return fields from the `lastBatchSequenced` function with signature `lastBatchSequenced()` and selector `[66, 63, 168, 86]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastBatchSequencedReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastForceBatch` function with signature `lastForceBatch()` and selector `[231, 167, 237, 2]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastForceBatchReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastForceBatchSequenced` function with signature `lastForceBatchSequenced()` and selector `[69, 96, 82, 103]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastForceBatchSequencedReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastPendingState` function with signature `lastPendingState()` and selector `[69, 140, 4, 119]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastPendingStateReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastPendingStateConsolidated` function with signature `lastPendingStateConsolidated()` and selector `[74, 26, 137, 167]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastPendingStateConsolidatedReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastTimestamp` function with signature `lastTimestamp()` and selector `[25, 216, 172, 97]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastTimestampReturn(pub u64);
    #[doc = "Container type for all return fields from the `lastVerifiedBatch` function with signature `lastVerifiedBatch()` and selector `[127, 203, 54, 83]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct LastVerifiedBatchReturn(pub u64);
    #[doc = "Container type for all return fields from the `matic` function with signature `matic()` and selector `[182, 176, 176, 151]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MaticReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `multiplierBatchFee` function with signature `multiplierBatchFee()` and selector `[175, 210, 60, 190]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct MultiplierBatchFeeReturn(pub u16);
    #[doc = "Container type for all return fields from the `networkName` function with signature `networkName()` and selector `[16, 123, 242, 140]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct NetworkNameReturn(pub String);
    #[doc = "Container type for all return fields from the `owner` function with signature `owner()` and selector `[141, 165, 203, 91]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct OwnerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `pendingStateTimeout` function with signature `pendingStateTimeout()` and selector `[217, 57, 179, 21]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingStateTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `pendingStateTransitions` function with signature `pendingStateTransitions(uint256)` and selector `[131, 122, 71, 56]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct PendingStateTransitionsReturn {
        pub timestamp: u64,
        pub last_verified_batch: u64,
        pub exit_root: [u8; 32],
        pub state_root: [u8; 32],
    }
    #[doc = "Container type for all return fields from the `rollupVerifier` function with signature `rollupVerifier()` and selector `[232, 191, 146, 237]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct RollupVerifierReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `sequencedBatches` function with signature `sequencedBatches(uint64)` and selector `[180, 214, 63, 88]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct SequencedBatchesReturn {
        pub acc_input_hash: [u8; 32],
        pub sequenced_timestamp: u64,
        pub previous_last_batch_sequenced: u64,
    }
    #[doc = "Container type for all return fields from the `trustedAggregator` function with signature `trustedAggregator()` and selector `[41, 135, 137, 131]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedAggregatorReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `trustedAggregatorTimeout` function with signature `trustedAggregatorTimeout()` and selector `[132, 27, 36, 215]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedAggregatorTimeoutReturn(pub u64);
    #[doc = "Container type for all return fields from the `trustedSequencer` function with signature `trustedSequencer()` and selector `[207, 168, 237, 71]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedSequencerReturn(pub ethers::core::types::Address);
    #[doc = "Container type for all return fields from the `trustedSequencerURL` function with signature `trustedSequencerURL()` and selector `[84, 32, 40, 213]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct TrustedSequencerURLReturn(pub String);
    #[doc = "Container type for all return fields from the `veryBatchTimeTarget` function with signature `veryBatchTimeTarget()` and selector `[170, 88, 186, 214]`"]
    #[derive(
        Clone,
        Debug,
        Eq,
        PartialEq,
        ethers :: contract :: EthAbiType,
        ethers :: contract :: EthAbiCodec,
        Default,
    )]
    pub struct VeryBatchTimeTargetReturn(pub u64);
}