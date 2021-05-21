//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/testing/marionette/components/nsIMarionette.idl
//


/// `interface nsIMarionette : nsISupports`
///

/// ```text
/// /** Interface for accessing the Marionette server instance. */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMarionette {
    vtable: *const nsIMarionetteVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMarionette.
unsafe impl XpCom for nsIMarionette {
    const IID: nsIID = nsID(0x13fa7d76, 0xf976, 0x4711,
        [0xa0, 0x0c, 0x29, 0xac, 0x9c, 0x18, 0x81, 0xe1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMarionette {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMarionette.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMarionetteCoerce {
    /// Cheaply cast a value of this type from a `nsIMarionette`.
    fn coerce_from(v: &nsIMarionette) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMarionetteCoerce for nsIMarionette {
    #[inline]
    fn coerce_from(v: &nsIMarionette) -> &Self {
        v
    }
}

impl nsIMarionette {
    /// Cast this `nsIMarionette` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMarionetteCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMarionette {
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
impl<T: nsISupportsCoerce> nsIMarionetteCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMarionette) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMarionette
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMarionetteVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean running; */
    pub GetRunning: unsafe extern "system" fn (this: *const nsIMarionette, aRunning: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMarionette {

    /// ```text
    /// /** Indicates whether the remote protocol is enabled. */
    /// ```
    ///

    /// `readonly attribute boolean running;`
    #[inline]
    pub unsafe fn GetRunning(&self, aRunning: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRunning)(self, aRunning)
    }


}


