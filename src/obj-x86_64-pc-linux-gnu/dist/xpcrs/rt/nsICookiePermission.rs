//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookiePermission.idl
//


/// `typedef int32_t  nsCookieAccess;`
///


pub type nsCookieAccess = i32;


/// `interface nsICookiePermission : nsISupports`
///

/// ```text
/// /**
///  * An interface to test for cookie permissions
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICookiePermission {
    vtable: *const nsICookiePermissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICookiePermission.
unsafe impl XpCom for nsICookiePermission {
    const IID: nsIID = nsID(0x11ddd4ed, 0x8f5b, 0x40b3,
        [0xb2, 0xa0, 0x27, 0xc2, 0x0e, 0xa1, 0xc8, 0x8d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICookiePermission {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICookiePermission.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICookiePermissionCoerce {
    /// Cheaply cast a value of this type from a `nsICookiePermission`.
    fn coerce_from(v: &nsICookiePermission) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICookiePermissionCoerce for nsICookiePermission {
    #[inline]
    fn coerce_from(v: &nsICookiePermission) -> &Self {
        v
    }
}

impl nsICookiePermission {
    /// Cast this `nsICookiePermission` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICookiePermissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICookiePermission {
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
impl<T: nsISupportsCoerce> nsICookiePermissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookiePermission) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICookiePermission
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICookiePermissionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICookiePermission {
    /// ```text
    /// /**
    ///    * nsCookieAccess values
    ///    */
    /// ```
    ///

    pub const ACCESS_DEFAULT: i64 = 0;


    pub const ACCESS_ALLOW: i64 = 1;


    pub const ACCESS_DENY: i64 = 2;

    /// ```text
    /// /**
    ///    * additional values for nsCookieAccess which may not match
    ///    * nsIPermissionManager. Keep 3-7 available to allow nsIPermissionManager to
    ///    * add values without colliding. ACCESS_SESSION is not directly returned by
    ///    * any methods on this interface.
    ///    */
    /// ```
    ///

    pub const ACCESS_SESSION: i64 = 8;


}


