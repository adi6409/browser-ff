//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIChildChannel.idl
//


/// `interface nsIChildChannel : nsISupports`
///

/// ```text
/// /**
///  * Implemented by content side of IPC protocols.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIChildChannel {
    vtable: *const nsIChildChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIChildChannel.
unsafe impl XpCom for nsIChildChannel {
    const IID: nsIID = nsID(0xc45b92ae, 0x4f07, 0x41dd,
        [0xb0, 0xef, 0xaa, 0x04, 0x4e, 0xea, 0xbb, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIChildChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIChildChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIChildChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIChildChannel`.
    fn coerce_from(v: &nsIChildChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIChildChannelCoerce for nsIChildChannel {
    #[inline]
    fn coerce_from(v: &nsIChildChannel) -> &Self {
        v
    }
}

impl nsIChildChannel {
    /// Cast this `nsIChildChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIChildChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIChildChannel {
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
impl<T: nsISupportsCoerce> nsIChildChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIChildChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIChildChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIChildChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void connectParent (in uint32_t registrarId); */
    pub ConnectParent: unsafe extern "system" fn (this: *const nsIChildChannel, registrarId: uint32_t) -> ::nserror::nsresult,

    /* void completeRedirectSetup (in nsIStreamListener aListener); */
    pub CompleteRedirectSetup: unsafe extern "system" fn (this: *const nsIChildChannel, aListener: *const nsIStreamListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIChildChannel {

    /// ```text
    /// /**
    ///    * Create the chrome side of the IPC protocol and join an existing 'real'
    ///    * channel on the parent process.  The id is provided by
    ///    * nsIRedirectChannelRegistrar on the chrome process and pushed to the child
    ///    * protocol as an argument to event starting a redirect.
    ///    *
    ///    * Primarilly used in HttpChannelChild::Redirect1Begin on a newly created
    ///    * child channel, where the new channel is intended to be created on the
    ///    * child process.
    ///    */
    /// ```
    ///

    /// `void connectParent (in uint32_t registrarId);`
    #[inline]
    pub unsafe fn ConnectParent(&self, registrarId: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).ConnectParent)(self, registrarId)
    }


    /// ```text
    /// /**
    ///    * As AsyncOpen is called on the chrome process for redirect target channels,
    ///    * we have to inform the child side of the protocol of that fact by a special
    ///    * method.
    ///    */
    /// ```
    ///

    /// `void completeRedirectSetup (in nsIStreamListener aListener);`
    #[inline]
    pub unsafe fn CompleteRedirectSetup(&self, aListener: *const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).CompleteRedirectSetup)(self, aListener)
    }


}


