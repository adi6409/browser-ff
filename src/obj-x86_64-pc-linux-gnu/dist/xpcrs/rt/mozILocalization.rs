//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/l10n/mozILocalization.idl
//


/// `interface mozILocalization : nsISupports`
///

/// ```text
/// /**
///  * mozILocalization is an internal API used by Localization class for formatting of translation
///  * units.
///  *
///  * There are three main formatting methods:
///  *    - formatMessages - formats a list of messages based on the requested keys.
///  *    - formatValues   - formats a list of values of messages based on the requested keys.
///  *    - formatValue    - formats a single value based on the requested id and args.
///  *
///  * Each method has a `Sync` variant that can be used if the instance is in the `sync` state.
///  * This mode is enabled via passing `true` to `aIsSync` either in `activate` or `onChange` method.
///  *
///  * When this value is set to `false`, those methods will throw.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozILocalization {
    vtable: *const mozILocalizationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozILocalization.
unsafe impl XpCom for mozILocalization {
    const IID: nsIID = nsID(0x7d468600, 0x551f, 0x4fe0,
        [0x98, 0xc9, 0x92, 0xa5, 0x3b, 0x63, 0xec, 0x8d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozILocalization {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozILocalization.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozILocalizationCoerce {
    /// Cheaply cast a value of this type from a `mozILocalization`.
    fn coerce_from(v: &mozILocalization) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozILocalizationCoerce for mozILocalization {
    #[inline]
    fn coerce_from(v: &mozILocalization) -> &Self {
        v
    }
}

impl mozILocalization {
    /// Cast this `mozILocalization` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozILocalizationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozILocalization {
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
impl<T: nsISupportsCoerce> mozILocalizationCoerce for T {
    #[inline]
    fn coerce_from(v: &mozILocalization) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozILocalization
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozILocalizationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* jsval generateBundles (in Array<AString> aResourceIds, in bool aIsSync, in bool eager, in jsval aGenerateBundles, in jsval aGenerateBundlesSync); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GenerateBundles: *const ::libc::c_void,

    /* Promise formatMessages (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FormatMessages: *const ::libc::c_void,

    /* Promise formatValues (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FormatValues: *const ::libc::c_void,

    /* Promise formatValue (in Array<AString> aResourceIds, in jsval aBundles, in AUTF8String aId, [optional] in jsval aArgs); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FormatValue: *const ::libc::c_void,

    /* AUTF8String formatValueSync (in Array<AString> aResourceIds, in jsval aBundles, in AUTF8String aId, [optional] in jsval aArgs); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FormatValueSync: *const ::libc::c_void,

    /* Array<AUTF8String> formatValuesSync (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FormatValuesSync: *const ::libc::c_void,

    /* Array<jsval> formatMessagesSync (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub FormatMessagesSync: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozILocalization {


    /// `jsval generateBundles (in Array<AString> aResourceIds, in bool aIsSync, in bool eager, in jsval aGenerateBundles, in jsval aGenerateBundlesSync);`
    const _GenerateBundles: () = ();


    /// `Promise formatMessages (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys);`
    const _FormatMessages: () = ();


    /// `Promise formatValues (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys);`
    const _FormatValues: () = ();


    /// `Promise formatValue (in Array<AString> aResourceIds, in jsval aBundles, in AUTF8String aId, [optional] in jsval aArgs);`
    const _FormatValue: () = ();


    /// `AUTF8String formatValueSync (in Array<AString> aResourceIds, in jsval aBundles, in AUTF8String aId, [optional] in jsval aArgs);`
    const _FormatValueSync: () = ();


    /// `Array<AUTF8String> formatValuesSync (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys);`
    const _FormatValuesSync: () = ();


    /// `Array<jsval> formatMessagesSync (in Array<AString> aResourceIds, in jsval aBundles, in Array<jsval> aKeys);`
    const _FormatMessagesSync: () = ();

}


