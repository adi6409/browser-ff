//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/intl/locale/mozILocaleService.idl
//


/// `interface mozILocaleService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozILocaleService {
    vtable: *const mozILocaleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozILocaleService.
unsafe impl XpCom for mozILocaleService {
    const IID: nsIID = nsID(0xc27f8983, 0xb48b, 0x4d1a,
        [0x92, 0xd7, 0xfe, 0xb8, 0x10, 0x6f, 0x21, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozILocaleService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozILocaleService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozILocaleServiceCoerce {
    /// Cheaply cast a value of this type from a `mozILocaleService`.
    fn coerce_from(v: &mozILocaleService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozILocaleServiceCoerce for mozILocaleService {
    #[inline]
    fn coerce_from(v: &mozILocaleService) -> &Self {
        v
    }
}

impl mozILocaleService {
    /// Cast this `mozILocaleService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozILocaleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozILocaleService {
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
impl<T: nsISupportsCoerce> mozILocaleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozILocaleService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozILocaleService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozILocaleServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString defaultLocale; */
    pub GetDefaultLocale: unsafe extern "system" fn (this: *const mozILocaleService, aDefaultLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString lastFallbackLocale; */
    pub GetLastFallbackLocale: unsafe extern "system" fn (this: *const mozILocaleService, aLastFallbackLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> appLocalesAsLangTags; */
    pub GetAppLocalesAsLangTags: unsafe extern "system" fn (this: *const mozILocaleService, aAppLocalesAsLangTags: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> appLocalesAsBCP47; */
    pub GetAppLocalesAsBCP47: unsafe extern "system" fn (this: *const mozILocaleService, aAppLocalesAsBCP47: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> regionalPrefsLocales; */
    pub GetRegionalPrefsLocales: unsafe extern "system" fn (this: *const mozILocaleService, aRegionalPrefsLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> webExposedLocales; */
    pub GetWebExposedLocales: unsafe extern "system" fn (this: *const mozILocaleService, aWebExposedLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* Array<ACString> negotiateLanguages (in Array<AUTF8String> aRequested, in Array<AUTF8String> aAvailable, [optional] in ACString aDefaultLocale, [optional] in long langNegStrategy); */
    pub NegotiateLanguages: unsafe extern "system" fn (this: *const mozILocaleService, aRequested: *const thin_vec::ThinVec<::nsstring::nsCString>, aAvailable: *const thin_vec::ThinVec<::nsstring::nsCString>, aDefaultLocale: *const ::nsstring::nsACString, langNegStrategy: i32, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute ACString appLocaleAsLangTag; */
    pub GetAppLocaleAsLangTag: unsafe extern "system" fn (this: *const mozILocaleService, aAppLocaleAsLangTag: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString appLocaleAsBCP47; */
    pub GetAppLocaleAsBCP47: unsafe extern "system" fn (this: *const mozILocaleService, aAppLocaleAsBCP47: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute Array<ACString> requestedLocales; */
    pub GetRequestedLocales: unsafe extern "system" fn (this: *const mozILocaleService, aRequestedLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* attribute Array<ACString> requestedLocales; */
    pub SetRequestedLocales: unsafe extern "system" fn (this: *const mozILocaleService, aRequestedLocales: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute ACString requestedLocale; */
    pub GetRequestedLocale: unsafe extern "system" fn (this: *const mozILocaleService, aRequestedLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute Array<ACString> availableLocales; */
    pub GetAvailableLocales: unsafe extern "system" fn (this: *const mozILocaleService, aAvailableLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* attribute Array<ACString> availableLocales; */
    pub SetAvailableLocales: unsafe extern "system" fn (this: *const mozILocaleService, aAvailableLocales: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute boolean isAppLocaleRTL; */
    pub GetIsAppLocaleRTL: unsafe extern "system" fn (this: *const mozILocaleService, aIsAppLocaleRTL: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute Array<ACString> packagedLocales; */
    pub GetPackagedLocales: unsafe extern "system" fn (this: *const mozILocaleService, aPackagedLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozILocaleService {
    /// ```text
    /// /**
    ///    * List of language negotiation strategies to use.
    ///    * For an example list of requested and available locales:
    ///    *
    ///    *   Requested: ['es-MX', 'fr-FR']
    ///    *   Available: ['fr', 'fr-CA', 'es', 'es-MX', 'it']
    ///    *   DefaultLocale: ['en-US']
    ///    *
    ///    * each of those strategies will build a different result:
    ///    *
    ///    *
    ///    * filtering (default) -
    ///    *   Matches as many of the available locales as possible.
    ///    *
    ///    *   Result:
    ///    *     Supported: ['es-MX', 'es', 'fr', 'fr-CA', 'en-US']
    ///    *
    ///    * matching -
    ///    *   Matches the best match from the available locales for every requested
    ///    *   locale.
    ///    *
    ///    *   Result:
    ///    *     Supported: ['es-MX', 'fr', 'en-US']
    ///    *
    ///    * lookup -
    ///    *   Matches a single best locale. This strategy always returns a list
    ///    *   of the length 1 and requires a defaultLocale to be set.
    ///    *
    ///    *   Result:
    ///    *     Supported: ['es-MX']
    ///    */
    /// ```
    ///

    pub const langNegStrategyFiltering: i64 = 0;


    pub const langNegStrategyMatching: i64 = 1;


    pub const langNegStrategyLookup: i64 = 2;

    /// ```text
    /// /**
    ///    * Default locale of the browser. The locale we are guaranteed to have
    ///    * resources for that should be used as a last resort fallack in cases
    ///    * where requested locales do not match available locales.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString defaultLocale;`
    #[inline]
    pub unsafe fn GetDefaultLocale(&self, aDefaultLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultLocale)(self, aDefaultLocale)
    }


    /// ```text
    /// /**
    ///    * Last fallback is the final fallback locale we're going to attempt if all
    ///    * else fails in any language negotiation or locale resource retrieval situations.
    ///    *
    ///    * At the moment it returns `en-US`.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString lastFallbackLocale;`
    #[inline]
    pub unsafe fn GetLastFallbackLocale(&self, aLastFallbackLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLastFallbackLocale)(self, aLastFallbackLocale)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales that the application should be localized to.
    ///    *
    ///    * The result is a ordered list of valid locale IDs and it should be
    ///    * used for all APIs that accept list of locales, like ECMA402 and L10n APIs.
    ///    *
    ///    * This API always returns at least one locale.
    ///    *
    ///    * When retrieving the locales for language negotiation and matching
    ///    * to language resources, use the language tag form.
    ///    * When retrieving the locales for Intl API or ICU locale settings,
    ///    * use the BCP47 form.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> appLocalesAsLangTags;`
    #[inline]
    pub unsafe fn GetAppLocalesAsLangTags(&self, aAppLocalesAsLangTags: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAppLocalesAsLangTags)(self, aAppLocalesAsLangTags)
    }



    /// `readonly attribute Array<ACString> appLocalesAsBCP47;`
    #[inline]
    pub unsafe fn GetAppLocalesAsBCP47(&self, aAppLocalesAsBCP47: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAppLocalesAsBCP47)(self, aAppLocalesAsBCP47)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales to use for any regional specific operations
    ///    * like date formatting, calendars, unit formatting etc.
    ///    *
    ///    * The result is a ordered list of valid locale IDs and it should be
    ///    * used for all APIs that accept list of locales, like ECMA402 and L10n APIs.
    ///    *
    ///    * This API always returns at least one locale.
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



    /// `readonly attribute Array<ACString> webExposedLocales;`
    #[inline]
    pub unsafe fn GetWebExposedLocales(&self, aWebExposedLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetWebExposedLocales)(self, aWebExposedLocales)
    }


    /// ```text
    /// /**
    ///    * Negotiates the best locales out of a ordered list of requested locales and
    ///    * a list of available locales.
    ///    *
    ///    * Internally it uses the following naming scheme:
    ///    *
    ///    *  Requested - locales requested by the user
    ///    *  Available - locales for which the data is available
    ///    *  Supported - locales negotiated by the algorithm
    ///    *
    ///    * Additionally, if defaultLocale is provided, it adds it to the end of the
    ///    * result list as a "last resort" locale.
    ///    *
    ///    * Strategy is one of the three strategies described at the top of this file.
    ///    *
    ///    * The result list is canonicalized and ordered according to the order
    ///    * of the requested locales.
    ///    *
    ///    * (See LocaleService.h for a more C++-friendly version of this.)
    ///    */
    /// ```
    ///

    /// `Array<ACString> negotiateLanguages (in Array<AUTF8String> aRequested, in Array<AUTF8String> aAvailable, [optional] in ACString aDefaultLocale, [optional] in long langNegStrategy);`
    #[inline]
    pub unsafe fn NegotiateLanguages(&self, aRequested: *const thin_vec::ThinVec<::nsstring::nsCString>, aAvailable: *const thin_vec::ThinVec<::nsstring::nsCString>, aDefaultLocale: *const ::nsstring::nsACString, langNegStrategy: i32, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).NegotiateLanguages)(self, aRequested, aAvailable, aDefaultLocale, langNegStrategy, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the best locale that the application should be localized to.
    ///    *
    ///    * The result is a valid locale ID and it should be
    ///    * used for all APIs that do not handle language negotiation.
    ///    *
    ///    * When retrieving the locales for language negotiation and matching
    ///    * to language resources, use the language tag form.
    ///    * When retrieving the locales for Intl API or ICU locale settings,
    ///    * use the BCP47 form.
    ///    *
    ///    * Where possible, appLocales* should be preferred over this API and
    ///    * all callsites should handle some form of "best effort" language
    ///    * negotiation to respect user preferences in case the use case does
    ///    * not have data for the first locale in the list.
    ///    *
    ///    * Example: "zh-Hans-HK"
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString appLocaleAsLangTag;`
    #[inline]
    pub unsafe fn GetAppLocaleAsLangTag(&self, aAppLocaleAsLangTag: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppLocaleAsLangTag)(self, aAppLocaleAsLangTag)
    }



    /// `readonly attribute ACString appLocaleAsBCP47;`
    #[inline]
    pub unsafe fn GetAppLocaleAsBCP47(&self, aAppLocaleAsBCP47: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAppLocaleAsBCP47)(self, aAppLocaleAsBCP47)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales that the user requested the app to be
    ///    * localized to.
    ///    *
    ///    * The result is an ordered list of locale IDs which should be
    ///    * used as a requestedLocales input list for language negotiation.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `attribute Array<ACString> requestedLocales;`
    #[inline]
    pub unsafe fn GetRequestedLocales(&self, aRequestedLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestedLocales)(self, aRequestedLocales)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales that the user requested the app to be
    ///    * localized to.
    ///    *
    ///    * The result is an ordered list of locale IDs which should be
    ///    * used as a requestedLocales input list for language negotiation.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `attribute Array<ACString> requestedLocales;`
    #[inline]
    pub unsafe fn SetRequestedLocales(&self, aRequestedLocales: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).SetRequestedLocales)(self, aRequestedLocales)
    }


    /// ```text
    /// /**
    ///    * Returns the top-requested locale from the user, or an empty string if none is set.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString requestedLocale;`
    #[inline]
    pub unsafe fn GetRequestedLocale(&self, aRequestedLocale: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRequestedLocale)(self, aRequestedLocale)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales that the app can be localized to.
    ///    *
    ///    * The result is an unordered list of locale IDs which should be
    ///    * used as a availableLocales input list for language negotiation.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `attribute Array<ACString> availableLocales;`
    #[inline]
    pub unsafe fn GetAvailableLocales(&self, aAvailableLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetAvailableLocales)(self, aAvailableLocales)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales that the app can be localized to.
    ///    *
    ///    * The result is an unordered list of locale IDs which should be
    ///    * used as a availableLocales input list for language negotiation.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `attribute Array<ACString> availableLocales;`
    #[inline]
    pub unsafe fn SetAvailableLocales(&self, aAvailableLocales: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).SetAvailableLocales)(self, aAvailableLocales)
    }


    /// ```text
    /// /**
    ///    * Returns whether the current app locale is RTL.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isAppLocaleRTL;`
    #[inline]
    pub unsafe fn GetIsAppLocaleRTL(&self, aIsAppLocaleRTL: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsAppLocaleRTL)(self, aIsAppLocaleRTL)
    }


    /// ```text
    /// /**
    ///    * Returns a list of locales packaged into the app bundle.
    ///    *
    ///    * Example: ["en-US", "de", "pl", "sr-Cyrl", "zh-Hans-HK"]
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<ACString> packagedLocales;`
    #[inline]
    pub unsafe fn GetPackagedLocales(&self, aPackagedLocales: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetPackagedLocales)(self, aPackagedLocales)
    }


}


