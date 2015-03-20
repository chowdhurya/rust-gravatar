//! `rust-gravatar` is a small Rust library that generates Gravatar image URLs
//! based on the
//! [official Gravatar specification](http://en.gravatar.com/site/implement/images/).
//!
//! Example
//! --------
//! ```ignore
//! extern crate gravatar;
//! use gravatar::{Gravatar, Rating};
//!
//! let mut g = Gravatar::new("email@example.com");
//! g.size = Some(150);
//! g.rating = Some(Rating::R);
//! assert_eq!(
//!     g.image_url(),
//!     "https://secure.gravatar.com/avatar/5658ffccee7f0ebfda2b226238b1eb6e\
//!      ?s=150&rating=r"
//! );
//! ```

#![feature(collections, core)]

extern crate crypto;
extern crate url;

use crypto::md5::Md5;
use crypto::digest::Digest;
use url::Url;

/// The default image to display if the user's email does not have a Gravatar.
///
/// See http://en.gravatar.com/site/implement/images/#default-image.
#[derive(Clone,Debug)]
pub enum Default {
    /// The URL of an image file to display as the default.
    Url(String),

    /// Instead of loading an image, the Gravatar URL will return an
    /// HTTP 404 (File Not Found) response if the email is not found.
    Http404,

    /// A transparent PNG image.
    Blank,

    /// A simple, cartoon-style silhouetted outline of a person that does not
    /// vary by email hash.
    MysteryMan,

    /// A geometric pattern based on the email hash.
    Identicon,

    /// A "monster" with different colors, faces, etc. that are generated by the
    /// email hash.
    MonsterId,

    /// A face with different features and backgrounds, generated by the email
    /// hash.
    Wavatar,

    /// An 8-bit arcade-style pixelated face that is generated by the email
    /// hash.
    Retro
}

/// The maximum rating level for which Gravatar will show the user's image
/// instead of the specified default.
///
/// See http://en.gravatar.com/site/implement/images/#rating.
#[derive(Clone,Debug)]
pub enum Rating {
    /// Show "G"-rated images only.
    G,

    /// Show "PG"-rated images or lower only.
    Pg,

    /// Show "R"-rated images or lower only.
    R,

    /// Show all images, up to and including "X"-rated ones.
    X
}

/// Representation of a single Gravatar image URL.
#[derive(Clone,Debug)]
pub struct Gravatar {
    /// The email of the user whose Gravatar you are trying to get.
    pub email: String,

    /// The desired image size. If `None` is provided, then no size is passed to
    /// Gravatar, which will then use a default of 80px by 80px. Gravatar will
    /// only provide images between 1px and 2048px by size.
    ///
    /// For more information, see
    /// http://en.gravatar.com/site/implement/images/#size.
    ///
    /// *Default value*: `None`
    pub size: Option<u16>,

    /// The default image to use if the user does not have a Gravatar. If `None`
    /// is provided, then Gravatar returns a blue Gravatar logo. The default
    /// image can be either a URL or one of Gravatar's premade defaults.
    ///
    /// For more information, see
    /// http://en.gravatar.com/site/implement/images/#default-image.
    ///
    /// *Default value*: `None`
    pub default: Option<Default>,

    /// If force_default is set to true, then Gravatar will always return the
    /// specified default image, whether or not the user's email exists.
    ///
    /// For more information, see
    /// http://en.gravatar.com/site/implement/images/#force-default.
    ///
    /// *Default value*: `false`
    pub force_default: bool,

    /// The maximum rating level for which Gravatar will show the user's image.
    /// If `None` is provided, then Gravatar will only deliver "G"-rated images
    /// by default. If an image is at a higher rating level than the requested
    /// one, the default image is returned instead.
    ///
    /// For more information, see
    /// http://en.gravatar.com/site/implement/images/#rating.
    ///
    /// *Default value*: `None`
    pub rating: Option<Rating>,

    /// If true, Gravatar's secure URL (https://secure.gravatar.com/avatar/...)
    /// is used. Otherwise, the non-SSL website is used instead
    /// (http://www.gravatar.com/avatar/...).
    ///
    /// *Default value*: `true`
    pub ssl: bool
}

impl Gravatar {
    /// Creates a new Gravatar with the given email and default values for the
    /// other parameters.
    pub fn new(email: &str) -> Gravatar {
        Gravatar {
            email: email.to_string(),
            size: None,
            default: None,
            force_default: false,
            rating: None,
            ssl: true
        }
    }

    /// Returns the image URL of the user's Gravatar with all specified
    /// parameters.
    pub fn image_url(self: &Self) -> String {
        // Generate MD5 hash of email
        let hash = {
            let mut hasher = Md5::new();
            hasher.input_str(&self.email.trim().to_lowercase());
            hasher.result_str()
        };

        // Create base URL using the hash
        let base_url = format!(
            "{}.gravatar.com/avatar/{}",
            match self.ssl {
                true => "https://secure",
                false => "http://www"
            },
            hash
        );

        // Create a list of all optional parameter values
        let mut params: Vec<(String, String)> = Vec::new();

        match self.size {
            Some(ref s) => {
                params.push(("s".to_string(), s.to_string()));
            },
            None => {}
        }
        match self.default {
            Some(ref d) => {
                let val = match *d {
                    Default::Url(ref u) => u.as_slice(),
                    Default::Http404 => "404",
                    Default::MysteryMan => "mm",
                    Default::Identicon => "identicon",
                    Default::MonsterId => "monsterid",
                    Default::Wavatar => "wavatar",
                    Default::Retro => "retro",
                    Default::Blank => "blank",
                };
                params.push(("d".to_string(), val.to_string()));
            },
            None => {}
        }
        match self.force_default {
            true => params.push(("f".to_string(), "y".to_string())),
            false => {}
        }
        match self.rating {
            Some(ref r) => {
                let val = match *r {
                    Rating::G => "g",
                    Rating::Pg => "pg",
                    Rating::R => "r",
                    Rating::X => "x"
                };
                params.push(("r".to_string(), val.to_string()));
            },
            None => {}
        };

        // Encode base URL with parameters if there are any and return
        if params.len() == 0 {
            base_url
        } else {
            let mut url = Url::parse(&base_url).ok().unwrap();
            url.set_query_from_pairs(
                params.iter().map(
                    |&(ref k, ref v)| (k.as_slice(), v.as_slice())
                )
            );
            url.serialize()
        }
    }

}
