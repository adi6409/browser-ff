//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISimpleStreamListener.idl
//


/// `interface nsISimpleStreamListener : nsIStreamListener`
///

/// ```text
/// /**
///  * A simple stream listener can be used with AsyncRead to supply data to
///  * a output stream.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISimpleStreamListener {
    vtable: *const nsISimpleStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISimpleStreamListener.
unsafe impl XpCom for nsISimpleStreamListener {
    const IID: nsIID = nsID(0xa9b84f6a, 0x0824, 0x4278,
        [0xba, 0xe6, 0xbf, 0xca, 0x05, 0x70, 0xa2, 0x6e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISimpleStreamListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISimpleStreamListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISimpleStreamListenerCoerce {
    /// Cheaply cast a value of this type from a `nsISimpleStreamListener`.
    fn coerce_from(v: &nsISimpleStreamListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISimpleStreamListenerCoerce for nsISimpleStreamListener {
    #[inline]
    fn coerce_from(v: &nsISimpleStreamListener) -> &Self {
        v
    }
}

impl nsISimpleStreamListener {
    /// Cast this `nsISimpleStreamListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISimpleStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISimpleStreamListener {
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
impl<T: nsIStreamListenerCoerce> nsISimpleStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISimpleStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISimpleStreamListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISimpleStreamListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* void init (in nsIOutputStream aSink, in nsIRequestObserver aObserver); */
    pub Init: unsafe extern "system" fn (this: *const nsISimpleStreamListener, aSink: *const nsIOutputStream, aObserver: *const nsIRequestObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISimpleStreamListener {

    /// ```text
    /// /**
    ///      * Initialize the simple stream listener.
    ///      *
    ///      * @param aSink data will be read from the channel to this output stream.
    ///      *              Must implement writeFrom.
    ///      * @param aObserver optional stream observer (can be NULL)
    ///      */
    /// ```
    ///

    /// `void init (in nsIOutputStream aSink, in nsIRequestObserver aObserver);`
    #[inline]
    pub unsafe fn Init(&self, aSink: *const nsIOutputStream, aObserver: *const nsIRequestObserver) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aSink, aObserver)
    }


}


