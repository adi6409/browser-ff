//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsIExternalProtocolService.idl
//


/// `interface nsIExternalProtocolService : nsISupports`
///

/// ```text
/// /**
///  * The external protocol service is used for finding and launching
///  * web handlers (a la registerProtocolHandler in the HTML5 draft) or
///  * platform-specific applications for handling particular protocols.
///  *
///  * You can ask the external protocol service if it has an external
///  * handler for a given protocol scheme. And you can ask it to load
///  * the url using the default handler.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIExternalProtocolService {
    vtable: *const nsIExternalProtocolServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIExternalProtocolService.
unsafe impl XpCom for nsIExternalProtocolService {
    const IID: nsIID = nsID(0x70f93b7a, 0x3ec6, 0x4bcb,
        [0xb0, 0x93, 0x92, 0xd9, 0x98, 0x4c, 0x9f, 0x83]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIExternalProtocolService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIExternalProtocolService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExternalProtocolServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIExternalProtocolService`.
    fn coerce_from(v: &nsIExternalProtocolService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExternalProtocolServiceCoerce for nsIExternalProtocolService {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolService) -> &Self {
        v
    }
}

impl nsIExternalProtocolService {
    /// Cast this `nsIExternalProtocolService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExternalProtocolServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIExternalProtocolService {
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
impl<T: nsISupportsCoerce> nsIExternalProtocolServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExternalProtocolService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIExternalProtocolService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExternalProtocolServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean externalProtocolHandlerExists (in string aProtocolScheme); */
    pub ExternalProtocolHandlerExists: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isExposedProtocol (in string aProtocolScheme); */
    pub IsExposedProtocol: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme); */
    pub GetProtocolHandlerInfo: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut*const nsIHandlerInfo) -> ::nserror::nsresult,

