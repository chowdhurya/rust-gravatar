extern crate gravatar;
use gravatar::{Default, Rating, Gravatar};

#[test]
fn test_base_url() {
    let g = Gravatar::new("email@example.com");
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e"
    );
}

#[test]
fn test_hash_procedure() {
    let g = Gravatar::new("  EMaiL@exAMplE.cOm ");
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e"
    );
}

#[test]
fn test_size() {
    let mut g = Gravatar::new("email@example.com");
    g.size = Some(50);
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
         ?s=50"
    );
}

#[test]
fn test_default() {
    let mut g = Gravatar::new("email@example.com");
    g.default = Some(Default::Wavatar);
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
         ?d=wavatar"
    );
}

#[test]
fn test_default_url() {
    let mut g = Gravatar::new("email@example.com");
    g.default = Some(Default::Url("http://example.org/im age".to_string()));
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
         ?d=http%3A%2F%2Fexample.org%2Fim+age"
    )
}

#[test]
fn test_force_default() {
    let mut g = Gravatar::new("email@example.com");
    g.force_default = true;
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
         ?f=y"
    );
}

#[test]
fn test_rating() {
    let mut g = Gravatar::new("email@example.com");
    g.rating = Some(Rating::R);
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
         ?r=r"
    );
}

#[test]
fn test_ssl_off() {
    let mut g = Gravatar::new("email@example.com");
    g.ssl = false;
    assert_eq!(
        g.image_url(),
        "http://www.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e"
    );
}

#[test]
fn test_multiple_params() {
    let mut g = Gravatar::new("email@example.com");
    g.size = Some(200);
    g.default = Some(Default::Identicon);
    g.force_default = true;
    g.rating = Some(Rating::R);
    assert_eq!(
        g.image_url(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
         ?s=200&d=identicon&f=y&r=r"
    );
}
