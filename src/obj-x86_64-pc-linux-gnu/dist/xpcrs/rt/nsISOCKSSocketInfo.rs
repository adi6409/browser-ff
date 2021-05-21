//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/socket/nsISOCKSSocketInfo.idl
//


/// `interface nsISOCKSSocketInfo : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISOCKSSocketInfo {
    vtable: *const nsISOCKSSocketInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISOCKSSocketInfo.
unsafe impl XpCom for nsISOCKSSocketInfo {
    const IID: nsIID = nsID(0xd5c0d1f9, 0x22d7, 0x47dc,
        [0xbf, 0x91, 0xd9, 0xac, 0x6e, 0x12, 0x51, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISOCKSSocketInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISOCKSSocketInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISOCKSSocketInfoCoerce {
    /// Cheaply cast a value of this type from a `nsISOCKSSocketInfo`.
    fn coerce_from(v: &nsISOCKSSocketInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISOCKSSocketInfoCoerce for nsISOCKSSocketInfo {
    #[inline]
    fn coerce_from(v: &nsISOCKSSocketInfo) -> &Self {
        v
    }
}

impl nsISOCKSSocketInfo {
    /// Cast this `nsISOCKSSocketInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISOCKSSocketInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISOCKSSocketInfo {
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
impl<T: nsISupportsCoerce> nsISOCKSSocketInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISOCKSSocketInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISOCKSSocketInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISOCKSSocketInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] attribute NetAddrPtr destinationAddr; */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub GetDestinationAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr destinationAddr; */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub SetDestinationAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr externalProxyAddr; */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub GetExternalProxyAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr externalProxyAddr; */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub SetExternalProxyAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr internalProxyAddr; */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub GetInternalProxyAddr: *const ::libc::c_void,

    /* [noscript] attribute NetAddrPtr internalProxyAddr; */
    /// Unable to generate binding because `native type mozilla::net::NetAddr unsupported`
    pub SetInternalProxyAddr: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISOCKSSocketInfo {


    /// `[noscript] attribute NetAddrPtr destinationAddr;`
    const _GetDestinationAddr: () = ();


    /// `[noscript] attribute NetAddrPtr destinationAddr;`
    const _SetDestinationAddr: () = ();


    /// `[noscript] attribute NetAddrPtr externalProxyAddr;`
    const _GetExternalProxyAddr: () = ();


    /// `[noscript] attribute NetAddrPtr externalProxyAddr;`
    const _SetExternalProxyAddr: () = ();


    /// `[noscript] attribute NetAddrPtr internalProxyAddr;`
    const _GetInternalProxyAddr: () = ();


    /// `[noscript] attribute NetAddrPtr internalProxyAddr;`
    const _SetInternalProxyAddr: () = ();

}


