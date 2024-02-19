#[cfg(test)]
mod tests {
    use ethers::providers::{Http, Provider};
    use frames_core::provider::farcaster::FarcasterProvider;

    #[tokio::test]
    async fn it_get_custody_address_by_fid() {
        let provider = Provider::<Http>::try_from("https://mainnet.optimism.io").unwrap();
        let farecaster_provider = FarcasterProvider::new(provider);

        let address = farecaster_provider.get_custody_address_by_fid(1).await;
        assert_eq!(
            address.unwrap(),
            Some(
                "0x8773442740C17C9d0F0B87022c722F9a136206eD".parse().expect("Parse Address Error")
            )
        );
    }
}
