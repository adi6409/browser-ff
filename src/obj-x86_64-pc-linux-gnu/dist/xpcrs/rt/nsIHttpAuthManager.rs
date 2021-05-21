//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpAuthManager.idl
//


/// `interface nsIHttpAuthManager : nsISupports`
///

/// ```text
/// /**
///  * nsIHttpAuthManager
///  *
///  * This service provides access to cached HTTP authentication
///  * user credentials (domain, username, password) for sites
///  * visited during the current browser session.
///  *
///  * This interface exists to provide other HTTP stacks with the
///  * ability to share HTTP authentication credentials with Necko.
///  * This is currently used by the Java plugin (version 1.5 and
    ///  * higher) to avoid duplicate authentication prompts when the
///  * Java client fetches content from a HTTP site that the user
///  * has already logged into.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpAuthManager {
    vtable: *const nsIHttpAuthManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpAuthManager.
unsafe impl XpCom for nsIHttpAuthManager {
    const IID: nsIID = nsID(0x54f90444, 0xc52b, 0x4d2d,
        [0x89, 0x16, 0xc5, 0x9a, 0x2b, 0xb2, 0x59, 0x38]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpAuthManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpAuthManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpAuthManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpAuthManager`.
    fn coerce_from(v: &nsIHttpAuthManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpAuthManagerCoerce for nsIHttpAuthManager {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthManager) -> &Self {
        v
    }
}

impl nsIHttpAuthManager {
    /// Cast this `nsIHttpAuthManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpAuthManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpAuthManager {
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
impl<T: nsISupportsCoerce> nsIHttpAuthManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpAuthManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpAuthManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpAuthManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void getAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, out AString aUserDomain, out AString aUserName, out AString aUserPassword, [optional] in bool aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
    pub GetAuthIdentity: unsafe extern "system" fn (this: *const nsIHttpAuthManager, aScheme: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: int32_t, aAuthType: *const ::nsstring::nsACString, aRealm: *const ::nsstring::nsACString, aPath: *const ::nsstring::nsACString, aUserDomain: *mut ::nsstring::nsAString, aUserName: *mut ::nsstring::nsAString, aUserPassword: *mut ::nsstring::nsAString, aIsPrivate: bool, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [must_use] void setAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, in AString aUserDomain, in AString aUserName, in AString aUserPassword, [optional] in boolean aIsPrivate, [optional] in nsIPrincipal aPrincipal); */
    pub SetAuthIdentity: unsafe extern "system" fn (this: *const nsIHttpAuthManager, aScheme: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: int32_t, aAuthType: *const ::nsstring::nsACString, aRealm: *const ::nsstring::nsACString, aPath: *const ::nsstring::nsACString, aUserDomain: *const ::nsstring::nsAString, aUserName: *const ::nsstring::nsAString, aUserPassword: *const ::nsstring::nsAString, aIsPrivate: bool, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* [must_use] void clearAll (); */
    pub ClearAll: unsafe extern "system" fn (this: *const nsIHttpAuthManager) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpAuthManager {

    /// ```text
    /// /**
    ///      * Lookup auth identity.
    ///      *
    ///      * @param aScheme
    ///      *        the URL scheme (e.g., "http").  NOTE: for proxy authentication,
    ///      *        this should be "http" (this includes authentication for CONNECT
        ///      *        tunneling).
    ///      * @param aHost
    ///      *        the host of the server issuing a challenge (ASCII only).
    ///      * @param aPort
    ///      *        the port of the server issuing a challenge.
    ///      * @param aAuthType
    ///      *        optional string identifying auth type used (e.g., "basic")
    ///      * @param aRealm
    ///      *        optional string identifying auth realm.
    ///      * @param aPath
    ///      *        optional string identifying auth path. empty for proxy auth.
    ///      * @param aUserDomain
    ///      *        return value containing user domain.
    ///      * @param aUserName
    ///      *        return value containing user name.
    ///      * @param aUserPassword
    ///      *        return value containing user password.
    ///      * @param aIsPrivate
    ///      *        whether to look up a private or public identity (they are
        ///      *        stored separately, for use by private browsing)
    ///      * @param aPrincipal
    ///      *        the principal from which to derive information about which
    ///      *        app/mozbrowser is in use for this request
    ///      */
    /// ```
    ///

    /// `[must_use] void getAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, out AString aUserDomain, out AString aUserName, out AString aUserPassword, [optional] in bool aIsPrivate, [optional] in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn GetAuthIdentity(&self, aScheme: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: int32_t, aAuthType: *const ::nsstring::nsACString, aRealm: *const ::nsstring::nsACString, aPath: *const ::nsstring::nsACString, aUserDomain: *mut ::nsstring::nsAString, aUserName: *mut ::nsstring::nsAString, aUserPassword: *mut ::nsstring::nsAString, aIsPrivate: bool, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetAuthIdentity)(self, aScheme, aHost, aPort, aAuthType, aRealm, aPath, aUserDomain, aUserName, aUserPassword, aIsPrivate, aPrincipal)
    }


    /// ```text
    /// /**
    ///      * Store auth identity.
    ///      *
    ///      * @param aScheme
    ///      *        the URL scheme (e.g., "http").  NOTE: for proxy authentication,
    ///      *        this should be "http" (this includes authentication for CONNECT
        ///      *        tunneling).
    ///      * @param aHost
    ///      *        the host of the server issuing a challenge (ASCII only).
    ///      * @param aPort
    ///      *        the port of the server issuing a challenge.
    ///      * @param aAuthType
    ///      *        optional string identifying auth type used (e.g., "basic")
    ///      * @param aRealm
    ///      *        optional string identifying auth realm.
    ///      * @param aPath
    ///      *        optional string identifying auth path. empty for proxy auth.
    ///      * @param aUserDomain
    ///      *        optional string containing user domain.
    ///      * @param aUserName
    ///      *        optional string containing user name.
    ///      * @param aUserPassword
    ///      *        optional string containing user password.
    ///      * @param aIsPrivate
    ///      *        whether to store a private or public identity (they are
        ///      *        stored separately, for use by private browsing)
    ///      * @param aPrincipal
    ///      *        the principal from which to derive information about which
    ///      *        app/mozbrowser is in use for this request
    ///      */
    /// ```
    ///

    /// `[must_use] void setAuthIdentity (in ACString aScheme, in ACString aHost, in int32_t aPort, in ACString aAuthType, in ACString aRealm, in ACString aPath, in AString aUserDomain, in AString aUserName, in AString aUserPassword, [optional] in boolean aIsPrivate, [optional] in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn SetAuthIdentity(&self, aScheme: *const ::nsstring::nsACString, aHost: *const ::nsstring::nsACString, aPort: int32_t, aAuthType: *const ::nsstring::nsACString, aRealm: *const ::nsstring::nsACString, aPath: *const ::nsstring::nsACString, aUserDomain: *const ::nsstring::nsAString, aUserName: *const ::nsstring::nsAString, aUserPassword: *const ::nsstring::nsAString, aIsPrivate: bool, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).SetAuthIdentity)(self, aScheme, aHost, aPort, aAuthType, aRealm, aPath, aUserDomain, aUserName, aUserPassword, aIsPrivate, aPrincipal)
    }


    /// ```text
    /// /**
    ///      * Clear all auth cache.
    ///      */
    /// ```
    ///

    /// `[must_use] void clearAll ();`
    #[inline]
    pub unsafe fn ClearAll(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearAll)(self, )
    }


}


