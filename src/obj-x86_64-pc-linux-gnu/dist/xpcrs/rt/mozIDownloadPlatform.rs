//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/downloads/mozIDownloadPlatform.idl
//


/// `interface mozIDownloadPlatform : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIDownloadPlatform {
    vtable: *const mozIDownloadPlatformVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIDownloadPlatform.
unsafe impl XpCom for mozIDownloadPlatform {
    const IID: nsIID = nsID(0x9f556e4a, 0xd9b3, 0x46c3,
        [0x9f, 0x8f, 0xd0, 0xdb, 0x1a, 0xc6, 0xc8, 0xc1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIDownloadPlatform {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIDownloadPlatform.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIDownloadPlatformCoerce {
    /// Cheaply cast a value of this type from a `mozIDownloadPlatform`.
    fn coerce_from(v: &mozIDownloadPlatform) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIDownloadPlatformCoerce for mozIDownloadPlatform {
    #[inline]
    fn coerce_from(v: &mozIDownloadPlatform) -> &Self {
        v
    }
}

impl mozIDownloadPlatform {
    /// Cast this `mozIDownloadPlatform` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIDownloadPlatformCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIDownloadPlatform {
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
impl<T: nsISupportsCoerce> mozIDownloadPlatformCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIDownloadPlatform) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIDownloadPlatform
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIDownloadPlatformVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] Promise downloadDone (in nsIURI aSource, in nsIURI aReferrer, in nsIFile aTarget, in ACString aContentType, in boolean aIsPrivate); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub DownloadDone: *const ::libc::c_void,

    /* unsigned long mapUrlToZone (in AString aURL); */
    pub MapUrlToZone: unsafe extern "system" fn (this: *const mozIDownloadPlatform, aURL: *const ::nsstring::nsAString, _retval: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIDownloadPlatform {
    /// ```text
    /// /**
    ///    * Security Zone constants. Used by mapUrlToZone().
    ///    */
    /// ```
    ///

    pub const ZONE_MY_COMPUTER: i64 = 0;


    pub const ZONE_INTRANET: i64 = 1;


    pub const ZONE_TRUSTED: i64 = 2;


    pub const ZONE_INTERNET: i64 = 3;


    pub const ZONE_RESTRICTED: i64 = 4;

    /// ```text
    /// /**
    ///    * Perform platform specific operations when a download is done.
    ///    *
    ///    *   Windows:
    ///    *     Add the download to the recent documents list
    ///    *     Set the file to be indexed for searching
    ///    *   Mac:
    ///    *     Bounce the downloads dock icon
    ///    *   GTK:
    ///    *     Add the download to the recent documents list
    ///    *     Save the source uri in the downloaded file's metadata
    ///    *   Android:
    ///    *     Scan media
    ///    *
    ///    * @param aSource
    ///    *        Source URI of the download
    ///    * @param aReferrer
    ///    *        Referrer URI of the download
    ///    * @param aTarget
    ///    *        Downloaded file
    ///    * @param aContentType
    ///    *        The source's content type
    ///    * @param aIsPrivate
    ///    *        True for private downloads
    ///    * @return Promise that resolves once operations have completed.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise downloadDone (in nsIURI aSource, in nsIURI aReferrer, in nsIFile aTarget, in ACString aContentType, in boolean aIsPrivate);`
    const _DownloadDone: () = ();

    /// ```text
    /// /**
    ///    * Proxy for IInternetSecurityManager::MapUrlToZone().
    ///    *
    ///    *   Windows only.
    ///    *
    ///    * @param aURL
    ///    *        URI of the download
    ///    * @return Security Zone corresponding to aURL.
    ///    */
    /// ```
    ///

    /// `unsigned long mapUrlToZone (in AString aURL);`
    #[inline]
    pub unsafe fn MapUrlToZone(&self, aURL: *const ::nsstring::nsAString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).MapUrlToZone)(self, aURL, _retval)
    }


}


