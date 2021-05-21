//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURI.idl
//


/// `interface nsIURI : nsISupports`
///

/// ```text
/// /**
///  * nsIURI - interface for an uniform resource identifier w/ i18n support.
///  *
///  * AUTF8String attributes may contain unescaped UTF-8 characters.
///  * Consumers should be careful to escape the UTF-8 strings as necessary, but
///  * should always try to "display" the UTF-8 version as provided by this
///  * interface.
///  *
///  * AUTF8String attributes may also contain escaped characters.
///  *
///  * Unescaping URI segments is unadvised unless there is intimate
///  * knowledge of the underlying charset or there is no plan to display (or
    ///  * otherwise enforce a charset on) the resulting URI substring.
///  *
///  * The correct way to create an nsIURI from a string is via
///  * nsIIOService.newURI.
///  *
///  * NOTE: nsBinaryInputStream::ReadObject contains a hackaround to intercept the
///  * old (pre-gecko6) nsIURI IID and swap in the current IID instead, in order
///  * for sessionstore to work after an upgrade.  If this IID is revved further,
///  * we will need to add additional checks there for all intermediate IIDs, until
///  * ContentPrincipal is fixed to serialize its URIs as nsISupports (bug 662693).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURI {
    vtable: *const nsIURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURI.
unsafe impl XpCom for nsIURI {
    const IID: nsIID = nsID(0x92073a54, 0x6d78, 0x4f30,
        [0x91, 0x3a, 0xb8, 0x71, 0x81, 0x32, 0x08, 0xc6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURICoerce {
    /// Cheaply cast a value of this type from a `nsIURI`.
    fn coerce_from(v: &nsIURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURICoerce for nsIURI {
    #[inline]
    fn coerce_from(v: &nsIURI) -> &Self {
        v
    }
}

impl nsIURI {
    /// Cast this `nsIURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURI {
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
impl<T: nsISupportsCoerce> nsIURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String spec; */
    pub GetSpec: unsafe extern "system" fn (this: *const nsIURI, aSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String prePath; */
    pub GetPrePath: unsafe extern "system" fn (this: *const nsIURI, aPrePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString scheme; */
    pub GetScheme: unsafe extern "system" fn (this: *const nsIURI, aScheme: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String userPass; */
    pub GetUserPass: unsafe extern "system" fn (this: *const nsIURI, aUserPass: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String username; */
    pub GetUsername: unsafe extern "system" fn (this: *const nsIURI, aUsername: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String password; */
    pub GetPassword: unsafe extern "system" fn (this: *const nsIURI, aPassword: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String hostPort; */
    pub GetHostPort: unsafe extern "system" fn (this: *const nsIURI, aHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String host; */
    pub GetHost: unsafe extern "system" fn (this: *const nsIURI, aHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long port; */
    pub GetPort: unsafe extern "system" fn (this: *const nsIURI, aPort: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String pathQueryRef; */
    pub GetPathQueryRef: unsafe extern "system" fn (this: *const nsIURI, aPathQueryRef: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean equals (in nsIURI other); */
    pub Equals: unsafe extern "system" fn (this: *const nsIURI, other: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* [infallible] boolean schemeIs (in string scheme); */
    pub SchemeIs: unsafe extern "system" fn (this: *const nsIURI, scheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* AUTF8String resolve (in AUTF8String relativePath); */
    pub Resolve: unsafe extern "system" fn (this: *const nsIURI, relativePath: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString asciiSpec; */
    pub GetAsciiSpec: unsafe extern "system" fn (this: *const nsIURI, aAsciiSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString asciiHostPort; */
    pub GetAsciiHostPort: unsafe extern "system" fn (this: *const nsIURI, aAsciiHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString asciiHost; */
    pub GetAsciiHost: unsafe extern "system" fn (this: *const nsIURI, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String ref; */
    pub GetRef: unsafe extern "system" fn (this: *const nsIURI, aRef: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean equalsExceptRef (in nsIURI other); */
    pub EqualsExceptRef: unsafe extern "system" fn (this: *const nsIURI, other: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String specIgnoringRef; */
    pub GetSpecIgnoringRef: unsafe extern "system" fn (this: *const nsIURI, aSpecIgnoringRef: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean hasRef; */
    pub GetHasRef: unsafe extern "system" fn (this: *const nsIURI, aHasRef: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String filePath; */
    pub GetFilePath: unsafe extern "system" fn (this: *const nsIURI, aFilePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String query; */
    pub GetQuery: unsafe extern "system" fn (this: *const nsIURI, aQuery: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String displayHost; */
    pub GetDisplayHost: unsafe extern "system" fn (this: *const nsIURI, aDisplayHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String displayHostPort; */
    pub GetDisplayHostPort: unsafe extern "system" fn (this: *const nsIURI, aDisplayHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String displaySpec; */
    pub GetDisplaySpec: unsafe extern "system" fn (this: *const nsIURI, aDisplaySpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String displayPrePath; */
    pub GetDisplayPrePath: unsafe extern "system" fn (this: *const nsIURI, aDisplayPrePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIURIMutator mutate (); */
    pub Mutate: unsafe extern "system" fn (this: *const nsIURI, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void serialize (in URIParams aParams); */
    /// Unable to generate binding because `native type mozilla::ipc::URIParams unsupported`
    pub Serialize: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURI {

    /// ```text
    /// /************************************************************************
    ///      * The URI is broken down into the following principal components:
    ///      */
    /// /**
    ///      * Returns a string representation of the URI.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String spec;`
    #[inline]
    pub unsafe fn GetSpec(&self, aSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSpec)(self, aSpec)
    }


    /// ```text
    /// /**
    ///      * The prePath (eg. scheme://user:password@host:port) returns the string
    ///      * before the path.  This is useful for authentication or managing sessions.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String prePath;`
    #[inline]
    pub unsafe fn GetPrePath(&self, aPrePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPrePath)(self, aPrePath)
    }


    /// ```text
    /// /**
    ///      * The Scheme is the protocol to which this URI refers.  The scheme is
    ///      * restricted to the US-ASCII charset per RFC3986.
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
    ///      * The username:password (or username only if value doesn't contain a ':')
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String userPass;`
    #[inline]
    pub unsafe fn GetUserPass(&self, aUserPass: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUserPass)(self, aUserPass)
    }


    /// ```text
    /// /**
    ///      * The optional username and password, assuming the preHost consists of
    ///      * username:password.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String username;`
    #[inline]
    pub unsafe fn GetUsername(&self, aUsername: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUsername)(self, aUsername)
    }



    /// `readonly attribute AUTF8String password;`
    #[inline]
    pub unsafe fn GetPassword(&self, aPassword: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPassword)(self, aPassword)
    }


    /// ```text
    /// /**
    ///      * The host:port (or simply the host, if port == -1).
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String hostPort;`
    #[inline]
    pub unsafe fn GetHostPort(&self, aHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHostPort)(self, aHostPort)
    }


    /// ```text
    /// /**
    ///      * The host is the internet domain name to which this URI refers.  It could
    ///      * be an IPv4 (or IPv6) address literal. Otherwise it is an ASCII or punycode
    ///      * encoded string.
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
    ///      * A port value of -1 corresponds to the protocol's default port (eg. -1
        ///      * implies port 80 for http URIs).
    ///      */
    /// ```
    ///

    /// `readonly attribute long port;`
    #[inline]
    pub unsafe fn GetPort(&self, aPort: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPort)(self, aPort)
    }


    /// ```text
    /// /**
    ///      * The path, typically including at least a leading '/' (but may also be
        ///      * empty, depending on the protocol).
    ///      *
    ///      * Some characters may be escaped.
    ///      *
    ///      * This attribute contains query and ref parts for historical reasons.
    ///      * Use the 'filePath' attribute if you do not want those parts included.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String pathQueryRef;`
    #[inline]
    pub unsafe fn GetPathQueryRef(&self, aPathQueryRef: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPathQueryRef)(self, aPathQueryRef)
    }


    /// ```text
    /// /************************************************************************
    ///      * An URI supports the following methods:
    ///      */
    /// /**
    ///      * URI equivalence test (not a strict string comparison).
    ///      *
    ///      * eg. http://foo.com:80/ == http://foo.com/
    ///      */
    /// ```
    ///

    /// `boolean equals (in nsIURI other);`
    #[inline]
    pub unsafe fn Equals(&self, other: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///      * An optimization to do scheme checks without requiring the users of nsIURI
    ///      * to GetScheme, thereby saving extra allocating and freeing. Returns true if
    ///      * the schemes match (case ignored).
    ///      */
    /// ```
    ///

    /// `[infallible] boolean schemeIs (in string scheme);`
    #[inline]
    pub unsafe fn SchemeIs(&self, scheme: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SchemeIs)(self, scheme, _retval)
    }


    /// ```text
    /// /**
    ///      * This method resolves a relative string into an absolute URI string,
    ///      * using this URI as the base.
    ///      *
    ///      * NOTE: some implementations may have no concept of a relative URI.
    ///      */
    /// ```
    ///

    /// `AUTF8String resolve (in AUTF8String relativePath);`
    #[inline]
    pub unsafe fn Resolve(&self, relativePath: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Resolve)(self, relativePath, _retval)
    }


    /// ```text
    /// /************************************************************************
    ///      * Additional attributes:
    ///      */
    /// /**
    ///      * The URI spec with an ASCII compatible encoding.  Host portion follows
    ///      * the IDNA draft spec.  Other parts are URL-escaped per the rules of
    ///      * RFC2396.  The result is strictly ASCII.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString asciiSpec;`
    #[inline]
    pub unsafe fn GetAsciiSpec(&self, aAsciiSpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiSpec)(self, aAsciiSpec)
    }


    /// ```text
    /// /**
    ///      * The host:port (or simply the host, if port == -1), with an ASCII compatible
    ///      * encoding.  Host portion follows the IDNA draft spec.  The result is strictly
    ///      * ASCII.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString asciiHostPort;`
    #[inline]
    pub unsafe fn GetAsciiHostPort(&self, aAsciiHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiHostPort)(self, aAsciiHostPort)
    }


    /// ```text
    /// /**
    ///      * The URI host with an ASCII compatible encoding.  Follows the IDNA
    ///      * draft spec for converting internationalized domain names (UTF-8) to
    ///      * ASCII for compatibility with existing internet infrasture.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString asciiHost;`
    #[inline]
    pub unsafe fn GetAsciiHost(&self, aAsciiHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAsciiHost)(self, aAsciiHost)
    }


    /// ```text
    /// /************************************************************************
    ///      * Additional attribute & methods added for .ref support:
    ///      */
    /// /**
    ///      * Returns the reference portion (the part after the "#") of the URI.
    ///      * If there isn't one, an empty string is returned.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String ref;`
    #[inline]
    pub unsafe fn GetRef(&self, aRef: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRef)(self, aRef)
    }


    /// ```text
    /// /**
    ///      * URI equivalence test (not a strict string comparison), ignoring
    ///      * the value of the .ref member.
    ///      *
    ///      * eg. http://foo.com/# == http://foo.com/
    ///      *     http://foo.com/#aaa == http://foo.com/#bbb
    ///      */
    /// ```
    ///

    /// `boolean equalsExceptRef (in nsIURI other);`
    #[inline]
    pub unsafe fn EqualsExceptRef(&self, other: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).EqualsExceptRef)(self, other, _retval)
    }


    /// ```text
    /// /**
    ///      * returns a string for the current URI with the ref element cleared.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String specIgnoringRef;`
    #[inline]
    pub unsafe fn GetSpecIgnoringRef(&self, aSpecIgnoringRef: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetSpecIgnoringRef)(self, aSpecIgnoringRef)
    }


    /// ```text
    /// /**
    ///      * Returns if there is a reference portion (the part after the "#") of the URI.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean hasRef;`
    #[inline]
    pub unsafe fn GetHasRef(&self, aHasRef: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasRef)(self, aHasRef)
    }


    /// ```text
    /// /************************************************************************
    ///      * Additional attributes added for .query support:
    ///      */
    /// /**
    ///      * Returns a path including the directory and file portions of a
    ///      * URL.  For example, the filePath of "http://host/foo/bar.html#baz"
    ///      * is "/foo/bar.html".
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String filePath;`
    #[inline]
    pub unsafe fn GetFilePath(&self, aFilePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFilePath)(self, aFilePath)
    }


    /// ```text
    /// /**
    ///      * Returns the query portion (the part after the "?") of the URL.
    ///      * If there isn't one, an empty string is returned.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String query;`
    #[inline]
    pub unsafe fn GetQuery(&self, aQuery: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetQuery)(self, aQuery)
    }


    /// ```text
    /// /**
    ///      * If the URI has a punycode encoded hostname, this will hold the UTF8
    ///      * representation of that hostname (if that representation doesn't contain
        ///      * blacklisted characters, and the network.IDN_show_punycode pref is false)
    ///      * Otherwise, if the hostname is ASCII, it will return the same as .asciiHost
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String displayHost;`
    #[inline]
    pub unsafe fn GetDisplayHost(&self, aDisplayHost: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayHost)(self, aDisplayHost)
    }


    /// ```text
    /// /**
    ///      * The displayHost:port (or simply the displayHost, if port == -1).
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String displayHostPort;`
    #[inline]
    pub unsafe fn GetDisplayHostPort(&self, aDisplayHostPort: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayHostPort)(self, aDisplayHostPort)
    }


    /// ```text
    /// /**
    ///      * Returns the same as calling .spec, only with a UTF8 encoded hostname
    ///      * (if that hostname doesn't contain blacklisted characters, and
        ///      * the network.IDN_show_punycode pref is false)
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String displaySpec;`
    #[inline]
    pub unsafe fn GetDisplaySpec(&self, aDisplaySpec: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplaySpec)(self, aDisplaySpec)
    }


    /// ```text
    /// /**
    ///      * Returns the same as calling .prePath, only with a UTF8 encoded hostname
    ///      * (if that hostname doesn't contain blacklisted characters, and
        ///      * the network.IDN_show_punycode pref is false)
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String displayPrePath;`
    #[inline]
    pub unsafe fn GetDisplayPrePath(&self, aDisplayPrePath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayPrePath)(self, aDisplayPrePath)
    }


    /// ```text
    /// /**
    ///      * Returns an nsIURIMutator that can be used to make changes to the URI.
    ///      * After performing the setter operations on the mutator, one may call
    ///      * mutator.finalize() to get a new immutable URI with the desired
    ///      * properties.
    ///      */
    /// ```
    ///

    /// `nsIURIMutator mutate ();`
    #[inline]
    pub unsafe fn Mutate(&self, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).Mutate)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Serializes a URI object to a URIParams data structure in order for being
    ///      * passed over IPC.  For deserialization, see nsIURIMutator.
    ///      */
    /// ```
    ///

    /// `[noscript,notxpcom] void serialize (in URIParams aParams);`
    const _Serialize: () = ();

}


