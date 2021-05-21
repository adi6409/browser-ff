//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/mozIOSPreferences.idl
//


/// `interface mozIOSPreferences : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIOSPreferences {
    vtable: *const mozIOSPreferencesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIOSPreferences.
unsafe impl XpCom for mozIOSPreferences {
    const IID: nsIID = nsID(0x65944815, 0xe9ae, 0x48bd,
        [0xa2, 0xbf, 0xf1, 0x10, 0x87, 0x20, 0x95, 0x0c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIOSPreferences {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIOSPreferences.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIOSPreferencesCoerce {
    /// Cheaply cast a value of this type from a `mozIOSPreferences`.
    fn coerce_from(v: &mozIOSPreferences) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIOSPreferencesCoerce for mozIOSPreferences {
    #[inline]
    fn coerce_from(v: &mozIOSPreferences) -> &Self {
        v
    }
}

impl mozIOSPreferences {
    /// Cast this `mozIOSPreferences` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIOSPreferencesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIOSPreferences {
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
impl<T: nsISupportsCoerce> mozIOSPreferencesCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIOSPreferences) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIOSPreferences
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIOSPreferencesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute Array<ACString> systemLocales; */
    pub GetSystemLocales: unsafe extern "system" fn (this: *const mozIOSPreferences, aSystemLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> regionalPrefsLocales; */
    pub GetRegionalPrefsLocales: unsafe extern "system" fn (this: *const mozIOSPreferences, aRegionalPrefsLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute ACString systemLocale; */
    pub GetSystemLocale: unsafe extern "system" fn (this: *const mozIOSPreferences, aSystemLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getDateTimePattern (in long timeFormatStyle, in long dateFormatStyle, [optional] in ACString locale); */
    pub GetDateTimePattern: unsafe extern "system" fn (this: *const mozIOSPreferences, timeFormatStyle: i32, dateFormatStyle: i32, locale: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIOSPreferences {

    pub const dateTimeFormatStyleNone: i64 = 0;


    pub const dateTimeFormatStyleShort: i64 = 1;


    pub const dateTimeFormatStyleMedium: i64 = 2;


    pub const dateTimeFormatStyleLong: i64 = 3;


    pub const dateTimeFormatStyleFull: i64 = 4;

    /// ```text
    /// /**
    ///    * Returns a list of locales used by the host environment for UI
    ///    * localization.
    ///    *
    ///    * The result is a sorted list and we expect that the OS attempts to
    ///    * use the top locale from the list for which it has data.
    ///    *
    ///    * Each element of the list is a valid locale ID that can be passed to ICU
    ///    * and ECMA402 Intl APIs,
    ///    * At the same time each element is a valid BCP47 language tag that can be
    ///    * used for language negotiation.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> systemLocales;`
    #[inline]
    pub unsafe fn GetSystemLocales(&self, aSystemLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetSystemLocales)(self, aSystemLocales)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales used by host environment for regional
    ///    * preferences internationalization.
    ///    *
    ///    * The result is a sorted list and we expect that the OS attempts to
    ///    * use the top locale from the list for which it has data.
    ///    *
    ///    * Each element of the list is a valid locale ID that can be passed to ICU
    ///    * and ECMA402 Intl APIs,
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> regionalPrefsLocales;`
    #[inline]
    pub unsafe fn GetRegionalPrefsLocales(&self, aRegionalPrefsLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetRegionalPrefsLocales)(self, aRegionalPrefsLocales)
    }


    /// ```text
    /// /**
    ///    * Returns the best locale that the host environment is localized to.
    ///    *
    ///    * The result is a valid locale ID and it should be
    ///    * used for all APIs that do not handle language negotiation.
    ///    *
    ///    * In any scenario involving language negotiation, systemLocales should
    ///    * be preferred over the single value.
    ///    *
    ///    * Example: "zh-Hans-HK"
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString systemLocale;`
    #[inline]
    pub unsafe fn GetSystemLocale(&self, aSystemLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSystemLocale)(self, aSystemLocale)
    }


    /// ```text
    /// /**
    ///    * Returns the best possible date/time pattern for the host environment
    ///    * taking into account date/time regional settings user defined in the OS
    ///    * preferences.
    ///    *
    ///    * Notice, that depending on the OS it may take into account those settings
    ///    * for all locales, or only if the locale matches the OS locale.
    ///    *
    ///    * It takes two integer arguments that must be valid `dateTimeFormatStyle*`
    ///    * values (see constants defined above), and a string representing a
    ///    * BCP47 locale.
    ///    *
    ///    * It returns a string with a LDML date/time pattern.
    ///    *
    ///    * If no pattern can be retrieved from the host environment, it will
    ///    * lookup the best available pattern from ICU.
    ///    *
    ///    * Notice, this is a pretty unique method in this API in that it does
    ///    * more than look up into host environment.
    ///    * The reason for that is that constructing the right date/time pattern
    ///    * requires a lot of OS-specific logic and it ends up being easier to just
    ///    * handle all scenarios, including with cases where we fail to retrieve
    ///    * anything from the OS, here.
    ///    */
    /// ```
    ///

    /// `AUTF8String getDateTimePattern (in long timeFormatStyle, in long dateFormatStyle, [optional] in ACString locale);`
    #[inline]
    pub unsafe fn GetDateTimePattern(&self, timeFormatStyle: i32, dateFormatStyle: i32, locale: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDateTimePattern)(self, timeFormatStyle, dateFormatStyle, locale, _retval)
    }


}


