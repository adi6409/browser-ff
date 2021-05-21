//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/mozintl/mozIMozIntl.idl
//


/// `interface mozIMozIntl : nsISupports`
///

/// ```text
/// /**
///  * This is a set of APIs that are of general usefulness for user interface
///  * internationalization.
///  *
///  * They're all in various stages of the standardization process through
///  * ECMA402, so they are exposed to privileged content only but are written
///  * in the way to allow for easy migration to standard Intl object once
///  * the appropriate stage of the ECMA402 is achieved.
///  *
///  * The exact structure of the code is a little bit complex because of that:
///  *
///  * 1) The core is in SpiderMonkey together with other Intl APIs
///  *
///  * This allows us to write the code once, stick to the spec language
///  * of the proposal, reuse our ICU bindings in Spidermonkey and use
///  * the code to inform us on refining the spec proposal for the given API itself.
///  *
///  * 2) The MozIntlHelper API exposes the SpiderMonkey APIs
///  *
///  * This helper API allows attaching the new APIs on any regular object.
///  *
///  * 3) The MozIntl API provides the access to those APIs
///  *
///  * This API exposes the actual functionality and wraps around the MozIntlHelper
///  * lazily retrieving and setting the accessors.
///  * On top of that, the API also binds additional functionality like using
///  * current application locale by default, and fetching OS regional preferences
///  * for date time format.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIMozIntl {
vtable: *const mozIMozIntlVTable,

/// This field is a phantomdata to ensure that the VTable type and any
/// struct containing it is not safe to send across threads, as XPCOM is
/// generally not threadsafe.
///
/// XPCOM interfaces in general are not safe to send across threads.
__nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIMozIntl.
unsafe impl XpCom for mozIMozIntl {
const IID: nsIID = nsID(0x7f63279a, 0x1a29, 0x4ae6,
[0x9e, 0x7a, 0xdc, 0x96, 0x84, 0xa2, 0x35, 0x30]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIMozIntl {
#[inline]
unsafe fn addref(&self) {
self.AddRef();
}
#[inline]
unsafe fn release(&self) {
self.Release();
}
}

// This trait is implemented on all types which can be coerced to from mozIMozIntl.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIMozIntlCoerce {
/// Cheaply cast a value of this type from a `mozIMozIntl`.
fn coerce_from(v: &mozIMozIntl) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIMozIntlCoerce for mozIMozIntl {
#[inline]
fn coerce_from(v: &mozIMozIntl) -> &Self {
v
}
}

impl mozIMozIntl {
/// Cast this `mozIMozIntl` to one of its base interfaces.
#[inline]
pub fn coerce<T: mozIMozIntlCoerce>(&self) -> &T {
T::coerce_from(self)
}
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIMozIntl {
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
impl<T: nsISupportsCoerce> mozIMozIntlCoerce for T {
#[inline]
fn coerce_from(v: &mozIMozIntl) -> &Self {
T::coerce_from(v)
}
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIMozIntl
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIMozIntlVTable {
/// We need to include the members from the base interface's vtable at the start
/// of the VTable definition.
pub __base: nsISupportsVTable,

/* jsval getCalendarInfo ([optional] in jsval locales); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetCalendarInfo: *const ::libc::c_void,

/* jsval getDisplayNames ([optional] in jsval locales, [optional] in jsval options); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetDisplayNames: *const ::libc::c_void,

/* jsval getLocaleInfo ([optional] in jsval locales); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetLocaleInfo: *const ::libc::c_void,

/* jsval getAvailableLocaleDisplayNames (in jsval type); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetAvailableLocaleDisplayNames: *const ::libc::c_void,

/* jsval getLanguageDisplayNames (in jsval locales, in jsval langCodes); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetLanguageDisplayNames: *const ::libc::c_void,

/* jsval getRegionDisplayNames (in jsval locales, in jsval regionCodes); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetRegionDisplayNames: *const ::libc::c_void,

/* jsval getLocaleDisplayNames (in jsval locales, in jsval localeCodes); */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetLocaleDisplayNames: *const ::libc::c_void,

/* readonly attribute jsval Collator; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetCollator: *const ::libc::c_void,

/* readonly attribute jsval DateTimeFormat; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetDateTimeFormat: *const ::libc::c_void,

/* readonly attribute jsval ListFormat; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetListFormat: *const ::libc::c_void,

/* readonly attribute jsval Locale; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetLocale: *const ::libc::c_void,

/* readonly attribute jsval NumberFormat; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetNumberFormat: *const ::libc::c_void,

/* readonly attribute jsval PluralRules; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetPluralRules: *const ::libc::c_void,

/* readonly attribute jsval RelativeTimeFormat; */
/// Unable to generate binding because `specialtype jsval unsupported`
pub GetRelativeTimeFormat: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIMozIntl {


/// `jsval getCalendarInfo ([optional] in jsval locales);`
const _GetCalendarInfo: () = ();


/// `jsval getDisplayNames ([optional] in jsval locales, [optional] in jsval options);`
const _GetDisplayNames: () = ();


/// `jsval getLocaleInfo ([optional] in jsval locales);`
const _GetLocaleInfo: () = ();

/// ```text
/// /**
///    * Returns a list of locale codes for a given type.
///    * At the moment only type="region" is supported.
///    *
///    * Example:
///    *   let codes = getAvailableLocaleDisplayNames("region");
///    *   codes === ["ar", "ae", "af", ...]
///    */
/// ```
///

/// `jsval getAvailableLocaleDisplayNames (in jsval type);`
const _GetAvailableLocaleDisplayNames: () = ();

/// ```text
/// /**
///    * Returns a list of language names formatted for display.
///    *
///    * Example:
///    *   let langs = getLanguageDisplayNames(["pl"], ["fr", "de", "en"]);
///    *   langs === ["Francuski", "Niemiecki", "Angielski"]
///    */
/// ```
///

/// `jsval getLanguageDisplayNames (in jsval locales, in jsval langCodes);`
const _GetLanguageDisplayNames: () = ();

/// ```text
/// /**
///    * Returns a list of region names formatted for display.
///    *
///    * Example:
///    *   let regs = getRegionDisplayNames(["pl"], ["US", "CA", "MX"]);
///    *   regs === ["Stany Zjednoczone", "Kanada", "Meksyk"]
///    */
/// ```
///

/// `jsval getRegionDisplayNames (in jsval locales, in jsval regionCodes);`
const _GetRegionDisplayNames: () = ();

/// ```text
/// /**
///    * Returns a list of locale names formatted for display.
///    *
///    * Example:
///    *   let locs = getLocaleDisplayNames(["pl"], ["sr-RU", "es-MX", "fr-CA"]);
///    *   locs === ["Serbski (Rosja)", "Hiszpański (Meksyk)", "Francuski (Kanada)"]
///    */
/// ```
///

/// `jsval getLocaleDisplayNames (in jsval locales, in jsval localeCodes);`
const _GetLocaleDisplayNames: () = ();


/// `readonly attribute jsval Collator;`
const _GetCollator: () = ();


/// `readonly attribute jsval DateTimeFormat;`
const _GetDateTimeFormat: () = ();


/// `readonly attribute jsval ListFormat;`
const _GetListFormat: () = ();


/// `readonly attribute jsval Locale;`
const _GetLocale: () = ();


/// `readonly attribute jsval NumberFormat;`
const _GetNumberFormat: () = ();


/// `readonly attribute jsval PluralRules;`
const _GetPluralRules: () = ();


/// `readonly attribute jsval RelativeTimeFormat;`
const _GetRelativeTimeFormat: () = ();

}


