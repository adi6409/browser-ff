//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAuthModule.idl
//


/// `interface nsIAuthModule : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAuthModule {
    vtable: *const nsIAuthModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAuthModule.
unsafe impl XpCom for nsIAuthModule {
    const IID: nsIID = nsID(0x6e35dbc0, 0x49ef, 0x4e2c,
        [0xb1, 0xea, 0xb7, 0x2e, 0xc6, 0x44, 0x50, 0xa2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAuthModule {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAuthModule.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAuthModuleCoerce {
    /// Cheaply cast a value of this type from a `nsIAuthModule`.
    fn coerce_from(v: &nsIAuthModule) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAuthModuleCoerce for nsIAuthModule {
    #[inline]
    fn coerce_from(v: &nsIAuthModule) -> &Self {
        v
    }
}

impl nsIAuthModule {
    /// Cast this `nsIAuthModule` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAuthModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAuthModule {
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
impl<T: nsISupportsCoerce> nsIAuthModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAuthModule) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAuthModule
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAuthModuleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword); */
    pub Init: unsafe extern "system" fn (this: *const nsIAuthModule, aServiceName: *const libc::c_char, aServiceFlags: u32, aDomain: *const i16, aUsername: *const i16, aPassword: *const i16) -> ::nserror::nsresult,

    /* void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    pub GetNextToken: unsafe extern "system" fn (this: *const nsIAuthModule, aInToken: *const libc::c_void, aInTokenLength: u32, aOutToken: *mut *mut libc::c_void, aOutTokenLength: *mut u32) -> ::nserror::nsresult,

    /* void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    pub Wrap: unsafe extern "system" fn (this: *const nsIAuthModule, aInToken: *const libc::c_void, aInTokenLength: u32, confidential: bool, aOutToken: *mut *mut libc::c_void, aOutTokenLength: *mut u32) -> ::nserror::nsresult,

    /* void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength); */
    pub Unwrap: unsafe extern "system" fn (this: *const nsIAuthModule, aInToken: *const libc::c_void, aInTokenLength: u32, aOutToken: *mut *mut libc::c_void, aOutTokenLength: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAuthModule {
    /// ```text
    /// /**
    ///      * Default behavior.
    ///      */
    /// ```
    ///

    pub const REQ_DEFAULT: i64 = 0;

    /// ```text
    /// /**
    ///      * Client and server will be authenticated.
    ///      */
    /// ```
    ///

    pub const REQ_MUTUAL_AUTH: i64 = 1;

    /// ```text
    /// /**
    ///      * The server is allowed to impersonate the client.  The REQ_MUTUAL_AUTH
    ///      * flag may also need to be specified in order for this flag to take
    ///      * effect.
    ///      */
    /// ```
    ///

    pub const REQ_DELEGATE: i64 = 2;

    /// ```text
    /// /**
    ///      * The authentication is required for a proxy connection.
    ///      */
    /// ```
    ///

    pub const REQ_PROXY_AUTH: i64 = 4;

    /// ```text
    /// /**
    ///      * Flags used for telemetry.
    ///      */
    /// ```
    ///

    pub const NTLM_MODULE_SAMBA_AUTH_PROXY: i64 = 0;


    pub const NTLM_MODULE_SAMBA_AUTH_DIRECT: i64 = 1;


    pub const NTLM_MODULE_WIN_API_PROXY: i64 = 2;


    pub const NTLM_MODULE_WIN_API_DIRECT: i64 = 3;


    pub const NTLM_MODULE_GENERIC_PROXY: i64 = 4;


    pub const NTLM_MODULE_GENERIC_DIRECT: i64 = 5;


    pub const NTLM_MODULE_KERBEROS_PROXY: i64 = 6;


    pub const NTLM_MODULE_KERBEROS_DIRECT: i64 = 7;

    /// ```text
    /// /** Other flags may be defined in the future */
    /// /**
    ///      * Called to initialize an auth module.  The other methods cannot be called
    ///      * unless this method succeeds.
    ///      *
    ///      * @param aServiceName
    ///      *        the service name, which may be null if not applicable (e.g., for
        ///      *        NTLM, this parameter should be null).
    ///      * @param aServiceFlags
    ///      *        a bitwise-or of the REQ_ flags defined above (pass REQ_DEFAULT
        ///      *        for default behavior).
    ///      * @param aDomain
    ///      *        the authentication domain, which may be null if not applicable.
    ///      * @param aUsername
    ///      *        the user's login name
    ///      * @param aPassword
    ///      *        the user's password
    ///      */
    /// ```
    ///

    /// `void init (in string aServiceName, in unsigned long aServiceFlags, in wstring aDomain, in wstring aUsername, in wstring aPassword);`
    #[inline]
    pub unsafe fn Init(&self, aServiceName: *const libc::c_char, aServiceFlags: u32, aDomain: *const i16, aUsername: *const i16, aPassword: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aServiceName, aServiceFlags, aDomain, aUsername, aPassword)
    }


    /// ```text
    /// /**
    ///      * Called to get the next token in a sequence of authentication steps.
    ///      *
    ///      * @param aInToken
    ///      *        A buffer containing the input token (e.g., a challenge from a
        ///      *        server).  This may be null.
    ///      * @param aInTokenLength
    ///      *        The length of the input token.
    ///      * @param aOutToken
    ///      *        If getNextToken succeeds, then aOutToken will point to a buffer
    ///      *        to be sent in response to the server challenge.  The length of
    ///      *        this buffer is given by aOutTokenLength.  The buffer at aOutToken
    ///      *        must be recycled with a call to free.
    ///      * @param aOutTokenLength
    ///      *        If getNextToken succeeds, then aOutTokenLength contains the
    ///      *        length of the buffer (number of bytes) pointed to by aOutToken.
    ///      */
    /// ```
    ///

    /// `void getNextToken ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength);`
    #[inline]
    pub unsafe fn GetNextToken(&self, aInToken: *const libc::c_void, aInTokenLength: u32, aOutToken: *mut *mut libc::c_void, aOutTokenLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNextToken)(self, aInToken, aInTokenLength, aOutToken, aOutTokenLength)
    }


    /// ```text
    /// /**
    ///      * Once a security context has been established through calls to GetNextToken()
    ///      * it may be used to protect data exchanged between client and server. Calls
    ///      * to Wrap() are used to protect items of data to be sent to the server.
    ///      *
    ///      * @param aInToken
    ///      *        A buffer containing the data to be sent to the server
    ///      * @param aInTokenLength
    ///      *        The length of the input token
    ///      * @param confidential
    ///      *        If set to true, Wrap() will encrypt the data, otherwise data will
    ///      *        just be integrity protected (checksummed)
    ///      * @param aOutToken
    ///      *        A buffer containing the resulting data to be sent to the server
    ///      * @param aOutTokenLength
    ///      *        The length of the output token buffer
    ///      *
    ///      * Wrap() may return NS_ERROR_NOT_IMPLEMENTED, if the underlying authentication
    ///      * mechanism does not support security layers.
    ///      */
    /// ```
    ///

    /// `void wrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, in boolean confidential, out voidPtr aOutToken, out unsigned long aOutTokenLength);`
    #[inline]
    pub unsafe fn Wrap(&self, aInToken: *const libc::c_void, aInTokenLength: u32, confidential: bool, aOutToken: *mut *mut libc::c_void, aOutTokenLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Wrap)(self, aInToken, aInTokenLength, confidential, aOutToken, aOutTokenLength)
    }


    /// ```text
    /// /**
    ///      * Unwrap() is used to unpack, decrypt, and verify the checksums on data
    ///      * returned by a server when security layers are in use.
    ///      *
    ///      * @param aInToken
    ///      *        A buffer containing the data received from the server
    ///      * @param aInTokenLength
    ///      *        The length of the input token
    ///      * @param aOutToken
    ///      *        A buffer containing the plaintext data from the server
    ///      * @param aOutTokenLength
    ///      *        The length of the output token buffer
    ///      *
    ///      * Unwrap() may return NS_ERROR_NOT_IMPLEMENTED, if the underlying
    ///      * authentication mechanism does not support security layers.
    ///      */
    /// ```
    ///

    /// `void unwrap ([const] in voidPtr aInToken, in unsigned long aInTokenLength, out voidPtr aOutToken, out unsigned long aOutTokenLength);`
    #[inline]
    pub unsafe fn Unwrap(&self, aInToken: *const libc::c_void, aInTokenLength: u32, aOutToken: *mut *mut libc::c_void, aOutTokenLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Unwrap)(self, aInToken, aInTokenLength, aOutToken, aOutTokenLength)
    }


}


