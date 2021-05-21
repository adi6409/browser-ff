//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/IUrlClassifierUITelemetry.idl
//


/// `interface IUrlClassifierUITelemetry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct IUrlClassifierUITelemetry {
    vtable: *const IUrlClassifierUITelemetryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for IUrlClassifierUITelemetry.
unsafe impl XpCom for IUrlClassifierUITelemetry {
    const IID: nsIID = nsID(0xa6c62ce5, 0x3a95, 0x41bb,
        [0xb0, 0xf1, 0x8c, 0xd4, 0xf4, 0xca, 0x00, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for IUrlClassifierUITelemetry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from IUrlClassifierUITelemetry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait IUrlClassifierUITelemetryCoerce {
    /// Cheaply cast a value of this type from a `IUrlClassifierUITelemetry`.
    fn coerce_from(v: &IUrlClassifierUITelemetry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl IUrlClassifierUITelemetryCoerce for IUrlClassifierUITelemetry {
    #[inline]
    fn coerce_from(v: &IUrlClassifierUITelemetry) -> &Self {
        v
    }
}

impl IUrlClassifierUITelemetry {
    /// Cast this `IUrlClassifierUITelemetry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: IUrlClassifierUITelemetryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for IUrlClassifierUITelemetry {
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
impl<T: nsISupportsCoerce> IUrlClassifierUITelemetryCoerce for T {
    #[inline]
    fn coerce_from(v: &IUrlClassifierUITelemetry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every IUrlClassifierUITelemetry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct IUrlClassifierUITelemetryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl IUrlClassifierUITelemetry {

    pub const WARNING_MALWARE_PAGE_TOP: i64 = 1;


    pub const WARNING_MALWARE_PAGE_TOP_WHY_BLOCKED: i64 = 2;


    pub const WARNING_MALWARE_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 3;


    pub const WARNING_MALWARE_PAGE_TOP_IGNORE_WARNING: i64 = 4;


    pub const WARNING_MALWARE_PAGE_FRAME: i64 = 5;


    pub const WARNING_MALWARE_PAGE_FRAME_WHY_BLOCKED: i64 = 6;


    pub const WARNING_MALWARE_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 7;


    pub const WARNING_MALWARE_PAGE_FRAME_IGNORE_WARNING: i64 = 8;


    pub const WARNING_PHISHING_PAGE_TOP: i64 = 9;


    pub const WARNING_PHISHING_PAGE_TOP_WHY_BLOCKED: i64 = 10;


    pub const WARNING_PHISHING_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 11;


    pub const WARNING_PHISHING_PAGE_TOP_IGNORE_WARNING: i64 = 12;


    pub const WARNING_PHISHING_PAGE_FRAME: i64 = 13;


    pub const WARNING_PHISHING_PAGE_FRAME_WHY_BLOCKED: i64 = 14;


    pub const WARNING_PHISHING_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 15;


    pub const WARNING_PHISHING_PAGE_FRAME_IGNORE_WARNING: i64 = 16;


    pub const WARNING_UNWANTED_PAGE_TOP: i64 = 17;


    pub const WARNING_UNWANTED_PAGE_TOP_WHY_BLOCKED: i64 = 18;


    pub const WARNING_UNWANTED_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 19;


    pub const WARNING_UNWANTED_PAGE_TOP_IGNORE_WARNING: i64 = 20;


    pub const WARNING_UNWANTED_PAGE_FRAME: i64 = 21;


    pub const WARNING_UNWANTED_PAGE_FRAME_WHY_BLOCKED: i64 = 22;


    pub const WARNING_UNWANTED_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 23;


    pub const WARNING_UNWANTED_PAGE_FRAME_IGNORE_WARNING: i64 = 24;


    pub const WARNING_HARMFUL_PAGE_TOP: i64 = 25;


    pub const WARNING_HARMFUL_PAGE_TOP_WHY_BLOCKED: i64 = 26;


    pub const WARNING_HARMFUL_PAGE_TOP_GET_ME_OUT_OF_HERE: i64 = 27;


    pub const WARNING_HARMFUL_PAGE_TOP_IGNORE_WARNING: i64 = 28;


    pub const WARNING_HARMFUL_PAGE_FRAME: i64 = 29;


    pub const WARNING_HARMFUL_PAGE_FRAME_WHY_BLOCKED: i64 = 30;


    pub const WARNING_HARMFUL_PAGE_FRAME_GET_ME_OUT_OF_HERE: i64 = 31;


    pub const WARNING_HARMFUL_PAGE_FRAME_IGNORE_WARNING: i64 = 32;


}