    /* nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound); */
    pub GetProtocolHandlerInfoFromOS: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aProtocolScheme: *const ::nsstring::nsACString, aFound: *mut bool, _retval: *mut*const nsIHandlerInfo) -> ::nserror::nsresult,

    /* void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists); */
    pub SetProtocolHandlerDefaults: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aHandlerInfo: *const nsIHandlerInfo, aOSHandlerExists: bool) -> ::nserror::nsresult,

    /* void loadURI (in nsIURI aURI, [optional] in nsIPrincipal aTriggeringPrincipal, [optional] in BrowsingContext aBrowsingContext); */
    pub LoadURI: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult,

    /* AString getApplicationDescription (in AUTF8String aScheme); */
    pub GetApplicationDescription: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aScheme: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* bool isCurrentAppOSDefaultForProtocol (in AUTF8String aScheme); */
    pub IsCurrentAppOSDefaultForProtocol: unsafe extern "system" fn (this: *const nsIExternalProtocolService, aScheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIExternalProtocolService {

    /// ```text
    /// /**
    ///    * Check whether a handler for a specific protocol exists.  Specifically,
    ///    * this looks to see whether there are any known possible application handlers
    ///    * in either the nsIHandlerService datastore or registered with the OS.
    ///    *
    ///    * @param aProtocolScheme The scheme from a url: http, ftp, mailto, etc.
    ///    *
    ///    * @return true if we have a handler and false otherwise.
    ///    *
    ///    * XXX shouldn't aProtocolScheme be an ACString like nsIURI::scheme?
    ///    */
    /// ```
    ///

    /// `boolean externalProtocolHandlerExists (in string aProtocolScheme);`
    #[inline]
    pub unsafe fn ExternalProtocolHandlerExists(&self, aProtocolScheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExternalProtocolHandlerExists)(self, aProtocolScheme, _retval)
    }


    /// ```text
    /// /**
    ///    * Check whether a handler for a specific protocol is "exposed" as a visible
    ///    * feature of the current application.
    ///    *
    ///    * An exposed protocol handler is one that can be used in all contexts.  A
    ///    * non-exposed protocol handler is one that can only be used internally by the
    ///    * application.  For example, a non-exposed protocol would not be loaded by the
    ///    * application in response to a link click or a X-remote openURL command.
    ///    * Instead, it would be deferred to the system's external protocol handler.
    ///    * XXX shouldn't aProtocolScheme be an ACString like nsIURI::scheme?
    ///    */
    /// ```
    ///

    /// `boolean isExposedProtocol (in string aProtocolScheme);`
    #[inline]
    pub unsafe fn IsExposedProtocol(&self, aProtocolScheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsExposedProtocol)(self, aProtocolScheme, _retval)
    }


    /// ```text
    /// /**
    ///    * Retrieve the handler for the given protocol.  If neither the application
    ///    * nor the OS knows about a handler for the protocol, the object this method
    ///    * returns will represent a default handler for unknown content.
    ///    *
    ///    * @param aProtocolScheme the scheme from a URL: http, ftp, mailto, etc.
    ///    *
    ///    * Note: aProtocolScheme should not include a trailing colon, which is part
    ///    * of the URI syntax, not part of the scheme itself (i.e. pass "mailto" not
        ///    * "mailto:").
    ///    *
    ///    * @return the handler, if any; otherwise a default handler
    ///    */
    /// ```
    ///

    /// `nsIHandlerInfo getProtocolHandlerInfo (in ACString aProtocolScheme);`
    #[inline]
    pub unsafe fn GetProtocolHandlerInfo(&self, aProtocolScheme: *const ::nsstring::nsACString, _retval: *mut*const nsIHandlerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolHandlerInfo)(self, aProtocolScheme, _retval)
    }


    /// ```text
    /// /**
    ///    * Given a scheme, looks up the protocol info from the OS.  This should be
    ///    * overridden by each OS's implementation.
    ///    *
    ///    * @param aScheme The protocol scheme we are looking for.
    ///    * @param aFound  Was an OS default handler for this scheme found?
    ///    * @return An nsIHanderInfo for the protocol.
    ///    */
    /// ```
    ///

    /// `nsIHandlerInfo getProtocolHandlerInfoFromOS (in ACString aProtocolScheme, out boolean aFound);`
    #[inline]
    pub unsafe fn GetProtocolHandlerInfoFromOS(&self, aProtocolScheme: *const ::nsstring::nsACString, aFound: *mut bool, _retval: *mut*const nsIHandlerInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolHandlerInfoFromOS)(self, aProtocolScheme, aFound, _retval)
    }


    /// ```text
    /// /**
    ///    * Set some sane defaults for a protocol handler object.
    ///    *
    ///    * @param aHandlerInfo      nsIHandlerInfo object, as returned by
    ///    *                          getProtocolHandlerInfoFromOS
    ///    * @param aOSHandlerExists  was the object above created for an extant
    ///    *                          OS default handler?  This is generally the
    ///    *                          value of the aFound out param from
    ///    *                          getProtocolHandlerInfoFromOS.
    ///    */
    /// ```
    ///

    /// `void setProtocolHandlerDefaults (in nsIHandlerInfo aHandlerInfo, in boolean aOSHandlerExists);`
    #[inline]
    pub unsafe fn SetProtocolHandlerDefaults(&self, aHandlerInfo: *const nsIHandlerInfo, aOSHandlerExists: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetProtocolHandlerDefaults)(self, aHandlerInfo, aOSHandlerExists)
    }


    /// ```text
    /// /**
    ///    * Used to load a URI via an external application. Might prompt the user for
    ///    * permission to load the external application.
    ///    *
    ///    * @param aURI
    ///    *        The URI to load
    ///    *
    ///    * @param aTriggeringPrincipal
    ///    *        The principal triggering this load.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The context to parent the dialog against, and, if a web handler
    ///    *        is chosen, it is loaded in this window as well.  This parameter
    ///    *        may be ultimately passed nsIURILoader.openURI in the case of a
    ///    *        web handler, and aWindowContext is null or not present, web
    ///    *        handlers will fail.  We need to do better than that; bug 394483
    ///    *        filed in order to track.
    ///    *
    ///    * @note  Embedders that do not expose the http protocol should not currently
    ///    *        use web-based protocol handlers, as handoff won't work correctly
    ///    *        (bug 394479).
    ///    */
    /// ```
    ///

    /// `void loadURI (in nsIURI aURI, [optional] in nsIPrincipal aTriggeringPrincipal, [optional] in BrowsingContext aBrowsingContext);`
    #[inline]
    pub unsafe fn LoadURI(&self, aURI: *const nsIURI, aTriggeringPrincipal: *const nsIPrincipal, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).LoadURI)(self, aURI, aTriggeringPrincipal, aBrowsingContext)
    }


    /// ```text
    /// /**
    ///    * Gets a human-readable description for the application responsible for
    ///    * handling a specific protocol.
    ///    *
    ///    * @param aScheme The scheme to look up. For example, "mms".
    ///    *
    ///    * @throw NS_ERROR_NOT_IMPLEMENTED
    ///    *        If getting descriptions for protocol helpers is not supported
    ///    * @throw NS_ERROR_NOT_AVAILABLE
    ///    *        If no protocol helper exists for this scheme, or if it is not
    ///    *        possible to get a description for it.
    ///    */
    /// ```
    ///

    /// `AString getApplicationDescription (in AUTF8String aScheme);`
    #[inline]
    pub unsafe fn GetApplicationDescription(&self, aScheme: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationDescription)(self, aScheme, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if this app is registered as the OS default for a given scheme.
    ///    *
    ///    * @param aScheme The scheme to look up. For example, "mms".
    ///    */
    /// ```
    ///

    /// `bool isCurrentAppOSDefaultForProtocol (in AUTF8String aScheme);`
    #[inline]
    pub unsafe fn IsCurrentAppOSDefaultForProtocol(&self, aScheme: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCurrentAppOSDefaultForProtocol)(self, aScheme, _retval)
    }


}


