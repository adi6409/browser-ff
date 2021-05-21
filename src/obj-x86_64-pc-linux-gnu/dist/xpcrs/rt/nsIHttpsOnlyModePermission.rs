//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/security/nsIHttpsOnlyModePermission.idl
//


/// `interface nsIHttpsOnlyModePermission : nsISupports`
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
pub struct nsIHttpsOnlyModePermission {
    vtable: *const nsIHttpsOnlyModePermissionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpsOnlyModePermission.
unsafe impl XpCom for nsIHttpsOnlyModePermission {
    const IID: nsIID = nsID(0x73f4f039, 0xd6ff, 0x41a7,
        [0x9e, 0xb3, 0x00, 0xdb, 0x57, 0xb0, 0xb7, 0xf4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpsOnlyModePermission {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpsOnlyModePermission.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpsOnlyModePermissionCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpsOnlyModePermission`.
    fn coerce_from(v: &nsIHttpsOnlyModePermission) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpsOnlyModePermissionCoerce for nsIHttpsOnlyModePermission {
    #[inline]
    fn coerce_from(v: &nsIHttpsOnlyModePermission) -> &Self {
        v
    }
}

impl nsIHttpsOnlyModePermission {
    /// Cast this `nsIHttpsOnlyModePermission` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpsOnlyModePermissionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpsOnlyModePermission {
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
impl<T: nsISupportsCoerce> nsIHttpsOnlyModePermissionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpsOnlyModePermission) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpsOnlyModePermission
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpsOnlyModePermissionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpsOnlyModePermission {
    /// ```text
    /// /**
    ///    * nsIPermissionManager permission values
    ///    */
    /// ```
    ///

    pub const LOAD_INSECURE_DEFAULT: i64 = 0;


    pub const LOAD_INSECURE_ALLOW: i64 = 1;


    pub const LOAD_INSECURE_BLOCK: i64 = 2;

    /// ```text
    /// /**
    ///    * additional values which do not match
    ///    * nsIPermissionManager. Keep space available to allow nsIPermissionManager to
    ///    * add values without colliding. ACCESS_SESSION is not directly returned by
    ///    * any methods on this interface.
    ///    */
    /// ```
    ///

    pub const LOAD_INSECURE_ALLOW_SESSION: i64 = 9;


}


