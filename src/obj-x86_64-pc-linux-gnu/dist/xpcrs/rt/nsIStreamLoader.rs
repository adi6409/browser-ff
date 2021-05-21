//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamLoader.idl
//


/// `interface nsIStreamLoaderObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamLoaderObserver {
    vtable: *const nsIStreamLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamLoaderObserver.
unsafe impl XpCom for nsIStreamLoaderObserver {
    const IID: nsIID = nsID(0x359f7990, 0xd4e9, 0x11d3,
        [0xa1, 0xa5, 0x00, 0x50, 0x04, 0x1c, 0xaf, 0x44]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamLoaderObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamLoaderObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamLoaderObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamLoaderObserver`.
    fn coerce_from(v: &nsIStreamLoaderObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamLoaderObserverCoerce for nsIStreamLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIStreamLoaderObserver) -> &Self {
        v
    }
}

impl nsIStreamLoaderObserver {
    /// Cast this `nsIStreamLoaderObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamLoaderObserver {
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
impl<T: nsISupportsCoerce> nsIStreamLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamLoaderObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamLoaderObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onStreamComplete (in nsIStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
    pub OnStreamComplete: unsafe extern "system" fn (this: *const nsIStreamLoaderObserver, loader: *const nsIStreamLoader, ctxt: *const nsISupports, status: ::nserror::nsresult, resultLength: u32, result: *const u8) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamLoaderObserver {

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
    ///      * nsIStreamLoader involved, on the thread that called the
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

    /// `void onStreamComplete (in nsIStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result);`
    #[inline]
    pub unsafe fn OnStreamComplete(&self, loader: *const nsIStreamLoader, ctxt: *const nsISupports, status: ::nserror::nsresult, resultLength: u32, result: *const u8) -> ::nserror::nsresult {
        ((*self.vtable).OnStreamComplete)(self, loader, ctxt, status, resultLength, result)
    }


}


/// `interface nsIStreamLoader : nsIStreamListener`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamLoader {
    vtable: *const nsIStreamLoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamLoader.
unsafe impl XpCom for nsIStreamLoader {
    const IID: nsIID = nsID(0x323bcff1, 0x7513, 0x4e1f,
        [0xa5, 0x41, 0x1c, 0x92, 0x13, 0xc2, 0xed, 0x1b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamLoader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamLoader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamLoaderCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamLoader`.
    fn coerce_from(v: &nsIStreamLoader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamLoaderCoerce for nsIStreamLoader {
    #[inline]
    fn coerce_from(v: &nsIStreamLoader) -> &Self {
        v
    }
}

impl nsIStreamLoader {
    /// Cast this `nsIStreamLoader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamLoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamLoader {
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
impl<T: nsIStreamListenerCoerce> nsIStreamLoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamLoader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamLoader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamLoaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver); */
    pub Init: unsafe extern "system" fn (this: *const nsIStreamLoader, aStreamObserver: *const nsIStreamLoaderObserver, aRequestObserver: *const nsIRequestObserver) -> ::nserror::nsresult,

    /* readonly attribute unsigned long numBytesRead; */
    pub GetNumBytesRead: unsafe extern "system" fn (this: *const nsIStreamLoader, aNumBytesRead: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsIRequest request; */
    pub GetRequest: unsafe extern "system" fn (this: *const nsIStreamLoader, aRequest: *mut*const nsIRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamLoader {

    /// ```text
    /// /**
    ///  * Asynchronously loads a channel into a memory buffer.
    ///  *
    ///  * To use this interface, first call init() with a nsIStreamLoaderObserver
    ///  * that will be notified when the data has been loaded. Then call asyncOpen()
    ///  * on the channel with the nsIStreamLoader as the listener. The context
    ///  * argument in the asyncOpen() call will be passed to the onStreamComplete()
    ///  * callback.
    ///  *
    ///  * XXX define behaviour for sizes >4 GB
    ///  */
    /// /**
    ///      * Initialize this stream loader, and start loading the data.
    ///      *
    ///      * @param aStreamObserver
    ///      *        An observer that will be notified when the data is complete.
    ///      * @param aRequestObserver
    ///      *        An optional observer that will be notified when the request
    ///      *        has started or stopped.
    ///      */
    /// ```
    ///

    /// `void init (in nsIStreamLoaderObserver aStreamObserver, [optional] in nsIRequestObserver aRequestObserver);`
    #[inline]
    pub unsafe fn Init(&self, aStreamObserver: *const nsIStreamLoaderObserver, aRequestObserver: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aStreamObserver, aRequestObserver)
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


