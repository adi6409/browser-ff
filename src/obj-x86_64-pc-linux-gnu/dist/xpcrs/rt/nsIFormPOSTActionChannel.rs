//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFormPOSTActionChannel.idl
//


/// `interface nsIFormPOSTActionChannel : nsIUploadChannel`
///

/// ```text
/// /**
///  * nsIFormPOSTActionChannel
///  *
///  * Channel classes that want to be allowed for HTML form POST action must
///  * implement this interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFormPOSTActionChannel {
    vtable: *const nsIFormPOSTActionChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFormPOSTActionChannel.
unsafe impl XpCom for nsIFormPOSTActionChannel {
    const IID: nsIID = nsID(0xfc826b53, 0x0db8, 0x42b4,
        [0xaa, 0x6a, 0x5d, 0xd2, 0xcf, 0xca, 0x52, 0xa4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFormPOSTActionChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFormPOSTActionChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFormPOSTActionChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIFormPOSTActionChannel`.
    fn coerce_from(v: &nsIFormPOSTActionChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFormPOSTActionChannelCoerce for nsIFormPOSTActionChannel {
    #[inline]
    fn coerce_from(v: &nsIFormPOSTActionChannel) -> &Self {
        v
    }
}

impl nsIFormPOSTActionChannel {
    /// Cast this `nsIFormPOSTActionChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFormPOSTActionChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFormPOSTActionChannel {
    type Target = nsIUploadChannel;
    #[inline]
    fn deref(&self) -> &nsIUploadChannel {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIUploadChannelCoerce> nsIFormPOSTActionChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFormPOSTActionChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFormPOSTActionChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFormPOSTActionChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIUploadChannelVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFormPOSTActionChannel {


}


