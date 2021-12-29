
use bs58;


pub fn main() {

    let alice_peer_id ="12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE";
    let alice_peerid_hex=alice_peer_id.as_bytes().to_vec();
    println!("{}",alice_peerid_hex.iter().map(|x|format!("{:02x?}",x)).collect::<Vec<_>>().join(""));



    let charlie = decode("12D3KooWJvyP3VJYymTqG7eH4PM5rN4T2agk5cdNCfNymAqwqcvZ");
    println!("charlie's:{}",charlie);
    let alice = decode("12D3KooWFzXtYJhUkMsWTXQodYaXhs6ah52xVExicuFPvmUQoZrE");
    println!("alice's:{}",alice);
    let bob = decode("12D3KooWFqHLgcSSB6dgAZ3SeCFYR4Dpr5w8TEhTdq8JKDxTUvGH");
    println!("bob's:{}",bob);

}

fn decode(str: &str) ->String{
    let bs58str = bs58::decode(str);
    let bs58String =bs58str.into_vec().unwrap();
    println!("{:?}",bs58String);
    // lower case
    println!("{:x?}", bs58String);
// upper case
    println!("{:X?}", bs58String);
    println!("{:02X?}", bs58String);
    println!("{:#04X?}", bs58String);

    let ss= bs58String.iter().map(|x|format!("{:02x?}",x)).collect::<Vec<_>>().join("");
    println!("ss:{}",ss);
    let data = [0x0, 0x1, 0xe, 0xf, 0xff];
// print the leading zero
    println!("{:02X?}", data);
// It can be combined with the pretty modifier as well
    println!("{:#04X?}", data);

    ss
}


