//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/thumbnails/nsIPageThumbsStorageService.idl
//


/// `interface nsIPageThumbsStorageService : nsISupports`
///

/// ```text
/// /**
///  * A service which returns information about file paths where the
///  * screenshots for URLs are stored. These screenshots are used by the
///  * moz-page-thumb protocol
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPageThumbsStorageService {
    vtable: *const nsIPageThumbsStorageServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPageThumbsStorageService.
unsafe impl XpCom for nsIPageThumbsStorageService {
    const IID: nsIID = nsID(0x97943eec, 0x0e48, 0x49ef,
        [0xb7, 0xb7, 0xcf, 0x4a, 0xa0, 0x10, 0x9b, 0xb6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPageThumbsStorageService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPageThumbsStorageService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPageThumbsStorageServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPageThumbsStorageService`.
    fn coerce_from(v: &nsIPageThumbsStorageService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPageThumbsStorageServiceCoerce for nsIPageThumbsStorageService {
    #[inline]
    fn coerce_from(v: &nsIPageThumbsStorageService) -> &Self {
        v
    }
}

impl nsIPageThumbsStorageService {
    /// Cast this `nsIPageThumbsStorageService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPageThumbsStorageServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPageThumbsStorageService {
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
impl<T: nsISupportsCoerce> nsIPageThumbsStorageServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPageThumbsStorageService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPageThumbsStorageService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPageThumbsStorageServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString getLeafNameForURL (in AString aURL); */
    pub GetLeafNameForURL: unsafe extern "system" fn (this: *const nsIPageThumbsStorageService, aURL: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString path; */
    pub GetPath: unsafe extern "system" fn (this: *const nsIPageThumbsStorageService, aPath: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getFilePathForURL (in AString aURL); */
    pub GetFilePathForURL: unsafe extern "system" fn (this: *const nsIPageThumbsStorageService, aURL: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPageThumbsStorageService {

    /// ```text
    /// /**
    ///    * Returns the leaf name of the file containing the screenshot for a given URL
    ///    */
    /// ```
    ///

    /// `AString getLeafNameForURL (in AString aURL);`
    #[inline]
    pub unsafe fn GetLeafNameForURL(&self, aURL: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLeafNameForURL)(self, aURL, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the path where the thumbnails are stored
    ///    */
    /// ```
    ///

    /// `readonly attribute AString path;`
    #[inline]
    pub unsafe fn GetPath(&self, aPath: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPath)(self, aPath)
    }


    /// ```text
    /// /**
    ///    * Returns the full file path containing the screenshot for a given URL
    ///    */
    /// ```
    ///

    /// `AString getFilePathForURL (in AString aURL);`
    #[inline]
    pub unsafe fn GetFilePathForURL(&self, aURL: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFilePathForURL)(self, aURL, _retval)
    }


}


