//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUploadChannel2.idl
//


/// `interface nsIUploadChannel2 : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUploadChannel2 {
    vtable: *const nsIUploadChannel2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUploadChannel2.
unsafe impl XpCom for nsIUploadChannel2 {
    const IID: nsIID = nsID(0x2f712b52, 0x19c5, 0x4e0c,
        [0x9e, 0x8f, 0xb5, 0xc7, 0xc3, 0xb6, 0x70, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUploadChannel2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUploadChannel2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUploadChannel2Coerce {
    /// Cheaply cast a value of this type from a `nsIUploadChannel2`.
    fn coerce_from(v: &nsIUploadChannel2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUploadChannel2Coerce for nsIUploadChannel2 {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel2) -> &Self {
        v
    }
}

impl nsIUploadChannel2 {
    /// Cast this `nsIUploadChannel2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUploadChannel2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUploadChannel2 {
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
impl<T: nsISupportsCoerce> nsIUploadChannel2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUploadChannel2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUploadChannel2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void explicitSetUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength, in ACString aMethod, in boolean aStreamHasHeaders); */
    pub ExplicitSetUploadStream: unsafe extern "system" fn (this: *const nsIUploadChannel2, aStream: *const nsIInputStream, aContentType: *const ::nsstring::nsACString, aContentLength: i64, aMethod: *const ::nsstring::nsACString, aStreamHasHeaders: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean uploadStreamHasHeaders; */
    pub GetUploadStreamHasHeaders: unsafe extern "system" fn (this: *const nsIUploadChannel2, aUploadStreamHasHeaders: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void ensureUploadStreamIsCloneable (in nsIRunnable aCallback); */
    pub EnsureUploadStreamIsCloneable: unsafe extern "system" fn (this: *const nsIUploadChannel2, aCallback: *const nsIRunnable) -> ::nserror::nsresult,

    /* [noscript] nsIInputStream cloneUploadStream (out long long aContentLength); */
    pub CloneUploadStream: unsafe extern "system" fn (this: *const nsIUploadChannel2, aContentLength: *mut i64, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUploadChannel2 {

    /// ```text
    /// /**
    ///      * Sets a stream to be uploaded by this channel with the specified
    ///      * Content-Type and Content-Length header values.
    ///      *
    ///      * Most implementations of this interface require that the stream:
    ///      *   (1) implement threadsafe addRef and release
    ///      *   (2) implement nsIInputStream::readSegments
    ///      *   (3) implement nsISeekableStream::seek
    ///      *
    ///      * @param aStream
    ///      *        The stream to be uploaded by this channel.
    ///      * @param aContentType
    ///      *        This value will replace any existing Content-Type
    ///      *        header on the HTTP request, regardless of whether
    ///      *        or not its empty.
    ///      * @param aContentLength
    ///      *        A value of -1 indicates that the length of the stream should be
    ///      *        determined by calling the stream's |available| method.
    ///      * @param aMethod
    ///      *        The HTTP request method to set on the stream.
    ///      * @param aStreamHasHeaders
    ///      *        True if the stream already contains headers for the HTTP request.
    ///      */
    /// ```
    ///

    /// `void explicitSetUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength, in ACString aMethod, in boolean aStreamHasHeaders);`
    #[inline]
    pub unsafe fn ExplicitSetUploadStream(&self, aStream: *const nsIInputStream, aContentType: *const ::nsstring::nsACString, aContentLength: i64, aMethod: *const ::nsstring::nsACString, aStreamHasHeaders: bool) -> ::nserror::nsresult {
        ((*self.vtable).ExplicitSetUploadStream)(self, aStream, aContentType, aContentLength, aMethod, aStreamHasHeaders)
    }


    /// ```text
    /// /**
    ///      * Value of aStreamHasHeaders from the last successful call to
    ///      * explicitSetUploadStream.  TRUE indicates the attached upload stream
    ///      * contains request headers.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean uploadStreamHasHeaders;`
    #[inline]
    pub unsafe fn GetUploadStreamHasHeaders(&self, aUploadStreamHasHeaders: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUploadStreamHasHeaders)(self, aUploadStreamHasHeaders)
    }


    /// ```text
    /// /**
    ///      * Ensure the upload stream, if any, is cloneable.  This may involve
    ///      * async copying, so a callback runnable must be provided.  It will
    ///      * invoked on the current thread when the upload stream is ready
    ///      * for cloning.  If the stream is already cloneable, then the callback
    ///      * will be invoked synchronously.
    ///      */
    /// ```
    ///

    /// `[noscript] void ensureUploadStreamIsCloneable (in nsIRunnable aCallback);`
    #[inline]
    pub unsafe fn EnsureUploadStreamIsCloneable(&self, aCallback: *const nsIRunnable) -> ::nserror::nsresult {
        ((*self.vtable).EnsureUploadStreamIsCloneable)(self, aCallback)
    }


    /// ```text
    /// /**
    ///      * Clones the upload stream.  May return failure if the upload stream
    ///      * is not cloneable.  If this is not acceptable, use the
    ///      * ensureUploadStreamIsCloneable() method first.
    ///      * aContentLength could be -1 in case the size of the stream is unknown,
    ///      * otherwise it will contain the known size of the stream.
    ///      */
    /// ```
    ///

    /// `[noscript] nsIInputStream cloneUploadStream (out long long aContentLength);`
    #[inline]
    pub unsafe fn CloneUploadStream(&self, aContentLength: *mut i64, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).CloneUploadStream)(self, aContentLength, _retval)
    }


}


