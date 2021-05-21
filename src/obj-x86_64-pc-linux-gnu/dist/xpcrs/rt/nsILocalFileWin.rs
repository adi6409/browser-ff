//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsILocalFileWin.idl
//


/// `interface nsILocalFileWin : nsIFile`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILocalFileWin {
    vtable: *const nsILocalFileWinVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILocalFileWin.
unsafe impl XpCom for nsILocalFileWin {
    const IID: nsIID = nsID(0xe7a3a954, 0x384b, 0x4aeb,
        [0xa5, 0xf7, 0x55, 0x62, 0x6b, 0x0d, 0xe9, 0xbe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILocalFileWin {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILocalFileWin.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILocalFileWinCoerce {
    /// Cheaply cast a value of this type from a `nsILocalFileWin`.
    fn coerce_from(v: &nsILocalFileWin) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILocalFileWinCoerce for nsILocalFileWin {
    #[inline]
    fn coerce_from(v: &nsILocalFileWin) -> &Self {
        v
    }
}

impl nsILocalFileWin {
    /// Cast this `nsILocalFileWin` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILocalFileWinCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILocalFileWin {
    type Target = nsIFile;
    #[inline]
    fn deref(&self) -> &nsIFile {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIFileCoerce> nsILocalFileWinCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalFileWin) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILocalFileWin
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILocalFileWinVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIFileVTable,

    /* void initWithCommandLine (in AString aCommandLine); */
    pub InitWithCommandLine: unsafe extern "system" fn (this: *const nsILocalFileWin, aCommandLine: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getVersionInfoField (in string aField); */
    pub GetVersionInfoField: unsafe extern "system" fn (this: *const nsILocalFileWin, aField: *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString canonicalPath; */
    pub GetCanonicalPath: unsafe extern "system" fn (this: *const nsILocalFileWin, aCanonicalPath: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] readonly attribute ACString nativeCanonicalPath; */
    pub GetNativeCanonicalPath: unsafe extern "system" fn (this: *const nsILocalFileWin, aNativeCanonicalPath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute unsigned long fileAttributesWin; */
    pub GetFileAttributesWin: unsafe extern "system" fn (this: *const nsILocalFileWin, aFileAttributesWin: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long fileAttributesWin; */
    pub SetFileAttributesWin: unsafe extern "system" fn (this: *const nsILocalFileWin, aFileAttributesWin: u32) -> ::nserror::nsresult,

    /* attribute boolean useDOSDevicePathSyntax; */
    pub GetUseDOSDevicePathSyntax: unsafe extern "system" fn (this: *const nsILocalFileWin, aUseDOSDevicePathSyntax: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean useDOSDevicePathSyntax; */
    pub SetUseDOSDevicePathSyntax: unsafe extern "system" fn (this: *const nsILocalFileWin, aUseDOSDevicePathSyntax: bool) -> ::nserror::nsresult,

    /* [noscript] PRFileDescStar openNSPRFileDescShareDelete (in long flags, in long mode); */
    /// Unable to generate binding because `native type PRFileDesc unsupported`
    pub OpenNSPRFileDescShareDelete: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILocalFileWin {
    /// ```text
    /// /**
    ///      * Windows specific file attributes.
    ///      */
    /// ```
    ///

    pub const WFA_SEARCH_INDEXED: i64 = 1;


    pub const WFA_READONLY: i64 = 2;


    pub const WFA_READWRITE: i64 = 4;

    /// ```text
    /// /**
    ///      *  initWithCommandLine
    ///      *
    ///      *  Initialize this object based on the main app path of a commandline
    ///      *  handler.
    ///      *
    ///      *   @param aCommandLine
    ///      *       the commandline to parse an app path out of.
    ///      */
    /// ```
    ///

    /// `void initWithCommandLine (in AString aCommandLine);`
    #[inline]
    pub unsafe fn InitWithCommandLine(&self, aCommandLine: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).InitWithCommandLine)(self, aCommandLine)
    }


    /// ```text
    /// /**
    ///     * getVersionInfoValue
    ///     *
    ///     * Retrieve a metadata field from the file's VERSIONINFO block.
    ///     * Throws NS_ERROR_FAILURE if no value is found, or the value is empty.
    ///     *
    ///     * @param   aField         The field to look up.
    ///     *
    ///     */
    /// ```
    ///

    /// `AString getVersionInfoField (in string aField);`
    #[inline]
    pub unsafe fn GetVersionInfoField(&self, aField: *const libc::c_char, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetVersionInfoField)(self, aField, _retval)
    }


    /// ```text
    /// /**
    ///      * The canonical path of the file, which avoids short/long
    ///      * pathname inconsistencies. The nsIFile persistent
    ///      * descriptor is not guaranteed to be canonicalized (it may
        ///      * persist either the long or the short path name). The format of
    ///      * the canonical path will vary with the underlying file system:
    ///      * it will typically be the short pathname on filesystems that
    ///      * support both short and long path forms.
    ///      */
    /// ```
    ///

    /// `readonly attribute AString canonicalPath;`
    #[inline]
    pub unsafe fn GetCanonicalPath(&self, aCanonicalPath: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCanonicalPath)(self, aCanonicalPath)
    }



    /// `[noscript] readonly attribute ACString nativeCanonicalPath;`
    #[inline]
    pub unsafe fn GetNativeCanonicalPath(&self, aNativeCanonicalPath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNativeCanonicalPath)(self, aNativeCanonicalPath)
    }


    /// ```text
    /// /**
    ///      * fileAttributesWin
    ///      *
    ///      * Set or get windows specific file attributes.
    ///      *
    ///      * Throws NS_ERROR_FILE_INVALID_PATH for an invalid file.
    ///      * Throws NS_ERROR_FAILURE if the set or get fails.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long fileAttributesWin;`
    #[inline]
    pub unsafe fn GetFileAttributesWin(&self, aFileAttributesWin: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetFileAttributesWin)(self, aFileAttributesWin)
    }


    /// ```text
    /// /**
    ///      * fileAttributesWin
    ///      *
    ///      * Set or get windows specific file attributes.
    ///      *
    ///      * Throws NS_ERROR_FILE_INVALID_PATH for an invalid file.
    ///      * Throws NS_ERROR_FAILURE if the set or get fails.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long fileAttributesWin;`
    #[inline]
    pub unsafe fn SetFileAttributesWin(&self, aFileAttributesWin: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetFileAttributesWin)(self, aFileAttributesWin)
    }


    /// ```text
    /// /**
    ///      * Setting this to true will prepend the prefix "\\?\" to all parsed file
    ///      * paths which match ^[A-Za-z]:\\.* (regex) syntax.
    ///      *
    ///      * There are two known issues (and potentially more) which can be resolved
    ///      * by the prefix:
    ///      * - In the Windows API, the maximum length for a path is MAX_PATH in
    ///      *   general. However, Windows API has many functions that also have Unicode
    ///      *   versions to permit an extended-length path for a maximum total path
    ///      *   length of 32,767 characters.
    ///      *
    ///      * - A path component which ends with a dot is not allowed for Windows
    ///      *   API.
    ///      *
    ///      * If either of these issues are expected to be common in your code, you
    ///      * should set this flag to true. (You should probably not have to set this
        ///      * flag to true.)
    ///      */
    /// ```
    ///

    /// `attribute boolean useDOSDevicePathSyntax;`
    #[inline]
    pub unsafe fn GetUseDOSDevicePathSyntax(&self, aUseDOSDevicePathSyntax: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetUseDOSDevicePathSyntax)(self, aUseDOSDevicePathSyntax)
    }


    /// ```text
    /// /**
    ///      * Setting this to true will prepend the prefix "\\?\" to all parsed file
    ///      * paths which match ^[A-Za-z]:\\.* (regex) syntax.
    ///      *
    ///      * There are two known issues (and potentially more) which can be resolved
    ///      * by the prefix:
    ///      * - In the Windows API, the maximum length for a path is MAX_PATH in
    ///      *   general. However, Windows API has many functions that also have Unicode
    ///      *   versions to permit an extended-length path for a maximum total path
    ///      *   length of 32,767 characters.
    ///      *
    ///      * - A path component which ends with a dot is not allowed for Windows
    ///      *   API.
    ///      *
    ///      * If either of these issues are expected to be common in your code, you
    ///      * should set this flag to true. (You should probably not have to set this
        ///      * flag to true.)
    ///      */
    /// ```
    ///

    /// `attribute boolean useDOSDevicePathSyntax;`
    #[inline]
    pub unsafe fn SetUseDOSDevicePathSyntax(&self, aUseDOSDevicePathSyntax: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetUseDOSDevicePathSyntax)(self, aUseDOSDevicePathSyntax)
    }


    /// ```text
    /// /**
    ///      * Identical to nsIFile::openNSPRFileDesc except it also uses the
    ///      * FILE_SHARE_DELETE flag.
    ///     */
    /// ```
    ///

    /// `[noscript] PRFileDescStar openNSPRFileDescShareDelete (in long flags, in long mode);`
    const _OpenNSPRFileDescShareDelete: () = ();

}


