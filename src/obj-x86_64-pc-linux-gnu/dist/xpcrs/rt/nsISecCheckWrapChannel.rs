//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISecCheckWrapChannel.idl
//


/// `interface nsISecCheckWrapChannel : nsISupports`
///

/// ```text
/// /**
///  * nsISecCheckWrapChannel
///  * Describes an XPCOM component used to wrap channels for performing
///  * security checks. Channels wrapped inside this class can use
///  * this interface to query the wrapped inner channel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISecCheckWrapChannel {
    vtable: *const nsISecCheckWrapChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISecCheckWrapChannel.
unsafe impl XpCom for nsISecCheckWrapChannel {
    const IID: nsIID = nsID(0x9446c5d5, 0xc9fb, 0x4a6e,
        [0xac, 0xf9, 0xca, 0x4f, 0xc6, 0x66, 0xef, 0xe0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISecCheckWrapChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISecCheckWrapChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISecCheckWrapChannelCoerce {
    /// Cheaply cast a value of this type from a `nsISecCheckWrapChannel`.
    fn coerce_from(v: &nsISecCheckWrapChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISecCheckWrapChannelCoerce for nsISecCheckWrapChannel {
    #[inline]
    fn coerce_from(v: &nsISecCheckWrapChannel) -> &Self {
        v
    }
}

impl nsISecCheckWrapChannel {
    /// Cast this `nsISecCheckWrapChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISecCheckWrapChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISecCheckWrapChannel {
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
impl<T: nsISupportsCoerce> nsISecCheckWrapChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecCheckWrapChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISecCheckWrapChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISecCheckWrapChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIChannel innerChannel; */
    pub GetInnerChannel: unsafe extern "system" fn (this: *const nsISecCheckWrapChannel, aInnerChannel: *mut*const nsIChannel) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISecCheckWrapChannel {

    /// ```text
    /// /**
    ///    * Returns the wrapped channel inside this class.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIChannel innerChannel;`
    #[inline]
    pub unsafe fn GetInnerChannel(&self, aInnerChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetInnerChannel)(self, aInnerChannel)
    }


}


