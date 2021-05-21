//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/mozintl/mozIMozIntlHelper.idl
//


/// `interface mozIMozIntlHelper : nsISupports`
///

/// ```text
/// /**
///  * This is an internal helper for mozIMozIntl API. There should be virtually
///  * no reason for you to call this API except from mozIMozIntl implementation.
///  *
///  * This API helps accessing the SpiderMonkey Intl APIs, but it is mozIMozIntl
///  * that exposes the thin wrapper around them that binds the functionality
///  * to Gecko.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIMozIntlHelper {
    vtable: *const mozIMozIntlHelperVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIMozIntlHelper.
unsafe impl XpCom for mozIMozIntlHelper {
    const IID: nsIID = nsID(0x189eaa7d, 0xb29a, 0x43a9,
        [0xb1, 0xfb, 0x76, 0x58, 0x99, 0x0d, 0xf9, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIMozIntlHelper {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIMozIntlHelper.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIMozIntlHelperCoerce {
    /// Cheaply cast a value of this type from a `mozIMozIntlHelper`.
    fn coerce_from(v: &mozIMozIntlHelper) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIMozIntlHelperCoerce for mozIMozIntlHelper {
    #[inline]
    fn coerce_from(v: &mozIMozIntlHelper) -> &Self {
        v
    }
}

impl mozIMozIntlHelper {
    /// Cast this `mozIMozIntlHelper` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIMozIntlHelperCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIMozIntlHelper {
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
impl<T: nsISupportsCoerce> mozIMozIntlHelperCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIMozIntlHelper) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIMozIntlHelper
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIMozIntlHelperVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void addGetCalendarInfo (in jsval intlObject); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddGetCalendarInfo: *const ::libc::c_void,

    /* [implicit_jscontext] void addGetDisplayNames (in jsval intlObject); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddGetDisplayNames: *const ::libc::c_void,

    /* [implicit_jscontext] void addGetLocaleInfo (in jsval intlObject); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddGetLocaleInfo: *const ::libc::c_void,

    /* [implicit_jscontext] void addDateTimeFormatConstructor (in jsval intlObject); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddDateTimeFormatConstructor: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIMozIntlHelper {


    /// `[implicit_jscontext] void addGetCalendarInfo (in jsval intlObject);`
    const _AddGetCalendarInfo: () = ();


    /// `[implicit_jscontext] void addGetDisplayNames (in jsval intlObject);`
    const _AddGetDisplayNames: () = ();


    /// `[implicit_jscontext] void addGetLocaleInfo (in jsval intlObject);`
    const _AddGetLocaleInfo: () = ();

    /// ```text
    /// /**
    ///    * Adds a MozDateTimeFormat contructor to the given object. This function may only
    ///    * be called once within a realm/global object: calling it multiple times will
    ///    * throw.
    ///    *
    ///    * The difference between regular Intl.DateTimeFormat and the method created here
    ///    * is that we support two more options:
    ///    *
    ///    *    timeStyle: full | long | medium | short
    ///    *    dateStyle: full | long | medium | short
    ///    *
    ///    * which allow user to create normalized date/time style formats.
    ///    * Additionally, when those options are used instead of the regular atomic
    ///    * options (hour, minute, month, etc.) this code will look into host
    ///    * Operating System regional preferences and adjust for that.
    ///    *
    ///    * That means that if user will manually select time format (hour12/24) or
    ///    * adjust how the date should be displayed, MozDateTimeFormat will use that.
    ///    *
    ///    * This API should be used everywhere in the UI instead of regular Intl  API.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void addDateTimeFormatConstructor (in jsval intlObject);`
    const _AddDateTimeFormatConstructor: () = ();

}


