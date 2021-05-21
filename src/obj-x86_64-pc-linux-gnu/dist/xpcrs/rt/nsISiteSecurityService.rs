//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsISiteSecurityService.idl
//


/// `interface nsISiteSecurityState : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISiteSecurityState {
    vtable: *const nsISiteSecurityStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISiteSecurityState.
unsafe impl XpCom for nsISiteSecurityState {
    const IID: nsIID = nsID(0x31313372, 0x842c, 0x4110,
        [0xbd, 0xf1, 0x6a, 0xea, 0x17, 0xc8, 0x45, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISiteSecurityState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISiteSecurityState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISiteSecurityStateCoerce {
    /// Cheaply cast a value of this type from a `nsISiteSecurityState`.
    fn coerce_from(v: &nsISiteSecurityState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISiteSecurityStateCoerce for nsISiteSecurityState {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityState) -> &Self {
        v
    }
}

impl nsISiteSecurityState {
    /// Cast this `nsISiteSecurityState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISiteSecurityStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISiteSecurityState {
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
impl<T: nsISupportsCoerce> nsISiteSecurityStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISiteSecurityState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISiteSecurityStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] readonly attribute ACString hostname; */
    pub GetHostname: unsafe extern "system" fn (this: *const nsISiteSecurityState, aHostname: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible] readonly attribute long long expireTime; */
    pub GetExpireTime: unsafe extern "system" fn (this: *const nsISiteSecurityState, aExpireTime: *mut i64) -> ::nserror::nsresult,

    /* [infallible] readonly attribute short securityPropertyState; */
    pub GetSecurityPropertyState: unsafe extern "system" fn (this: *const nsISiteSecurityState, aSecurityPropertyState: *mut i16) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean includeSubdomains; */
    pub GetIncludeSubdomains: unsafe extern "system" fn (this: *const nsISiteSecurityState, aIncludeSubdomains: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext,must_use] readonly attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISiteSecurityState {

    pub const SECURITY_PROPERTY_UNSET: i64 = 0;


    pub const SECURITY_PROPERTY_SET: i64 = 1;


    pub const SECURITY_PROPERTY_KNOCKOUT: i64 = 2;


    pub const SECURITY_PROPERTY_NEGATIVE: i64 = 3;


    /// `[must_use] readonly attribute ACString hostname;`
    #[inline]
    pub unsafe fn GetHostname(&self, aHostname: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHostname)(self, aHostname)
    }



    /// `[infallible] readonly attribute long long expireTime;`
    #[inline]
    pub unsafe fn GetExpireTime(&self) -> i64 {
        let mut result = <i64 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetExpireTime)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute short securityPropertyState;`
    #[inline]
    pub unsafe fn GetSecurityPropertyState(&self) -> i16 {
        let mut result = <i16 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetSecurityPropertyState)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean includeSubdomains;`
    #[inline]
    pub unsafe fn GetIncludeSubdomains(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIncludeSubdomains)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[implicit_jscontext,must_use] readonly attribute jsval originAttributes;`
    const _GetOriginAttributes: () = ();

}


/// `interface nsISiteHSTSState : nsISiteSecurityState`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISiteHSTSState {
    vtable: *const nsISiteHSTSStateVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISiteHSTSState.
unsafe impl XpCom for nsISiteHSTSState {
    const IID: nsIID = nsID(0x9ff16e40, 0x1029, 0x496c,
        [0x95, 0xc2, 0xbc, 0x81, 0x98, 0x72, 0xb2, 0x16]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISiteHSTSState {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISiteHSTSState.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISiteHSTSStateCoerce {
    /// Cheaply cast a value of this type from a `nsISiteHSTSState`.
    fn coerce_from(v: &nsISiteHSTSState) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISiteHSTSStateCoerce for nsISiteHSTSState {
    #[inline]
    fn coerce_from(v: &nsISiteHSTSState) -> &Self {
        v
    }
}

impl nsISiteHSTSState {
    /// Cast this `nsISiteHSTSState` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISiteHSTSStateCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISiteHSTSState {
    type Target = nsISiteSecurityState;
    #[inline]
    fn deref(&self) -> &nsISiteSecurityState {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISiteSecurityStateCoerce> nsISiteHSTSStateCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteHSTSState) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISiteHSTSState
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISiteHSTSStateVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISiteSecurityStateVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISiteHSTSState {


}


/// `interface nsISiteSecurityService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISiteSecurityService {
    vtable: *const nsISiteSecurityServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISiteSecurityService.
unsafe impl XpCom for nsISiteSecurityService {
    const IID: nsIID = nsID(0x275127f8, 0xdbd7, 0x4681,
        [0xaf, 0xbf, 0x6d, 0xf0, 0xc6, 0x58, 0x7a, 0x01]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISiteSecurityService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISiteSecurityService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISiteSecurityServiceCoerce {
    /// Cheaply cast a value of this type from a `nsISiteSecurityService`.
    fn coerce_from(v: &nsISiteSecurityService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISiteSecurityServiceCoerce for nsISiteSecurityService {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityService) -> &Self {
        v
    }
}

impl nsISiteSecurityService {
    /// Cast this `nsISiteSecurityService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISiteSecurityServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISiteSecurityService {
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
impl<T: nsISupportsCoerce> nsISiteSecurityServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISiteSecurityService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISiteSecurityService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISiteSecurityServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [binaryname(ProcessHeader),must_use,noscript] void processHeaderNative (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsITransportSecurityInfo aSecInfo, in uint32_t aFlags, in uint32_t aSource, in const_OriginAttributesRef aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub ProcessHeader: *const ::libc::c_void,

    /* [binaryname(ProcessHeaderScriptable),implicit_jscontext,must_use,optional_argc] void processHeader (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsITransportSecurityInfo aSecInfo, in uint32_t aFlags, in uint32_t aSource, [optional] in jsval aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ProcessHeaderScriptable: *const ::libc::c_void,

    /* [implicit_jscontext,must_use,optional_argc] void resetState (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub ResetState: *const ::libc::c_void,

    /* [binaryname(IsSecureURI),must_use,noscript] boolean isSecureURINative (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out boolean aCached, [optional] out uint32_t aSource); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub IsSecureURI: *const ::libc::c_void,

    /* [binaryname(IsSecureURIScriptable),implicit_jscontext,must_use,optional_argc] boolean isSecureURI (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out boolean aCached, [optional] out uint32_t aSource); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub IsSecureURIScriptable: *const ::libc::c_void,

    /* [must_use] void clearAll (); */
    pub ClearAll: unsafe extern "system" fn (this: *const nsISiteSecurityService) -> ::nserror::nsresult,

    /* [must_use] void clearPreloads (); */
    pub ClearPreloads: unsafe extern "system" fn (this: *const nsISiteSecurityService) -> ::nserror::nsresult,

    /* [must_use] boolean setHSTSPreload (in ACString aHost, in boolean aIncludesSubdomains, in int64_t aExpires); */
    pub SetHSTSPreload: unsafe extern "system" fn (this: *const nsISiteSecurityService, aHost: *const ::nsstring::nsACString, aIncludesSubdomains: bool, aExpires: int64_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] nsISimpleEnumerator enumerate (in uint32_t aType); */
    pub Enumerate: unsafe extern "system" fn (this: *const nsISiteSecurityService, aType: uint32_t, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISiteSecurityService {

    pub const HEADER_HSTS: i64 = 0;


    pub const STATIC_PINNING: i64 = 1;


    pub const Success: i64 = 0;


    pub const ERROR_UNKNOWN: i64 = 1;


    pub const ERROR_UNTRUSTWORTHY_CONNECTION: i64 = 2;


    pub const ERROR_COULD_NOT_PARSE_HEADER: i64 = 3;


    pub const ERROR_NO_MAX_AGE: i64 = 4;


    pub const ERROR_MULTIPLE_MAX_AGES: i64 = 5;


    pub const ERROR_INVALID_MAX_AGE: i64 = 6;


    pub const ERROR_MULTIPLE_INCLUDE_SUBDOMAINS: i64 = 7;


    pub const ERROR_INVALID_INCLUDE_SUBDOMAINS: i64 = 8;


    pub const ERROR_COULD_NOT_SAVE_STATE: i64 = 13;

    /// ```text
    /// /**
    ///      * nsISiteSecurityService::IsSecureURI can optionally return a flag
    ///      * indicating the source of the HSTS cache entry, if it comes from the
    ///      * preload list, was seen naturally, or is a result of HSTS priming.
    ///      */
    /// ```
    ///

    pub const SOURCE_UNKNOWN: i64 = 0;


    pub const SOURCE_PRELOAD_LIST: i64 = 1;


    pub const SOURCE_ORGANIC_REQUEST: i64 = 2;

    /// ```text
    /// /**
    ///      * Parses a given HTTP header and records the results internally.
    ///      * Currently one header type is supported: HSTS (aka STS).
    ///      * The format of the HSTS header is defined by the HSTS specification:
    ///      * https://tools.ietf.org/html/rfc6797
    ///      * and allows a host to specify that future HTTP requests should be
    ///      * upgraded to HTTPS.
    ///      *
    ///      * @param aType the type of security header in question.
    ///      * @param aSourceURI the URI of the resource with the HTTP header.
    ///      * @param aHeader the HTTP response header specifying security data.
    ///      * @param aSecInfo the TransportSecurityInfo of the current channel.
    ///      * @param aFlags  options for this request as defined in nsISocketProvider:
    ///      *                  NO_PERMANENT_STORAGE
    ///      * @param aOriginAttributes the origin attributes that isolate this origin,
    ///      *                          (note that this implementation does not isolate
        ///      *                          by userContextId because of the risk of man-in-
        ///      *                          the-middle attacks before trust-on-second-use
        ///      *                          happens).
    ///      * @param aSource the source of the header, whether it was from the preload
    ///      *                list, an organic header, or HSTS priming, or unknown.
    ///      * @param aMaxAge the parsed max-age directive of the header.
    ///      * @param aIncludeSubdomains the parsed includeSubdomains directive.
    ///      * @param aFailureResult a more specific failure result if NS_ERROR_FAILURE
    ///                              was returned.
    ///      * @return NS_OK            if it succeeds
    ///      *         NS_ERROR_FAILURE if it can't be parsed
    ///      *         NS_SUCCESS_LOSS_OF_INSIGNIFICANT_DATA
    ///      *                          if there are unrecognized tokens in the header.
    ///      */
    /// ```
    ///

    /// `[binaryname(ProcessHeader),must_use,noscript] void processHeaderNative (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsITransportSecurityInfo aSecInfo, in uint32_t aFlags, in uint32_t aSource, in const_OriginAttributesRef aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult);`
    const _ProcessHeader: () = ();


    /// `[binaryname(ProcessHeaderScriptable),implicit_jscontext,must_use,optional_argc] void processHeader (in uint32_t aType, in nsIURI aSourceURI, in ACString aHeader, in nsITransportSecurityInfo aSecInfo, in uint32_t aFlags, in uint32_t aSource, [optional] in jsval aOriginAttributes, [optional] out unsigned long long aMaxAge, [optional] out boolean aIncludeSubdomains, [optional] out uint32_t aFailureResult);`
    const _ProcessHeaderScriptable: () = ();

    /// ```text
    /// /**
    ///      * Given a header type, resets state relating to that header of a host,
    ///      * including the includeSubdomains state that would affect subdomains.
    ///      * This essentially removes the state for the domain tree rooted at this
    ///      * host. If any preloaded information is present for that host, that
    ///      * information will then be used instead of any other previously existing
    ///      * state.
    ///      *
    ///      * @param aType   the type of security state in question
    ///      * @param aURI    the URI of the target host
    ///      * @param aFlags  options for this request as defined in nsISocketProvider:
    ///      *                  NO_PERMANENT_STORAGE
    ///      * @param aOriginAttributes the origin attributes that isolate this origin,
    ///      *                          (note that this implementation does not isolate
        ///      *                          by userContextId because of the risk of man-in-
        ///      *                          the-middle attacks before trust-on-second-use
        ///      *                          happens).
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext,must_use,optional_argc] void resetState (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes);`
    const _ResetState: () = ();

    /// ```text
    /// /**
    ///      * Checks whether or not the URI's hostname has a given security state set.
    ///      * For example, for HSTS:
    ///      * The URI is an HSTS URI if either the host has the HSTS state set, or one
    ///      * of its super-domains has the HSTS "includeSubdomains" flag set.
    ///      * NOTE: this function makes decisions based only on the
    ///      * host contained in the URI, and disregards other portions of the URI
    ///      * such as path and port.
    ///      *
    ///      * @param aType the type of security state in question.
    ///      * @param aURI the URI to query for STS state.
    ///      * @param aFlags  options for this request as defined in nsISocketProvider:
    ///      *                  NO_PERMANENT_STORAGE
    ///      * @param aOriginAttributes the origin attributes that isolate this origin,
    ///      *                          (note that this implementation does not isolate
        ///      *                          by userContextId because of the risk of man-in-
        ///      *                          the-middle attacks before trust-on-second-use
        ///      *                          happens).
    ///      * @param aCached true if we have cached information about this host, even
    ///      *                if the security state is false.
    ///      * @param aSource the source of the HSTS entry. One of SOURCE_PRELOAD_LIST,
    ///      *                SOURCE_ORGANIC_REQUEST, SOURCE_HSTS_PRIMING, or
    ///      *                SOURCE_UNKNOWN.
    ///      */
    /// ```
    ///

    /// `[binaryname(IsSecureURI),must_use,noscript] boolean isSecureURINative (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, in const_OriginAttributesRef aOriginAttributes, [optional] out boolean aCached, [optional] out uint32_t aSource);`
    const _IsSecureURI: () = ();


    /// `[binaryname(IsSecureURIScriptable),implicit_jscontext,must_use,optional_argc] boolean isSecureURI (in uint32_t aType, in nsIURI aURI, in uint32_t aFlags, [optional] in jsval aOriginAttributes, [optional] out boolean aCached, [optional] out uint32_t aSource);`
    const _IsSecureURIScriptable: () = ();

    /// ```text
    /// /**
    ///      * Removes all non-preloaded security state by resetting to factory-original
    ///      * settings.
    ///      */
    /// ```
    ///

    /// `[must_use] void clearAll ();`
    #[inline]
    pub unsafe fn ClearAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearAll)(self, )
    }


    /// ```text
    /// /**
    ///      * Removes all preloaded security state.
    ///      */
    /// ```
    ///

    /// `[must_use] void clearPreloads ();`
    #[inline]
    pub unsafe fn ClearPreloads(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearPreloads)(self, )
    }


    /// ```text
    /// /**
    ///      * Set an HSTS preload entry for a host. The resulting entries will be
    ///      * permanent and visible from private and non-private contexts. These
    ///      * entries replace any already set by this mechanism or those built-in to
    ///      * Gecko.
    ///      *
    ///      * @param aHost the hostname (punycode) that the entry applies to
    ///      * @param aIncludeSubdomains whether this entry also applies to subdomains
    ///      * @param aExpires the time this entry should expire (millis since epoch)
    ///      */
    /// ```
    ///

    /// `[must_use] boolean setHSTSPreload (in ACString aHost, in boolean aIncludesSubdomains, in int64_t aExpires);`
    #[inline]
    pub unsafe fn SetHSTSPreload(&self, aHost: *const ::nsstring::nsACString, aIncludesSubdomains: bool, aExpires: int64_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SetHSTSPreload)(self, aHost, aIncludesSubdomains, aExpires, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns an enumerator of the nsISiteSecurityService storage. Each item in
    ///      * the enumeration is a nsISiteSecurityState that can be QueryInterfaced to
    ///      * nsISiteHSTSState.
    ///      * Doesn't include preloaded entries (either the hard-coded ones or the
        ///      * preloaded-delivered-by-kinto ones).
    ///      *
    ///      * @param aType the type of security state in question.
    ///      */
    /// ```
    ///

    /// `[must_use] nsISimpleEnumerator enumerate (in uint32_t aType);`
    #[inline]
    pub unsafe fn Enumerate(&self, aType: uint32_t, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Enumerate)(self, aType, _retval)
    }


}


