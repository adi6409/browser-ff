//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIncrementalDownload.idl
//


/// `interface nsIIncrementalDownload : nsIRequest`
///

/// ```text
/// /**
///  * An incremental download object attempts to fetch a file piecemeal over time
///  * in an effort to minimize network bandwidth usage.
///  *
///  * Canceling a background download does not cause the file on disk to be
///  * deleted.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIncrementalDownload {
    vtable: *const nsIIncrementalDownloadVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIncrementalDownload.
unsafe impl XpCom for nsIIncrementalDownload {
    const IID: nsIID = nsID(0x6687823f, 0x56c4, 0x461d,
        [0x93, 0xa1, 0x7f, 0x6c, 0xb7, 0xdf, 0xbf, 0xba]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIncrementalDownload {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIncrementalDownload.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIncrementalDownloadCoerce {
    /// Cheaply cast a value of this type from a `nsIIncrementalDownload`.
    fn coerce_from(v: &nsIIncrementalDownload) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIncrementalDownloadCoerce for nsIIncrementalDownload {
    #[inline]
    fn coerce_from(v: &nsIIncrementalDownload) -> &Self {
        v
    }
}

impl nsIIncrementalDownload {
    /// Cast this `nsIIncrementalDownload` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIncrementalDownloadCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIncrementalDownload {
    type Target = nsIRequest;
    #[inline]
    fn deref(&self) -> &nsIRequest {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestCoerce> nsIIncrementalDownloadCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIncrementalDownload) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIncrementalDownload
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIncrementalDownloadVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestVTable,

    /* void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds); */
    pub Init: unsafe extern "system" fn (this: *const nsIIncrementalDownload, uri: *const nsIURI, destination: *const nsIFile, chunkSize: i32, intervalInSeconds: i32) -> ::nserror::nsresult,

    /* readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsIIncrementalDownload, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIURI finalURI; */
    pub GetFinalURI: unsafe extern "system" fn (this: *const nsIIncrementalDownload, aFinalURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute nsIFile destination; */
    pub GetDestination: unsafe extern "system" fn (this: *const nsIIncrementalDownload, aDestination: *mut*const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute long long totalSize; */
    pub GetTotalSize: unsafe extern "system" fn (this: *const nsIIncrementalDownload, aTotalSize: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long currentSize; */
    pub GetCurrentSize: unsafe extern "system" fn (this: *const nsIIncrementalDownload, aCurrentSize: *mut i64) -> ::nserror::nsresult,

    /* void start (in nsIRequestObserver observer, in nsISupports ctxt); */
    pub Start: unsafe extern "system" fn (this: *const nsIIncrementalDownload, observer: *const nsIRequestObserver, ctxt: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIncrementalDownload {

    /// ```text
    /// /**
    ///    * Initialize the incremental download object.  If the destination file
    ///    * already exists, then only the remaining portion of the file will be
    ///    * fetched.
    ///    *
    ///    * NOTE: The downloader will create the destination file if it does not
    ///    * already exist.  It will create the file with the permissions 0600 if
    ///    * needed.  To affect the permissions of the file, consumers of this
    ///    * interface may create an empty file at the specified destination prior to
    ///    * starting the incremental download.
    ///    *
    ///    * NOTE: Since this class may create a temporary file at the specified
    ///    * destination, it is advisable for the consumer of this interface to specify
    ///    * a file name for the destination that would not tempt the user into
    ///    * double-clicking it.  For example, it might be wise to append a file
    ///    * extension like ".part" to the end of the destination to protect users from
    ///    * accidentally running "blah.exe" before it is a complete file.
    ///    *
    ///    * @param uri
    ///    *        The URI to fetch.
    ///    * @param destination
    ///    *        The location where the file is to be stored.
    ///    * @param chunkSize
    ///    *        The size of the chunks to fetch.  A non-positive value results in
    ///    *        the default chunk size being used.
    ///    * @param intervalInSeconds
    ///    *        The amount of time to wait between fetching chunks.  Pass a
    ///    *        negative to use the default interval, or 0 to fetch the remaining
    ///    *        part of the file in one chunk.
    ///    */
    /// ```
    ///

    /// `void init (in nsIURI uri, in nsIFile destination, in long chunkSize, in long intervalInSeconds);`
    #[inline]
    pub unsafe fn Init(&self, uri: *const nsIURI, destination: *const nsIFile, chunkSize: i32, intervalInSeconds: i32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, uri, destination, chunkSize, intervalInSeconds)
    }


    /// ```text
    /// /**
    ///    * The URI being fetched.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI URI;`
    #[inline]
    pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///    * The URI being fetched after any redirects have been followed.  This
    ///    * attribute is set just prior to calling OnStartRequest on the observer
    ///    * passed to the start method.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI finalURI;`
    #[inline]
    pub unsafe fn GetFinalURI(&self, aFinalURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetFinalURI)(self, aFinalURI)
    }


    /// ```text
    /// /**
    ///    * The file where the download is being written.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIFile destination;`
    #[inline]
    pub unsafe fn GetDestination(&self, aDestination: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetDestination)(self, aDestination)
    }


    /// ```text
    /// /**
    ///    * The total number of bytes for the requested file.  This attribute is set
    ///    * just prior to calling OnStartRequest on the observer passed to the start
    ///    * method.
    ///    *
    ///    * This attribute has a value of -1 if the total size is unknown.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long totalSize;`
    #[inline]
    pub unsafe fn GetTotalSize(&self, aTotalSize: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetTotalSize)(self, aTotalSize)
    }


    /// ```text
    /// /**
    ///    * The current number of bytes downloaded so far.  This attribute is set just
    ///    * prior to calling OnStartRequest on the observer passed to the start
    ///    * method.
    ///    *
    ///    * This attribute has a value of -1 if the current size is unknown.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long currentSize;`
    #[inline]
    pub unsafe fn GetCurrentSize(&self, aCurrentSize: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentSize)(self, aCurrentSize)
    }


    /// ```text
    /// /**
    ///    * Start the incremental download.
    ///    *
    ///    * @param observer
    ///    *        An observer to be notified of various events.  OnStartRequest is
    ///    *        called when finalURI and totalSize have been determined or when an
    ///    *        error occurs.  OnStopRequest is called when the file is completely
    ///    *        downloaded or when an error occurs.  If this object implements
    ///    *        nsIProgressEventSink, then its OnProgress method will be called as
    ///    *        data is written to the destination file.  If this object implements
    ///    *        nsIInterfaceRequestor, then it will be assigned as the underlying
    ///    *        channel's notification callbacks, which allows it to provide a
    ///    *        nsIAuthPrompt implementation if needed by the channel, for example.
    ///    * @param ctxt
    ///    *        User defined object forwarded to the observer's methods.
    ///    */
    /// ```
    ///

    /// `void start (in nsIRequestObserver observer, in nsISupports ctxt);`
    #[inline]
    pub unsafe fn Start(&self, observer: *const nsIRequestObserver, ctxt: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Start)(self, observer, ctxt)
    }


}


