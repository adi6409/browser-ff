//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookie.idl
//


/// `typedef int32_t  nsCookieStatus;`
///

/// ```text
/// /**
///  * Main cookie object interface.
///  */
/// ```
///

pub type nsCookieStatus = i32;


/// `typedef int32_t  nsCookiePolicy;`
///


pub type nsCookiePolicy = i32;


/// `interface nsICookie : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICookie {
    vtable: *const nsICookieVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICookie.
unsafe impl XpCom for nsICookie {
    const IID: nsIID = nsID(0xadf0db5e, 0x211e, 0x45a3,
        [0xbe, 0x14, 0x44, 0x86, 0xac, 0x43, 0x0a, 0x58]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICookie {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICookie.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICookieCoerce {
    /// Cheaply cast a value of this type from a `nsICookie`.
    fn coerce_from(v: &nsICookie) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICookieCoerce for nsICookie {
    #[inline]
    fn coerce_from(v: &nsICookie) -> &Self {
        v
    }
}

impl nsICookie {
    /// Cast this `nsICookie` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICookieCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICookie {
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
impl<T: nsISupportsCoerce> nsICookieCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookie) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICookie
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICookieVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsICookie, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String value; */
    pub GetValue: unsafe extern "system" fn (this: *const nsICookie, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean isDomain; */
    pub GetIsDomain: unsafe extern "system" fn (this: *const nsICookie, aIsDomain: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsICookie, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String rawHost; */
    pub GetRawHost: unsafe extern "system" fn (this: *const nsICookie, aRawHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String path; */
    pub GetPath: unsafe extern "system" fn (this: *const nsICookie, aPath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean isSecure; */
    pub GetIsSecure: unsafe extern "system" fn (this: *const nsICookie, aIsSecure: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute uint64_t expires; */
    pub GetExpires: unsafe extern "system" fn (this: *const nsICookie, aExpires: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute int64_t expiry; */
    pub GetExpiry: unsafe extern "system" fn (this: *const nsICookie, aExpiry: *mut int64_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* readonly attribute boolean isSession; */
    pub GetIsSession: unsafe extern "system" fn (this: *const nsICookie, aIsSession: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isHttpOnly; */
    pub GetIsHttpOnly: unsafe extern "system" fn (this: *const nsICookie, aIsHttpOnly: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute int64_t creationTime; */
    pub GetCreationTime: unsafe extern "system" fn (this: *const nsICookie, aCreationTime: *mut int64_t) -> ::nserror::nsresult,

    /* readonly attribute int64_t lastAccessed; */
    pub GetLastAccessed: unsafe extern "system" fn (this: *const nsICookie, aLastAccessed: *mut int64_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t sameSite; */
    pub GetSameSite: unsafe extern "system" fn (this: *const nsICookie, aSameSite: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute nsICookie_schemeType schemeMap; */
    pub GetSchemeMap: unsafe extern "system" fn (this: *const nsICookie, aSchemeMap: *mut u8) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICookie {

    pub const SAMESITE_NONE: i64 = 0;


    pub const SAMESITE_LAX: i64 = 1;


    pub const SAMESITE_STRICT: i64 = 2;

    /// ```text
    /// /**
    ///      * the name of the cookie
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * the cookie value
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String value;`
    #[inline]
    pub unsafe fn GetValue(&self, aValue: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetValue)(self, aValue)
    }


    /// ```text
    /// /**
    ///      * true if the cookie is a domain cookie, false otherwise
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isDomain;`
    #[inline]
    pub unsafe fn GetIsDomain(&self, aIsDomain: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDomain)(self, aIsDomain)
    }


    /// ```text
    /// /**
    ///      * the host (possibly fully qualified) of the cookie
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String host;`
    #[inline]
    pub unsafe fn GetHost(&self, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHost)(self, aHost)
    }


    /// ```text
    /// /**
    ///      * the host (possibly fully qualified) of the cookie,
    ///      * without a leading dot to represent if it is a
    ///      * domain cookie.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String rawHost;`
    #[inline]
    pub unsafe fn GetRawHost(&self, aRawHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRawHost)(self, aRawHost)
    }


    /// ```text
    /// /**
    ///      * the path pertaining to the cookie
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String path;`
    #[inline]
    pub unsafe fn GetPath(&self, aPath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPath)(self, aPath)
    }


    /// ```text
    /// /**
    ///      * true if the cookie was transmitted over ssl, false otherwise
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isSecure;`
    #[inline]
    pub unsafe fn GetIsSecure(&self, aIsSecure: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSecure)(self, aIsSecure)
    }


    /// ```text
    /// /**
    ///      * @DEPRECATED use nsICookie.expiry and nsICookie.isSession instead.
    ///      *
    ///      * expiration time in seconds since midnight (00:00:00), January 1, 1970 UTC.
    ///      * expires = 0 represents a session cookie.
    ///      * expires = 1 represents an expiration time earlier than Jan 1, 1970.
    ///      */
    /// ```
    ///

    /// `readonly attribute uint64_t expires;`
    #[inline]
    pub unsafe fn GetExpires(&self, aExpires: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetExpires)(self, aExpires)
    }


    /// ```text
    /// /**
    ///      * the actual expiry time of the cookie, in seconds
    ///      * since midnight (00:00:00), January 1, 1970 UTC.
    ///      *
    ///      * this is distinct from nsICookie::expires, which
    ///      * has different and obsolete semantics.
    ///      */
    /// ```
    ///

    /// `readonly attribute int64_t expiry;`
    #[inline]
    pub unsafe fn GetExpiry(&self, aExpiry: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetExpiry)(self, aExpiry)
    }


    /// ```text
    /// /**
    ///      * The origin attributes for this cookie
    ///      */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval originAttributes;`
    const _GetOriginAttributes: () = ();

    /// ```text
    /// /**
    ///      * true if the cookie is a session cookie.
    ///      * note that expiry time will also be honored
    ///      * for session cookies (see below); thus, whichever is
    ///      * the more restrictive of the two will take effect.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isSession;`
    #[inline]
    pub unsafe fn GetIsSession(&self, aIsSession: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSession)(self, aIsSession)
    }


    /// ```text
    /// /**
    ///      * true if the cookie is an http only cookie
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isHttpOnly;`
    #[inline]
    pub unsafe fn GetIsHttpOnly(&self, aIsHttpOnly: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsHttpOnly)(self, aIsHttpOnly)
    }


    /// ```text
    /// /**
    ///      * the creation time of the cookie, in microseconds
    ///      * since midnight (00:00:00), January 1, 1970 UTC.
    ///      */
    /// ```
    ///

    /// `readonly attribute int64_t creationTime;`
    #[inline]
    pub unsafe fn GetCreationTime(&self, aCreationTime: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCreationTime)(self, aCreationTime)
    }


    /// ```text
    /// /**
    ///      * the last time the cookie was accessed (i.e. created,
        ///      * modified, or read by the server), in microseconds
    ///      * since midnight (00:00:00), January 1, 1970 UTC.
    ///      *
    ///      * note that this time may be approximate.
    ///      */
    /// ```
    ///

    /// `readonly attribute int64_t lastAccessed;`
    #[inline]
    pub unsafe fn GetLastAccessed(&self, aLastAccessed: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastAccessed)(self, aLastAccessed)
    }


    /// ```text
    /// /**
    ///      * the SameSite attribute; this controls the cookie behavior for cross-site
    ///      * requests as per
    ///      * https://tools.ietf.org/html/draft-west-first-party-cookies-07
    ///      *
    ///      * This should be one of:
    ///      * - SAMESITE_NONE - the SameSite attribute is not present
    ///      * - SAMESITE_LAX - the SameSite attribute is present, but not strict
    ///      * - SAMESITE_STRICT - the SameSite attribute is present and strict
    ///      */
    /// ```
    ///

    /// `readonly attribute int32_t sameSite;`
    #[inline]
    pub unsafe fn GetSameSite(&self, aSameSite: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetSameSite)(self, aSameSite)
    }


    /// ```text
    /// /**
    ///      * Bitmap of schemes.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsICookie_schemeType schemeMap;`
    #[inline]
    pub unsafe fn GetSchemeMap(&self, aSchemeMap: *mut u8) -> ::nserror::nsresult {
        ((*self.vtable).GetSchemeMap)(self, aSchemeMap)
    }


}


