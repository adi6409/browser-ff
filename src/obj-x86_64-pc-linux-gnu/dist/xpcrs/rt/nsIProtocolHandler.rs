//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolHandler.idl
//


/// `interface nsIProtocolHandlerWithDynamicFlags : nsISupports`
///

/// ```text
/// /**
///  * nsIProtocolHandlerWithDynamicFlags
///  *
///  * Protocols that wish to return different flags depending on the URI should
///  * implement this interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolHandlerWithDynamicFlags {
    vtable: *const nsIProtocolHandlerWithDynamicFlagsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolHandlerWithDynamicFlags.
unsafe impl XpCom for nsIProtocolHandlerWithDynamicFlags {
    const IID: nsIID = nsID(0x65a8e823, 0x0591, 0x4fc0,
        [0xa5, 0x6a, 0x03, 0x26, 0x5e, 0x0a, 0x4c, 0xe8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolHandlerWithDynamicFlags {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolHandlerWithDynamicFlags.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolHandlerWithDynamicFlagsCoerce {
    /// Cheaply cast a value of this type from a `nsIProtocolHandlerWithDynamicFlags`.
    fn coerce_from(v: &nsIProtocolHandlerWithDynamicFlags) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolHandlerWithDynamicFlagsCoerce for nsIProtocolHandlerWithDynamicFlags {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandlerWithDynamicFlags) -> &Self {
        v
    }
}

impl nsIProtocolHandlerWithDynamicFlags {
    /// Cast this `nsIProtocolHandlerWithDynamicFlags` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolHandlerWithDynamicFlagsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolHandlerWithDynamicFlags {
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
impl<T: nsISupportsCoerce> nsIProtocolHandlerWithDynamicFlagsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandlerWithDynamicFlags) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolHandlerWithDynamicFlags
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolHandlerWithDynamicFlagsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* unsigned long getFlagsForURI (in nsIURI aURI); */
    pub GetFlagsForURI: unsafe extern "system" fn (this: *const nsIProtocolHandlerWithDynamicFlags, aURI: *const nsIURI, _retval: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolHandlerWithDynamicFlags {


    /// `unsigned long getFlagsForURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetFlagsForURI(&self, aURI: *const nsIURI, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFlagsForURI)(self, aURI, _retval)
    }


}


/// `interface nsIProtocolHandler : nsISupports`
///

/// ```text
/// /**
///  * nsIProtocolHandler
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIProtocolHandler {
    vtable: *const nsIProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIProtocolHandler.
unsafe impl XpCom for nsIProtocolHandler {
    const IID: nsIID = nsID(0xa87210e6, 0x7c8c, 0x41f7,
        [0x86, 0x4d, 0xdf, 0x80, 0x90, 0x15, 0x19, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIProtocolHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIProtocolHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIProtocolHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIProtocolHandler`.
    fn coerce_from(v: &nsIProtocolHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIProtocolHandlerCoerce for nsIProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandler) -> &Self {
        v
    }
}

impl nsIProtocolHandler {
    /// Cast this `nsIProtocolHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIProtocolHandler {
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
impl<T: nsISupportsCoerce> nsIProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIProtocolHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIProtocolHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString scheme; */
    pub GetScheme: unsafe extern "system" fn (this: *const nsIProtocolHandler, aScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long defaultPort; */
    pub GetDefaultPort: unsafe extern "system" fn (this: *const nsIProtocolHandler, aDefaultPort: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long protocolFlags; */
    pub GetProtocolFlags: unsafe extern "system" fn (this: *const nsIProtocolHandler, aProtocolFlags: *mut u32) -> ::nserror::nsresult,

    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadinfo); */
    pub NewChannel: unsafe extern "system" fn (this: *const nsIProtocolHandler, aURI: *const nsIURI, aLoadinfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* boolean allowPort (in long port, in string scheme); */
    pub AllowPort: unsafe extern "system" fn (this: *const nsIProtocolHandler, port: i32, scheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIProtocolHandler {
    /// ```text
    /// /**************************************************************************
    ///      * Constants for the protocol flags (the first is the default mask, the
        ///      * others are deviations):
    ///      *
    ///      * NOTE: Implementation must ignore any flags they do not understand.
    ///      */
    /// /**
    ///      * standard full URI with authority component and concept of relative
    ///      * URIs (http, ftp, ...)
    ///      */
    /// ```
    ///

    pub const URI_STD: i64 = 0;

    /// ```text
    /// /**
    ///      * no concept of relative URIs (about, javascript, finger, ...)
    ///      */
    /// ```
    ///

    pub const URI_NORELATIVE: i64 = 1;

    /// ```text
    /// /**
    ///      * no authority component (file, ...)
    ///      */
    /// ```
    ///

    pub const URI_NOAUTH: i64 = 2;

    /// ```text
    /// /**
    ///      * This protocol handler can be proxied via a proxy (socks or http)
    ///      * (e.g., irc, smtp, http, etc.).  If the protocol supports transparent
    ///      * proxying, the handler should implement nsIProxiedProtocolHandler.
    ///      *
    ///      * If it supports only HTTP proxying, then it need not support
    ///      * nsIProxiedProtocolHandler, but should instead set the ALLOWS_PROXY_HTTP
    ///      * flag (see below).
    ///      *
    ///      * @see nsIProxiedProtocolHandler
    ///      */
    /// ```
    ///

    pub const ALLOWS_PROXY: i64 = 4;

    /// ```text
    /// /**
    ///      * This protocol handler can be proxied using a http proxy (e.g., http,
        ///      * ftp, etc.).  nsIIOService::newChannelFromURI will feed URIs from this
    ///      * protocol handler to the HTTP protocol handler instead.  This flag is
    ///      * ignored if ALLOWS_PROXY is not set.
    ///      */
    /// ```
    ///

    pub const ALLOWS_PROXY_HTTP: i64 = 8;

    /// ```text
    /// /**
    ///      * The URIs for this protocol have no inherent security context, so
    ///      * documents loaded via this protocol should inherit the security context
    ///      * from the document that loads them.
    ///      */
    /// ```
    ///

    pub const URI_INHERITS_SECURITY_CONTEXT: i64 = 16;

    /// ```text
    /// /**
    ///      * "Automatic" loads that would replace the document (e.g. <meta> refresh,
        ///      * certain types of XLinks, possibly other loads that the application
        ///      * decides are not user triggered) are not allowed if the originating (NOT
        ///      * the target) URI has this protocol flag.  Note that the decision as to
    ///      * what constitutes an "automatic" load is made externally, by the caller
    ///      * of nsIScriptSecurityManager::CheckLoadURI.  See documentation for that
    ///      * method for more information.
    ///      *
    ///      * A typical protocol that might want to set this flag is a protocol that
    ///      * shows highly untrusted content in a viewing area that the user expects
    ///      * to have a lot of control over, such as an e-mail reader.
    ///      */
    /// ```
    ///

    pub const URI_FORBIDS_AUTOMATIC_DOCUMENT_REPLACEMENT: i64 = 32;

    /// ```text
    /// /**
    ///      * +-------------------------------------------------------------------+
    ///      * |                                                                   |
    ///      * |  ALL PROTOCOL HANDLERS MUST SET ONE OF THE FOLLOWING FIVE FLAGS.  |
    ///      * |                                                                   |
    ///      * +-------------------------------------------------------------------+
    ///      *
    ///      *    * URI_LOADABLE_BY_ANYONE
    ///      *    * URI_DANGEROUS_TO_LOAD
    ///      *    * URI_IS_UI_RESOURCE
    ///      *    * URI_IS_LOCAL_FILE
    ///      *    * URI_LOADABLE_BY_SUBSUMERS
    ///      *
    ///      * These flags are used to determine who is allowed to load URIs for this
    ///      * protocol.  Note that if a URI is nested, only the flags for the
    ///      * innermost URI matter.  See nsINestedURI.
    ///      *
    ///      * If none of these five flags are set, the ContentSecurityManager will
    ///      * deny the load.
    ///      */
    /// /**
    ///      * The URIs for this protocol can be loaded by anyone.  For example, any
    ///      * website should be allowed to trigger a load of a URI for this protocol.
    ///      * Web-safe protocols like "http" should set this flag.
    ///      */
    /// ```
    ///

    pub const URI_LOADABLE_BY_ANYONE: i64 = 64;

    /// ```text
    /// /**
    ///      * The URIs for this protocol are UNSAFE if loaded by untrusted (web)
    ///      * content and may only be loaded by privileged code (for example, code
        ///      * which has the system principal).  Various internal protocols should set
    ///      * this flag.
    ///      */
    /// ```
    ///

    pub const URI_DANGEROUS_TO_LOAD: i64 = 128;

    /// ```text
    /// /**
    ///      * The URIs for this protocol point to resources that are part of the
    ///      * application's user interface.  There are cases when such resources may
    ///      * be made accessible to untrusted content such as web pages, so this is
    ///      * less restrictive than URI_DANGEROUS_TO_LOAD but more restrictive than
    ///      * URI_LOADABLE_BY_ANYONE.  See the documentation for
    ///      * nsIScriptSecurityManager::CheckLoadURI.
    ///      */
    /// ```
    ///

    pub const URI_IS_UI_RESOURCE: i64 = 256;

    /// ```text
    /// /**
    ///      * Loading of URIs for this protocol from other origins should only be
    ///      * allowed if those origins should have access to the local filesystem.
    ///      * It's up to the application to decide what origins should have such
    ///      * access.  Protocols like "file" that point to local data should set this
    ///      * flag.
    ///      */
    /// ```
    ///

    pub const URI_IS_LOCAL_FILE: i64 = 512;

    /// ```text
    /// /**
    ///      * The URIs for this protocol can be loaded only by callers with a
    ///      * principal that subsumes this uri. For example, privileged code and
    ///      * websites that are same origin as this uri.
    ///      */
    /// ```
    ///

    pub const URI_LOADABLE_BY_SUBSUMERS: i64 = 1024;

    /// ```text
    /// /**
    ///      * Channels using this protocol never call OnDataAvailable
    ///      * on the listener passed to AsyncOpen and they therefore
    ///      * do not return any data that we can use.
    ///      */
    /// ```
    ///

    pub const URI_DOES_NOT_RETURN_DATA: i64 = 2048;

    /// ```text
    /// /**
    ///      * URIs for this protocol are considered to be local resources.  This could
    ///      * be a local file (URI_IS_LOCAL_FILE), a UI resource (URI_IS_UI_RESOURCE),
    ///      * or something else that would not hit the network.
    ///      */
    /// ```
    ///

    pub const URI_IS_LOCAL_RESOURCE: i64 = 4096;

    /// ```text
    /// /**
    ///      * URIs for this protocol execute script when they are opened.
    ///      */
    /// ```
    ///

    pub const URI_OPENING_EXECUTES_SCRIPT: i64 = 8192;

    /// ```text
    /// /**
    ///      * Loading channels from this protocol has side-effects that make
    ///      * it unsuitable for saving to a local file.
    ///      */
    /// ```
    ///

    pub const URI_NON_PERSISTABLE: i64 = 16384;

    /// ```text
    /// /**
    ///      * URIs for this protocol require the webapps permission on the principal
    ///      * when opening URIs for a different domain. See bug#773886
    ///      */
    /// ```
    ///

    pub const URI_CROSS_ORIGIN_NEEDS_WEBAPPS_PERM: i64 = 32768;

    /// ```text
    /// /**
    ///      * Channels for this protocol don't need to spin the event loop to handle
    ///      * Open() and reads on the resulting stream.
    ///      */
    /// ```
    ///

    pub const URI_SYNC_LOAD_IS_OK: i64 = 65536;

    /// ```text
    /// /**
    ///      * All the origins whose URI has this scheme are considered potentially
    ///      * trustworthy.
    ///      * Per the SecureContext spec, https: and wss: should be considered
    ///      * a priori secure, and implementations may consider other,
    ///      * implementation-specific URI schemes as secure.
    ///      */
    /// ```
    ///

    pub const URI_IS_POTENTIALLY_TRUSTWORTHY: i64 = 131072;

    /// ```text
    /// /**
    ///      * This URI may be fetched and the contents are visible to anyone. This is
    ///      * semantically equivalent to the resource being served with all-access CORS
    ///      * headers.
    ///      */
    /// ```
    ///

    pub const URI_FETCHABLE_BY_ANYONE: i64 = 262144;

    /// ```text
    /// /**
    ///      * If this flag is set, then the origin for this protocol is the full URI
    ///      * spec, not just the scheme + host + port.
    ///      *
    ///      * Note: this is not supported in Firefox.  It is currently only available
    ///      * in Thunderbird and SeaMonkey.
    ///      */
    /// ```
    ///

    pub const ORIGIN_IS_FULL_SPEC: i64 = 524288;

    /// ```text
    /// /**
    ///      * If this flag is set, the URI does not always allow content using the same
    ///      * protocol to link to it.
    ///      */
    /// ```
    ///

    pub const URI_SCHEME_NOT_SELF_LINKABLE: i64 = 1048576;

    /// ```text
    /// /**
    ///      * The URIs for this protocol can be loaded by extensions.
    ///      */
    /// ```
    ///

    pub const URI_LOADABLE_BY_EXTENSIONS: i64 = 2097152;

    /// ```text
    /// /**
    ///      * The URIs for this protocol can not be loaded into private contexts.
    ///      */
    /// ```
    ///

    pub const URI_DISALLOW_IN_PRIVATE_CONTEXT: i64 = 4194304;

    /// ```text
    /// /**
    ///      * This protocol handler forbids accessing cookies e.g. for mail related
    ///      * protocols. Only used in Mailnews (comm-central).
    ///      */
    /// ```
    ///

    pub const URI_FORBIDS_COOKIE_ACCESS: i64 = 8388608;

    /// ```text
    /// /**
    ///      * The scheme of this protocol (e.g., "file").
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString scheme;`
    #[inline]
    pub unsafe fn GetScheme(&self, aScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetScheme)(self, aScheme)
    }


    /// ```text
    /// /**
    ///      * The default port is the port that this protocol normally uses.
    ///      * If a port does not make sense for the protocol (e.g., "about:")
    ///      * then -1 will be returned.
    ///      */
    /// ```
    ///

    /// `readonly attribute long defaultPort;`
    #[inline]
    pub unsafe fn GetDefaultPort(&self, aDefaultPort: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultPort)(self, aDefaultPort)
    }


    /// ```text
    /// /**
    ///      * Returns the protocol specific flags (see flag definitions below).
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long protocolFlags;`
    #[inline]
    pub unsafe fn GetProtocolFlags(&self, aProtocolFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetProtocolFlags)(self, aProtocolFlags)
    }


    /// ```text
    /// /**
    ///      * Constructs a new channel from the given URI for this protocol handler and
    ///      * sets the loadInfo for the constructed channel.
    ///      */
    /// ```
    ///

    /// `nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadinfo);`
    #[inline]
    pub unsafe fn NewChannel(&self, aURI: *const nsIURI, aLoadinfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewChannel)(self, aURI, aLoadinfo, _retval)
    }


    /// ```text
    /// /**
    ///      * Allows a protocol to override blacklisted ports.
    ///      *
    ///      * This method will be called when there is an attempt to connect to a port
    ///      * that is blacklisted.  For example, for most protocols, port 25 (Simple Mail
        ///      * Transfer) is banned.  When a URI containing this "known-to-do-bad-things"
    ///      * port number is encountered, this function will be called to ask if the
    ///      * protocol handler wants to override the ban.
    ///      */
    /// ```
    ///

    /// `boolean allowPort (in long port, in string scheme);`
    #[inline]
    pub unsafe fn AllowPort(&self, port: i32, scheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AllowPort)(self, port, scheme, _retval)
    }


}


