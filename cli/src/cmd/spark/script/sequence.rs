use super::NestedValue;
use crate::cmd::spark::{
    init::get_commit_hash,
    script::{
        transaction::{wrapper, AdditionalContract, TransactionWithMetadata},
        verify::VerifyBundle,
    },
};
use corebc::{
    abi::Address,
    prelude::{artifacts::Libraries, ArtifactId, TransactionReceipt, TxHash},
    types::{transaction::eip2718::TypedTransaction, Network},
};
use eyre::{ContextCompat, WrapErr};
use foxar_common::{fs, shell, SELECTOR_LEN};
use foxar_config::Config;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, VecDeque},
    io::BufWriter,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::trace;
use yansi::Paint;

pub const DRY_RUN_DIR: &str = "dry-run";

/// Helper that saves the transactions sequence and its state on which transactions have been
/// broadcasted
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct ScriptSequence {
    pub transactions: VecDeque<TransactionWithMetadata>,
    #[serde(serialize_with = "wrapper::serialize_receipts")]
    pub receipts: Vec<TransactionReceipt>,
    pub libraries: Vec<String>,
    pub pending: Vec<TxHash>,
    #[serde(skip)]
    pub path: PathBuf,
    #[serde(skip)]
    pub sensitive_path: PathBuf,
    pub returns: HashMap<String, NestedValue>,
    pub timestamp: u64,
    pub network: Network,
    /// If `True`, the sequence belongs to a `MultiChainSequence` and won't save to disk as usual.
    pub multi: bool,
    pub commit: Option<String>,
}

/// Sensitive values from the transactions in a script sequence
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct SensitiveTransactionMetadata {
    pub rpc: Option<String>,
}

/// Sensitive info from the script sequence which is saved into the cache folder
#[derive(Deserialize, Serialize, Clone, Default)]
pub struct SensitiveScriptSequence {
    pub transactions: VecDeque<SensitiveTransactionMetadata>,
}

impl From<&mut ScriptSequence> for SensitiveScriptSequence {
    fn from(sequence: &mut ScriptSequence) -> Self {
        SensitiveScriptSequence {
            transactions: sequence
                .transactions
                .iter()
                .map(|tx| SensitiveTransactionMetadata { rpc: tx.rpc.clone() })
                .collect(),
        }
    }
}

impl ScriptSequence {
    pub fn new(
        transactions: VecDeque<TransactionWithMetadata>,
        returns: HashMap<String, NestedValue>,
        sig: &str,
        target: &ArtifactId,
        config: &Config,
        broadcasted: bool,
        is_multi: bool,
    ) -> eyre::Result<Self> {
        let chain = config.network_id.unwrap_or_default();

        let (path, sensitive_path) = ScriptSequence::get_paths(
            &config.broadcast,
            &config.cache_path,
            sig,
            target,
            chain,
            broadcasted && !is_multi,
        )?;

        let commit = get_commit_hash(&config.__root.0);

        Ok(ScriptSequence {
            transactions,
            returns,
            receipts: vec![],
            pending: vec![],
            path,
            sensitive_path,
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Wrong system time.")
                .as_secs(),
            libraries: vec![],
            network: chain,
            multi: is_multi,
            commit,
        })
    }

    /// Loads The sequence for the corresponding json file
    pub fn load(
        config: &Config,
        sig: &str,
        target: &ArtifactId,
        network_id: Network,
        broadcasted: bool,
    ) -> eyre::Result<Self> {
        let (path, sensitive_path) = ScriptSequence::get_paths(
            &config.broadcast,
            &config.cache_path,
            sig,
            target,
            network_id,
            broadcasted,
        )?;

        let mut script_sequence: Self = corebc::ylem::utils::read_json_file(&path)
            .wrap_err(format!("Deployment not found for network `{network_id}`."))?;

        let sensitive_script_sequence: SensitiveScriptSequence =
            corebc::ylem::utils::read_json_file(&sensitive_path).wrap_err(format!(
                "Deployment's sensitive details not found for network `{network_id}`."
            ))?;

        script_sequence
            .transactions
            .iter_mut()
            .enumerate()
            .for_each(|(i, tx)| tx.rpc.clone_from(&sensitive_script_sequence.transactions[i].rpc));

        script_sequence.path = path;
        script_sequence.sensitive_path = sensitive_path;

        Ok(script_sequence)
    }

