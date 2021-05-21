//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIForcePendingChannel.idl
//


/// `interface nsIForcePendingChannel : nsISupports`
///

/// ```text
/// /**
///  * nsIForcePending interface exposes a function that enables overwriting of the normal
///  * behavior for the channel's IsPending(), forcing 'true' to be returned.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIForcePendingChannel {
    vtable: *const nsIForcePendingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIForcePendingChannel.
unsafe impl XpCom for nsIForcePendingChannel {
    const IID: nsIID = nsID(0x2ac3e1ca, 0x049f, 0x44c3,
        [0xa5, 0x19, 0xf0, 0x68, 0x1f, 0x51, 0xe9, 0xb1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIForcePendingChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIForcePendingChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIForcePendingChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIForcePendingChannel`.
    fn coerce_from(v: &nsIForcePendingChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIForcePendingChannelCoerce for nsIForcePendingChannel {
    #[inline]
    fn coerce_from(v: &nsIForcePendingChannel) -> &Self {
        v
    }
}

impl nsIForcePendingChannel {
    /// Cast this `nsIForcePendingChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIForcePendingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIForcePendingChannel {
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
impl<T: nsISupportsCoerce> nsIForcePendingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIForcePendingChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIForcePendingChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIForcePendingChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void forcePending (in boolean aForcePending); */
    pub ForcePending: unsafe extern "system" fn (this: *const nsIForcePendingChannel, aForcePending: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIForcePendingChannel {

    /// ```text
    /// /**
    ///      * forcePending(true) overrides the normal behavior for the
    ///      * channel's IsPending(), forcing 'true' to be returned. A call to
    ///      * forcePending(false) reverts IsPending() back to normal behavior.
    ///      */
    /// ```
    ///

    /// `void forcePending (in boolean aForcePending);`
    #[inline]
    pub unsafe fn ForcePending(&self, aForcePending: bool) -> ::nserror::nsresult {
        ((*self.vtable).ForcePending)(self, aForcePending)
    }


}


