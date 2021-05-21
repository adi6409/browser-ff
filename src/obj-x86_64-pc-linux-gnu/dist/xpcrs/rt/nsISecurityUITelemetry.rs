//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISecurityUITelemetry.idl
//


/// `interface nsISecurityUITelemetry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISecurityUITelemetry {
    vtable: *const nsISecurityUITelemetryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISecurityUITelemetry.
unsafe impl XpCom for nsISecurityUITelemetry {
    const IID: nsIID = nsID(0x5d1acf82, 0x223a, 0x46fb,
        [0xa8, 0xf3, 0xa1, 0xb1, 0x6e, 0x2c, 0xeb, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISecurityUITelemetry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISecurityUITelemetry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISecurityUITelemetryCoerce {
    /// Cheaply cast a value of this type from a `nsISecurityUITelemetry`.
    fn coerce_from(v: &nsISecurityUITelemetry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISecurityUITelemetryCoerce for nsISecurityUITelemetry {
    #[inline]
    fn coerce_from(v: &nsISecurityUITelemetry) -> &Self {
        v
    }
}

impl nsISecurityUITelemetry {
    /// Cast this `nsISecurityUITelemetry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISecurityUITelemetryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISecurityUITelemetry {
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
impl<T: nsISupportsCoerce> nsISecurityUITelemetryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecurityUITelemetry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISecurityUITelemetry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISecurityUITelemetryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISecurityUITelemetry {

    pub const WARNING_ADDON_ASKING_PREVENTED: i64 = 1;


    pub const WARNING_ADDON_ASKING_PREVENTED_CLICK_THROUGH: i64 = 2;


    pub const WARNING_CONFIRM_ADDON_INSTALL: i64 = 3;


    pub const WARNING_CONFIRM_ADDON_INSTALL_CLICK_THROUGH: i64 = 4;


    pub const WARNING_CONFIRM_POST_TO_INSECURE_FROM_SECURE: i64 = 9;


    pub const WARNING_CONFIRM_POST_TO_INSECURE_FROM_SECURE_CLICK_THROUGH: i64 = 10;


    pub const WARNING_BAD_CERT_ADD_EXCEPTION_BASE: i64 = 30;


    pub const WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;


    pub const WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;


    pub const WARNING_BAD_CERT_ADD_EXCEPTION_FLAG_TIME: i64 = 4;


    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_BASE: i64 = 38;


    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;


    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;


    pub const WARNING_BAD_CERT_CONFIRM_ADD_EXCEPTION_FLAG_TIME: i64 = 4;


    pub const WARNING_BAD_CERT_TOP_CLICK_VIEW_CERT: i64 = 71;


    pub const WARNING_BAD_CERT_TOP_DONT_REMEMBER_EXCEPTION: i64 = 72;


    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_BASE: i64 = 76;


    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;


    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;


    pub const WARNING_BAD_CERT_TOP_ADD_EXCEPTION_FLAG_TIME: i64 = 4;


    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_BASE: i64 = 84;


    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_UNTRUSTED: i64 = 1;


    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_DOMAIN: i64 = 2;


    pub const WARNING_BAD_CERT_TOP_CONFIRM_ADD_EXCEPTION_FLAG_TIME: i64 = 4;


}


