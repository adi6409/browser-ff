//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsITaggingService.idl
//


/// `interface nsITaggingService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITaggingService {
    vtable: *const nsITaggingServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITaggingService.
unsafe impl XpCom for nsITaggingService {
    const IID: nsIID = nsID(0x9759bd0e, 0x78e2, 0x4421,
        [0x9e, 0xd1, 0xc6, 0x76, 0xe1, 0xaf, 0x35, 0x13]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITaggingService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITaggingService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITaggingServiceCoerce {
    /// Cheaply cast a value of this type from a `nsITaggingService`.
    fn coerce_from(v: &nsITaggingService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITaggingServiceCoerce for nsITaggingService {
    #[inline]
    fn coerce_from(v: &nsITaggingService) -> &Self {
        v
    }
}

impl nsITaggingService {
    /// Cast this `nsITaggingService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITaggingServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITaggingService {
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
impl<T: nsISupportsCoerce> nsITaggingServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITaggingService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITaggingService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITaggingServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void tagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
    pub TagURI: unsafe extern "system" fn (this: *const nsITaggingService, aURI: *const nsIURI, aTags: *const nsIVariant, aSource: u16) -> ::nserror::nsresult,

    /* void untagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource); */
    pub UntagURI: unsafe extern "system" fn (this: *const nsITaggingService, aURI: *const nsIURI, aTags: *const nsIVariant, aSource: u16) -> ::nserror::nsresult,

    /* Array<AString> getTagsForURI (in nsIURI aURI); */
    pub GetTagsForURI: unsafe extern "system" fn (this: *const nsITaggingService, aURI: *const nsIURI, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITaggingService {

    /// ```text
    /// /**
    ///    * Tags a URL with the given set of tags. Current tags set for the URL
    ///    * persist. Tags in aTags which are already set for the given URL are
    ///    * ignored.
    ///    *
    ///    * @param aURI
    ///    *        the URL to tag.
    ///    * @param aTags
    ///    *        Array of tags to set for the given URL.  Each element within the
    ///    *        array can be either a tag name (non-empty string) or a concrete
    ///    *        itemId of a tag container.
    ///    * @param [optional] aSource
    ///    *        A change source constant from nsINavBookmarksService::SOURCE_*.
    ///    *        Defaults to SOURCE_DEFAULT if omitted.
    ///    */
    /// ```
    ///

    /// `void tagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn TagURI(&self, aURI: *const nsIURI, aTags: *const nsIVariant, aSource: u16) -> ::nserror::nsresult {
        ((*self.vtable).TagURI)(self, aURI, aTags, aSource)
    }


    /// ```text
    /// /**
    ///    * Removes tags from a URL. Tags from aTags which are not set for the
    ///    * given URL are ignored.
    ///    *
    ///    * @param aURI
    ///    *        the URL to un-tag.
    ///    * @param aTags
    ///    *        Array of tags to unset.  Pass null to remove all tags from the given
    ///    *        url.  Each element within the array can be either a tag name
    ///    *        (non-empty string) or a concrete itemId of a tag container.
    ///    * @param [optional] aSource
    ///    *        A change source constant from nsINavBookmarksService::SOURCE_*.
    ///    *        Defaults to SOURCE_DEFAULT if omitted.
    ///    */
    /// ```
    ///

    /// `void untagURI (in nsIURI aURI, in nsIVariant aTags, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn UntagURI(&self, aURI: *const nsIURI, aTags: *const nsIVariant, aSource: u16) -> ::nserror::nsresult {
        ((*self.vtable).UntagURI)(self, aURI, aTags, aSource)
    }


    /// ```text
    /// /**
    ///    * Retrieves all tags set for the given URL.
    ///    *
    ///    * @param aURI
    ///    *        a URL.
    ///    * @returns array of tags (sorted by name).
    ///    */
    /// ```
    ///

    /// `Array<AString> getTagsForURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetTagsForURI(&self, aURI: *const nsIURI, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).GetTagsForURI)(self, aURI, _retval)
    }


}


