extern crate gravatar;
extern crate url;

use gravatar::{Default, Gravatar, Rating};
use url::Url;

#[test]
fn test_base_url() {
    let url = Gravatar::new("email@example.com").image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e"
    );
}

#[test]
fn test_hash_procedure() {
    let url = Gravatar::new("  EMaiL@exAMplE.cOm ").image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e"
    );
}

#[test]
fn test_size() {
    let url = Gravatar::new("email@example.com")
        .set_size(Some(50))
        .image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?s=50"
    );
}

#[test]
fn test_default() {
    let url = Gravatar::new("email@example.com")
        .set_default(Some(Default::Wavatar))
        .image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?d=wavatar"
    );
}

#[test]
fn test_default_url() {
    let mut g = Gravatar::new("email@example.com");
    g.set_default(Some(Default::ImageUrl(
        Url::parse("http://example.org/im age").unwrap(),
    )));
    assert_eq!(
        g.image_url().as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?d=http%3A%2F%2Fexample.org%2Fim%2520age"
    )
}

#[test]
fn test_force_default() {
    let url = Gravatar::new("email@example.com")
        .set_force_default(true)
        .image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?f=y"
    );
}

#[test]
fn test_rating() {
    let url = Gravatar::new("email@example.com")
        .set_rating(Some(Rating::R))
        .image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?r=r"
    );
}

#[test]
fn test_ssl_off() {
    let url = Gravatar::new("email@example.com")
        .set_ssl(false)
        .image_url();
    assert_eq!(
        url.as_str(),
        "http://www.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e"
    );
}

#[test]
fn test_multiple_params() {
    let url = Gravatar::new("email@example.com")
        .set_size(Some(200))
        .set_default(Some(Default::Identicon))
        .set_force_default(true)
        .set_rating(Some(Rating::R))
        .image_url();
    assert_eq!(
        url.as_str(),
        "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e?s=200&d=identicon&f=y&r=r"
    );
}