    /// Saves the transactions as file if it's a standalone deployment.
    pub fn save(&mut self) -> eyre::Result<()> {
        if !self.multi && !self.transactions.is_empty() {
            self.timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();

            let sensitive_script_sequence: SensitiveScriptSequence = self.into();

            let path = self.path.to_string_lossy();
            let sensitive_path = self.sensitive_path.to_string_lossy();

            // broadcast folder writes
            //../run-latest.json
            serde_json::to_writer_pretty(BufWriter::new(fs::create_file(&self.path)?), &self)?;
            //../run-[timestamp].json
            serde_json::to_writer_pretty(
                BufWriter::new(fs::create_file(
                    path.replace("latest.json", &format!("{}.json", self.timestamp)),
                )?),
                &self,
            )?;

            // cache folder writes
            //../run-latest.json
            serde_json::to_writer_pretty(
                BufWriter::new(fs::create_file(&self.sensitive_path)?),
                &sensitive_script_sequence,
            )?;
            //../run-[timestamp].json
            serde_json::to_writer_pretty(
                BufWriter::new(fs::create_file(
                    sensitive_path.replace("latest.json", &format!("{}.json", self.timestamp)),
                )?),
                &sensitive_script_sequence,
            )?;

            shell::println(format!("\nTransactions saved to: {path}\n"))?;
            shell::println(format!("Sensitive values saved to: {sensitive_path}\n"))?;
        }

        Ok(())
    }

    pub fn add_receipt(&mut self, receipt: TransactionReceipt) {
        self.receipts.push(receipt);
    }

    /// Sorts all receipts with ascending transaction index
    pub fn sort_receipts(&mut self) {
        self.receipts.sort_unstable()
    }

    pub fn add_pending(&mut self, index: usize, tx_hash: TxHash) {
        if !self.pending.contains(&tx_hash) {
            self.transactions[index].hash = Some(tx_hash);
            self.pending.push(tx_hash);
        }
    }

    pub fn remove_pending(&mut self, tx_hash: TxHash) {
        self.pending.retain(|element| element != &tx_hash);
    }

    pub fn add_libraries(&mut self, libraries: Libraries) {
        self.libraries = libraries
            .libs
            .iter()
            .flat_map(|(file, libs)| {
                libs.iter()
                    .map(|(name, address)| format!("{}:{name}:{address}", file.to_string_lossy()))
            })
            .collect();
    }

    /// Gets paths in the formats
    /// ./broadcast/[contract_filename]/[chain_id]/[sig]-[timestamp].json and
    /// ./cache/[contract_filename]/[chain_id]/[sig]-[timestamp].json
    pub fn get_paths(
        broadcast: &Path,
        cache: &Path,
        sig: &str,
        target: &ArtifactId,
        chain_id: Network,
        broadcasted: bool,
    ) -> eyre::Result<(PathBuf, PathBuf)> {
        let mut broadcast = broadcast.to_path_buf();
        let mut cache = cache.to_path_buf();
        let mut common = PathBuf::new();

        let target_fname = target.source.file_name().wrap_err("No filename.")?;
        common.push(target_fname);
        common.push(chain_id.to_string());
        if !broadcasted {
            common.push(DRY_RUN_DIR);
        }

        broadcast.push(common.clone());
        cache.push(common);

        fs::create_dir_all(&broadcast)?;
        fs::create_dir_all(&cache)?;

        // TODO: ideally we want the name of the function here if sig is calldata
        let filename = sig_to_file_name(sig);

        broadcast.push(format!("{filename}-latest.json"));
        cache.push(format!("{filename}-latest.json"));

        Ok((broadcast, cache))
    }

