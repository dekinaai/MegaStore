use megastore_search::search::SearchIndex;
use megastore_search::data::load_sample_products;

#[test]
fn integration_demo() {
    let products = load_sample_products().unwrap();
    let mut idx = SearchIndex::new();
    for p in products.clone() {
        idx.add_product(p);
    }
    idx.link_products(1,2);
    let r = idx.search("megaphone");
    assert!(!r.is_empty());
    let recs = idx.recommend(1, 3);
    assert!(recs.len() >= 1);
}
