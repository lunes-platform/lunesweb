use lunesweb::wallet::Wallet;

fn main() {
    let w = Wallet::new_seed(None, None, Some(0));

    dbg!(w);
}
