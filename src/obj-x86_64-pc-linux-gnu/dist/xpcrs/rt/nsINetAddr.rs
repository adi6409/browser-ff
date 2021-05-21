//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetAddr.idl
//


/// `interface nsINetAddr : nsISupports`
///

/// ```text
/// /**
///  * nsINetAddr
///  *
///  * This interface represents a native NetAddr struct in a readonly
///  * interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetAddr {
    vtable: *const nsINetAddrVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetAddr.
unsafe impl XpCom for nsINetAddr {
    const IID: nsIID = nsID(0x652b9ec5, 0xd159, 0x45d7,
        [0x91, 0x27, 0x50, 0xbb, 0x55, 0x94, 0x86, 0xcd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetAddr {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetAddr.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetAddrCoerce {
    /// Cheaply cast a value of this type from a `nsINetAddr`.
    fn coerce_from(v: &nsINetAddr) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetAddrCoerce for nsINetAddr {
    #[inline]
    fn coerce_from(v: &nsINetAddr) -> &Self {
        v
    }
}

impl nsINetAddr {
    /// Cast this `nsINetAddr` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetAddrCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetAddr {
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
impl<T: nsISupportsCoerce> nsINetAddrCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetAddr) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetAddr
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetAddrVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short family; */
    pub GetFamily: unsafe extern "system" fn (this: *const nsINetAddr, aFamily: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String address; */
    pub GetAddress: unsafe extern "system" fn (this: *const nsINetAddr, aAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned short port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsINetAddr, aPort: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned long flow; */
    pub GetFlow: unsafe extern "system" fn (this: *const nsINetAddr, aFlow: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long scope; */
    pub GetScope: unsafe extern "system" fn (this: *const nsINetAddr, aScope: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute boolean isV4Mapped; */
    pub GetIsV4Mapped: unsafe extern "system" fn (this: *const nsINetAddr, aIsV4Mapped: *mut bool) -> ::nserror::nsresult,

    /* [noscript] NetAddr getNetAddr (); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetNetAddr: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetAddr {
    /// ```text
    /// /**
    ///      * Network address families. These correspond to all the network address
    ///      * families supported by the NetAddr struct.
    ///      */
    /// ```
    ///

    pub const FAMILY_INET: i64 = 1;


    pub const FAMILY_INET6: i64 = 2;


    pub const FAMILY_LOCAL: i64 = 3;

    /// ```text
    /// /**
    ///      * @return the address family of the network address, which corresponds to
    ///      * one of the FAMILY_ constants.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned short family;`
    #[inline]
    pub unsafe fn GetFamily(&self, aFamily: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetFamily)(self, aFamily)
    }


    /// ```text
    /// /**
    ///      * @return Either the IP address (FAMILY_INET, FAMILY_INET6) or the path
    ///      * (FAMILY_LOCAL) in string form. IP addresses are in the format produced by
    ///      * mozilla::net::NetAddr::ToStringBuffer.
    ///      *
    ///      * Note: Paths for FAMILY_LOCAL may have length limitations which are
    ///      * implementation dependent and not documented as part of this interface.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String address;`
    #[inline]
    pub unsafe fn GetAddress(&self, aAddress: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAddress)(self, aAddress)
    }


    /// ```text
    /// /**
    ///      * @return the port number for a FAMILY_INET or FAMILY_INET6 address.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if the address family is not FAMILY_INET or
    ///      * FAMILY_INET6.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned short port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///      * @return the flow label for a FAMILY_INET6 address.
    ///      *
    ///      * @see http://www.ietf.org/rfc/rfc3697.txt
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if the address family is not FAMILY_INET6
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long flow;`
    #[inline]
    pub unsafe fn GetFlow(&self, aFlow: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFlow)(self, aFlow)
    }


    /// ```text
    /// /**
    ///      * @return the address scope of a FAMILY_INET6 address.
    ///      *
    ///      * @see http://tools.ietf.org/html/rfc4007
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if the address family is not FAMILY_INET6
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long scope;`
    #[inline]
    pub unsafe fn GetScope(&self, aScope: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetScope)(self, aScope)
    }


    /// ```text
    /// /**
    ///      * @return whether a FAMILY_INET6 address is mapped from FAMILY_INET.
    ///      *
    ///      * @throws NS_ERROR_NOT_AVAILABLE if the address family is not FAMILY_INET6
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isV4Mapped;`
    #[inline]
    pub unsafe fn GetIsV4Mapped(&self, aIsV4Mapped: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsV4Mapped)(self, aIsV4Mapped)
    }


    /// ```text
    /// /**
    ///      * @return the underlying NetAddr struct.
    ///      */
    /// ```
    ///

    /// `[noscript] NetAddr getNetAddr ();`
    const _GetNetAddr: () = ();

}


