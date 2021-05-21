//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHyperLink.idl
//


/// `interface nsIAccessibleHyperLink : nsISupports`
///

/// ```text
/// /**
///  * A cross-platform interface that supports hyperlink-specific properties and
///  * methods.  Anchors, image maps, xul:labels with class="text-link" implement this interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleHyperLink {
    vtable: *const nsIAccessibleHyperLinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleHyperLink.
unsafe impl XpCom for nsIAccessibleHyperLink {
    const IID: nsIID = nsID(0x883643d4, 0x93a5, 0x4f32,
        [0x92, 0x2c, 0x6f, 0x06, 0xe0, 0x13, 0x63, 0xc1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleHyperLink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleHyperLink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleHyperLinkCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleHyperLink`.
    fn coerce_from(v: &nsIAccessibleHyperLink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleHyperLinkCoerce for nsIAccessibleHyperLink {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperLink) -> &Self {
        v
    }
}

impl nsIAccessibleHyperLink {
    /// Cast this `nsIAccessibleHyperLink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleHyperLinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleHyperLink {
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
impl<T: nsISupportsCoerce> nsIAccessibleHyperLinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHyperLink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleHyperLink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleHyperLinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long startIndex; */
    pub GetStartIndex: unsafe extern "system" fn (this: *const nsIAccessibleHyperLink, aStartIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long endIndex; */
    pub GetEndIndex: unsafe extern "system" fn (this: *const nsIAccessibleHyperLink, aEndIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute boolean valid; */
    pub GetValid: unsafe extern "system" fn (this: *const nsIAccessibleHyperLink, aValid: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long anchorCount; */
    pub GetAnchorCount: unsafe extern "system" fn (this: *const nsIAccessibleHyperLink, aAnchorCount: *mut i32) -> ::nserror::nsresult,

    /* nsIURI getURI (in long index); */
    pub GetURI: unsafe extern "system" fn (this: *const nsIAccessibleHyperLink, index: i32, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* nsIAccessible getAnchor (in long index); */
    pub GetAnchor: unsafe extern "system" fn (this: *const nsIAccessibleHyperLink, index: i32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleHyperLink {

    /// ```text
    /// /**
    ///    * Returns the offset of the link within the parent accessible.
    ///    */
    /// ```
    ///

    /// `readonly attribute long startIndex;`
    #[inline]
    pub unsafe fn GetStartIndex(&self, aStartIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetStartIndex)(self, aStartIndex)
    }


    /// ```text
    /// /**
    ///    * Returns the end index of the link within the parent accessible.
    ///    *
    ///    * @note  The link itself is represented by one embedded character within the
    ///    * parent text, so the endIndex should be startIndex + 1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long endIndex;`
    #[inline]
    pub unsafe fn GetEndIndex(&self, aEndIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetEndIndex)(self, aEndIndex)
    }


    /// ```text
    /// /**
    ///    * Determines whether the link is valid (e. g. points to a valid URL).
    ///    *
    ///    * @note  XXX Currently only used with ARIA links, and the author has to
    ///    * specify that the link is invalid via the aria-invalid="true" attribute.
    ///    * In all other cases, TRUE is returned.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean valid;`
    #[inline]
    pub unsafe fn GetValid(&self, aValid: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetValid)(self, aValid)
    }


    /// ```text
    /// /**
    ///    * The numbber of anchors within this Hyperlink. Is normally 1 for anchors.
    ///    * This anchor is, for example, the visible output of the html:a tag.
    ///    * With an Image Map, reflects the actual areas within the map.
    ///    */
    /// ```
    ///

    /// `readonly attribute long anchorCount;`
    #[inline]
    pub unsafe fn GetAnchorCount(&self, aAnchorCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetAnchorCount)(self, aAnchorCount)
    }


    /// ```text
    /// /**
    ///    * Returns the URI at the given index.
    ///    *
    ///    * @note  ARIA hyperlinks do not have an URI to point to, since clicks are
    ///    * processed via JavaScript. Therefore this property does not work on ARIA
    ///    * links.
    ///    *
    ///    * @param index  The 0-based index of the URI to be returned.
    ///    *
    ///    * @return the nsIURI object containing the specifications for the URI.
    ///    */
    /// ```
    ///

    /// `nsIURI getURI (in long index);`
    #[inline]
    pub unsafe fn GetURI(&self, index: i32, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns a reference to the object at the given index.
    ///    *
    ///    * @param index  The 0-based index whose object is to be returned.
    ///    *
    ///    * @return the nsIAccessible object at the desired index.
    ///    */
    /// ```
    ///

    /// `nsIAccessible getAnchor (in long index);`
    #[inline]
    pub unsafe fn GetAnchor(&self, index: i32, _retval: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetAnchor)(self, index, _retval)
    }


}


