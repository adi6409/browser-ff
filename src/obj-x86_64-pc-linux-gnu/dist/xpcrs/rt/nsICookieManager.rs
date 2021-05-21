//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieManager.idl
//


/// `interface nsICookieManager : nsISupports`
///

/// ```text
/// /**
///  * An optional interface for accessing or removing the cookies
///  * that are in the cookie list
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICookieManager {
    vtable: *const nsICookieManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICookieManager.
unsafe impl XpCom for nsICookieManager {
    const IID: nsIID = nsID(0xaaab6710, 0x0f2c, 0x11d5,
        [0xa5, 0x3b, 0x00, 0x10, 0xa4, 0x01, 0xeb, 0x10]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICookieManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICookieManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICookieManagerCoerce {
    /// Cheaply cast a value of this type from a `nsICookieManager`.
    fn coerce_from(v: &nsICookieManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICookieManagerCoerce for nsICookieManager {
    #[inline]
    fn coerce_from(v: &nsICookieManager) -> &Self {
        v
    }
}

impl nsICookieManager {
    /// Cast this `nsICookieManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICookieManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICookieManager {
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
impl<T: nsISupportsCoerce> nsICookieManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICookieManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICookieManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void removeAll (); */
    pub RemoveAll: unsafe extern "system" fn (this: *const nsICookieManager) -> ::nserror::nsresult,

    /* readonly attribute Array<nsICookie> cookies; */
    pub GetCookies: unsafe extern "system" fn (this: *const nsICookieManager, aCookies: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult,

    /* readonly attribute Array<nsICookie> sessionCookies; */
    pub GetSessionCookies: unsafe extern "system" fn (this: *const nsICookieManager, aSessionCookies: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult,

    /* readonly attribute uint32_t cookieBehavior; */
    pub GetCookieBehavior: unsafe extern "system" fn (this: *const nsICookieManager, aCookieBehavior: *mut uint32_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] void remove (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Remove: *const ::libc::c_void,

    /* [notxpcom] nsresult removeNative (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in OriginAttributesPtr aOriginAttributes); */
    /// Unable to generate binding because `native type mozilla::OriginAttributes unsupported`
    pub RemoveNative: *const ::libc::c_void,

    /* [implicit_jscontext] void add (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in AUTF8String aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in jsval aOriginAttributes, in int32_t aSameSite, in nsICookie_schemeType aSchemeMap); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Add: *const ::libc::c_void,

    /* [notxpcom] nsresult addNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in AUTF8String aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in OriginAttributesPtr aOriginAttributes, in int32_t aSameSite, in nsICookie_schemeType aSchemeMap); */
    /// Unable to generate binding because `native type mozilla::OriginAttributes unsupported`
    pub AddNative: *const ::libc::c_void,

    /* [implicit_jscontext] boolean cookieExists (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CookieExists: *const ::libc::c_void,

    /* [notxpcom] nsresult cookieExistsNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in OriginAttributesPtr aOriginAttributes, out boolean aExists); */
    /// Unable to generate binding because `native type mozilla::OriginAttributes unsupported`
    pub CookieExistsNative: *const ::libc::c_void,

    /* unsigned long countCookiesFromHost (in AUTF8String aHost); */
    pub CountCookiesFromHost: unsafe extern "system" fn (this: *const nsICookieManager, aHost: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult,

    /* [implicit_jscontext] Array<nsICookie> getCookiesFromHost (in AUTF8String aHost, in jsval aOriginAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetCookiesFromHost: *const ::libc::c_void,

    /* Array<nsICookie> getCookiesWithOriginAttributes (in AString aPattern, [optional] in AUTF8String aHost); */
    pub GetCookiesWithOriginAttributes: unsafe extern "system" fn (this: *const nsICookieManager, aPattern: *const ::nsstring::nsAString, aHost: *const ::nsstring::nsACString, _retval: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult,

    /* void removeCookiesWithOriginAttributes (in AString aPattern, [optional] in AUTF8String aHost); */
    pub RemoveCookiesWithOriginAttributes: unsafe extern "system" fn (this: *const nsICookieManager, aPattern: *const ::nsstring::nsAString, aHost: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void removeCookiesFromExactHost (in AUTF8String aHost, in AString aPattern); */
    pub RemoveCookiesFromExactHost: unsafe extern "system" fn (this: *const nsICookieManager, aHost: *const ::nsstring::nsACString, aPattern: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] Promise removeAllSince (in int64_t aSinceWhen); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub RemoveAllSince: *const ::libc::c_void,

    /* Array<nsICookie> getCookiesSince (in int64_t aSinceWhen); */
    pub GetCookiesSince: unsafe extern "system" fn (this: *const nsICookieManager, aSinceWhen: int64_t, _retval: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICookieManager {

    /// ```text
    /// /**
    ///    * Called to remove all cookies from the cookie list
    ///    */
    /// ```
    ///

    /// `void removeAll ();`
    #[inline]
    pub unsafe fn RemoveAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveAll)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns an array of cookies in the cookie list.
    ///    * The objects in the array are of type nsICookie
    ///    * This array only contains non-private browsing cookies.
    ///    * To retrieve an array of private browsing cookies, use
    ///    * getCookiesWithOriginAttributes.
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<nsICookie> cookies;`
    #[inline]
    pub unsafe fn GetCookies(&self, aCookies: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult {
        ((*self.vtable).GetCookies)(self, aCookies)
    }


    /// ```text
    /// /**
    ///    * Returns an array of session cookies in the cookie list.
    ///    * The objects in the array are of type nsICookie
    ///    * This array only contains non-private browsing cookies.
    ///    */
    /// ```
    ///

    /// `readonly attribute Array<nsICookie> sessionCookies;`
    #[inline]
    pub unsafe fn GetSessionCookies(&self, aSessionCookies: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult {
        ((*self.vtable).GetSessionCookies)(self, aSessionCookies)
    }


    /// ```text
    /// /**
    ///    * Returns current effective value of the "network.cookie.cookieBehavior".
    ///    */
    /// ```
    ///

    /// `readonly attribute uint32_t cookieBehavior;`
    #[inline]
    pub unsafe fn GetCookieBehavior(&self, aCookieBehavior: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCookieBehavior)(self, aCookieBehavior)
    }


    /// ```text
    /// /**
    ///    * Called to remove an individual cookie from the cookie list, specified
    ///    * by host, name, and path. If the cookie cannot be found, no exception
    ///    * is thrown. Typically, the arguments to this method will be obtained
    ///    * directly from the desired nsICookie object.
    ///    *
    ///    * @param aHost The host or domain for which the cookie was set. @see
    ///    *              nsICookieManager::add for a description of acceptable host
    ///    *              strings. If the target cookie is a domain cookie, a leading
    ///    *              dot must be present.
    ///    * @param aName The name specified in the cookie
    ///    * @param aPath The path for which the cookie was set
    ///    * @param aOriginAttributes The originAttributes of this cookie.
    ///    *
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void remove (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in jsval aOriginAttributes);`
    const _Remove: () = ();


    /// `[notxpcom] nsresult removeNative (in AUTF8String aHost, in ACString aName, in AUTF8String aPath, in OriginAttributesPtr aOriginAttributes);`
    const _RemoveNative: () = ();

    /// ```text
    /// /**
    ///    * Add a cookie. nsICookieService is the normal way to do this. This
    ///    * method is something of a backdoor.
    ///    *
    ///    * @param aHost
    ///    *        the host or domain for which the cookie is set. presence of a
    ///    *        leading dot indicates a domain cookie; otherwise, the cookie
    ///    *        is treated as a non-domain cookie (see RFC2109). The host string
    ///    *        will be normalized to ASCII or ACE; any trailing dot will be
    ///    *        stripped. To be a domain cookie, the host must have at least two
    ///    *        subdomain parts (e.g. '.foo.com', not '.com'), otherwise an
    ///    *        exception will be thrown. An empty string is acceptable
    ///    *        (e.g. file:// URI's).
    ///    * @param aPath
    ///    *        path within the domain for which the cookie is valid
    ///    * @param aName
    ///    *        cookie name
    ///    * @param aValue
    ///    *        cookie data
    ///    * @param aIsSecure
    ///    *        true if the cookie should only be sent over a secure connection.
    ///    * @param aIsHttpOnly
    ///    *        true if the cookie should only be sent to, and can only be
    ///    *        modified by, an http connection.
    ///    * @param aIsSession
    ///    *        true if the cookie should exist for the current session only.
    ///    *        see aExpiry.
    ///    * @param aExpiry
    ///    *        expiration date, in seconds since midnight (00:00:00), January 1,
    ///    *        1970 UTC. note that expiry time will also be honored for session cookies;
    ///    *        in this way, the more restrictive of the two will take effect.
    ///    * @param aOriginAttributes
    ///    *        the originAttributes of this cookie.
    ///    * @param aSameSite
    ///    *        the SameSite attribute.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void add (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in AUTF8String aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in jsval aOriginAttributes, in int32_t aSameSite, in nsICookie_schemeType aSchemeMap);`
    const _Add: () = ();


    /// `[notxpcom] nsresult addNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in AUTF8String aValue, in boolean aIsSecure, in boolean aIsHttpOnly, in boolean aIsSession, in int64_t aExpiry, in OriginAttributesPtr aOriginAttributes, in int32_t aSameSite, in nsICookie_schemeType aSchemeMap);`
    const _AddNative: () = ();

    /// ```text
    /// /**
    ///    * Find whether a given cookie already exists.
    ///    *
    ///    * @param aHost
    ///    *        the cookie's host to look for
    ///    * @param aPath
    ///    *        the cookie's path to look for
    ///    * @param aName
    ///    *        the cookie's name to look for
    ///    * @param aOriginAttributes
    ///    *        the cookie's originAttributes to look for
    ///    *
    ///    * @return true if a cookie was found which matches the host, path, name and
    ///    *         originAttributes fields of aCookie
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] boolean cookieExists (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in jsval aOriginAttributes);`
    const _CookieExists: () = ();


    /// `[notxpcom] nsresult cookieExistsNative (in AUTF8String aHost, in AUTF8String aPath, in ACString aName, in OriginAttributesPtr aOriginAttributes, out boolean aExists);`
    const _CookieExistsNative: () = ();

    /// ```text
    /// /**
    ///    * Count how many cookies exist within the base domain of 'aHost'.
    ///    * Thus, for a host "weather.yahoo.com", the base domain would be "yahoo.com",
    ///    * and any host or domain cookies for "yahoo.com" and its subdomains would be
    ///    * counted.
    ///    *
    ///    * @param aHost
    ///    *        the host string to search for, e.g. "google.com". this should consist
    ///    *        of only the host portion of a URI. see @add for a description of
    ///    *        acceptable host strings.
    ///    *
    ///    * @return the number of cookies found.
    ///    */
    /// ```
    ///

    /// `unsigned long countCookiesFromHost (in AUTF8String aHost);`
    #[inline]
    pub unsafe fn CountCookiesFromHost(&self, aHost: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).CountCookiesFromHost)(self, aHost, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns an array of cookies that exist within the base domain of
    ///    * 'aHost'. Thus, for a host "weather.yahoo.com", the base domain would be
    ///    * "yahoo.com", and any host or domain cookies for "yahoo.com" and its
    ///    * subdomains would be returned.
    ///    *
    ///    * @param aHost
    ///    *        the host string to search for, e.g. "google.com". this should consist
    ///    *        of only the host portion of a URI. see @add for a description of
    ///    *        acceptable host strings.
    ///    * @param aOriginAttributes The originAttributes of cookies that would be
    ///    *                          retrived.
    ///    *
    ///    * @return an array of nsICookie objects.
    ///    *
    ///    * @see countCookiesFromHost
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Array<nsICookie> getCookiesFromHost (in AUTF8String aHost, in jsval aOriginAttributes);`
    const _GetCookiesFromHost: () = ();

    /// ```text
    /// /**
    ///    * Returns an array of all cookies whose origin attributes matches aPattern
    ///    *
    ///    * @param aPattern origin attribute pattern in JSON format
    ///    *
    ///    * @param aHost
    ///    *        the host string to search for, e.g. "google.com". this should consist
    ///    *        of only the host portion of a URI. see @add for a description of
    ///    *        acceptable host strings. This attribute is optional. It will search
    ///    *        all hosts if this attribute is not given.
    ///    */
    /// ```
    ///

    /// `Array<nsICookie> getCookiesWithOriginAttributes (in AString aPattern, [optional] in AUTF8String aHost);`
    #[inline]
    pub unsafe fn GetCookiesWithOriginAttributes(&self, aPattern: *const ::nsstring::nsAString, aHost: *const ::nsstring::nsACString, _retval: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult {
        ((*self.vtable).GetCookiesWithOriginAttributes)(self, aPattern, aHost, _retval)
    }


    /// ```text
    /// /**
    ///    * Remove all the cookies whose origin attributes matches aPattern
    ///    *
    ///    * @param aPattern origin attribute pattern in JSON format
    ///    */
    /// ```
    ///

    /// `void removeCookiesWithOriginAttributes (in AString aPattern, [optional] in AUTF8String aHost);`
    #[inline]
    pub unsafe fn RemoveCookiesWithOriginAttributes(&self, aPattern: *const ::nsstring::nsAString, aHost: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCookiesWithOriginAttributes)(self, aPattern, aHost)
    }


    /// ```text
    /// /**
    ///    * Remove all the cookies whose origin attributes matches aPattern and the
    ///    * host is exactly aHost (without subdomain matching).
    ///    *
    ///    * @param aHost the host to match
    ///    * @param aPattern origin attribute pattern in JSON format
    ///    */
    /// ```
    ///

    /// `void removeCookiesFromExactHost (in AUTF8String aHost, in AString aPattern);`
    #[inline]
    pub unsafe fn RemoveCookiesFromExactHost(&self, aHost: *const ::nsstring::nsACString, aPattern: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveCookiesFromExactHost)(self, aHost, aPattern)
    }


    /// ```text
    /// /**
    ///    * Removes all cookies that were created on or after aSinceWhen, and returns
    ///    * a Promise which will be resolved when the last such cookie has been
    ///    * removed.
    ///    *
    ///    * @param aSinceWhen the starting point in time after which no cookies should
    ///    *        be created when the Promise returned from this method is resolved.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise removeAllSince (in int64_t aSinceWhen);`
    const _RemoveAllSince: () = ();

    /// ```text
    /// /**
    ///    * Retrieves all the cookies that were created on or after aSinceWhen, order
    ///    * by creation time */
    /// ```
    ///

    /// `Array<nsICookie> getCookiesSince (in int64_t aSinceWhen);`
    #[inline]
    pub unsafe fn GetCookiesSince(&self, aSinceWhen: int64_t, _retval: *mut thin_vec::ThinVec<RefPtr<nsICookie>>) -> ::nserror::nsresult {
        ((*self.vtable).GetCookiesSince)(self, aSinceWhen, _retval)
    }


}


