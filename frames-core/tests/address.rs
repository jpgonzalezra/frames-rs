#[cfg(test)]
mod tests {
    use ethers::providers::{Http, Provider};
    use frames_core::get_custody_address_by_fid;

    #[tokio::test]
    async fn it_get_custody_address_by_fid() {
        let provider = Provider::<Http>::try_from("https://mainnet.optimism.io").unwrap();
        let address = get_custody_address_by_fid(1, provider).await;
        assert_eq!(
            address.unwrap(),
            Some(
                "0x8773442740C17C9d0F0B87022c722F9a136206eD".parse().expect("Parse Address Error")
            )
        );
    }
}
