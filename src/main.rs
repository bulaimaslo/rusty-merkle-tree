use rusty_merkle_tree::MerkleTree;

fn main() {
    let transactions: Vec<String> = vec![
        String::from("alice -> bob -> 10"),
        String::from("bob -> carol -> 20"),
        String::from("carol -> dave -> 30"),
        String::from("dave -> eve -> 40"),
        String::from("eve -> frank -> 50"),
        String::from("frank -> gina -> 60"),
        String::from("gina -> hank -> 70"),
        String::from("hank -> ian -> 80"),
        String::from("ian -> jill -> 90"),
        String::from("jill -> kate -> 100"),
        String::from("kate -> luke -> 110"),
        String::from("luke -> mary -> 120"),
        String::from("mary -> nate -> 130"),
        String::from("nate -> olivia -> 140"),
        String::from("olivia -> paul -> 150"),
        String::from("paul -> quinn -> 160"),
        String::from("quinn -> rick -> 170"),
    ];

    let mut tree = MerkleTree::new();


    let slice = &transactions[0..3];
    tree.add_node(slice.to_vec());
    tree.add_node(transactions[3..6].to_vec());
    tree.add_node(transactions[6..9].to_vec());
    tree.add_node(transactions[9..12].to_vec());
    tree.print();
    println!("{}", tree.validate_tree());
}
