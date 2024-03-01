mod node;
mod network;
use node::Node;
fn main() {
    let id3 = String::from("13");
    let node = Node::new(network::get_local_ip().unwrap(),1337);
    node.print_info();
}
