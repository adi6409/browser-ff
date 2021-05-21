//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURLParser.idl
//


/// `interface nsIURLParser : nsISupports`
///

/// ```text
/// /**
///  * nsIURLParser specifies the interface to an URL parser that attempts to
///  * follow the definitions of RFC 2396.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURLParser {
    vtable: *const nsIURLParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURLParser.
unsafe impl XpCom for nsIURLParser {
    const IID: nsIID = nsID(0x78c5d19f, 0xf5d2, 0x4732,
        [0x8d, 0x3d, 0xd5, 0xa7, 0xd7, 0x13, 0x3b, 0xc0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURLParser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURLParser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURLParserCoerce {
    /// Cheaply cast a value of this type from a `nsIURLParser`.
    fn coerce_from(v: &nsIURLParser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURLParserCoerce for nsIURLParser {
    #[inline]
    fn coerce_from(v: &nsIURLParser) -> &Self {
        v
    }
}

impl nsIURLParser {
    /// Cast this `nsIURLParser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURLParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURLParser {
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
impl<T: nsISupportsCoerce> nsIURLParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURLParser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURLParser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURLParserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen); */
    pub ParseURL: unsafe extern "system" fn (this: *const nsIURLParser, spec: *const libc::c_char, specLen: i32, schemePos: *mut u32, schemeLen: *mut i32, authorityPos: *mut u32, authorityLen: *mut i32, pathPos: *mut u32, pathLen: *mut i32) -> ::nserror::nsresult,

    /* void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
    pub ParseAuthority: unsafe extern "system" fn (this: *const nsIURLParser, authority: *const libc::c_char, authorityLen: i32, usernamePos: *mut u32, usernameLen: *mut i32, passwordPos: *mut u32, passwordLen: *mut i32, hostnamePos: *mut u32, hostnameLen: *mut i32, port: *mut i32) -> ::nserror::nsresult,

    /* void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen); */
    pub ParseUserInfo: unsafe extern "system" fn (this: *const nsIURLParser, userinfo: *const libc::c_char, userinfoLen: i32, usernamePos: *mut u32, usernameLen: *mut i32, passwordPos: *mut u32, passwordLen: *mut i32) -> ::nserror::nsresult,

    /* void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port); */
    pub ParseServerInfo: unsafe extern "system" fn (this: *const nsIURLParser, serverinfo: *const libc::c_char, serverinfoLen: i32, hostnamePos: *mut u32, hostnameLen: *mut i32, port: *mut i32) -> ::nserror::nsresult,

    /* void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen); */
    pub ParsePath: unsafe extern "system" fn (this: *const nsIURLParser, path: *const libc::c_char, pathLen: i32, filepathPos: *mut u32, filepathLen: *mut i32, queryPos: *mut u32, queryLen: *mut i32, refPos: *mut u32, refLen: *mut i32) -> ::nserror::nsresult,

    /* void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
    pub ParseFilePath: unsafe extern "system" fn (this: *const nsIURLParser, filepath: *const libc::c_char, filepathLen: i32, directoryPos: *mut u32, directoryLen: *mut i32, basenamePos: *mut u32, basenameLen: *mut i32, extensionPos: *mut u32, extensionLen: *mut i32) -> ::nserror::nsresult,

    /* void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen); */
    pub ParseFileName: unsafe extern "system" fn (this: *const nsIURLParser, filename: *const libc::c_char, filenameLen: i32, basenamePos: *mut u32, basenameLen: *mut i32, extensionPos: *mut u32, extensionLen: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURLParser {

    /// ```text
    /// /**
    ///      * The string to parse in the following methods may be given as a null
    ///      * terminated string, in which case the length argument should be -1.
    ///      *
    ///      * Out parameters of the following methods are all optional (ie. the caller
        ///      * may pass-in a NULL value if the corresponding results are not needed).
    ///      * Signed out parameters may hold a value of -1 if the corresponding result
    ///      * is not part of the string being parsed.
    ///      *
    ///      * The parsing routines attempt to be as forgiving as possible.
    ///      */
    /// /**
    ///      * ParseSpec breaks the URL string up into its 3 major components: a scheme,
    ///      * an authority section (hostname, etc.), and a path.
    ///      *
    ///      * spec = <scheme>://<authority><path>
    ///      */
    /// ```
    ///

    /// `void parseURL (in string spec, in long specLen, out unsigned long schemePos, out long schemeLen, out unsigned long authorityPos, out long authorityLen, out unsigned long pathPos, out long pathLen);`
    #[inline]
    pub unsafe fn ParseURL(&self, spec: *const libc::c_char, specLen: i32, schemePos: *mut u32, schemeLen: *mut i32, authorityPos: *mut u32, authorityLen: *mut i32, pathPos: *mut u32, pathLen: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParseURL)(self, spec, specLen, schemePos, schemeLen, authorityPos, authorityLen, pathPos, pathLen)
    }


    /// ```text
    /// /**
    ///      * ParseAuthority breaks the authority string up into its 4 components:
    ///      * username, password, hostname, and hostport.
    ///      *
    ///      * auth = <username>:<password>@<hostname>:<port>
    ///      */
    /// ```
    ///

    /// `void parseAuthority (in string authority, in long authorityLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen, out unsigned long hostnamePos, out long hostnameLen, out long port);`
    #[inline]
    pub unsafe fn ParseAuthority(&self, authority: *const libc::c_char, authorityLen: i32, usernamePos: *mut u32, usernameLen: *mut i32, passwordPos: *mut u32, passwordLen: *mut i32, hostnamePos: *mut u32, hostnameLen: *mut i32, port: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParseAuthority)(self, authority, authorityLen, usernamePos, usernameLen, passwordPos, passwordLen, hostnamePos, hostnameLen, port)
    }


    /// ```text
    /// /**
    ///      * userinfo = <username>:<password>
    ///      */
    /// ```
    ///

    /// `void parseUserInfo (in string userinfo, in long userinfoLen, out unsigned long usernamePos, out long usernameLen, out unsigned long passwordPos, out long passwordLen);`
    #[inline]
    pub unsafe fn ParseUserInfo(&self, userinfo: *const libc::c_char, userinfoLen: i32, usernamePos: *mut u32, usernameLen: *mut i32, passwordPos: *mut u32, passwordLen: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParseUserInfo)(self, userinfo, userinfoLen, usernamePos, usernameLen, passwordPos, passwordLen)
    }


    /// ```text
    /// /**
    ///      * serverinfo = <hostname>:<port>
    ///      */
    /// ```
    ///

    /// `void parseServerInfo (in string serverinfo, in long serverinfoLen, out unsigned long hostnamePos, out long hostnameLen, out long port);`
    #[inline]
    pub unsafe fn ParseServerInfo(&self, serverinfo: *const libc::c_char, serverinfoLen: i32, hostnamePos: *mut u32, hostnameLen: *mut i32, port: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParseServerInfo)(self, serverinfo, serverinfoLen, hostnamePos, hostnameLen, port)
    }


    /// ```text
    /// /**
    ///      * ParsePath breaks the path string up into its 3 major components: a file path,
    ///      * a query string, and a reference string.
    ///      *
    ///      * path = <filepath>?<query>#<ref>
    ///      */
    /// ```
    ///

    /// `void parsePath (in string path, in long pathLen, out unsigned long filepathPos, out long filepathLen, out unsigned long queryPos, out long queryLen, out unsigned long refPos, out long refLen);`
    #[inline]
    pub unsafe fn ParsePath(&self, path: *const libc::c_char, pathLen: i32, filepathPos: *mut u32, filepathLen: *mut i32, queryPos: *mut u32, queryLen: *mut i32, refPos: *mut u32, refLen: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParsePath)(self, path, pathLen, filepathPos, filepathLen, queryPos, queryLen, refPos, refLen)
    }


    /// ```text
    /// /**
    ///      * ParseFilePath breaks the file path string up into: the directory portion,
    ///      * file base name, and file extension.
    ///      *
    ///      * filepath = <directory><basename>.<extension>
    ///      */
    /// ```
    ///

    /// `void parseFilePath (in string filepath, in long filepathLen, out unsigned long directoryPos, out long directoryLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen);`
    #[inline]
    pub unsafe fn ParseFilePath(&self, filepath: *const libc::c_char, filepathLen: i32, directoryPos: *mut u32, directoryLen: *mut i32, basenamePos: *mut u32, basenameLen: *mut i32, extensionPos: *mut u32, extensionLen: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParseFilePath)(self, filepath, filepathLen, directoryPos, directoryLen, basenamePos, basenameLen, extensionPos, extensionLen)
    }


    /// ```text
    /// /**
    ///      * filename = <basename>.<extension>
    ///      */
    /// ```
    ///

    /// `void parseFileName (in string filename, in long filenameLen, out unsigned long basenamePos, out long basenameLen, out unsigned long extensionPos, out long extensionLen);`
    #[inline]
    pub unsafe fn ParseFileName(&self, filename: *const libc::c_char, filenameLen: i32, basenamePos: *mut u32, basenameLen: *mut i32, extensionPos: *mut u32, extensionLen: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).ParseFileName)(self, filename, filenameLen, basenamePos, basenameLen, extensionPos, extensionLen)
    }


}


