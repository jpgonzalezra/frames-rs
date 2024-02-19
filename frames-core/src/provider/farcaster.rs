use std::sync::Arc;

use ethers::providers::Middleware;
use ethers::{
    abi::Abi,
    contract::Contract,
    types::{Address, U256},
};

struct FarcasterProvider<T: Middleware + 'static> {
    inner: T,
}

impl<T: Middleware + 'static> FarcasterProvider<T> {
    pub fn new(provider: T) -> Self {
        Self { inner: provider }
    }

    async fn get_custody_address_by_fid(
        &self,
        fid: usize,
    ) -> Result<Option<Address>, Box<dyn std::error::Error>> {
        let contract_address: Address =
            "0x00000000Fc6c5F01Fc30151999387Bb99A9f489b".parse().expect("Parse Address Error");
        let contract_abi: Abi = serde_json::from_str(
            r#"[{
            "inputs":[{"internalType":"uint256","name":"fid","type":"uint256"}],
            "name":"custodyOf",
            "outputs":[{"internalType":"address","name":"","type":"address"}],
            "stateMutability":"view","type":"function"
        }]"#,
        )?;

        let contract = Contract::new(contract_address, contract_abi, Arc::new(self.inner));
        match contract.method::<_, Address>("custodyOf", U256::from(fid)).unwrap().call().await {
            Ok(value) => Ok(Some(value)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
