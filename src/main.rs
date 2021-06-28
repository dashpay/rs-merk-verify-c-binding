use merk::verify;

fn main() {
    let ops = [3, 1, 5, 0, 1, 5];
    let hash = [
        6, 189, 52, 109, 141, 122, 22, 148, 79, 245, 104, 135, 5, 52, 111, 160, 37, 228, 109, 246,
        123, 215, 130, 95, 215, 226, 166, 136, 61, 174, 227, 43,
    ];

    let map = verify(&ops, hash).unwrap();

    assert_eq!(
        map.get(vec![5].as_slice()).unwrap().unwrap(),
        vec![5].as_slice()
    );
}
