//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthenticator.idl
//


/// `interface nsIHttpAuthenticator : nsISupports`
///

/// ```text
/// /**
///  * nsIHttpAuthenticator
///  *
///  * Interface designed to allow for pluggable HTTP authentication modules.
///  * Implementations are registered under the ContractID:
///  *
///  *   "@mozilla.org/network/http-authenticator;1?scheme=<auth-scheme>"
///  *
///  * where <auth-scheme> is the lower-cased value of the authentication scheme
///  * found in the server challenge per the rules of RFC 2617.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpAuthenticator {
    vtable: *const nsIHttpAuthenticatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpAuthenticator.
unsafe impl XpCom for nsIHttpAuthenticator {
    const IID: nsIID = nsID(0xfef7db8a, 0xa4e2, 0x49d1,
        [0x96, 0x85, 0x19, 0xed, 0x7e, 0x30, 0x9b, 0x7d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpAuthenticator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpAuthenticator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpAuthenticatorCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpAuthenticator`.
    fn coerce_from(v: &nsIHttpAuthenticator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpAuthenticatorCoerce for nsIHttpAuthenticator {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticator) -> &Self {
        v
    }
}

impl nsIHttpAuthenticator {
    /// Cast this `nsIHttpAuthenticator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpAuthenticatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpAuthenticator {
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
impl<T: nsISupportsCoerce> nsIHttpAuthenticatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthenticator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpAuthenticator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpAuthenticatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void challengeReceived (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, inout nsISupports aSessionState, inout nsISupports aContinuationState, out boolean aInvalidatesIdentity); */
    pub ChallengeReceived: unsafe extern "system" fn (this: *const nsIHttpAuthenticator, aChannel: *const nsIHttpAuthenticableChannel, aChallenge: *const libc::c_char, aProxyAuth: bool, aSessionState: *mut *const nsISupports, aContinuationState: *mut *const nsISupports, aInvalidatesIdentity: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void generateCredentialsAsync (in nsIHttpAuthenticableChannel aChannel, in nsIHttpAuthenticatorCallback aCallback, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, in nsISupports aSessionState, in nsISupports aContinuationState, out nsICancelable aCancel); */
    pub GenerateCredentialsAsync: unsafe extern "system" fn (this: *const nsIHttpAuthenticator, aChannel: *const nsIHttpAuthenticableChannel, aCallback: *const nsIHttpAuthenticatorCallback, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const i16, aUser: *const i16, aPassword: *const i16, aSessionState: *const nsISupports, aContinuationState: *const nsISupports, aCancel: *mut*const nsICancelable) -> ::nserror::nsresult,

    /* [must_use] string generateCredentials (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, inout nsISupports aSessionState, inout nsISupports aContinuationState, out unsigned long aFlags); */
    pub GenerateCredentials: unsafe extern "system" fn (this: *const nsIHttpAuthenticator, aChannel: *const nsIHttpAuthenticableChannel, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const i16, aUser: *const i16, aPassword: *const i16, aSessionState: *mut *const nsISupports, aContinuationState: *mut *const nsISupports, aFlags: *mut u32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* [must_use] readonly attribute unsigned long authFlags; */
    pub GetAuthFlags: unsafe extern "system" fn (this: *const nsIHttpAuthenticator, aAuthFlags: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpAuthenticator {
    /// ```text
    /// /**
    ///      * Generate flags
    ///      */
    /// /**
    ///      * Indicates that the authenticator has used an out-of-band or internal
    ///      * source of identity and tells the consumer that it must not cache
    ///      * the returned identity because it might not be valid and would overwrite
    ///      * the cached identity.  See bug 542318 comment 32.
    ///      */
    /// ```
    ///

    pub const USING_INTERNAL_IDENTITY: i64 = 1;

    /// ```text
    /// /**
    ///      * A request based authentication scheme only authenticates an individual
    ///      * request (or a set of requests under the same authentication domain as
        ///      * defined by RFC 2617).  BASIC and DIGEST are request based authentication
    ///      * schemes.
    ///      */
    /// ```
    ///

    pub const REQUEST_BASED: i64 = 1;

    /// ```text
    /// /**
    ///      * A connection based authentication scheme authenticates an individual
    ///      * connection.  Multiple requests may be issued over the connection without
    ///      * repeating the authentication steps.  Connection based authentication
    ///      * schemes can associate state with the connection being authenticated via
    ///      * the aContinuationState parameter (see generateCredentials).
    ///      */
    /// ```
    ///

    pub const CONNECTION_BASED: i64 = 2;

    /// ```text
    /// /**
    ///      * The credentials returned from generateCredentials may be reused with any
    ///      * other URLs within "the protection space" as defined by RFC 2617 section
    ///      * 1.2.  If this flag is not set, then generateCredentials must be called
    ///      * for each request within the protection space.  REUSABLE_CREDENTIALS
    ///      * implies REUSABLE_CHALLENGE.
    ///      */
    /// ```
    ///

    pub const REUSABLE_CREDENTIALS: i64 = 4;

    /// ```text
    /// /**
    ///      * A challenge may be reused to later generate credentials in anticipation
    ///      * of a duplicate server challenge for URLs within "the protection space"
    ///      * as defined by RFC 2617 section 1.2.
    ///      */
    /// ```
    ///

    pub const REUSABLE_CHALLENGE: i64 = 8;

    /// ```text
    /// /**
    ///      * This flag indicates that the identity of the user is not required by
    ///      * this authentication scheme.
    ///      */
    /// ```
    ///

    pub const IDENTITY_IGNORED: i64 = 1024;

    /// ```text
    /// /**
    ///      * This flag indicates that the identity of the user includes a domain
    ///      * attribute that the user must supply.
    ///      */
    /// ```
    ///

    pub const IDENTITY_INCLUDES_DOMAIN: i64 = 2048;

    /// ```text
    /// /**
    ///      * This flag indicates that the identity will be sent encrypted. It does
    ///      * not make sense to combine this flag with IDENTITY_IGNORED.
    ///      */
    /// ```
    ///

    pub const IDENTITY_ENCRYPTED: i64 = 4096;

    /// ```text
    /// /**
    ///      * Upon receipt of a server challenge, this function is called to determine
    ///      * whether or not the current user identity has been rejected.  If true,
    ///      * then the user will be prompted by the channel to enter (or revise) their
    ///      * identity.  Following this, generateCredentials will be called.
    ///      *
    ///      * If the IDENTITY_IGNORED auth flag is set, then the aInvalidateIdentity
    ///      * return value will be ignored, and user prompting will be suppressed.
    ///      *
    ///      * @param aChannel
    ///      *        the http channel that received the challenge.
    ///      * @param aChallenge
    ///      *        the challenge from the WWW-Authenticate/Proxy-Authenticate
    ///      *        server response header.  (possibly from the auth cache.)
    ///      * @param aProxyAuth
    ///      *        flag indicating whether or not aChallenge is from a proxy.
    ///      * @param aSessionState
    ///      *        see description below for generateCredentials.
    ///      * @param aContinuationState
    ///      *        see description below for generateCredentials.
    ///      * @param aInvalidateIdentity
    ///      *        return value indicating whether or not to prompt the user for a
    ///      *        revised identity.
    ///      */
    /// ```
    ///

    /// `[must_use] void challengeReceived (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, inout nsISupports aSessionState, inout nsISupports aContinuationState, out boolean aInvalidatesIdentity);`
    #[inline]
    pub unsafe fn ChallengeReceived(&self, aChannel: *const nsIHttpAuthenticableChannel, aChallenge: *const libc::c_char, aProxyAuth: bool, aSessionState: *mut *const nsISupports, aContinuationState: *mut *const nsISupports, aInvalidatesIdentity: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ChallengeReceived)(self, aChannel, aChallenge, aProxyAuth, aSessionState, aContinuationState, aInvalidatesIdentity)
    }


    /// ```text
    /// /**
    ///      * Called to generate the authentication credentials for a particular
    ///      * server/proxy challenge asynchronously. Credentials will be sent back
    ///      * to the server via an Authorization/Proxy-Authorization header.
    ///      *
    ///      * @param aChannel
    ///      *        the http channel requesting credentials
    ///      * @param aCallback
    ///      *        callback function to be called when credentials are available
    ///      * @param aChallenge
    ///      *        the challenge from the WWW-Authenticate/Proxy-Authenticate
    ///      *        server response header.  (possibly from the auth cache.)
    ///      * @param aProxyAuth
    ///      *        flag indicating whether or not aChallenge is from a proxy.
    ///      * @param aDomain
    ///      *        string containing the domain name (if appropriate)
    ///      * @param aUser
    ///      *        string containing the user name
    ///      * @param aPassword
    ///      *        string containing the password
    ///      * @param aSessionState
    ///      *        state stored along side the user's identity in the auth cache
    ///      *        for the lifetime of the browser session.  if a new auth cache
    ///      *        entry is created for this challenge, then this parameter will
    ///      *        be null.  on return, the result will be stored in the new auth
    ///      *        cache entry.  this parameter is non-null when an auth cache entry
    ///      *        is being reused. currently modification of session state is not
    ///      *        communicated to caller, thus caching credentials obtained by
    ///      *        asynchronous way is not supported.
    ///      * @param aContinuationState
    ///      *        state held by the channel between consecutive calls to
    ///      *        generateCredentials, assuming multiple calls are required
    ///      *        to authenticate.  this state is held for at most the lifetime of
    ///      *        the channel.
    ///      * @pram aCancel
    ///      *        returns cancellable runnable object which caller can use to cancel
    ///      *        calling aCallback when finished.
    ///      */
    /// ```
    ///

    /// `[must_use] void generateCredentialsAsync (in nsIHttpAuthenticableChannel aChannel, in nsIHttpAuthenticatorCallback aCallback, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, in nsISupports aSessionState, in nsISupports aContinuationState, out nsICancelable aCancel);`
    #[inline]
    pub unsafe fn GenerateCredentialsAsync(&self, aChannel: *const nsIHttpAuthenticableChannel, aCallback: *const nsIHttpAuthenticatorCallback, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const i16, aUser: *const i16, aPassword: *const i16, aSessionState: *const nsISupports, aContinuationState: *const nsISupports, aCancel: *mut*const nsICancelable) -> ::nserror::nsresult {
        ((*self.vtable).GenerateCredentialsAsync)(self, aChannel, aCallback, aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState, aContinuationState, aCancel)
    }


    /// ```text
    /// /**
    ///      * Called to generate the authentication credentials for a particular
    ///      * server/proxy challenge.  This is the value that will be sent back
    ///      * to the server via an Authorization/Proxy-Authorization header.
    ///      *
    ///      * This function may be called using a cached challenge provided the
    ///      * authenticator sets the REUSABLE_CHALLENGE flag.
    ///      *
    ///      * @param aChannel
    ///      *        the http channel requesting credentials
    ///      * @param aChallenge
    ///      *        the challenge from the WWW-Authenticate/Proxy-Authenticate
    ///      *        server response header.  (possibly from the auth cache.)
    ///      * @param aProxyAuth
    ///      *        flag indicating whether or not aChallenge is from a proxy.
    ///      * @param aDomain
    ///      *        string containing the domain name (if appropriate)
    ///      * @param aUser
    ///      *        string containing the user name
    ///      * @param aPassword
    ///      *        string containing the password
    ///      * @param aSessionState
    ///      *        state stored along side the user's identity in the auth cache
    ///      *        for the lifetime of the browser session.  if a new auth cache
    ///      *        entry is created for this challenge, then this parameter will
    ///      *        be null.  on return, the result will be stored in the new auth
    ///      *        cache entry.  this parameter is non-null when an auth cache entry
    ///      *        is being reused.
    ///      * @param aContinuationState
    ///      *        state held by the channel between consecutive calls to
    ///      *        generateCredentials, assuming multiple calls are required
    ///      *        to authenticate.  this state is held for at most the lifetime of
    ///      *        the channel.
    ///      * @param aFlags
    ///      *        authenticator may return one of the generate flags bellow.
    ///      */
    /// ```
    ///

    /// `[must_use] string generateCredentials (in nsIHttpAuthenticableChannel aChannel, in string aChallenge, in boolean aProxyAuth, in wstring aDomain, in wstring aUser, in wstring aPassword, inout nsISupports aSessionState, inout nsISupports aContinuationState, out unsigned long aFlags);`
    #[inline]
    pub unsafe fn GenerateCredentials(&self, aChannel: *const nsIHttpAuthenticableChannel, aChallenge: *const libc::c_char, aProxyAuth: bool, aDomain: *const i16, aUser: *const i16, aPassword: *const i16, aSessionState: *mut *const nsISupports, aContinuationState: *mut *const nsISupports, aFlags: *mut u32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GenerateCredentials)(self, aChannel, aChallenge, aProxyAuth, aDomain, aUser, aPassword, aSessionState, aContinuationState, aFlags, _retval)
    }


    /// ```text
    /// /**
    ///      * Flags defining various properties of the authenticator.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute unsigned long authFlags;`
    #[inline]
    pub unsafe fn GetAuthFlags(&self, aAuthFlags: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetAuthFlags)(self, aAuthFlags)
    }


}


