use sharks::{ Sharks, Share };

fn main() {
    // Set a minimum threshold of 10 shares
    let sharks = Sharks(2);

    // Obtain an iterator over the shares for secret [1, 2, 3, 4]
    let dealer = sharks.dealer(&[1, 2, 3, 4]);

    // Get 10 shares
    let shares: Vec<Share> = dealer.take(3).collect();

    // Recover the original secret!
    let secret = sharks.recover(shares.as_slice()).unwrap();
    assert_eq!(secret, vec![1, 2, 3, 4]);
}
