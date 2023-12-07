use std::hash::Hash;
use std::num::NonZeroUsize;

use ecow::EcoString;

use crate::engine::Engine;
use crate::foundations::{cast, func, scope, ty, Dict, Repr};
use crate::model::Numbering;

/// Identifies an element in the document.
///
/// A location uniquely identifies an element in the document and lets you
/// access its absolute position on the pages. You can retrieve the current
/// location with the [`locate`]($locate) function and the location of a queried
/// or shown element with the [`location()`]($content.location) method on
/// content.
#[ty(scope)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location(pub(super) u128);

impl Location {
    /// Create the initial location at the root of the hierarchy.
    pub fn root() -> Self {
        Location(0)
    }

    /// Produce a variant of this location.
    pub fn variant(self, n: usize) -> Self {
        Self(crate::util::hash128(&(self, n)))
    }
}

#[scope]
impl Location {
    /// Return the page number for this location.
    ///
    /// Note that this does not return the value of the [page counter]($counter)
    /// at this location, but the true page number (starting from one).
    ///
    /// If you want to know the value of the page counter, use
    /// `{counter(page).at(loc)}` instead.
    #[func]
    pub fn page(self, engine: &mut Engine) -> NonZeroUsize {
        engine.introspector.page(self)
    }

    /// Return a dictionary with the page number and the x, y position for this
    /// location. The page number starts at one and the coordinates are measured
    /// from the top-left of the page.
    ///
    /// If you only need the page number, use `page()` instead as it allows
    /// Typst to skip unnecessary work.
    #[func]
    pub fn position(self, engine: &mut Engine) -> Dict {
        engine.introspector.position(self).into()
    }

    /// Returns the page numbering pattern of the page at this location. This
    /// can be used when displaying the page counter in order to obtain the
    /// local numbering. This is useful if you are building custom indices or
    /// outlines.
    ///
    /// If the page numbering is set to `none` at that location, this function
    /// returns `none`.
    #[func]
    pub fn page_numbering(self, engine: &mut Engine) -> Option<Numbering> {
        engine.introspector.page_numbering(self).cloned()
    }
}

impl Repr for Location {
    fn repr(&self) -> EcoString {
        "..".into()
    }
}

cast! {
    type Location,
}

/// Makes this element locatable through `engine.locate`.
pub trait Locatable {}
