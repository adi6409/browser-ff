//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURL.idl
//


/// `interface nsIURL : nsIURI`
///

/// ```text
/// /**
///  * The nsIURL interface provides convenience methods that further
///  * break down the path portion of nsIURI:
///  *
///  * http://host/directory/fileBaseName.fileExtension?query
///  * http://host/directory/fileBaseName.fileExtension#ref
///  *            \          \                       /
///  *             \          -----------------------
///  *              \                   |          /
///  *               \               fileName     /
///  *                ----------------------------
///  *                            |
///  *                        filePath
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURL {
    vtable: *const nsIURLVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURL.
unsafe impl XpCom for nsIURL {
    const IID: nsIID = nsID(0x86adcd89, 0x0b70, 0x47a2,
        [0xb0, 0xfe, 0x5b, 0xb2, 0xc5, 0xf3, 0x7e, 0x31]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURL {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURL.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURLCoerce {
    /// Cheaply cast a value of this type from a `nsIURL`.
    fn coerce_from(v: &nsIURL) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURLCoerce for nsIURL {
    #[inline]
    fn coerce_from(v: &nsIURL) -> &Self {
        v
    }
}

impl nsIURL {
    /// Cast this `nsIURL` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURLCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURL {
    type Target = nsIURI;
    #[inline]
    fn deref(&self) -> &nsIURI {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIURICoerce> nsIURLCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURL) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURL
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURLVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIURIVTable,

    /* readonly attribute AUTF8String directory; */
    pub GetDirectory: unsafe extern "system" fn (this: *const nsIURL, aDirectory: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String fileName; */
    pub GetFileName: unsafe extern "system" fn (this: *const nsIURL, aFileName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String fileBaseName; */
    pub GetFileBaseName: unsafe extern "system" fn (this: *const nsIURL, aFileBaseName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String fileExtension; */
    pub GetFileExtension: unsafe extern "system" fn (this: *const nsIURL, aFileExtension: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare); */
    pub GetCommonBaseSpec: unsafe extern "system" fn (this: *const nsIURL, aURIToCompare: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getRelativeSpec (in nsIURI aURIToCompare); */
    pub GetRelativeSpec: unsafe extern "system" fn (this: *const nsIURL, aURIToCompare: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURL {

    /// ```text
    /// /*************************************************************************
    ///      * The URL path is broken down into the following principal components:
    ///      *
    ///      * attribute AUTF8String filePath;
    ///      * attribute AUTF8String query;
    ///      *
    ///      * These are inherited from nsIURI.
    ///      */
    /// /*************************************************************************
    ///      * The URL filepath is broken down into the following sub-components:
    ///      */
    /// /**
    ///      * Returns the directory portion of a URL.  If the URL denotes a path to a
    ///      * directory and not a file, e.g. http://host/foo/bar/, then the Directory
    ///      * attribute accesses the complete /foo/bar/ portion, and the FileName is
    ///      * the empty string. If the trailing slash is omitted, then the Directory
    ///      * is /foo/ and the file is bar (i.e. this is a syntactic, not a semantic
        ///      * breakdown of the Path).  And hence don't rely on this for something to
    ///      * be a definitely be a file. But you can get just the leading directory
    ///      * portion for sure.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String directory;`
    #[inline]
    pub unsafe fn GetDirectory(&self, aDirectory: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDirectory)(self, aDirectory)
    }


    /// ```text
    /// /**
    ///      * Returns the file name portion of a URL.  If the URL denotes a path to a
    ///      * directory and not a file, e.g. http://host/foo/bar/, then the Directory
    ///      * attribute accesses the complete /foo/bar/ portion, and the FileName is
    ///      * the empty string. Note that this is purely based on searching for the
    ///      * last trailing slash. And hence don't rely on this to be a definite file.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String fileName;`
    #[inline]
    pub unsafe fn GetFileName(&self, aFileName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFileName)(self, aFileName)
    }


    /// ```text
    /// /*************************************************************************
    ///      * The URL filename is broken down even further:
    ///      */
    /// /**
    ///      * Returns the file basename portion of a filename in a url.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String fileBaseName;`
    #[inline]
    pub unsafe fn GetFileBaseName(&self, aFileBaseName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFileBaseName)(self, aFileBaseName)
    }


    /// ```text
    /// /**
    ///      * Returns the file extension portion of a filename in a url.  If a file
    ///      * extension does not exist, the empty string is returned.
    ///      *
    ///      * Some characters may be escaped.
    ///      */
    /// ```
    ///

    /// `readonly attribute AUTF8String fileExtension;`
    #[inline]
    pub unsafe fn GetFileExtension(&self, aFileExtension: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFileExtension)(self, aFileExtension)
    }


    /// ```text
    /// /**
    ///      * This method takes a uri and compares the two.  The common uri portion
    ///      * is returned as a string.  The minimum common uri portion is the
    ///      * protocol, and any of these if present:  login, password, host and port
    ///      * If no commonality is found, "" is returned.  If they are identical, the
    ///      * whole path with file/ref/etc. is returned.  For file uris, it is
    ///      * expected that the common spec would be at least "file:///" since '/' is
    ///      * a shared common root.
    ///      *
    ///      * Examples:
    ///      *    this.spec               aURIToCompare.spec        result
    ///      * 1) http://mozilla.org/     http://www.mozilla.org/   ""
///      * 2) http://foo.com/bar/     ftp://foo.com/bar/        ""
///      * 3) http://foo.com:8080/    http://foo.com/bar/       ""
///      * 4) ftp://user@foo.com/     ftp://user:pw@foo.com/    ""
///      * 5) ftp://foo.com/bar/      ftp://foo.com/bar         ftp://foo.com/
///      * 6) ftp://foo.com/bar/      ftp://foo.com/bar/b.html  ftp://foo.com/bar/
///      * 7) http://foo.com/a.htm#i  http://foo.com/b.htm      http://foo.com/
///      * 8) ftp://foo.com/c.htm#i   ftp://foo.com/c.htm       ftp://foo.com/c.htm
///      * 9) file:///a/b/c.html      file:///d/e/c.html        file:///
///      */
/// ```
///

/// `AUTF8String getCommonBaseSpec (in nsIURI aURIToCompare);`
#[inline]
pub unsafe fn GetCommonBaseSpec(&self, aURIToCompare: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).GetCommonBaseSpec)(self, aURIToCompare, _retval)
}


/// ```text
/// /**
///      * This method tries to create a string which specifies the location of the
///      * argument relative to |this|.  If the argument and |this| are equal, the
///      * method returns "".  If any of the URIs' scheme, host, userpass, or port
///      * don't match, the method returns the full spec of the argument.
///      *
///      * Examples:
///      *    this.spec               aURIToCompare.spec        result
///      * 1) http://mozilla.org/     http://www.mozilla.org/   http://www.mozilla.org/
///      * 2) http://mozilla.org/     http://www.mozilla.org    http://www.mozilla.org/
///      * 3) http://foo.com/bar/     http://foo.com:80/bar/    ""
///      * 4) http://foo.com/         http://foo.com/a.htm#b    a.html#b
///      * 5) http://foo.com/a/b/     http://foo.com/c          ../../c
///      */
/// ```
///

/// `AUTF8String getRelativeSpec (in nsIURI aURIToCompare);`
#[inline]
pub unsafe fn GetRelativeSpec(&self, aURIToCompare: *const nsIURI, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
((*self.vtable).GetRelativeSpec)(self, aURIToCompare, _retval)
}


}


/// `interface nsIURLMutator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURLMutator {
vtable: *const nsIURLMutatorVTable,

/// This field is a phantomdata to ensure that the VTable type and any
/// struct containing it is not safe to send across threads, as XPCOM is
/// generally not threadsafe.
///
/// XPCOM interfaces in general are not safe to send across threads.
__nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURLMutator.
unsafe impl XpCom for nsIURLMutator {
const IID: nsIID = nsID(0x25072eb8, 0xf1e6, 0x482f,
[0x9c, 0xa9, 0xed, 0xdd, 0x3d, 0x65, 0x16, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURLMutator {
#[inline]
unsafe fn addref(&self) {
self.AddRef();
}
#[inline]
unsafe fn release(&self) {
self.Release();
}
}

// This trait is implemented on all types which can be coerced to from nsIURLMutator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURLMutatorCoerce {
/// Cheaply cast a value of this type from a `nsIURLMutator`.
fn coerce_from(v: &nsIURLMutator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURLMutatorCoerce for nsIURLMutator {
#[inline]
fn coerce_from(v: &nsIURLMutator) -> &Self {
v
}
}

impl nsIURLMutator {
/// Cast this `nsIURLMutator` to one of its base interfaces.
#[inline]
pub fn coerce<T: nsIURLMutatorCoerce>(&self) -> &T {
T::coerce_from(self)
}
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURLMutator {
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
impl<T: nsISupportsCoerce> nsIURLMutatorCoerce for T {
#[inline]
fn coerce_from(v: &nsIURLMutator) -> &Self {
T::coerce_from(v)
}
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURLMutator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURLMutatorVTable {
/// We need to include the members from the base interface's vtable at the start
/// of the VTable definition.
pub __base: nsISupportsVTable,

/* [must_use] nsIURIMutator setFileName (in AUTF8String aFileName); */
pub SetFileName: unsafe extern "system" fn (this: *const nsIURLMutator, aFileName: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

/* [must_use] nsIURIMutator setFileBaseName (in AUTF8String aFileBaseName); */
pub SetFileBaseName: unsafe extern "system" fn (this: *const nsIURLMutator, aFileBaseName: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

/* [must_use] nsIURIMutator setFileExtension (in AUTF8String aFileExtension); */
pub SetFileExtension: unsafe extern "system" fn (this: *const nsIURLMutator, aFileExtension: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURLMutator {


/// `[must_use] nsIURIMutator setFileName (in AUTF8String aFileName);`
#[inline]
pub unsafe fn SetFileName(&self, aFileName: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
((*self.vtable).SetFileName)(self, aFileName, _retval)
}



/// `[must_use] nsIURIMutator setFileBaseName (in AUTF8String aFileBaseName);`
#[inline]
pub unsafe fn SetFileBaseName(&self, aFileBaseName: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
((*self.vtable).SetFileBaseName)(self, aFileBaseName, _retval)
}



/// `[must_use] nsIURIMutator setFileExtension (in AUTF8String aFileExtension);`
#[inline]
pub unsafe fn SetFileExtension(&self, aFileExtension: *const ::nsstring::nsACString, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
((*self.vtable).SetFileExtension)(self, aFileExtension, _retval)
}


}


