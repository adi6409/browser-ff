//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/file/nsIFileProtocolHandler.idl
//


/// `interface nsIFileProtocolHandler : nsIProtocolHandler`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileProtocolHandler {
    vtable: *const nsIFileProtocolHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileProtocolHandler.
unsafe impl XpCom for nsIFileProtocolHandler {
    const IID: nsIID = nsID(0x1fb25bd5, 0x4354, 0x4dcd,
        [0x8d, 0x97, 0x62, 0x1b, 0x7b, 0x3e, 0xd2, 0xe4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileProtocolHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileProtocolHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileProtocolHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIFileProtocolHandler`.
    fn coerce_from(v: &nsIFileProtocolHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileProtocolHandlerCoerce for nsIFileProtocolHandler {
    #[inline]
    fn coerce_from(v: &nsIFileProtocolHandler) -> &Self {
        v
    }
}

impl nsIFileProtocolHandler {
    /// Cast this `nsIFileProtocolHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileProtocolHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileProtocolHandler {
    type Target = nsIProtocolHandler;
    #[inline]
    fn deref(&self) -> &nsIProtocolHandler {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIProtocolHandlerCoerce> nsIFileProtocolHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileProtocolHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileProtocolHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileProtocolHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIProtocolHandlerVTable,

    /* nsIURI newFileURI (in nsIFile aFile); */
    pub NewFileURI: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, aFile: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* nsIURIMutator newFileURIMutator (in nsIFile file); */
    pub NewFileURIMutator: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult,

    /* AUTF8String getURLSpecFromFile (in nsIFile file); */
    pub GetURLSpecFromFile: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getURLSpecFromActualFile (in nsIFile file); */
    pub GetURLSpecFromActualFile: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getURLSpecFromDir (in nsIFile file); */
    pub GetURLSpecFromDir: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIFile getFileFromURLSpec (in AUTF8String url); */
    pub GetFileFromURLSpec: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, url: *const ::nsstring::nsACString, _retval: *mut*const nsIFile) -> ::nserror::nsresult,

    /* nsIURI readURLFile (in nsIFile file); */
    pub ReadURLFile: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult,

    /* nsIURI readShellLink (in nsIFile file); */
    pub ReadShellLink: unsafe extern "system" fn (this: *const nsIFileProtocolHandler, file: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileProtocolHandler {

    /// ```text
    /// /**
    ///      * This method constructs a new file URI
    ///      *
    ///      * @param aFile nsIFile
    ///      * @return reference to a new nsIURI object
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
    ///      * This method constructs a new file URI, and returns a URI mutator
    ///      * that has not yet been finalized, allowing the URI to be changed without
    ///      * being cloned.
    ///      *
    ///      * @param aFile nsIFile
    ///      * @return reference to a new nsIURIMutator object
    ///      */
    /// ```
    ///

    /// `nsIURIMutator newFileURIMutator (in nsIFile file);`
    #[inline]
    pub unsafe fn NewFileURIMutator(&self, file: *const nsIFile, _retval: *mut*const nsIURIMutator) -> ::nserror::nsresult {
        ((*self.vtable).NewFileURIMutator)(self, file, _retval)
    }


    /// ```text
    /// /**
    ///      * Converts the nsIFile to the corresponding URL string.  NOTE: under
    ///      * some platforms this is a lossy conversion (e.g., Mac Carbon build).
    ///      * If the nsIFile is a local file, then the result will be a file://
    ///      * URL string.
    ///      *
    ///      * The resulting string may contain URL-escaped characters.
    ///      * NOTE: Callers should use getURLSpecFromActualFile or
    ///      * getURLSpecFromDirFile if possible, for performance reasons.
    ///      */
    /// ```
    ///

    /// `AUTF8String getURLSpecFromFile (in nsIFile file);`
    #[inline]
    pub unsafe fn GetURLSpecFromFile(&self, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetURLSpecFromFile)(self, file, _retval)
    }


    /// ```text
    /// /**
    ///      * Converts the nsIFile to the corresponding URL string. Should
    ///      * only be called on files which are not directories. Otherwise
    ///      * identical to getURLSpecFromFile, but is usually more efficient.
    ///      * WARNING: This restriction may not be enforced at runtime!
    ///      */
    /// ```
    ///

    /// `AUTF8String getURLSpecFromActualFile (in nsIFile file);`
    #[inline]
    pub unsafe fn GetURLSpecFromActualFile(&self, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetURLSpecFromActualFile)(self, file, _retval)
    }


    /// ```text
    /// /**
    ///      * Converts the nsIFile to the corresponding URL string. Should
    ///      * only be called on files which are directories. Otherwise
    ///      * identical to getURLSpecFromFile, but is usually more efficient.
    ///      * WARNING: This restriction may not be enforced at runtime!
    ///      */
    /// ```
    ///

    /// `AUTF8String getURLSpecFromDir (in nsIFile file);`
    #[inline]
    pub unsafe fn GetURLSpecFromDir(&self, file: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetURLSpecFromDir)(self, file, _retval)
    }


    /// ```text
    /// /**
    ///      * Converts the URL string into the corresponding nsIFile if possible.
    ///      * A local file will be created if the URL string begins with file://.
    ///      */
    /// ```
    ///

    /// `nsIFile getFileFromURLSpec (in AUTF8String url);`
    #[inline]
    pub unsafe fn GetFileFromURLSpec(&self, url: *const ::nsstring::nsACString, _retval: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFileFromURLSpec)(self, url, _retval)
    }


    /// ```text
    /// /**
    ///      * Takes a local file and tries to interpret it as an internet shortcut
    ///      * (e.g. .url files on windows).
    ///      * @param file The local file to read
    ///      * @return The URI the file refers to
    ///      *
    ///      * @throw NS_ERROR_NOT_AVAILABLE if the OS does not support such files.
    ///      * @throw NS_ERROR_NOT_AVAILABLE if this file is not an internet shortcut.
    ///      */
    /// ```
    ///

    /// `nsIURI readURLFile (in nsIFile file);`
    #[inline]
    pub unsafe fn ReadURLFile(&self, file: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).ReadURLFile)(self, file, _retval)
    }


    /// ```text
    /// /**
    ///      * Takes a local file and tries to interpret it as a shell link file
    ///      * (.lnk files on Windows)
    ///      * @param file The local file to read
    ///      * @return The URI the file refers to
    ///      *
    ///      * @throw NS_ERROR_NOT_AVAILABLE if the OS does not support such files.
    ///      * @throw NS_ERROR_NOT_AVAILABLE if this file is not a shell link.
    ///      */
    /// ```
    ///

    /// `nsIURI readShellLink (in nsIFile file);`
    #[inline]
    pub unsafe fn ReadShellLink(&self, file: *const nsIFile, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).ReadShellLink)(self, file, _retval)
    }


}


