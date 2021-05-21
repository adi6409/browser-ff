//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamLength.idl
//


/// `interface nsIInputStreamLength : nsISupports`
///

/// ```text
/// /**
///  * Note: Instead of using these interfaces directly, consider to use
///  * InputStreamLengthHelper class.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamLength {
    vtable: *const nsIInputStreamLengthVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamLength.
unsafe impl XpCom for nsIInputStreamLength {
    const IID: nsIID = nsID(0x452d059f, 0x9a9c, 0x4434,
        [0x88, 0x39, 0xe1, 0x0d, 0x14, 0x05, 0x64, 0x7c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamLength {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamLength.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamLengthCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamLength`.
    fn coerce_from(v: &nsIInputStreamLength) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamLengthCoerce for nsIInputStreamLength {
    #[inline]
    fn coerce_from(v: &nsIInputStreamLength) -> &Self {
        v
    }
}

impl nsIInputStreamLength {
    /// Cast this `nsIInputStreamLength` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamLengthCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamLength {
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
impl<T: nsISupportsCoerce> nsIInputStreamLengthCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamLength) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamLength
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamLengthVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* long long length (); */
    pub Length: unsafe extern "system" fn (this: *const nsIInputStreamLength, _retval: *mut i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamLength {

    /// ```text
    /// /**
    ///    * Returns the total length of the stream if known. Otherwise it returns -1.
    ///    * This is different than calling available() which returns the number of
    ///    * bytes ready to be read from the stream.
    ///    * -1 is a valid value for a stream that doesn't know its length. For
    ///    * instance, a pipe stream could return such value.
    ///    *
    ///    * It could throw NS_BASE_STREAM_WOULD_BLOCK if the inputStream is
    ///    * non-blocking. If this happens, you should use
    ///    * nsIAsyncInputStreamLength::asyncLengthWait().
    ///    *
    ///    * If the stream has already been read (read()/readSegments()/close()/seek()
        ///    * methods has been called), length() returns NS_ERROR_NOT_AVAILABLE.
    ///    *
    ///    * This is not an attribute because a stream can change its length. For
    ///    * instance, if the stream is a file inputStream and the underlying OS file
    ///    * changes, its length will change as well.
    ///    */
    /// ```
    ///

    /// `long long length ();`
    #[inline]
    pub unsafe fn Length(&self, _retval: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).Length)(self, _retval)
    }


}


/// `interface nsIAsyncInputStreamLength : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncInputStreamLength {
    vtable: *const nsIAsyncInputStreamLengthVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncInputStreamLength.
unsafe impl XpCom for nsIAsyncInputStreamLength {
    const IID: nsIID = nsID(0xb63f9ecf, 0x4668, 0x44a3,
        [0x93, 0xbd, 0x72, 0xdb, 0xc6, 0x5a, 0x61, 0x25]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncInputStreamLength {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncInputStreamLength.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncInputStreamLengthCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncInputStreamLength`.
    fn coerce_from(v: &nsIAsyncInputStreamLength) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncInputStreamLengthCoerce for nsIAsyncInputStreamLength {
    #[inline]
    fn coerce_from(v: &nsIAsyncInputStreamLength) -> &Self {
        v
    }
}

impl nsIAsyncInputStreamLength {
    /// Cast this `nsIAsyncInputStreamLength` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncInputStreamLengthCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncInputStreamLength {
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
impl<T: nsISupportsCoerce> nsIAsyncInputStreamLengthCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncInputStreamLength) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncInputStreamLength
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncInputStreamLengthVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void asyncLengthWait (in nsIInputStreamLengthCallback aCallback, in nsIEventTarget aEventTarget); */
    pub AsyncLengthWait: unsafe extern "system" fn (this: *const nsIAsyncInputStreamLength, aCallback: *const nsIInputStreamLengthCallback, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncInputStreamLength {

    /// ```text
    /// /**
    ///    * If the stream is non-blocking, nsIInputStreamLength::length() can return
    ///    * NS_BASE_STREAM_WOULD_BLOCK. The caller must then wait for the stream to
    ///    * know its length.
    ///    *
    ///    * If the stream implements nsIAsyncInputStreamLength, then the caller can
    ///    * use this interface to request an asynchronous notification when the
    ///    * stream's length becomes known (via the AsyncLengthWait method).
    ///    * If the length is already known, the aCallback will be still called
    ///    * asynchronously.
    ///    *
    ///    * If the stream has already been read (read()/readSegments()/close()/seek()
        ///    * methods has been called), length() returns NS_ERROR_NOT_AVAILABLE.
    ///    *
    ///    * @param aCallback
    ///    *        This object is notified when the length becomes known. This
    ///    *        parameter may be null to clear an existing callback.
    ///    * @param aEventTarget
    ///    *        Specify that the notification must be delivered to a specific event
    ///    *        target.
    ///    */
    /// ```
    ///

    /// `void asyncLengthWait (in nsIInputStreamLengthCallback aCallback, in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn AsyncLengthWait(&self, aCallback: *const nsIInputStreamLengthCallback, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).AsyncLengthWait)(self, aCallback, aEventTarget)
    }


}


/// `interface nsIInputStreamLengthCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputStreamLengthCallback {
    vtable: *const nsIInputStreamLengthCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputStreamLengthCallback.
unsafe impl XpCom for nsIInputStreamLengthCallback {
    const IID: nsIID = nsID(0x9c0c13b9, 0x1b33, 0x445d,
        [0x8a, 0xdb, 0xa8, 0xa7, 0x86, 0x6a, 0x6c, 0x06]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputStreamLengthCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputStreamLengthCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputStreamLengthCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIInputStreamLengthCallback`.
    fn coerce_from(v: &nsIInputStreamLengthCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputStreamLengthCallbackCoerce for nsIInputStreamLengthCallback {
    #[inline]
    fn coerce_from(v: &nsIInputStreamLengthCallback) -> &Self {
        v
    }
}

impl nsIInputStreamLengthCallback {
    /// Cast this `nsIInputStreamLengthCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputStreamLengthCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputStreamLengthCallback {
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
impl<T: nsISupportsCoerce> nsIInputStreamLengthCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputStreamLengthCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputStreamLengthCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputStreamLengthCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onInputStreamLengthReady (in nsIAsyncInputStreamLength aStream, in long long aLength); */
    pub OnInputStreamLengthReady: unsafe extern "system" fn (this: *const nsIInputStreamLengthCallback, aStream: *const nsIAsyncInputStreamLength, aLength: i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputStreamLengthCallback {

    /// ```text
    /// /**
    ///  * This is a companion interface for
    ///  * nsIAsyncInputStreamLength::asyncLengthWait.
    ///  */
    /// /**
    ///    * Called to inform what the total length of the stream is.
    ///    *
    ///    * @param aStream
    ///    *        The stream whose asyncLengthWait method was called.
    ///    * @param aLength
    ///    *        The stream's length. It can be -1 if the stream doesn't know its
    ///    *        length. For instance, this can happen for a pipe inputStream.
    ///    */
    /// ```
    ///

    /// `void onInputStreamLengthReady (in nsIAsyncInputStreamLength aStream, in long long aLength);`
    #[inline]
    pub unsafe fn OnInputStreamLengthReady(&self, aStream: *const nsIAsyncInputStreamLength, aLength: i64) -> ::nserror::nsresult {
        ((*self.vtable).OnInputStreamLengthReady)(self, aStream, aLength)
    }


}


