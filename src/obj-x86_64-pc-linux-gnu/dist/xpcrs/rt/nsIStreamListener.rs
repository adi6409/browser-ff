//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStreamListener.idl
//


/// `interface nsIStreamListener : nsIRequestObserver`
///

/// ```text
/// /**
///  * nsIStreamListener
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStreamListener {
    vtable: *const nsIStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStreamListener.
unsafe impl XpCom for nsIStreamListener {
    const IID: nsIID = nsID(0x3b4c8a77, 0x76ba, 0x4610,
        [0xb3, 0x16, 0x67, 0x8c, 0x73, 0xa3, 0xb8, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStreamListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStreamListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStreamListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIStreamListener`.
    fn coerce_from(v: &nsIStreamListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStreamListenerCoerce for nsIStreamListener {
    #[inline]
    fn coerce_from(v: &nsIStreamListener) -> &Self {
        v
    }
}

impl nsIStreamListener {
    /// Cast this `nsIStreamListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStreamListener {
    type Target = nsIRequestObserver;
    #[inline]
    fn deref(&self) -> &nsIRequestObserver {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIRequestObserverCoerce> nsIStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStreamListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStreamListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIRequestObserverVTable,

    /* void onDataAvailable (in nsIRequest aRequest, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount); */
    pub OnDataAvailable: unsafe extern "system" fn (this: *const nsIStreamListener, aRequest: *const nsIRequest, aInputStream: *const nsIInputStream, aOffset: u64, aCount: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStreamListener {

    /// ```text
    /// /**
    ///      * Called when the next chunk of data (corresponding to the request) may
    ///      * be read without blocking the calling thread.  The onDataAvailable impl
    ///      * must read exactly |aCount| bytes of data before returning.
    ///      *
    ///      * @param aRequest request corresponding to the source of the data
    ///      * @param aInputStream input stream containing the data chunk
    ///      * @param aOffset
    ///      *        Number of bytes that were sent in previous onDataAvailable calls
    ///      *        for this request. In other words, the sum of all previous count
    ///      *        parameters.
    ///      * @param aCount number of bytes available in the stream
    ///      *
    ///      * NOTE: The aInputStream parameter must implement readSegments.
    ///      *
    ///      * An exception thrown from onDataAvailable has the side-effect of
    ///      * causing the request to be canceled.
    ///      */
    /// ```
    ///

    /// `void onDataAvailable (in nsIRequest aRequest, in nsIInputStream aInputStream, in unsigned long long aOffset, in unsigned long aCount);`
    #[inline]
    pub unsafe fn OnDataAvailable(&self, aRequest: *const nsIRequest, aInputStream: *const nsIInputStream, aOffset: u64, aCount: u32) -> ::nserror::nsresult {
        ((*self.vtable).OnDataAvailable)(self, aRequest, aInputStream, aOffset, aCount)
    }


}


