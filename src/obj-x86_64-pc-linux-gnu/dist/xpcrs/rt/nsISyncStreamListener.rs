//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISyncStreamListener.idl
//


/// `interface nsISyncStreamListener : nsIStreamListener`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISyncStreamListener {
    vtable: *const nsISyncStreamListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISyncStreamListener.
unsafe impl XpCom for nsISyncStreamListener {
    const IID: nsIID = nsID(0x7e1aa658, 0x6e3f, 0x4521,
        [0x99, 0x46, 0x96, 0x85, 0xa1, 0x69, 0xf7, 0x64]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISyncStreamListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISyncStreamListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISyncStreamListenerCoerce {
    /// Cheaply cast a value of this type from a `nsISyncStreamListener`.
    fn coerce_from(v: &nsISyncStreamListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISyncStreamListenerCoerce for nsISyncStreamListener {
    #[inline]
    fn coerce_from(v: &nsISyncStreamListener) -> &Self {
        v
    }
}

impl nsISyncStreamListener {
    /// Cast this `nsISyncStreamListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISyncStreamListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISyncStreamListener {
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
impl<T: nsIStreamListenerCoerce> nsISyncStreamListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISyncStreamListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISyncStreamListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISyncStreamListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* readonly attribute nsIInputStream inputStream; */
    pub GetInputStream: unsafe extern "system" fn (this: *const nsISyncStreamListener, aInputStream: *mut*const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISyncStreamListener {

    /// ```text
    /// /**
    ///      * Returns an input stream that when read will fetch data delivered to the
    ///      * sync stream listener.  The nsIInputStream implementation will wait for
    ///      * OnDataAvailable events before returning from Read.
    ///      *
    ///      * NOTE: Reading from the returned nsIInputStream may spin the current
    ///      * thread's event queue, which could result in any event being processed.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIInputStream inputStream;`
    #[inline]
    pub unsafe fn GetInputStream(&self, aInputStream: *mut*const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetInputStream)(self, aInputStream)
    }


}


