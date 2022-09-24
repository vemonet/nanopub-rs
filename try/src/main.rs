use nanopub_rs::nanopub::Nanopub;

fn main() {
    let np = Nanopub {
        rdf: String::from("toast"),
    };
    println!("{}", np.get_rdf());
}