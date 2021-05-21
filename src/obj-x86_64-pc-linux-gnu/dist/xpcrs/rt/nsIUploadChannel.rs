//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIUploadChannel.idl
//


/// `interface nsIUploadChannel : nsISupports`
///

/// ```text
/// /**
///  * nsIUploadChannel
///  *
///  * A channel may optionally implement this interface if it supports the
///  * notion of uploading a data stream.  The upload stream may only be set
///  * prior to the invocation of asyncOpen on the channel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUploadChannel {
    vtable: *const nsIUploadChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUploadChannel.
unsafe impl XpCom for nsIUploadChannel {
    const IID: nsIID = nsID(0x5cfe15bd, 0x5adb, 0x4a7f,
        [0x9e, 0x55, 0x4f, 0x5a, 0x67, 0xd1, 0x57, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUploadChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUploadChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUploadChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIUploadChannel`.
    fn coerce_from(v: &nsIUploadChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUploadChannelCoerce for nsIUploadChannel {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel) -> &Self {
        v
    }
}

impl nsIUploadChannel {
    /// Cast this `nsIUploadChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUploadChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUploadChannel {
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
impl<T: nsISupportsCoerce> nsIUploadChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUploadChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUploadChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUploadChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength); */
    pub SetUploadStream: unsafe extern "system" fn (this: *const nsIUploadChannel, aStream: *const nsIInputStream, aContentType: *const ::nsstring::nsACString, aContentLength: i64) -> ::nserror::nsresult,

    /* readonly attribute nsIInputStream uploadStream; */
    pub GetUploadStream: unsafe extern "system" fn (this: *const nsIUploadChannel, aUploadStream: *mut*const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUploadChannel {

    /// ```text
    /// /**
    ///      * Sets a stream to be uploaded by this channel.
    ///      *
    ///      * Most implementations of this interface require that the stream:
    ///      *   (1) implement threadsafe addRef and release
    ///      *   (2) implement nsIInputStream::readSegments
    ///      *   (3) implement nsISeekableStream::seek
    ///      *
    ///      * History here is that we need to support both streams that already have
    ///      * headers (e.g., Content-Type and Content-Length) information prepended to
    ///      * the stream (by plugins) as well as clients (composer, uploading
        ///      * application) that want to upload data streams without any knowledge of
    ///      * protocol specifications.  For this reason, we have a special meaning
    ///      * for the aContentType parameter (see below).
    ///      *
    ///      * @param aStream
    ///      *        The stream to be uploaded by this channel.
    ///      * @param aContentType
    ///      *        If aContentType is empty, the protocol will assume that no
    ///      *        content headers are to be added to the uploaded stream and that
    ///      *        any required headers are already encoded in the stream.  In the
    ///      *        case of HTTP, if this parameter is non-empty, then its value will
    ///      *        replace any existing Content-Type header on the HTTP request.
    ///      *        In the case of FTP and FILE, this parameter is ignored.
    ///      * @param aContentLength
    ///      *        A value of -1 indicates that the length of the stream should be
    ///      *        determined by calling the stream's |available| method.
    ///      */
    /// ```
    ///

    /// `void setUploadStream (in nsIInputStream aStream, in ACString aContentType, in long long aContentLength);`
    #[inline]
    pub unsafe fn SetUploadStream(&self, aStream: *const nsIInputStream, aContentType: *const ::nsstring::nsACString, aContentLength: i64) -> ::nserror::nsresult {
        ((*self.vtable).SetUploadStream)(self, aStream, aContentType, aContentLength)
    }


    /// ```text
    /// /**
    ///      * Get the stream (to be) uploaded by this channel.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIInputStream uploadStream;`
    #[inline]
    pub unsafe fn GetUploadStream(&self, aUploadStream: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetUploadStream)(self, aUploadStream)
    }


}


