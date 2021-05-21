//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/mozIPlacesAutoComplete.idl
//


/// `interface mozIPlacesAutoComplete : nsISupports`
///

/// ```text
/// /**
///  * This interface provides some constants used by the Places AutoComplete
///  * search provider as well as methods to track opened pages for AutoComplete
///  * purposes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIPlacesAutoComplete {
    vtable: *const mozIPlacesAutoCompleteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIPlacesAutoComplete.
unsafe impl XpCom for mozIPlacesAutoComplete {
    const IID: nsIID = nsID(0x61b6348a, 0x09e1, 0x4810,
        [0x80, 0x57, 0xf8, 0xcb, 0x3c, 0xec, 0x6e, 0xf8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIPlacesAutoComplete {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIPlacesAutoComplete.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIPlacesAutoCompleteCoerce {
    /// Cheaply cast a value of this type from a `mozIPlacesAutoComplete`.
    fn coerce_from(v: &mozIPlacesAutoComplete) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIPlacesAutoCompleteCoerce for mozIPlacesAutoComplete {
    #[inline]
    fn coerce_from(v: &mozIPlacesAutoComplete) -> &Self {
        v
    }
}

impl mozIPlacesAutoComplete {
    /// Cast this `mozIPlacesAutoComplete` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIPlacesAutoCompleteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIPlacesAutoComplete {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> mozIPlacesAutoCompleteCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPlacesAutoComplete) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIPlacesAutoComplete
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIPlacesAutoCompleteVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void populatePreloadedSiteStorage (in jsval sites); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub PopulatePreloadedSiteStorage: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIPlacesAutoComplete {
    /// ```text
    /// /**
    ///    * Match anywhere in each searchable term.
    ///    */
    /// ```
    ///

    pub const MATCH_ANYWHERE: i64 = 0;

    /// ```text
    /// /**
    ///    * Match first on word boundaries, and if we do not get enough results, then
    ///    * match anywhere in each searchable term.
    ///    */
    /// ```
    ///

    pub const MATCH_BOUNDARY_ANYWHERE: i64 = 1;

    /// ```text
    /// /**
    ///    * Match on word boundaries in each searchable term.
    ///    */
    /// ```
    ///

    pub const MATCH_BOUNDARY: i64 = 2;

    /// ```text
    /// /**
    ///    * Match only the beginning of each search term.
    ///    */
    /// ```
    ///

    pub const MATCH_BEGINNING: i64 = 3;

    /// ```text
    /// /**
    ///    * Match anywhere in each searchable term without doing any transformation
    ///    * or stripping on the underlying data.
    ///    */
    /// ```
    ///

    pub const MATCH_ANYWHERE_UNMODIFIED: i64 = 4;

    /// ```text
    /// /**
    ///    * Match only the beginning of each search term using a case sensitive
    ///    * comparator.
    ///    */
    /// ```
    ///

    pub const MATCH_BEGINNING_CASE_SENSITIVE: i64 = 5;

    /// ```text
    /// /**
    ///    * Search through history.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_HISTORY: i64 = 1;

    /// ```text
    /// /**
    ///    * Search though bookmarks.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_BOOKMARK: i64 = 2;

    /// ```text
    /// /**
    ///    * Search through tags.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_TAG: i64 = 4;

    /// ```text
    /// /**
    ///    * Search the title of pages.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_TITLE: i64 = 8;

    /// ```text
    /// /**
    ///    * Search the URL of pages.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_URL: i64 = 16;

    /// ```text
    /// /**
    ///    * Search for typed pages.
    ///    * No more supported by Firefox, it is still being used by comm-central clients.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_TYPED: i64 = 32;

    /// ```text
    /// /**
    ///    * Search javascript: URLs.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_JAVASCRIPT: i64 = 64;

    /// ```text
    /// /**
    ///    * Search for pages that have been marked as being opened, such as a tab
    ///    * in a tabbrowser.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_OPENPAGE: i64 = 128;

    /// ```text
    /// /**
    ///    * Use intersection between history, typed, bookmark, tag and openpage
    ///    * instead of union, when the restrict bit is set.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_RESTRICT: i64 = 256;

    /// ```text
    /// /**
    ///    * Include search suggestions from the currently selected search provider.
    ///    */
    /// ```
    ///

    pub const BEHAVIOR_SEARCH: i64 = 512;

    /// ```text
    /// /**
    ///    * Populate list of Preloaded Sites from JSON.
    ///    *
    ///    * @param sites
    ///    *        Array of [url,title] to populate from.
    ///    */
    /// ```
    ///

    /// `void populatePreloadedSiteStorage (in jsval sites);`
    const _PopulatePreloadedSiteStorage: () = ();

}


