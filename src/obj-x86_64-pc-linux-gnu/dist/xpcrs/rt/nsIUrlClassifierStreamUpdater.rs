//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierStreamUpdater.idl
//


/// `interface nsIUrlClassifierStreamUpdater : nsISupports`
///

/// ```text
/// /**
///  * This is a class to manage large table updates from the server.  Rather than
///  * downloading the whole update and then updating the sqlite database, we
///  * update tables as the data is streaming in.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierStreamUpdater {
    vtable: *const nsIUrlClassifierStreamUpdaterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierStreamUpdater.
unsafe impl XpCom for nsIUrlClassifierStreamUpdater {
    const IID: nsIID = nsID(0xe1797597, 0xf4d6, 0x4dd3,
        [0xa1, 0xe1, 0x74, 0x5a, 0xd3, 0x52, 0xcd, 0x80]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierStreamUpdater {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierStreamUpdater.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierStreamUpdaterCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierStreamUpdater`.
    fn coerce_from(v: &nsIUrlClassifierStreamUpdater) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierStreamUpdaterCoerce for nsIUrlClassifierStreamUpdater {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierStreamUpdater) -> &Self {
        v
    }
}

impl nsIUrlClassifierStreamUpdater {
    /// Cast this `nsIUrlClassifierStreamUpdater` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierStreamUpdaterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierStreamUpdater {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierStreamUpdaterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierStreamUpdater) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierStreamUpdater
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierStreamUpdaterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean downloadUpdates (in ACString aRequestTables, in ACString aRequestPayload, in boolean aIsPostRequest, in ACString aUpdateUrl, in nsIUrlClassifierCallback aSuccessCallback, in nsIUrlClassifierCallback aUpdateErrorCallback, in nsIUrlClassifierCallback aDownloadErrorCallback); */
    pub DownloadUpdates: unsafe extern "system" fn (this: *const nsIUrlClassifierStreamUpdater, aRequestTables: *const ::nsstring::nsACString, aRequestPayload: *const ::nsstring::nsACString, aIsPostRequest: bool, aUpdateUrl: *const ::nsstring::nsACString, aSuccessCallback: *const nsIUrlClassifierCallback, aUpdateErrorCallback: *const nsIUrlClassifierCallback, aDownloadErrorCallback: *const nsIUrlClassifierCallback, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierStreamUpdater {

    /// ```text
    /// /**
    ///    * Try to download updates from updateUrl. If an update is already in
    ///    * progress, queues the requested update. This is used in nsIUrlListManager
    ///    * as well as in testing.
    ///    * @param aRequestTables Comma-separated list of tables included in this
    ///    *        update.
    ///    * @param aRequestPayload The payload for the request.
    ///    * @param aIsPostRequest Whether the request should be sent by POST method.
    ///    *                       Should be 'true' for v2 usage.
    ///    * @param aUpdateUrl The plaintext url from which to request updates.
    ///    * @param aSuccessCallback Called after a successful update.
    ///    * @param aUpdateErrorCallback Called for problems applying the update
    ///    * @param aDownloadErrorCallback Called if we get an http error or a
    ///    *        connection refused error.
    ///    */
    /// ```
    ///

    /// `boolean downloadUpdates (in ACString aRequestTables, in ACString aRequestPayload, in boolean aIsPostRequest, in ACString aUpdateUrl, in nsIUrlClassifierCallback aSuccessCallback, in nsIUrlClassifierCallback aUpdateErrorCallback, in nsIUrlClassifierCallback aDownloadErrorCallback);`
    #[inline]
    pub unsafe fn DownloadUpdates(&self, aRequestTables: *const ::nsstring::nsACString, aRequestPayload: *const ::nsstring::nsACString, aIsPostRequest: bool, aUpdateUrl: *const ::nsstring::nsACString, aSuccessCallback: *const nsIUrlClassifierCallback, aUpdateErrorCallback: *const nsIUrlClassifierCallback, aDownloadErrorCallback: *const nsIUrlClassifierCallback, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).DownloadUpdates)(self, aRequestTables, aRequestPayload, aIsPostRequest, aUpdateUrl, aSuccessCallback, aUpdateErrorCallback, aDownloadErrorCallback, _retval)
    }


}


