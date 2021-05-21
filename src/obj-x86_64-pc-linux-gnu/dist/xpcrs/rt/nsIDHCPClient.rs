//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDHCPClient.idl
//


/// `interface nsIDHCPClient : nsISupports`
///

/// ```text
/// /**
///  * This interface allows the proxy code to access the DHCP Options in a platform-specific way
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDHCPClient {
    vtable: *const nsIDHCPClientVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDHCPClient.
unsafe impl XpCom for nsIDHCPClient {
    const IID: nsIID = nsID(0xaee75dc0, 0xbe1a, 0x46b9,
        [0x9e, 0x0c, 0x31, 0xa6, 0x89, 0x9c, 0x17, 0x5c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDHCPClient {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDHCPClient.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDHCPClientCoerce {
    /// Cheaply cast a value of this type from a `nsIDHCPClient`.
    fn coerce_from(v: &nsIDHCPClient) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDHCPClientCoerce for nsIDHCPClient {
    #[inline]
    fn coerce_from(v: &nsIDHCPClient) -> &Self {
        v
    }
}

impl nsIDHCPClient {
    /// Cast this `nsIDHCPClient` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDHCPClientCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDHCPClient {
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
impl<T: nsISupportsCoerce> nsIDHCPClientCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDHCPClient) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDHCPClient
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDHCPClientVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getOption (in uint8_t option); */
    pub GetOption: unsafe extern "system" fn (this: *const nsIDHCPClient, option: uint8_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDHCPClient {

    /// ```text
    /// /**
    ///     * returns the DHCP Option designated by the option parameter
    ///     */
    /// ```
    ///

    /// `ACString getOption (in uint8_t option);`
    #[inline]
    pub unsafe fn GetOption(&self, option: uint8_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetOption)(self, option, _retval)
    }


}


