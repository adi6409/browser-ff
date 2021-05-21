//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDownloader.idl
//


/// `interface nsIDownloader : nsIStreamListener`
///

/// ```text
/// /**
///  * nsIDownloader
///  *
///  * A downloader is a special implementation of a nsIStreamListener that will
///  * make the contents of the stream available as a file.  This may utilize the
///  * disk cache as an optimization to avoid an extra copy of the data on disk.
///  * The resulting file is valid from the time the downloader completes until
///  * the last reference to the downloader is released.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDownloader {
    vtable: *const nsIDownloaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDownloader.
unsafe impl XpCom for nsIDownloader {
    const IID: nsIID = nsID(0xfafe41a9, 0xa531, 0x4d6d,
        [0x89, 0xbc, 0x58, 0x8a, 0x65, 0x22, 0xfb, 0x4e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDownloader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDownloader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDownloaderCoerce {
    /// Cheaply cast a value of this type from a `nsIDownloader`.
    fn coerce_from(v: &nsIDownloader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDownloaderCoerce for nsIDownloader {
    #[inline]
    fn coerce_from(v: &nsIDownloader) -> &Self {
        v
    }
}

impl nsIDownloader {
    /// Cast this `nsIDownloader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDownloaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDownloader {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStreamListenerCoerce> nsIDownloaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDownloader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDownloaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIDownloadObserver observer, in nsIFile downloadLocation); */
    pub Init: unsafe extern "system" fn (this: *const nsIDownloader, observer: *const nsIDownloadObserver, downloadLocation: *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDownloader {

    /// ```text
    /// /**
    ///      * Initialize this downloader
    ///      *
    ///      * @param observer
    ///      *        the observer to be notified when the download completes.
    ///      * @param downloadLocation
    ///      *        the location where the stream contents should be written.
    ///      *        if null, the downloader will select a location and the
    ///      *        resulting file will be deleted (or otherwise made invalid)
    ///      *        when the downloader object is destroyed.  if an explicit
    ///      *        download location is specified then the resulting file will
    ///      *        not be deleted, and it will be the callers responsibility
    ///      *        to keep track of the file, etc.
    ///      */
    /// ```
    ///

    /// `void init (in nsIDownloadObserver observer, in nsIFile downloadLocation);`
    #[inline]
    pub unsafe fn Init(&self, observer: *const nsIDownloadObserver, downloadLocation: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, observer, downloadLocation)
    }


}


/// `interface nsIDownloadObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDownloadObserver {
    vtable: *const nsIDownloadObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDownloadObserver.
unsafe impl XpCom for nsIDownloadObserver {
    const IID: nsIID = nsID(0x44b3153e, 0xa54e, 0x4077,
        [0xa5, 0x27, 0xb0, 0x32, 0x5e, 0x40, 0x92, 0x4e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDownloadObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDownloadObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDownloadObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIDownloadObserver`.
    fn coerce_from(v: &nsIDownloadObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDownloadObserverCoerce for nsIDownloadObserver {
    #[inline]
    fn coerce_from(v: &nsIDownloadObserver) -> &Self {
        v
    }
}

impl nsIDownloadObserver {
    /// Cast this `nsIDownloadObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDownloadObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDownloadObserver {
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
impl<T: nsISupportsCoerce> nsIDownloadObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDownloadObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDownloadObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDownloadObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onDownloadComplete (in nsIDownloader downloader, in nsIRequest request, in nsISupports ctxt, in nsresult status, in nsIFile result); */
    pub OnDownloadComplete: unsafe extern "system" fn (this: *const nsIDownloadObserver, downloader: *const nsIDownloader, request: *const nsIRequest, ctxt: *const nsISupports, status: ::nserror::nsresult, result: *const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDownloadObserver {

    /// ```text
    /// /**
    ///      * Called to signal a download that has completed.
    ///      */
    /// ```
    ///

    /// `void onDownloadComplete (in nsIDownloader downloader, in nsIRequest request, in nsISupports ctxt, in nsresult status, in nsIFile result);`
    #[inline]
    pub unsafe fn OnDownloadComplete(&self, downloader: *const nsIDownloader, request: *const nsIRequest, ctxt: *const nsISupports, status: ::nserror::nsresult, result: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).OnDownloadComplete)(self, downloader, request, ctxt, status, result)
    }


}


