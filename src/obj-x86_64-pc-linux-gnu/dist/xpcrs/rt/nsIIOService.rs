//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIOService.idl
//


/// `interface nsIIOService : nsISupports`
///

/// ```text
/// /**
///  * nsIIOService provides a set of network utility functions.  This interface
///  * duplicates many of the nsIProtocolHandler methods in a protocol handler
///  * independent way (e.g., NewURI inspects the scheme in order to delegate
    ///  * creation of the new URI to the appropriate protocol handler).  nsIIOService
///  * also provides a set of URL parsing utility functions.  These are provided
///  * as a convenience to the programmer and in some cases to improve performance
///  * by eliminating intermediate data structures and interfaces.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIOService {
    vtable: *const nsIIOServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIOService.
unsafe impl XpCom for nsIIOService {
    const IID: nsIID = nsID(0x4286de5a, 0xb2ea, 0x446f,
        [0x8f, 0x70, 0xe2, 0xa4, 0x61, 0xf4, 0x26, 0x94]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIOService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIOService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIOServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIIOService`.
    fn coerce_from(v: &nsIIOService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIOServiceCoerce for nsIIOService {
    #[inline]
    fn coerce_from(v: &nsIIOService) -> &Self {
        v
    }
}

impl nsIIOService {
    /// Cast this `nsIIOService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIOServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIOService {
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
impl<T: nsISupportsCoerce> nsIIOServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIOService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIOServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIProtocolHandler getProtocolHandler (in string aScheme); */
    pub GetProtocolHandler: unsafe extern "system" fn (this: *const nsIIOService, aScheme: *const libc::c_char, _retval: *mut*const nsIProtocolHandler) -> ::nserror::nsresult,

    /* unsigned long getProtocolFlags (in string aScheme); */
    pub GetProtocolFlags: unsafe extern "system" fn (this: *const nsIIOService, aScheme: *const libc::c_char, _retval: *mut u32) -> ::nserror::nsresult,

    /* nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI); */
    pub NewURI: unsafe extern "system" fn (this: *const nsIIOService, aSpec: *const ::nsstring::nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* nsIURI newFileURI (in nsIFile aFile); */
    pub NewFileURI: unsafe extern "system" fn (this: *const nsIIOService, aFile: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* nsIURI createExposableURI (in nsIURI aURI); */
    pub CreateExposableURI: unsafe extern "system" fn (this: *const nsIIOService, aURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* nsIChannel newChannelFromURI (in nsIURI aURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
    pub NewChannelFromURI: unsafe extern "system" fn (this: *const nsIIOService, aURI: *const nsIURI, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] nsresult NewChannelFromURIWithClientAndController (in nsIURI aURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in const_MaybeClientInfoRef aLoadingClientInfo, in const_MaybeServiceWorkerDescriptorRef aController, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags, out nsIChannel aResult); */
    /// Unable to generate binding because `native type const mozilla::Maybe<mozilla::dom::ClientInfo> unsupported`
    pub NewChannelFromURIWithClientAndController: *const ::libc::c_void,

    /* nsIChannel newChannelFromURIWithLoadInfo (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
    pub NewChannelFromURIWithLoadInfo: unsafe extern "system" fn (this: *const nsIIOService, aURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* nsIChannel newChannel (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
    pub NewChannel: unsafe extern "system" fn (this: *const nsIIOService, aSpec: *const ::nsstring::nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* attribute boolean offline; */
    pub GetOffline: unsafe extern "system" fn (this: *const nsIIOService, aOffline: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean offline; */
    pub SetOffline: unsafe extern "system" fn (this: *const nsIIOService, aOffline: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean connectivity; */
    pub GetConnectivity: unsafe extern "system" fn (this: *const nsIIOService, aConnectivity: *mut bool) -> ::nserror::nsresult,

    /* boolean allowPort (in long aPort, in string aScheme); */
    pub AllowPort: unsafe extern "system" fn (this: *const nsIIOService, aPort: i32, aScheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* ACString extractScheme (in AUTF8String urlString); */
    pub ExtractScheme: unsafe extern "system" fn (this: *const nsIIOService, urlString: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean hostnameIsLocalIPAddress (in nsIURI aURI); */
    pub HostnameIsLocalIPAddress: unsafe extern "system" fn (this: *const nsIIOService, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean hostnameIsSharedIPAddress (in nsIURI aURI); */
    pub HostnameIsSharedIPAddress: unsafe extern "system" fn (this: *const nsIIOService, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean manageOfflineStatus; */
    pub GetManageOfflineStatus: unsafe extern "system" fn (this: *const nsIIOService, aManageOfflineStatus: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean manageOfflineStatus; */
    pub SetManageOfflineStatus: unsafe extern "system" fn (this: *const nsIIOService, aManageOfflineStatus: bool) -> ::nserror::nsresult,

    /* nsIChannel newChannelFromURIWithProxyFlags (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType); */
    pub NewChannelFromURIWithProxyFlags: unsafe extern "system" fn (this: *const nsIIOService, aURI: *const nsIURI, aProxyURI: *const nsIURI, aProxyFlags: u32, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* readonly attribute boolean socketProcessLaunched; */
    pub GetSocketProcessLaunched: unsafe extern "system" fn (this: *const nsIIOService, aSocketProcessLaunched: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIOService {

    /// ```text
    /// /**
    ///      * Returns a protocol handler for a given URI scheme.
    ///      *
    ///      * @param aScheme the URI scheme
    ///      * @return reference to corresponding nsIProtocolHandler
    ///      */
    /// ```
    ///

    /// `nsIProtocolHandler getProtocolHandler (in string aScheme);`
    #[inline]
    pub unsafe fn GetProtocolHandler(&self, aScheme: *const libc::c_char, _retval: *mut*const nsIProtocolHandler) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolHandler)(self, aScheme, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns the protocol flags for a given scheme.
    ///      *
    ///      * @param aScheme the URI scheme
    ///      * @return value of corresponding nsIProtocolHandler::protocolFlags
    ///      */
    /// ```
    ///

    /// `unsigned long getProtocolFlags (in string aScheme);`
    #[inline]
    pub unsafe fn GetProtocolFlags(&self, aScheme: *const libc::c_char, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolFlags)(self, aScheme, _retval)
    }


    /// ```text
    /// /**
    ///      * This method constructs a new URI by determining the scheme of the
    ///      * URI spec, and then delegating the construction of the URI to the
    ///      * protocol handler for that scheme. QueryInterface can be used on
    ///      * the resulting URI object to obtain a more specific type of URI.
    ///      *
    ///      * @see nsIProtocolHandler::newURI
    ///      */
    /// ```
    ///

    /// `nsIURI newURI (in AUTF8String aSpec, [optional] in string aOriginCharset, [optional] in nsIURI aBaseURI);`
    #[inline]
    pub unsafe fn NewURI(&self, aSpec: *const ::nsstring::nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).NewURI)(self, aSpec, aOriginCharset, aBaseURI, _retval)
    }


    /// ```text
    /// /**
    ///      * This method constructs a new URI from a nsIFile.
    ///      *
    ///      * @param aFile specifies the file path
    ///      * @return reference to a new nsIURI object
    ///      *
    ///      * Note: in the future, for perf reasons we should allow
    ///      * callers to specify whether this is a file or directory by
    ///      * splitting this  into newDirURI() and newActualFileURI().
    ///      */
    /// ```
    ///

    /// `nsIURI newFileURI (in nsIFile aFile);`
    #[inline]
    pub unsafe fn NewFileURI(&self, aFile: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).NewFileURI)(self, aFile, _retval)
    }


    /// ```text
    /// /**
    ///      * Converts an internal URI (e.g. one that has a username and password in
        ///      * it) into one which we can expose to the user, for example on the URL bar.
    ///      *
    ///      * @param  aURI The URI to be converted.
    ///      * @return nsIURI The converted, exposable URI.
    ///      */
    /// ```
    ///

    /// `nsIURI createExposableURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn CreateExposableURI(&self, aURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).CreateExposableURI)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Creates a channel for a given URI.
    ///      *
    ///      * @param aURI
    ///      *        nsIURI from which to make a channel
    ///      * @param aLoadingNode
    ///      * @param aLoadingPrincipal
    ///      * @param aTriggeringPrincipal
    ///      * @param aSecurityFlags
    ///      * @param aContentPolicyType
    ///      *        These will be used as values for the nsILoadInfo object on the
    ///      *        created channel. For details, see nsILoadInfo in nsILoadInfo.idl
    ///      * @return reference to the new nsIChannel object
    ///      *
    ///      * Please note, if you provide both a loadingNode and a loadingPrincipal,
    ///      * then loadingPrincipal must be equal to loadingNode->NodePrincipal().
    ///      * But less error prone is to just supply a loadingNode.
    ///      *
    ///      * Keep in mind that URIs coming from a webpage should *never* use the
    ///      * systemPrincipal as the loadingPrincipal.
    ///      */
    /// ```
    ///

    /// `nsIChannel newChannelFromURI (in nsIURI aURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn NewChannelFromURI(&self, aURI: *const nsIURI, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewChannelFromURI)(self, aURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval)
    }



    /// `[noscript,nostdcall,notxpcom] nsresult NewChannelFromURIWithClientAndController (in nsIURI aURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in const_MaybeClientInfoRef aLoadingClientInfo, in const_MaybeServiceWorkerDescriptorRef aController, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType, in unsigned long aSandboxFlags, out nsIChannel aResult);`
    const _NewChannelFromURIWithClientAndController: () = ();

    /// ```text
    /// /**
    ///      * Equivalent to newChannelFromURI(aURI, aLoadingNode, ...)
    ///      */
    /// ```
    ///

    /// `nsIChannel newChannelFromURIWithLoadInfo (in nsIURI aURI, in nsILoadInfo aLoadInfo);`
    #[inline]
    pub unsafe fn NewChannelFromURIWithLoadInfo(&self, aURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewChannelFromURIWithLoadInfo)(self, aURI, aLoadInfo, _retval)
    }


    /// ```text
    /// /**
    ///      * Equivalent to newChannelFromURI(newURI(...))
    ///      */
    /// ```
    ///

    /// `nsIChannel newChannel (in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn NewChannel(&self, aSpec: *const ::nsstring::nsACString, aOriginCharset: *const libc::c_char, aBaseURI: *const nsIURI, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewChannel)(self, aSpec, aOriginCharset, aBaseURI, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns true if networking is in "offline" mode. When in offline mode,
    ///      * attempts to access the network will fail (although this does not
        ///      * necessarily correlate with whether there is actually a network
        ///      * available -- that's hard to detect without causing the dialer to
        ///      * come up).
    ///      *
    ///      * Changing this fires observer notifications ... see below.
    ///      */
    /// ```
    ///

    /// `attribute boolean offline;`
    #[inline]
    pub unsafe fn GetOffline(&self, aOffline: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOffline)(self, aOffline)
    }


    /// ```text
    /// /**
    ///      * Returns true if networking is in "offline" mode. When in offline mode,
    ///      * attempts to access the network will fail (although this does not
        ///      * necessarily correlate with whether there is actually a network
        ///      * available -- that's hard to detect without causing the dialer to
        ///      * come up).
    ///      *
    ///      * Changing this fires observer notifications ... see below.
    ///      */
    /// ```
    ///

    /// `attribute boolean offline;`
    #[inline]
    pub unsafe fn SetOffline(&self, aOffline: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetOffline)(self, aOffline)
    }


    /// ```text
    /// /**
    ///      * Returns false if there are no interfaces for a network request
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean connectivity;`
    #[inline]
    pub unsafe fn GetConnectivity(&self, aConnectivity: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectivity)(self, aConnectivity)
    }


    /// ```text
    /// /**
    ///      * Checks if a port number is banned. This involves consulting a list of
    ///      * unsafe ports, corresponding to network services that may be easily
    ///      * exploitable. If the given port is considered unsafe, then the protocol
    ///      * handler (corresponding to aScheme) will be asked whether it wishes to
    ///      * override the IO service's decision to block the port. This gives the
    ///      * protocol handler ultimate control over its own security policy while
    ///      * ensuring reasonable, default protection.
    ///      *
    ///      * @see nsIProtocolHandler::allowPort
    ///      */
    /// ```
    ///

    /// `boolean allowPort (in long aPort, in string aScheme);`
    #[inline]
    pub unsafe fn AllowPort(&self, aPort: i32, aScheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AllowPort)(self, aPort, aScheme, _retval)
    }


    /// ```text
    /// /**
    ///      * Utility to extract the scheme from a URL string, consistently and
    ///      * according to spec (see RFC 2396).
    ///      *
    ///      * NOTE: Most URL parsing is done via nsIURI, and in fact the scheme
    ///      * can also be extracted from a URL string via nsIURI.  This method
    ///      * is provided purely as an optimization.
    ///      *
    ///      * @param aSpec the URL string to parse
    ///      * @return URL scheme, lowercase
    ///      *
    ///      * @throws NS_ERROR_MALFORMED_URI if URL string is not of the right form.
    ///      */
    /// ```
    ///

    /// `ACString extractScheme (in AUTF8String urlString);`
    #[inline]
    pub unsafe fn ExtractScheme(&self, urlString: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ExtractScheme)(self, urlString, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks if a URI host is a local IPv4 or IPv6 address literal.
    ///      *
    ///      * @param nsIURI the URI that contains the hostname to check
    ///      * @return true if the URI hostname is a local IP address
    ///      */
    /// ```
    ///

    /// `boolean hostnameIsLocalIPAddress (in nsIURI aURI);`
    #[inline]
    pub unsafe fn HostnameIsLocalIPAddress(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HostnameIsLocalIPAddress)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks if a URI host is a shared IPv4 address literal.
    ///      *
    ///      * @param nsIURI the URI that contains the hostname to check
    ///      * @return true if the URI hostname is a shared IP address
    ///      */
    /// ```
    ///

    /// `boolean hostnameIsSharedIPAddress (in nsIURI aURI);`
    #[inline]
    pub unsafe fn HostnameIsSharedIPAddress(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HostnameIsSharedIPAddress)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * While this is set, IOService will monitor an nsINetworkLinkService
    ///      * (if available) and set its offline status to "true" whenever
    ///      * isLinkUp is false.
    ///      *
    ///      * Applications that want to control changes to the IOService's offline
    ///      * status should set this to false, watch for network:link-status-changed
    ///      * broadcasts, and change nsIIOService::offline as they see fit. Note
    ///      * that this means during application startup, IOService may be offline
    ///      * if there is no link, until application code runs and can turn off
    ///      * this management.
    ///      */
    /// ```
    ///

    /// `attribute boolean manageOfflineStatus;`
    #[inline]
    pub unsafe fn GetManageOfflineStatus(&self, aManageOfflineStatus: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetManageOfflineStatus)(self, aManageOfflineStatus)
    }


    /// ```text
    /// /**
    ///      * While this is set, IOService will monitor an nsINetworkLinkService
    ///      * (if available) and set its offline status to "true" whenever
    ///      * isLinkUp is false.
    ///      *
    ///      * Applications that want to control changes to the IOService's offline
    ///      * status should set this to false, watch for network:link-status-changed
    ///      * broadcasts, and change nsIIOService::offline as they see fit. Note
    ///      * that this means during application startup, IOService may be offline
    ///      * if there is no link, until application code runs and can turn off
    ///      * this management.
    ///      */
    /// ```
    ///

    /// `attribute boolean manageOfflineStatus;`
    #[inline]
    pub unsafe fn SetManageOfflineStatus(&self, aManageOfflineStatus: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetManageOfflineStatus)(self, aManageOfflineStatus)
    }


    /// ```text
    /// /**
    ///      * Creates a channel for a given URI.
    ///      *
    ///      * @param aURI
    ///      *        nsIURI from which to make a channel
    ///      * @param aProxyURI
    ///      *        nsIURI to use for proxy resolution. Can be null in which
    ///      *        case aURI is used
    ///      * @param aProxyFlags flags from nsIProtocolProxyService to use
    ///      *        when resolving proxies for this new channel
    ///      * @param aLoadingNode
    ///      * @param aLoadingPrincipal
    ///      * @param aTriggeringPrincipal
    ///      * @param aSecurityFlags
    ///      * @param aContentPolicyType
    ///      *        These will be used as values for the nsILoadInfo object on the
    ///      *        created channel. For details, see nsILoadInfo in nsILoadInfo.idl
    ///      * @return reference to the new nsIChannel object
    ///      *
    ///      * Please note, if you provide both a loadingNode and a loadingPrincipal,
    ///      * then loadingPrincipal must be equal to loadingNode->NodePrincipal().
    ///      * But less error prone is to just supply a loadingNode.
    ///      */
    /// ```
    ///

    /// `nsIChannel newChannelFromURIWithProxyFlags (in nsIURI aURI, in nsIURI aProxyURI, in unsigned long aProxyFlags, in Node aLoadingNode, in nsIPrincipal aLoadingPrincipal, in nsIPrincipal aTriggeringPrincipal, in unsigned long aSecurityFlags, in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn NewChannelFromURIWithProxyFlags(&self, aURI: *const nsIURI, aProxyURI: *const nsIURI, aProxyFlags: u32, aLoadingNode: *const libc::c_void, aLoadingPrincipal: *const nsIPrincipal, aTriggeringPrincipal: *const nsIPrincipal, aSecurityFlags: u32, aContentPolicyType: nsContentPolicyType, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewChannelFromURIWithProxyFlags)(self, aURI, aProxyURI, aProxyFlags, aLoadingNode, aLoadingPrincipal, aTriggeringPrincipal, aSecurityFlags, aContentPolicyType, _retval)
    }


    /// ```text
    /// /**
    ///      * Return true if socket process is launched.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean socketProcessLaunched;`
    #[inline]
    pub unsafe fn GetSocketProcessLaunched(&self, aSocketProcessLaunched: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSocketProcessLaunched)(self, aSocketProcessLaunched)
    }


}


/// `interface nsIIOServiceInternal : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIOServiceInternal {
    vtable: *const nsIIOServiceInternalVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIOServiceInternal.
unsafe impl XpCom for nsIIOServiceInternal {
    const IID: nsIID = nsID(0x6633c0bf, 0xd97a, 0x428f,
        [0x8e, 0xce, 0xcb, 0x6a, 0x65, 0x5f, 0xb9, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIOServiceInternal {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIOServiceInternal.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIOServiceInternalCoerce {
    /// Cheaply cast a value of this type from a `nsIIOServiceInternal`.
    fn coerce_from(v: &nsIIOServiceInternal) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIOServiceInternalCoerce for nsIIOServiceInternal {
    #[inline]
    fn coerce_from(v: &nsIIOServiceInternal) -> &Self {
        v
    }
}

impl nsIIOServiceInternal {
    /// Cast this `nsIIOServiceInternal` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIOServiceInternalCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIOServiceInternal {
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
impl<T: nsISupportsCoerce> nsIIOServiceInternalCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOServiceInternal) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIOServiceInternal
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIOServiceInternalVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void SetConnectivity (in boolean connectivity); */
    pub SetConnectivity: unsafe extern "system" fn (this: *const nsIIOServiceInternal, connectivity: bool) -> ::nserror::nsresult,

    /* void NotifyWakeup (); */
    pub NotifyWakeup: unsafe extern "system" fn (this: *const nsIIOServiceInternal) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIOServiceInternal {

    /// ```text
    /// /**
    ///      * This is an internal method that should only be called from ContentChild
    ///      * in order to pass the connectivity state from the chrome process to the
    ///      * content process. It throws if called outside the content process.
    ///      */
    /// ```
    ///

    /// `void SetConnectivity (in boolean connectivity);`
    #[inline]
    pub unsafe fn SetConnectivity(&self, connectivity: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetConnectivity)(self, connectivity)
    }


    /// ```text
    /// /**
    ///      * An internal method to asynchronously run our notifications that happen
    ///      * when we wake from sleep
    ///      */
    /// ```
    ///

    /// `void NotifyWakeup ();`
    #[inline]
    pub unsafe fn NotifyWakeup(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyWakeup)(self, )
    }


}


