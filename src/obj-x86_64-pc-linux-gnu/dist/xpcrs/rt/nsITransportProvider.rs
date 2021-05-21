//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/websocket/nsITransportProvider.idl
//


/// `interface nsITransportProvider : nsISupports`
///

/// ```text
/// /**
///  * An interface which can be used to asynchronously request a nsITransport
///  * together with the input and output streams that go together with it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransportProvider {
    vtable: *const nsITransportProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransportProvider.
unsafe impl XpCom for nsITransportProvider {
    const IID: nsIID = nsID(0x6fcec704, 0xcfd2, 0x46ef,
        [0xa3, 0x94, 0xa6, 0x4d, 0x5c, 0xb1, 0x47, 0x5c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransportProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransportProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransportProviderCoerce {
    /// Cheaply cast a value of this type from a `nsITransportProvider`.
    fn coerce_from(v: &nsITransportProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransportProviderCoerce for nsITransportProvider {
    #[inline]
    fn coerce_from(v: &nsITransportProvider) -> &Self {
        v
    }
}

impl nsITransportProvider {
    /// Cast this `nsITransportProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransportProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransportProvider {
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
impl<T: nsISupportsCoerce> nsITransportProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransportProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransportProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransportProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void setListener (in nsIHttpUpgradeListener listener); */
    pub SetListener: unsafe extern "system" fn (this: *const nsITransportProvider, listener: *const nsIHttpUpgradeListener) -> ::nserror::nsresult,

    /* [must_use,noscript] PTransportProviderChild getIPCChild (); */
    /// Unable to generate binding because `native type mozilla::net::PTransportProviderChild unsupported`
    pub GetIPCChild: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransportProvider {


    /// `[must_use] void setListener (in nsIHttpUpgradeListener listener);`
    #[inline]
    pub unsafe fn SetListener(&self, listener: *const nsIHttpUpgradeListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, listener)
    }



    /// `[must_use,noscript] PTransportProviderChild getIPCChild ();`
    const _GetIPCChild: () = ();

}


