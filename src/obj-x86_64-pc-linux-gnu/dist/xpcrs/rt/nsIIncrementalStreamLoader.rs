//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIncrementalStreamLoader.idl
//


/// `interface nsIIncrementalStreamLoaderObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIncrementalStreamLoaderObserver {
    vtable: *const nsIIncrementalStreamLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIncrementalStreamLoaderObserver.
unsafe impl XpCom for nsIIncrementalStreamLoaderObserver {
    const IID: nsIID = nsID(0x07c3d2cc, 0x5454, 0x4618,
        [0x9f, 0x4f, 0xcd, 0x93, 0xde, 0x95, 0x04, 0xa4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIncrementalStreamLoaderObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIncrementalStreamLoaderObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIncrementalStreamLoaderObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIIncrementalStreamLoaderObserver`.
    fn coerce_from(v: &nsIIncrementalStreamLoaderObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIncrementalStreamLoaderObserverCoerce for nsIIncrementalStreamLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoaderObserver) -> &Self {
        v
    }
}

impl nsIIncrementalStreamLoaderObserver {
    /// Cast this `nsIIncrementalStreamLoaderObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIncrementalStreamLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIncrementalStreamLoaderObserver {
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
impl<T: nsISupportsCoerce> nsIIncrementalStreamLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIncrementalStreamLoaderObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIncrementalStreamLoaderObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onIncrementalData (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in unsigned long dataLength, [array, size_is (dataLength), const] in octet data, inout unsigned long consumedLength); */
    pub OnIncrementalData: unsafe extern "system" fn (this: *const nsIIncrementalStreamLoaderObserver, loader: *const nsIIncrementalStreamLoader, ctxt: *const nsISupports, dataLength: u32, data: *const u8, consumedLength: *mut u32) -> ::nserror::nsresult,

    /* void onStreamComplete (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
    pub OnStreamComplete: unsafe extern "system" fn (this: *const nsIIncrementalStreamLoaderObserver, loader: *const nsIIncrementalStreamLoader, ctxt: *const nsISupports, status: ::nserror::nsresult, resultLength: u32, result: *const u8) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIncrementalStreamLoaderObserver {

    /// ```text
    /// /**
    ///      * Called when new data has arrived on the stream.
    ///      *
    ///      * @param loader the stream loader that loaded the stream.
    ///      * @param ctxt the context parameter of the underlying channel
    ///      * @param dataLength the length of the new data received
    ///      * @param data the contents of the new data received.
    ///      *
    ///      * This method will always be called asynchronously by the
    ///      * nsIIncrementalStreamLoader involved, on the thread that called the
    ///      * loader's init() method.
    ///      *
    ///      * If the observer wants to not accumulate all or portional of the data in
    ///      * the internal buffer, the consumedLength shall be set to the value of
    ///      * the dataLength or less. By default the consumedLength value is assumed 0.
    ///      * The data and dataLength reflect the non-consumed data and will be
    ///      * accumulated if consumedLength is not set.
    ///      *
    ///      * In comparison with onStreamComplete(), the data buffer cannot be
    ///      * adopted if this method returns NS_SUCCESS_ADOPTED_DATA.
    ///      */
    /// ```
    ///

    /// `void onIncrementalData (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in unsigned long dataLength, [array, size_is (dataLength), const] in octet data, inout unsigned long consumedLength);`
    #[inline]
    pub unsafe fn OnIncrementalData(&self, loader: *const nsIIncrementalStreamLoader, ctxt: *const nsISupports, dataLength: u32, data: *const u8, consumedLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).OnIncrementalData)(self, loader, ctxt, dataLength, data, consumedLength)
    }


    /// ```text
    /// /**
    ///      * Called when the entire stream has been loaded.
    ///      *
    ///      * @param loader the stream loader that loaded the stream.
    ///      * @param ctxt the context parameter of the underlying channel
    ///      * @param status the status of the underlying channel
    ///      * @param resultLength the length of the data loaded
    ///      * @param result the data
    ///      *
    ///      * This method will always be called asynchronously by the
    ///      * nsIIncrementalStreamLoader involved, on the thread that called the
    ///      * loader's init() method.
    ///      *
    ///      * If the observer wants to take over responsibility for the
    ///      * data buffer (result), it returns NS_SUCCESS_ADOPTED_DATA
    ///      * in place of NS_OK as its success code. The loader will then
    ///      * "forget" about the data and not free() it after
    ///      * onStreamComplete() returns; observer must call free()
    ///      * when the data is no longer required.
    ///      */
    /// ```
    ///

    /// `void onStreamComplete (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result);`
    #[inline]
    pub unsafe fn OnStreamComplete(&self, loader: *const nsIIncrementalStreamLoader, ctxt: *const nsISupports, status: ::nserror::nsresult, resultLength: u32, result: *const u8) -> ::nserror::nsresult {
        ((*self.vtable).OnStreamComplete)(self, loader, ctxt, status, resultLength, result)
    }


}


/// `interface nsIIncrementalStreamLoader : nsIStreamListener`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIncrementalStreamLoader {
    vtable: *const nsIIncrementalStreamLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIncrementalStreamLoader.
unsafe impl XpCom for nsIIncrementalStreamLoader {
    const IID: nsIID = nsID(0xa023b060, 0xba23, 0x431a,
        [0xb4, 0x49, 0x2d, 0xd6, 0x3e, 0x22, 0x05, 0x54]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIncrementalStreamLoader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIncrementalStreamLoader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIncrementalStreamLoaderCoerce {
    /// Cheaply cast a value of this type from a `nsIIncrementalStreamLoader`.
    fn coerce_from(v: &nsIIncrementalStreamLoader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIncrementalStreamLoaderCoerce for nsIIncrementalStreamLoader {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoader) -> &Self {
        v
    }
}

impl nsIIncrementalStreamLoader {
    /// Cast this `nsIIncrementalStreamLoader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIncrementalStreamLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIncrementalStreamLoader {
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
impl<T: nsIStreamListenerCoerce> nsIIncrementalStreamLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIncrementalStreamLoader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIncrementalStreamLoader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIncrementalStreamLoaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIIncrementalStreamLoaderObserver aObserver); */
    pub Init: unsafe extern "system" fn (this: *const nsIIncrementalStreamLoader, aObserver: *const nsIIncrementalStreamLoaderObserver) -> ::nserror::nsresult,

    /* readonly attribute unsigned long numBytesRead; */
    pub GetNumBytesRead: unsafe extern "system" fn (this: *const nsIIncrementalStreamLoader, aNumBytesRead: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsIRequest request; */
    pub GetRequest: unsafe extern "system" fn (this: *const nsIIncrementalStreamLoader, aRequest: *mut*const nsIRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIncrementalStreamLoader {

    /// ```text
    /// /**
    ///  * Asynchronously loads a channel into a memory buffer.
    ///  *
    ///  * To use this interface, first call init() with a nsIIncrementalStreamLoaderObserver
    ///  * that will be notified when the data has been loaded. Then call asyncOpen()
    ///  * on the channel with the nsIIncrementalStreamLoader as the listener. The context
    ///  * argument in the asyncOpen() call will be passed to the onStreamComplete()
    ///  * callback.
    ///  *
    ///  * XXX define behaviour for sizes >4 GB
    ///  */
    /// /**
    ///      * Initialize this stream loader, and start loading the data.
    ///      *
    ///      * @param aObserver
    ///      *        An observer that will be notified when the data is complete.
    ///      */
    /// ```
    ///

    /// `void init (in nsIIncrementalStreamLoaderObserver aObserver);`
    #[inline]
    pub unsafe fn Init(&self, aObserver: *const nsIIncrementalStreamLoaderObserver) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aObserver)
    }


    /// ```text
    /// /**
    ///      * Gets the number of bytes read so far.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long numBytesRead;`
    #[inline]
    pub unsafe fn GetNumBytesRead(&self, aNumBytesRead: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumBytesRead)(self, aNumBytesRead)
    }


    /// ```text
    /// /**
    ///      * Gets the request that loaded this file.
    ///      * null after the request has finished loading.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIRequest request;`
    #[inline]
    pub unsafe fn GetRequest(&self, aRequest: *mut*const nsIRequest) -> ::nserror::nsresult {
        ((*self.vtable).GetRequest)(self, aRequest)
    }


}