    /// Given the broadcast log, it matches transactions with receipts, and tries to verify any
    /// created contract on etherscan.
    pub async fn verify_contracts(
        &mut self,
        _config: &Config,
        verify: VerifyBundle,
    ) -> eyre::Result<()> {
        trace!(target: "script", "verifying {} contracts [{}]", verify.known_contracts.len(), self.network);

        trace!(target: "script", "prepare future verifications");

        let mut future_verifications = Vec::with_capacity(self.receipts.len());
        let mut unverifiable_contracts = vec![];

        // Make sure the receipts have the right order first.
        self.sort_receipts();

        for (receipt, tx) in self.receipts.iter_mut().zip(self.transactions.iter()) {
            // create2 hash offset
            let mut offset = 0;

            if tx.is_create2() {
                receipt.contract_address = tx.contract_address;
                offset = 32;
            }

            // Verify contract created directly from the transaction
            if let (Some(address), Some(data)) = (receipt.contract_address, tx.typed_tx().data()) {
                match verify.get_verify_args(
                    address,
                    offset,
                    &data.0,
                    &self.libraries,
                    &self.network,
                ) {
                    Some(verify) => future_verifications.push(verify.run()),
                    None => unverifiable_contracts.push(address),
                };
            }

            // Verify potential contracts created during the transaction execution
            for AdditionalContract { address, init_code, .. } in &tx.additional_contracts {
                match verify.get_verify_args(*address, 0, init_code, &self.libraries, &self.network)
                {
                    Some(verify) => future_verifications.push(verify.run()),
                    None => unverifiable_contracts.push(*address),
                };
            }
        }

        trace!(target: "script", "collected {} verification jobs and {} unverifiable contracts", future_verifications.len(), unverifiable_contracts.len());

        self.check_unverified(unverifiable_contracts, verify);

        let num_verifications = future_verifications.len();
        println!("##\nStart verification for ({num_verifications}) contracts",);
        for verification in future_verifications {
            verification.await?;
        }

        println!("All ({num_verifications}) contracts were verified!");

        Ok(())
    }

    /// Let the user know if there are any contracts which can not be verified. Also, present some
    /// hints on potential causes.
    fn check_unverified(&self, unverifiable_contracts: Vec<Address>, verify: VerifyBundle) {
        if !unverifiable_contracts.is_empty() {
            println!(
                "\n{}",
                Paint::yellow(format!(
                    "We haven't found any matching bytecode for the following contracts: {:?}.\n\n{}",
                    unverifiable_contracts,
                    "This may occur when resuming a verification, but the underlying source code or compiler version has changed."
                ))
                .bold(),
            );

            if let Some(commit) = &self.commit {
                let current_commit = verify
                    .project_paths
                    .root
                    .map(|root| get_commit_hash(&root).unwrap_or_default())
                    .unwrap_or_default();

                if &current_commit != commit {
                    println!("\tScript was broadcasted on commit `{commit}`, but we are at `{current_commit}`.");
                }
            }
        }
    }

    /// Returns the list of the transactions without the metadata.
    pub fn typed_transactions(&self) -> Vec<(String, &TypedTransaction)> {
        self.transactions
            .iter()
            .map(|tx| {
                (tx.rpc.clone().expect("to have been filled with a proper rpc"), tx.typed_tx())
            })
            .collect()
    }
}

impl Drop for ScriptSequence {
    fn drop(&mut self) {
        self.sort_receipts();
        self.save().expect("not able to save deployment sequence");
    }
}

/// Converts the `sig` argument into the corresponding file path.
///
/// This accepts either the signature of the function or the raw calldata

fn sig_to_file_name(sig: &str) -> String {
    if let Some((name, _)) = sig.split_once('(') {
        // strip until call argument parenthesis
        return name.to_string();
    }
    // assume calldata if `sig` is hex
    if let Ok(calldata) = hex::decode(sig) {
        // in which case we return the function signature
        return hex::encode(&calldata[..SELECTOR_LEN]);
    }

    // return sig as is
    sig.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_convert_sig() {
        assert_eq!(sig_to_file_name("run()").as_str(), "run");
        assert_eq!(
            sig_to_file_name(
                "522bb70400000000000000000000cb58e5dd06163a480c22d540ec763325a0b5860fb56c"
            )
            .as_str(),
            "522bb704"
        );
    }
}
