//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginInfo.idl
//


/// `interface nsILoginInfo : nsISupports`
///

/// ```text
/// /**
///  * An object containing information for a login stored by the
///  * password manager.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginInfo {
    vtable: *const nsILoginInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginInfo.
unsafe impl XpCom for nsILoginInfo {
    const IID: nsIID = nsID(0xc41b7dff, 0x6b9b, 0x42fe,
        [0xb7, 0x8d, 0x11, 0x30, 0x51, 0xfa, 0xcb, 0x05]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginInfoCoerce {
    /// Cheaply cast a value of this type from a `nsILoginInfo`.
    fn coerce_from(v: &nsILoginInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginInfoCoerce for nsILoginInfo {
    #[inline]
    fn coerce_from(v: &nsILoginInfo) -> &Self {
        v
    }
}

impl nsILoginInfo {
    /// Cast this `nsILoginInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginInfo {
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
impl<T: nsISupportsCoerce> nsILoginInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString displayOrigin; */
    pub GetDisplayOrigin: unsafe extern "system" fn (this: *const nsILoginInfo, aDisplayOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString origin; */
    pub GetOrigin: unsafe extern "system" fn (this: *const nsILoginInfo, aOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString origin; */
    pub SetOrigin: unsafe extern "system" fn (this: *const nsILoginInfo, aOrigin: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString hostname; */
    pub GetHostname: unsafe extern "system" fn (this: *const nsILoginInfo, aHostname: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString hostname; */
    pub SetHostname: unsafe extern "system" fn (this: *const nsILoginInfo, aHostname: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString formActionOrigin; */
    pub GetFormActionOrigin: unsafe extern "system" fn (this: *const nsILoginInfo, aFormActionOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString formActionOrigin; */
    pub SetFormActionOrigin: unsafe extern "system" fn (this: *const nsILoginInfo, aFormActionOrigin: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString formSubmitURL; */
    pub GetFormSubmitURL: unsafe extern "system" fn (this: *const nsILoginInfo, aFormSubmitURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString formSubmitURL; */
    pub SetFormSubmitURL: unsafe extern "system" fn (this: *const nsILoginInfo, aFormSubmitURL: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString httpRealm; */
    pub GetHttpRealm: unsafe extern "system" fn (this: *const nsILoginInfo, aHttpRealm: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString httpRealm; */
    pub SetHttpRealm: unsafe extern "system" fn (this: *const nsILoginInfo, aHttpRealm: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString username; */
    pub GetUsername: unsafe extern "system" fn (this: *const nsILoginInfo, aUsername: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString username; */
    pub SetUsername: unsafe extern "system" fn (this: *const nsILoginInfo, aUsername: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString usernameField; */
    pub GetUsernameField: unsafe extern "system" fn (this: *const nsILoginInfo, aUsernameField: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString usernameField; */
    pub SetUsernameField: unsafe extern "system" fn (this: *const nsILoginInfo, aUsernameField: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString password; */
    pub GetPassword: unsafe extern "system" fn (this: *const nsILoginInfo, aPassword: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString password; */
    pub SetPassword: unsafe extern "system" fn (this: *const nsILoginInfo, aPassword: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString passwordField; */
    pub GetPasswordField: unsafe extern "system" fn (this: *const nsILoginInfo, aPasswordField: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString passwordField; */
    pub SetPasswordField: unsafe extern "system" fn (this: *const nsILoginInfo, aPasswordField: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void init (in AString aOrigin, in AString aFormActionOrigin, in AString aHttpRealm, in AString aUsername, in AString aPassword, [optional] in AString aUsernameField, [optional] in AString aPasswordField); */
    pub Init: unsafe extern "system" fn (this: *const nsILoginInfo, aOrigin: *const ::nsstring::nsAString, aFormActionOrigin: *const ::nsstring::nsAString, aHttpRealm: *const ::nsstring::nsAString, aUsername: *const ::nsstring::nsAString, aPassword: *const ::nsstring::nsAString, aUsernameField: *const ::nsstring::nsAString, aPasswordField: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean equals (in nsILoginInfo aLoginInfo); */
    pub Equals: unsafe extern "system" fn (this: *const nsILoginInfo, aLoginInfo: *const nsILoginInfo, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword); */
    pub Matches: unsafe extern "system" fn (this: *const nsILoginInfo, aLoginInfo: *const nsILoginInfo, ignorePassword: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsILoginInfo clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsILoginInfo, _retval: *mut *const nsILoginInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginInfo {

    /// ```text
    /// /**
    ///    * A string to display to the user for the origin which includes the httpRealm,
    ///    * where applicable.
    ///    * e.g. "site.com", "site.com:1234", or "site.com (My Secure Realm)"
    ///    */
    /// ```
    ///

    /// `readonly attribute AString displayOrigin;`
    #[inline]
    pub unsafe fn GetDisplayOrigin(&self, aDisplayOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayOrigin)(self, aDisplayOrigin)
    }


    /// ```text
    /// /**
    ///    * The origin the login applies to.
    ///    *
    ///    * For example,
    ///    * "https://site.com", "http://site.com:1234", "ftp://ftp.site.com",
    ///    * "moz-proxy://127.0.0.1:8888, "chrome://FirefoxAccounts", "file://".
    ///    */
    /// ```
    ///

    /// `attribute AString origin;`
    #[inline]
    pub unsafe fn GetOrigin(&self, aOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetOrigin)(self, aOrigin)
    }


    /// ```text
    /// /**
    ///    * The origin the login applies to.
    ///    *
    ///    * For example,
    ///    * "https://site.com", "http://site.com:1234", "ftp://ftp.site.com",
    ///    * "moz-proxy://127.0.0.1:8888, "chrome://FirefoxAccounts", "file://".
    ///    */
    /// ```
    ///

    /// `attribute AString origin;`
    #[inline]
    pub unsafe fn SetOrigin(&self, aOrigin: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetOrigin)(self, aOrigin)
    }


    /// ```text
    /// /**
    ///    * The origin the login applies to, incorrectly called a hostname.
    ///    * @deprecated in favor of `origin`
    ///    */
    /// ```
    ///

    /// `attribute AString hostname;`
    #[inline]
    pub unsafe fn GetHostname(&self, aHostname: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHostname)(self, aHostname)
    }


    /// ```text
    /// /**
    ///    * The origin the login applies to, incorrectly called a hostname.
    ///    * @deprecated in favor of `origin`
    ///    */
    /// ```
    ///

    /// `attribute AString hostname;`
    #[inline]
    pub unsafe fn SetHostname(&self, aHostname: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetHostname)(self, aHostname)
    }


    /// ```text
    /// /**
    ///    * The origin a form-based login was submitted to.
    ///    *
    ///    * For logins obtained from HTML forms, this field is the origin of the |action|
    ///    * attribute from the |form| element. For
    ///    * example "http://www.site.com". [Forms with no |action| attribute
        ///    * default to submitting to their origin URL, so we store that.]
    ///    *
    ///    * For logins obtained from a HTTP or FTP protocol authentication,
    ///    * this field is NULL.
    ///    */
    /// ```
    ///

    /// `attribute AString formActionOrigin;`
    #[inline]
    pub unsafe fn GetFormActionOrigin(&self, aFormActionOrigin: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFormActionOrigin)(self, aFormActionOrigin)
    }


    /// ```text
    /// /**
    ///    * The origin a form-based login was submitted to.
    ///    *
    ///    * For logins obtained from HTML forms, this field is the origin of the |action|
    ///    * attribute from the |form| element. For
    ///    * example "http://www.site.com". [Forms with no |action| attribute
        ///    * default to submitting to their origin URL, so we store that.]
    ///    *
    ///    * For logins obtained from a HTTP or FTP protocol authentication,
    ///    * this field is NULL.
    ///    */
    /// ```
    ///

    /// `attribute AString formActionOrigin;`
    #[inline]
    pub unsafe fn SetFormActionOrigin(&self, aFormActionOrigin: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFormActionOrigin)(self, aFormActionOrigin)
    }


    /// ```text
    /// /**
    ///    * The origin a form-based login was submitted to, incorrectly referred to as a URL.
    ///    * @deprecated in favor of `formActionOrigin`
    ///    */
    /// ```
    ///

    /// `attribute AString formSubmitURL;`
    #[inline]
    pub unsafe fn GetFormSubmitURL(&self, aFormSubmitURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetFormSubmitURL)(self, aFormSubmitURL)
    }


    /// ```text
    /// /**
    ///    * The origin a form-based login was submitted to, incorrectly referred to as a URL.
    ///    * @deprecated in favor of `formActionOrigin`
    ///    */
    /// ```
    ///

    /// `attribute AString formSubmitURL;`
    #[inline]
    pub unsafe fn SetFormSubmitURL(&self, aFormSubmitURL: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetFormSubmitURL)(self, aFormSubmitURL)
    }


    /// ```text
    /// /**
    ///    * The HTTP Realm a login was requested for.
    ///    *
    ///    * When an HTTP server sends a 401 result, the WWW-Authenticate
    ///    * header includes a realm to identify the "protection space." See
    ///    * RFC2617. If the response sent has a missing or blank realm, the
    ///    * hostname is used instead.
    ///    *
    ///    * For logins obtained from HTML forms, this field is NULL.
    ///    */
    /// ```
    ///

    /// `attribute AString httpRealm;`
    #[inline]
    pub unsafe fn GetHttpRealm(&self, aHttpRealm: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHttpRealm)(self, aHttpRealm)
    }


    /// ```text
    /// /**
    ///    * The HTTP Realm a login was requested for.
    ///    *
    ///    * When an HTTP server sends a 401 result, the WWW-Authenticate
    ///    * header includes a realm to identify the "protection space." See
    ///    * RFC2617. If the response sent has a missing or blank realm, the
    ///    * hostname is used instead.
    ///    *
    ///    * For logins obtained from HTML forms, this field is NULL.
    ///    */
    /// ```
    ///

    /// `attribute AString httpRealm;`
    #[inline]
    pub unsafe fn SetHttpRealm(&self, aHttpRealm: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetHttpRealm)(self, aHttpRealm)
    }


    /// ```text
    /// /**
    ///    * The username for the login.
    ///    */
    /// ```
    ///

    /// `attribute AString username;`
    #[inline]
    pub unsafe fn GetUsername(&self, aUsername: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUsername)(self, aUsername)
    }


    /// ```text
    /// /**
    ///    * The username for the login.
    ///    */
    /// ```
    ///

    /// `attribute AString username;`
    #[inline]
    pub unsafe fn SetUsername(&self, aUsername: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetUsername)(self, aUsername)
    }


    /// ```text
    /// /**
    ///    * The |name| attribute for the username input field.
    ///    *
    ///    * For logins obtained from a HTTP or FTP protocol authentication,
    ///    * this field is an empty string.
    ///    *
    ///    * @note This attribute is currently saved but not used.
    ///    */
    /// ```
    ///

    /// `attribute AString usernameField;`
    #[inline]
    pub unsafe fn GetUsernameField(&self, aUsernameField: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUsernameField)(self, aUsernameField)
    }


    /// ```text
    /// /**
    ///    * The |name| attribute for the username input field.
    ///    *
    ///    * For logins obtained from a HTTP or FTP protocol authentication,
    ///    * this field is an empty string.
    ///    *
    ///    * @note This attribute is currently saved but not used.
    ///    */
    /// ```
    ///

    /// `attribute AString usernameField;`
    #[inline]
    pub unsafe fn SetUsernameField(&self, aUsernameField: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetUsernameField)(self, aUsernameField)
    }


    /// ```text
    /// /**
    ///    * The password for the login.
    ///    */
    /// ```
    ///

    /// `attribute AString password;`
    #[inline]
    pub unsafe fn GetPassword(&self, aPassword: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPassword)(self, aPassword)
    }


    /// ```text
    /// /**
    ///    * The password for the login.
    ///    */
    /// ```
    ///

    /// `attribute AString password;`
    #[inline]
    pub unsafe fn SetPassword(&self, aPassword: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetPassword)(self, aPassword)
    }


    /// ```text
    /// /**
    ///    * The |name| attribute for the password input field.
    ///    *
    ///    * For logins obtained from a HTTP or FTP protocol authentication,
    ///    * this field is an empty string.
    ///    *
    ///    * @note This attribute is currently saved but not used.
    ///    */
    /// ```
    ///

    /// `attribute AString passwordField;`
    #[inline]
    pub unsafe fn GetPasswordField(&self, aPasswordField: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPasswordField)(self, aPasswordField)
    }


    /// ```text
    /// /**
    ///    * The |name| attribute for the password input field.
    ///    *
    ///    * For logins obtained from a HTTP or FTP protocol authentication,
    ///    * this field is an empty string.
    ///    *
    ///    * @note This attribute is currently saved but not used.
    ///    */
    /// ```
    ///

    /// `attribute AString passwordField;`
    #[inline]
    pub unsafe fn SetPasswordField(&self, aPasswordField: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetPasswordField)(self, aPasswordField)
    }


    /// ```text
    /// /**
    ///    * Initialize a newly created nsLoginInfo object.
    ///    *
    ///    * The arguments are the fields for the new object.
    ///    */
    /// ```
    ///

    /// `void init (in AString aOrigin, in AString aFormActionOrigin, in AString aHttpRealm, in AString aUsername, in AString aPassword, [optional] in AString aUsernameField, [optional] in AString aPasswordField);`
    #[inline]
    pub unsafe fn Init(&self, aOrigin: *const ::nsstring::nsAString, aFormActionOrigin: *const ::nsstring::nsAString, aHttpRealm: *const ::nsstring::nsAString, aUsername: *const ::nsstring::nsAString, aPassword: *const ::nsstring::nsAString, aUsernameField: *const ::nsstring::nsAString, aPasswordField: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aOrigin, aFormActionOrigin, aHttpRealm, aUsername, aPassword, aUsernameField, aPasswordField)
    }


    /// ```text
    /// /**
    ///    * Test for strict equality with another nsILoginInfo object.
    ///    *
    ///    * @param aLoginInfo
    ///    *        The other object to test.
    ///    */
    /// ```
    ///

    /// `boolean equals (in nsILoginInfo aLoginInfo);`
    #[inline]
    pub unsafe fn Equals(&self, aLoginInfo: *const nsILoginInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, aLoginInfo, _retval)
    }


    /// ```text
    /// /**
    ///    * Test for loose equivalency with another nsILoginInfo object. The
    ///    * passwordField and usernameField values are ignored, and the password
    ///    * values may be optionally ignored. If one login's formSubmitURL is an
    ///    * empty string (but not null), it will be treated as a wildcard. [The
        ///    * blank value indicates the login was stored before bug 360493 was fixed.]
    ///    *
    ///    * @param aLoginInfo
    ///    *        The other object to test.
    ///    * @param ignorePassword
    ///    *        If true, ignore the password when checking for match.
    ///    */
    /// ```
    ///

    /// `boolean matches (in nsILoginInfo aLoginInfo, in boolean ignorePassword);`
    #[inline]
    pub unsafe fn Matches(&self, aLoginInfo: *const nsILoginInfo, ignorePassword: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Matches)(self, aLoginInfo, ignorePassword, _retval)
    }


    /// ```text
    /// /**
    ///    * Create an identical copy of the login, duplicating all of the login's
    ///    * nsILoginInfo and nsILoginMetaInfo properties.
    ///    *
    ///    * This allows code to be forwards-compatible, when additional properties
    ///    * are added to nsILoginMetaInfo (or nsILoginInfo) in the future.
    ///    */
    /// ```
    ///

    /// `nsILoginInfo clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsILoginInfo) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


