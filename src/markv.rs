extern crate markov;

use markov::Chain;

struct Aboba {
    chain: Chain<String>
}

impl Aboba {
    fn init(&mut self, setchain: &Chain<String>) {
        self.chain = *setchain;
    }
}

pub fn init(chain: &mut Chain<String>) {

}

pub fn add(chain: &mut Chain<String>, text: String) {
    chain.feed_str(&text);
}

fn gen() {
    let mut chain: Chain<String>;
}
