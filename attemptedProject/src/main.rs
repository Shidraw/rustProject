fn main() {
    use rand::Rng;
    const BUZZWORDS: &[u8] = b"BlockchainBigDataCloudBitcoinDigital";
    const PASSWORD_LEN: usize = 10;
    let mut rng = rand::thread_rng();

    let my_new_client_project: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..BUZZWORDS.len());
            BUZZWORDS[idx] as char
        })
        .collect();

    println!("{:?}", my_new_client_project);
}
