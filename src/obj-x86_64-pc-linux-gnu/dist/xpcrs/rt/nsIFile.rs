//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIFile.idl
//


/// `interface nsIFile : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFile {
    vtable: *const nsIFileVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFile.
unsafe impl XpCom for nsIFile {
    const IID: nsIID = nsID(0x2fa6884a, 0xae65, 0x412a,
        [0x9d, 0x4c, 0xce, 0x6e, 0x34, 0x54, 0x4b, 0xa1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFile {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFile.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileCoerce {
    /// Cheaply cast a value of this type from a `nsIFile`.
    fn coerce_from(v: &nsIFile) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileCoerce for nsIFile {
    #[inline]
    fn coerce_from(v: &nsIFile) -> &Self {
        v
    }
}

impl nsIFile {
    /// Cast this `nsIFile` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFile {
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
impl<T: nsISupportsCoerce> nsIFileCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFile) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFile
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void append (in AString node); */
    pub Append: unsafe extern "system" fn (this: *const nsIFile, node: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void appendNative (in ACString node); */
    pub AppendNative: unsafe extern "system" fn (this: *const nsIFile, node: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void normalize (); */
    pub Normalize: unsafe extern "system" fn (this: *const nsIFile) -> ::nserror::nsresult,

    /* [must_use] void create (in unsigned long type, in unsigned long permissions); */
    pub Create: unsafe extern "system" fn (this: *const nsIFile, type_: u32, permissions: u32) -> ::nserror::nsresult,

    /* attribute AString leafName; */
    pub GetLeafName: unsafe extern "system" fn (this: *const nsIFile, aLeafName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString leafName; */
    pub SetLeafName: unsafe extern "system" fn (this: *const nsIFile, aLeafName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] attribute ACString nativeLeafName; */
    pub GetNativeLeafName: unsafe extern "system" fn (this: *const nsIFile, aNativeLeafName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] attribute ACString nativeLeafName; */
    pub SetNativeLeafName: unsafe extern "system" fn (this: *const nsIFile, aNativeLeafName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void copyTo (in nsIFile newParentDir, in AString newName); */
    pub CopyTo: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void CopyToNative (in nsIFile newParentDir, in ACString newName); */
    pub CopyToNative: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void copyToFollowingLinks (in nsIFile newParentDir, in AString newName); */
    pub CopyToFollowingLinks: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void copyToFollowingLinksNative (in nsIFile newParentDir, in ACString newName); */
    pub CopyToFollowingLinksNative: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void moveTo (in nsIFile newParentDir, in AString newName); */
    pub MoveTo: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void moveToNative (in nsIFile newParentDir, in ACString newName); */
    pub MoveToNative: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void moveToFollowingLinks (in nsIFile newParentDir, in AString newName); */
    pub MoveToFollowingLinks: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void moveToFollowingLinksNative (in nsIFile newParentDir, in ACString newName); */
    pub MoveToFollowingLinksNative: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void renameTo (in nsIFile newParentDir, in AString newName); */
    pub RenameTo: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void renameToNative (in nsIFile newParentDir, in ACString newName); */
    pub RenameToNative: unsafe extern "system" fn (this: *const nsIFile, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void remove (in boolean recursive); */
    pub Remove: unsafe extern "system" fn (this: *const nsIFile, recursive: bool) -> ::nserror::nsresult,

    /* attribute unsigned long permissions; */
    pub GetPermissions: unsafe extern "system" fn (this: *const nsIFile, aPermissions: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long permissions; */
    pub SetPermissions: unsafe extern "system" fn (this: *const nsIFile, aPermissions: u32) -> ::nserror::nsresult,

    /* attribute unsigned long permissionsOfLink; */
    pub GetPermissionsOfLink: unsafe extern "system" fn (this: *const nsIFile, aPermissionsOfLink: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long permissionsOfLink; */
    pub SetPermissionsOfLink: unsafe extern "system" fn (this: *const nsIFile, aPermissionsOfLink: u32) -> ::nserror::nsresult,

    /* attribute PRTime lastModifiedTime; */
    pub GetLastModifiedTime: unsafe extern "system" fn (this: *const nsIFile, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime lastModifiedTime; */
    pub SetLastModifiedTime: unsafe extern "system" fn (this: *const nsIFile, aLastModifiedTime: PRTime) -> ::nserror::nsresult,

    /* attribute PRTime lastModifiedTimeOfLink; */
    pub GetLastModifiedTimeOfLink: unsafe extern "system" fn (this: *const nsIFile, aLastModifiedTimeOfLink: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime lastModifiedTimeOfLink; */
    pub SetLastModifiedTimeOfLink: unsafe extern "system" fn (this: *const nsIFile, aLastModifiedTimeOfLink: PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime creationTime; */
    pub GetCreationTime: unsafe extern "system" fn (this: *const nsIFile, aCreationTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime creationTimeOfLink; */
    pub GetCreationTimeOfLink: unsafe extern "system" fn (this: *const nsIFile, aCreationTimeOfLink: *mut PRTime) -> ::nserror::nsresult,

    /* attribute int64_t fileSize; */
    pub GetFileSize: unsafe extern "system" fn (this: *const nsIFile, aFileSize: *mut int64_t) -> ::nserror::nsresult,

    /* attribute int64_t fileSize; */
    pub SetFileSize: unsafe extern "system" fn (this: *const nsIFile, aFileSize: int64_t) -> ::nserror::nsresult,

    /* readonly attribute int64_t fileSizeOfLink; */
    pub GetFileSizeOfLink: unsafe extern "system" fn (this: *const nsIFile, aFileSizeOfLink: *mut int64_t) -> ::nserror::nsresult,

    /* readonly attribute AString target; */
    pub GetTarget: unsafe extern "system" fn (this: *const nsIFile, aTarget: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] readonly attribute ACString nativeTarget; */
    pub GetNativeTarget: unsafe extern "system" fn (this: *const nsIFile, aNativeTarget: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AString path; */
    pub GetPath: unsafe extern "system" fn (this: *const nsIFile, aPath: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [must_use,nostdcall,notxpcom] PathString nativePath (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub NativePath: *const ::libc::c_void,

    /* boolean exists (); */
    pub Exists: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isWritable (); */
    pub IsWritable: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isReadable (); */
    pub IsReadable: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isExecutable (); */
    pub IsExecutable: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isHidden (); */
    pub IsHidden: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isDirectory (); */
    pub IsDirectory: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isFile (); */
    pub IsFile: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isSymlink (); */
    pub IsSymlink: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isSpecial (); */
    pub IsSpecial: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* [must_use] void createUnique (in unsigned long type, in unsigned long permissions); */
    pub CreateUnique: unsafe extern "system" fn (this: *const nsIFile, type_: u32, permissions: u32) -> ::nserror::nsresult,

    /* nsIFile clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsIFile, _retval: *mut *const nsIFile) -> ::nserror::nsresult,

    /* boolean equals (in nsIFile inFile); */
    pub Equals: unsafe extern "system" fn (this: *const nsIFile, inFile: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean contains (in nsIFile inFile); */
    pub Contains: unsafe extern "system" fn (this: *const nsIFile, inFile: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIFile parent; */
    pub GetParent: unsafe extern "system" fn (this: *const nsIFile, aParent: *mut *const nsIFile) -> ::nserror::nsresult,

    /* [binaryname(DirectoryEntriesImpl)] readonly attribute nsIDirectoryEnumerator directoryEntries; */
    pub GetDirectoryEntriesImpl: unsafe extern "system" fn (this: *const nsIFile, aDirectoryEntries: *mut *const nsIDirectoryEnumerator) -> ::nserror::nsresult,

    /* void initWithPath (in AString filePath); */
    pub InitWithPath: unsafe extern "system" fn (this: *const nsIFile, filePath: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void initWithNativePath (in ACString filePath); */
    pub InitWithNativePath: unsafe extern "system" fn (this: *const nsIFile, filePath: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void initWithFile (in nsIFile aFile); */
    pub InitWithFile: unsafe extern "system" fn (this: *const nsIFile, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* [must_use,noscript] PRFileDescStar openNSPRFileDesc (in long flags, in long mode); */
    /// Unable to generate binding because `native type PRFileDesc unsupported`
    pub OpenNSPRFileDesc: *const ::libc::c_void,

    /* [must_use,noscript] FILE openANSIFileDesc (in string mode); */
    /// Unable to generate binding because `native type FILE unsupported`
    pub OpenANSIFileDesc: *const ::libc::c_void,

    /* [must_use,noscript] PRLibraryStar load (); */
    /// Unable to generate binding because `native type PRLibrary unsupported`
    pub Load: *const ::libc::c_void,

    /* [must_use] readonly attribute int64_t diskSpaceAvailable; */
    pub GetDiskSpaceAvailable: unsafe extern "system" fn (this: *const nsIFile, aDiskSpaceAvailable: *mut int64_t) -> ::nserror::nsresult,

    /* void appendRelativePath (in AString relativeFilePath); */
    pub AppendRelativePath: unsafe extern "system" fn (this: *const nsIFile, relativeFilePath: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] void appendRelativeNativePath (in ACString relativeFilePath); */
    pub AppendRelativeNativePath: unsafe extern "system" fn (this: *const nsIFile, relativeFilePath: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] attribute ACString persistentDescriptor; */
    pub GetPersistentDescriptor: unsafe extern "system" fn (this: *const nsIFile, aPersistentDescriptor: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] attribute ACString persistentDescriptor; */
    pub SetPersistentDescriptor: unsafe extern "system" fn (this: *const nsIFile, aPersistentDescriptor: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void reveal (); */
    pub Reveal: unsafe extern "system" fn (this: *const nsIFile) -> ::nserror::nsresult,

    /* [must_use] void launch (); */
    pub Launch: unsafe extern "system" fn (this: *const nsIFile) -> ::nserror::nsresult,

    /* [must_use] ACString getRelativeDescriptor (in nsIFile fromFile); */
    pub GetRelativeDescriptor: unsafe extern "system" fn (this: *const nsIFile, fromFile: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setRelativeDescriptor (in nsIFile fromFile, in ACString relativeDesc); */
    pub SetRelativeDescriptor: unsafe extern "system" fn (this: *const nsIFile, fromFile: *const nsIFile, relativeDesc: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] AUTF8String getRelativePath (in nsIFile fromFile); */
    pub GetRelativePath: unsafe extern "system" fn (this: *const nsIFile, fromFile: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [must_use] void setRelativePath (in nsIFile fromFile, in AUTF8String relativeDesc); */
    pub SetRelativePath: unsafe extern "system" fn (this: *const nsIFile, fromFile: *const nsIFile, relativeDesc: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFile {
    /// ```text
    /// /**
    ///  * An nsIFile is an abstract representation of a filename. It manages
    ///  * filename encoding issues, pathname component separators ('/' vs. '\\'
        ///  * vs. ':') and weird stuff like differing volumes with identical names, as
    ///  * on pre-Darwin Macintoshes.
    ///  *
    ///  * This file has long introduced itself to new hackers with this opening
    ///  * paragraph:
    ///  *
    ///  *    This is the only correct cross-platform way to specify a file.
    ///  *    Strings are not such a way. If you grew up on windows or unix, you
    ///  *    may think they are.  Welcome to reality.
    ///  *
    ///  * While taking the pose struck here to heart would be uncalled for, one
    ///  * may safely conclude that writing cross-platform code is an embittering
    ///  * experience.
    ///  *
    ///  * All methods with string parameters have two forms.  The preferred
    ///  * form operates on UCS-2 encoded characters strings.  An alternate
    ///  * form operates on characters strings encoded in the "native" charset.
    ///  *
    ///  * A string containing characters encoded in the native charset cannot
    ///  * be safely passed to javascript via xpconnect.  Therefore, the "native
    ///  * methods" are not scriptable.
    ///  */
    /// /**
    ///      *  Create Types
    ///      *
    ///      *  NORMAL_FILE_TYPE - A normal file.
    ///      *  DIRECTORY_TYPE   - A directory/folder.
    ///      */
    /// ```
    ///

    pub const NORMAL_FILE_TYPE: i64 = 0;


    pub const DIRECTORY_TYPE: i64 = 1;

    /// ```text
    /// /**
    ///      * Flag for openNSPRFileDesc(), to hint to the OS that the file will be
    ///      * read sequentially with agressive readahead.
    ///      */
    /// ```
    ///

    pub const OS_READAHEAD: i64 = 1073741824;

    /// ```text
    /// /**
    ///      * Flag for openNSPRFileDesc(). Deprecated and unreliable!
    ///      * Instead use NS_OpenAnonymousTemporaryFile() to create a temporary
    ///      * file which will be deleted upon close!
    ///      */
    /// ```
    ///

    pub const DELETE_ON_CLOSE: i64 = 2147483648;

    /// ```text
    /// /**
    ///      *  append[Native]
    ///      *
    ///      *  This function is used for constructing a descendent of the
    ///      *  current nsIFile.
    ///      *
    ///      *   @param node
    ///      *       A string which is intended to be a child node of the nsIFile.
    ///      *       For the |appendNative| method, the node must be in the native
    ///      *       filesystem charset.
    ///      */
    /// ```
    ///

    /// `void append (in AString node);`
    #[inline]
    pub unsafe fn Append(&self, node: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Append)(self, node)
    }



    /// `[noscript] void appendNative (in ACString node);`
    #[inline]
    pub unsafe fn AppendNative(&self, node: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AppendNative)(self, node)
    }


    /// ```text
    /// /**
    ///      *  Normalize the pathName (e.g. removing .. and . components on Unix).
    ///      */
    /// ```
    ///

    /// `void normalize ();`
    #[inline]
    pub unsafe fn Normalize(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Normalize)(self, )
    }


    /// ```text
    /// /**
    ///      *  create
    ///      *
    ///      *  This function will create a new file or directory in the
    ///      *  file system. Any nodes that have not been created or
    ///      *  resolved, will be.  If the file or directory already
    ///      *  exists create() will return NS_ERROR_FILE_ALREADY_EXISTS.
    ///      *
    ///      *   @param type
    ///      *       This specifies the type of file system object
    ///      *       to be made.  The only two types at this time
    ///      *       are file and directory which are defined above.
    ///      *       If the type is unrecongnized, we will return an
    ///      *       error (NS_ERROR_FILE_UNKNOWN_TYPE).
    ///      *
    ///      *   @param permissions
    ///      *       The unix style octal permissions.  This may
    ///      *       be ignored on systems that do not need to do
    ///      *       permissions.
    ///      */
    /// ```
    ///

    /// `[must_use] void create (in unsigned long type, in unsigned long permissions);`
    #[inline]
    pub unsafe fn Create(&self, type_: u32, permissions: u32) -> ::nserror::nsresult {
        ((*self.vtable).Create)(self, type_, permissions)
    }


    /// ```text
    /// /**
    ///      *  Accessor to the leaf name of the file itself.
    ///      *  For the |nativeLeafName| method, the nativeLeafName must
    ///      *  be in the native filesystem charset.
    ///      */
    /// ```
    ///

    /// `attribute AString leafName;`
    #[inline]
    pub unsafe fn GetLeafName(&self, aLeafName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLeafName)(self, aLeafName)
    }


    /// ```text
    /// /**
    ///      *  Accessor to the leaf name of the file itself.
    ///      *  For the |nativeLeafName| method, the nativeLeafName must
    ///      *  be in the native filesystem charset.
    ///      */
    /// ```
    ///

    /// `attribute AString leafName;`
    #[inline]
    pub unsafe fn SetLeafName(&self, aLeafName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetLeafName)(self, aLeafName)
    }



    /// `[noscript] attribute ACString nativeLeafName;`
    #[inline]
    pub unsafe fn GetNativeLeafName(&self, aNativeLeafName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNativeLeafName)(self, aNativeLeafName)
    }



    /// `[noscript] attribute ACString nativeLeafName;`
    #[inline]
    pub unsafe fn SetNativeLeafName(&self, aNativeLeafName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetNativeLeafName)(self, aNativeLeafName)
    }


    /// ```text
    /// /**
    ///      *  copyTo[Native]
    ///      *
    ///      *  This will copy this file to the specified newParentDir.
    ///      *  If a newName is specified, the file will be renamed.
    ///      *  If 'this' is not created we will return an error
    ///      *  (NS_ERROR_FILE_TARGET_DOES_NOT_EXIST).
    ///      *
    ///      *  copyTo may fail if the file already exists in the destination
    ///      *  directory.
    ///      *
    ///      *  copyTo will NOT resolve aliases/shortcuts during the copy.
    ///      *
    ///      *   @param newParentDir
    ///      *       This param is the destination directory. If the
    ///      *       newParentDir is null, copyTo() will use the parent
    ///      *       directory of this file. If the newParentDir is not
    ///      *       empty and is not a directory, an error will be
    ///      *       returned (NS_ERROR_FILE_DESTINATION_NOT_DIR). For the
    ///      *       |CopyToNative| method, the newName must be in the
    ///      *       native filesystem charset.
    ///      *
    ///      *   @param newName
    ///      *       This param allows you to specify a new name for
    ///      *       the file to be copied. This param may be empty, in
    ///      *       which case the current leaf name will be used.
    ///      */
    /// ```
    ///

    /// `void copyTo (in nsIFile newParentDir, in AString newName);`
    #[inline]
    pub unsafe fn CopyTo(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).CopyTo)(self, newParentDir, newName)
    }



    /// `[noscript] void CopyToNative (in nsIFile newParentDir, in ACString newName);`
    #[inline]
    pub unsafe fn CopyToNative(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).CopyToNative)(self, newParentDir, newName)
    }


    /// ```text
    /// /**
    ///      *  copyToFollowingLinks[Native]
    ///      *
    ///      *  This function is identical to copyTo with the exception that,
    ///      *  as the name implies, it follows symbolic links.  The XP_UNIX
    ///      *  implementation always follow symbolic links when copying.  For
    ///      *  the |CopyToFollowingLinks| method, the newName must be in the
    ///      *  native filesystem charset.
    ///      */
    /// ```
    ///

    /// `void copyToFollowingLinks (in nsIFile newParentDir, in AString newName);`
    #[inline]
    pub unsafe fn CopyToFollowingLinks(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).CopyToFollowingLinks)(self, newParentDir, newName)
    }



    /// `[noscript] void copyToFollowingLinksNative (in nsIFile newParentDir, in ACString newName);`
    #[inline]
    pub unsafe fn CopyToFollowingLinksNative(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).CopyToFollowingLinksNative)(self, newParentDir, newName)
    }


    /// ```text
    /// /**
    ///      *  moveTo[Native]
    ///      *
    ///      *  A method to move this file or directory to newParentDir.
    ///      *  If a newName is specified, the file or directory will be renamed.
    ///      *  If 'this' is not created we will return an error
    ///      *  (NS_ERROR_FILE_TARGET_DOES_NOT_EXIST).
    ///      *  If 'this' is a file, and the destination file already exists, moveTo
    ///      *  will replace the old file.
    ///      *  This object is updated to refer to the new file.
    ///      *
    ///      *  moveTo will NOT resolve aliases/shortcuts during the copy.
    ///      *  moveTo will do the right thing and allow copies across volumes.
    ///      *  moveTo will return an error (NS_ERROR_FILE_DIR_NOT_EMPTY) if 'this' is
    ///      *  a directory and the destination directory is not empty.
    ///      *  moveTo will return an error (NS_ERROR_FILE_ACCESS_DENIED) if 'this' is
    ///      *  a directory and the destination directory is not writable.
    ///      *
    ///      *   @param newParentDir
    ///      *       This param is the destination directory. If the
    ///      *       newParentDir is empty, moveTo() will rename the file
    ///      *       within its current directory. If the newParentDir is
    ///      *       not empty and does not name a directory, an error will
    ///      *       be returned (NS_ERROR_FILE_DESTINATION_NOT_DIR).  For
    ///      *       the |moveToNative| method, the newName must be in the
    ///      *       native filesystem charset.
    ///      *
    ///      *   @param newName
    ///      *       This param allows you to specify a new name for
    ///      *       the file to be moved. This param may be empty, in
    ///      *       which case the current leaf name will be used.
    ///      */
    /// ```
    ///

    /// `void moveTo (in nsIFile newParentDir, in AString newName);`
    #[inline]
    pub unsafe fn MoveTo(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).MoveTo)(self, newParentDir, newName)
    }



    /// `[noscript] void moveToNative (in nsIFile newParentDir, in ACString newName);`
    #[inline]
    pub unsafe fn MoveToNative(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).MoveToNative)(self, newParentDir, newName)
    }


    /// ```text
    /// /**
    ///      *  moveToFollowingLinks[Native]
    ///      *
    ///      *  This function is identical to moveTo with the exception that,
    ///      *  as the name implies, it follows symbolic links.  The XP_UNIX
    ///      *  implementation always follows symbolic links when moving.  For
    ///      *  the |MoveToFollowingLinks| method, the newName ust be in the native
    ///      *  filesystem charset.
    ///      */
    /// ```
    ///

    /// `void moveToFollowingLinks (in nsIFile newParentDir, in AString newName);`
    #[inline]
    pub unsafe fn MoveToFollowingLinks(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).MoveToFollowingLinks)(self, newParentDir, newName)
    }



    /// `[noscript] void moveToFollowingLinksNative (in nsIFile newParentDir, in ACString newName);`
    #[inline]
    pub unsafe fn MoveToFollowingLinksNative(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).MoveToFollowingLinksNative)(self, newParentDir, newName)
    }


    /// ```text
    /// /**
    ///      *  renameTo
    ///      *
    ///      *  This method is identical to moveTo except that if this file or directory
    ///      *  is moved to a a different volume, it fails and returns an error
    ///      *  (NS_ERROR_FILE_ACCESS_DENIED).
    ///      *  This object will still point to the old location after renaming.
    ///      */
    /// ```
    ///

    /// `void renameTo (in nsIFile newParentDir, in AString newName);`
    #[inline]
    pub unsafe fn RenameTo(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RenameTo)(self, newParentDir, newName)
    }



    /// `[noscript] void renameToNative (in nsIFile newParentDir, in ACString newName);`
    #[inline]
    pub unsafe fn RenameToNative(&self, newParentDir: *const nsIFile, newName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RenameToNative)(self, newParentDir, newName)
    }


    /// ```text
    /// /**
    ///      *  This will try to delete this file.  The 'recursive' flag
    ///      *  must be PR_TRUE to delete directories which are not empty.
    ///      *
    ///      *  This will not resolve any symlinks.
    ///      */
    /// ```
    ///

    /// `void remove (in boolean recursive);`
    #[inline]
    pub unsafe fn Remove(&self, recursive: bool) -> ::nserror::nsresult {
        ((*self.vtable).Remove)(self, recursive)
    }


    /// ```text
    /// /**
    ///      *  Attributes of nsIFile.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long permissions;`
    #[inline]
    pub unsafe fn GetPermissions(&self, aPermissions: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissions)(self, aPermissions)
    }


    /// ```text
    /// /**
    ///      *  Attributes of nsIFile.
    ///      */
    /// ```
    ///

    /// `attribute unsigned long permissions;`
    #[inline]
    pub unsafe fn SetPermissions(&self, aPermissions: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPermissions)(self, aPermissions)
    }



    /// `attribute unsigned long permissionsOfLink;`
    #[inline]
    pub unsafe fn GetPermissionsOfLink(&self, aPermissionsOfLink: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissionsOfLink)(self, aPermissionsOfLink)
    }



    /// `attribute unsigned long permissionsOfLink;`
    #[inline]
    pub unsafe fn SetPermissionsOfLink(&self, aPermissionsOfLink: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPermissionsOfLink)(self, aPermissionsOfLink)
    }


    /// ```text
    /// /**
    ///      *  File Times are to be in milliseconds from
    ///      *  midnight (00:00:00), January 1, 1970 Greenwich Mean
    ///      *  Time (GMT).
    ///      */
    /// ```
    ///

    /// `attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn GetLastModifiedTime(&self, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModifiedTime)(self, aLastModifiedTime)
    }


    /// ```text
    /// /**
    ///      *  File Times are to be in milliseconds from
    ///      *  midnight (00:00:00), January 1, 1970 Greenwich Mean
    ///      *  Time (GMT).
    ///      */
    /// ```
    ///

    /// `attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn SetLastModifiedTime(&self, aLastModifiedTime: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetLastModifiedTime)(self, aLastModifiedTime)
    }



    /// `attribute PRTime lastModifiedTimeOfLink;`
    #[inline]
    pub unsafe fn GetLastModifiedTimeOfLink(&self, aLastModifiedTimeOfLink: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModifiedTimeOfLink)(self, aLastModifiedTimeOfLink)
    }



    /// `attribute PRTime lastModifiedTimeOfLink;`
    #[inline]
    pub unsafe fn SetLastModifiedTimeOfLink(&self, aLastModifiedTimeOfLink: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetLastModifiedTimeOfLink)(self, aLastModifiedTimeOfLink)
    }


    /// ```text
    /// /**
    ///      * The creation time of file in milliseconds from midnight, January 1, 1970
    ///      * GMT, if available.
    ///      *
    ///      * This attribute is only implemented on Windows and macOS. Accessing this
    ///      * on another platform will this will throw NS_ERROR_NOT_IMPLEMENTED.
    ///      */
    /// ```
    ///

    /// `readonly attribute PRTime creationTime;`
    #[inline]
    pub unsafe fn GetCreationTime(&self, aCreationTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetCreationTime)(self, aCreationTime)
    }



    /// `readonly attribute PRTime creationTimeOfLink;`
    #[inline]
    pub unsafe fn GetCreationTimeOfLink(&self, aCreationTimeOfLink: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetCreationTimeOfLink)(self, aCreationTimeOfLink)
    }


    /// ```text
    /// /**
    ///      *  WARNING!  On the Mac, getting/setting the file size with nsIFile
    ///      *  only deals with the size of the data fork.  If you need to
    ///      *  know the size of the combined data and resource forks use the
    ///      *  GetFileSizeWithResFork() method defined on nsILocalFileMac.
    ///      */
    /// ```
    ///

    /// `attribute int64_t fileSize;`
    #[inline]
    pub unsafe fn GetFileSize(&self, aFileSize: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFileSize)(self, aFileSize)
    }


    /// ```text
    /// /**
    ///      *  WARNING!  On the Mac, getting/setting the file size with nsIFile
    ///      *  only deals with the size of the data fork.  If you need to
    ///      *  know the size of the combined data and resource forks use the
    ///      *  GetFileSizeWithResFork() method defined on nsILocalFileMac.
    ///      */
    /// ```
    ///

    /// `attribute int64_t fileSize;`
    #[inline]
    pub unsafe fn SetFileSize(&self, aFileSize: int64_t) -> ::nserror::nsresult {
        ((*self.vtable).SetFileSize)(self, aFileSize)
    }



    /// `readonly attribute int64_t fileSizeOfLink;`
    #[inline]
    pub unsafe fn GetFileSizeOfLink(&self, aFileSizeOfLink: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetFileSizeOfLink)(self, aFileSizeOfLink)
    }


    /// ```text
    /// /**
    ///      *  target & path
    ///      *
    ///      *  Accessor to the string path.  The native version of these
    ///      *  strings are not guaranteed to be a usable path to pass to
    ///      *  NSPR or the C stdlib.  There are problems that affect
    ///      *  platforms on which a path does not fully specify a file
    ///      *  because two volumes can have the same name (e.g., mac).
    ///      *  This is solved by holding "private", native data in the
    ///      *  nsIFile implementation.  This native data is lost when
    ///      *  you convert to a string.
    ///      *
    ///      *      DO NOT PASS TO USE WITH NSPR OR STDLIB!
    ///      *
    ///      *  target
    ///      *      Find out what the symlink points at.  Will give error
    ///      *      (NS_ERROR_FILE_INVALID_PATH) if not a symlink.
    ///      *
    ///      *  path
    ///      *      Find out what the nsIFile points at.
    ///      *
    ///      *  Note that the ACString attributes are returned in the
    ///      *  native filesystem charset.
    ///      *
    ///      */
    /// ```
    ///

    /// `readonly attribute AString target;`
    #[inline]
    pub unsafe fn GetTarget(&self, aTarget: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTarget)(self, aTarget)
    }



    /// `[noscript] readonly attribute ACString nativeTarget;`
    #[inline]
    pub unsafe fn GetNativeTarget(&self, aNativeTarget: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNativeTarget)(self, aNativeTarget)
    }



    /// `readonly attribute AString path;`
    #[inline]
    pub unsafe fn GetPath(&self, aPath: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPath)(self, aPath)
    }



    /// `[must_use,nostdcall,notxpcom] PathString nativePath ();`
    const _NativePath: () = ();


    /// `boolean exists ();`
    #[inline]
    pub unsafe fn Exists(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Exists)(self, _retval)
    }



    /// `boolean isWritable ();`
    #[inline]
    pub unsafe fn IsWritable(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsWritable)(self, _retval)
    }



    /// `boolean isReadable ();`
    #[inline]
    pub unsafe fn IsReadable(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsReadable)(self, _retval)
    }



    /// `boolean isExecutable ();`
    #[inline]
    pub unsafe fn IsExecutable(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsExecutable)(self, _retval)
    }



    /// `boolean isHidden ();`
    #[inline]
    pub unsafe fn IsHidden(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsHidden)(self, _retval)
    }



    /// `boolean isDirectory ();`
    #[inline]
    pub unsafe fn IsDirectory(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDirectory)(self, _retval)
    }



    /// `boolean isFile ();`
    #[inline]
    pub unsafe fn IsFile(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsFile)(self, _retval)
    }



    /// `boolean isSymlink ();`
    #[inline]
    pub unsafe fn IsSymlink(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSymlink)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Not a regular file, not a directory, not a symlink.
    ///      */
    /// ```
    ///

    /// `boolean isSpecial ();`
    #[inline]
    pub unsafe fn IsSpecial(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsSpecial)(self, _retval)
    }


    /// ```text
    /// /**
    ///      *  createUnique
    ///      *
    ///      *  This function will create a new file or directory in the
    ///      *  file system. Any nodes that have not been created or
    ///      *  resolved, will be.  If this file already exists, we try
    ///      *  variations on the leaf name "suggestedName" until we find
    ///      *  one that did not already exist.
    ///      *
    ///      *  If the search for nonexistent files takes too long
    ///      *  (thousands of the variants already exist), we give up and
    ///      *  return NS_ERROR_FILE_TOO_BIG.
    ///      *
    ///      *   @param type
    ///      *       This specifies the type of file system object
    ///      *       to be made.  The only two types at this time
    ///      *       are file and directory which are defined above.
    ///      *       If the type is unrecongnized, we will return an
    ///      *       error (NS_ERROR_FILE_UNKNOWN_TYPE).
    ///      *
    ///      *   @param permissions
    ///      *       The unix style octal permissions.  This may
    ///      *       be ignored on systems that do not need to do
    ///      *       permissions.
    ///      */
    /// ```
    ///

    /// `[must_use] void createUnique (in unsigned long type, in unsigned long permissions);`
    #[inline]
    pub unsafe fn CreateUnique(&self, type_: u32, permissions: u32) -> ::nserror::nsresult {
        ((*self.vtable).CreateUnique)(self, type_, permissions)
    }


    /// ```text
    /// /**
    ///       * clone()
    ///       *
    ///       * This function will allocate and initialize a nsIFile object to the
    ///       * exact location of the |this| nsIFile.
    ///       *
    ///       *   @param file
    ///       *          A nsIFile which this object will be initialize
    ///       *          with.
    ///       *
    ///       */
    /// ```
    ///

    /// `nsIFile clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


    /// ```text
    /// /**
    ///      *  Will determine if the inFile equals this.
    ///      */
    /// ```
    ///

    /// `boolean equals (in nsIFile inFile);`
    #[inline]
    pub unsafe fn Equals(&self, inFile: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, inFile, _retval)
    }


    /// ```text
    /// /**
    ///      *  Will determine if inFile is a descendant of this file.
    ///      *  This routine looks in subdirectories too.
    ///      */
    /// ```
    ///

    /// `boolean contains (in nsIFile inFile);`
    #[inline]
    pub unsafe fn Contains(&self, inFile: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Contains)(self, inFile, _retval)
    }


    /// ```text
    /// /**
    ///      *  Parent will be null when this is at the top of the volume.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile parent;`
    #[inline]
    pub unsafe fn GetParent(&self, aParent: *mut *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetParent)(self, aParent)
    }


    /// ```text
    /// /**
    ///      *  Returns an enumeration of the elements in a directory. Each
    ///      *  element in the enumeration is an nsIFile.
    ///      *
    ///      *   @throws NS_ERROR_FILE_NOT_DIRECTORY if the current nsIFile does
    ///      *           not specify a directory.
    ///      */
    /// ```
    ///

    /// `[binaryname(DirectoryEntriesImpl)] readonly attribute nsIDirectoryEnumerator directoryEntries;`
    #[inline]
    pub unsafe fn GetDirectoryEntriesImpl(&self, aDirectoryEntries: *mut *const nsIDirectoryEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetDirectoryEntriesImpl)(self, aDirectoryEntries)
    }


    /// ```text
    /// /**
    ///      *  initWith[Native]Path
    ///      *
    ///      *  This function will initialize the nsIFile object.  Any
    ///      *  internal state information will be reset.
    ///      *
    ///      *   @param filePath
    ///      *       A string which specifies a full file path to a
    ///      *       location.  Relative paths will be treated as an
    ///      *       error (NS_ERROR_FILE_UNRECOGNIZED_PATH).  For
    ///      *       initWithNativePath, the filePath must be in the native
    ///      *       filesystem charset.
    ///      */
    /// ```
    ///

    /// `void initWithPath (in AString filePath);`
    #[inline]
    pub unsafe fn InitWithPath(&self, filePath: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).InitWithPath)(self, filePath)
    }



    /// `[noscript] void initWithNativePath (in ACString filePath);`
    #[inline]
    pub unsafe fn InitWithNativePath(&self, filePath: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).InitWithNativePath)(self, filePath)
    }


    /// ```text
    /// /**
    ///      *  initWithFile
    ///      *
    ///      *  Initialize this object with another file
    ///      *
    ///      *   @param aFile
    ///      *       the file this becomes equivalent to
    ///      */
    /// ```
    ///

    /// `void initWithFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn InitWithFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).InitWithFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///      * Return the result of PR_Open on the file.  The caller is
    ///      * responsible for calling PR_Close on the result.  On success, the
    ///      * returned PRFileDescr must be non-null.
    ///      *
    ///      * @param flags the PR_Open flags from prio.h, plus optionally
    ///      * OS_READAHEAD or DELETE_ON_CLOSE. OS_READAHEAD is a hint to the
    ///      * OS that the file will be read sequentially with agressive
    ///      * readahead. DELETE_ON_CLOSE is unreliable on Windows and is deprecated.
    ///      * Instead use NS_OpenAnonymousTemporaryFile() to create a temporary
    ///      * file which will be deleted upon close.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] PRFileDescStar openNSPRFileDesc (in long flags, in long mode);`
    const _OpenNSPRFileDesc: () = ();

    /// ```text
    /// /**
    ///      * Return the result of fopen on the file.  The caller is
    ///      * responsible for calling fclose on the result.  On success, the
    ///      * returned FILE pointer must be non-null.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] FILE openANSIFileDesc (in string mode);`
    const _OpenANSIFileDesc: () = ();

    /// ```text
    /// /**
    ///      * Return the result of PR_LoadLibrary on the file.  The caller is
    ///      * responsible for calling PR_UnloadLibrary on the result.
    ///      */
    /// ```
    ///

    /// `[must_use,noscript] PRLibraryStar load ();`
    const _Load: () = ();


    /// `[must_use] readonly attribute int64_t diskSpaceAvailable;`
    #[inline]
    pub unsafe fn GetDiskSpaceAvailable(&self, aDiskSpaceAvailable: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetDiskSpaceAvailable)(self, aDiskSpaceAvailable)
    }


    /// ```text
    /// /**
    ///      *  appendRelative[Native]Path
    ///      *
    ///      *  Append a relative path to the current path of the nsIFile object.
    ///      *
    ///      *   @param relativeFilePath
    ///      *       relativeFilePath is a native relative path. For security reasons,
    ///      *       this cannot contain .. or cannot start with a directory separator.
    ///      *       For the |appendRelativeNativePath| method, the relativeFilePath
    ///      *       must be in the native filesystem charset.
    ///      */
    /// ```
    ///

    /// `void appendRelativePath (in AString relativeFilePath);`
    #[inline]
    pub unsafe fn AppendRelativePath(&self, relativeFilePath: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AppendRelativePath)(self, relativeFilePath)
    }



    /// `[noscript] void appendRelativeNativePath (in ACString relativeFilePath);`
    #[inline]
    pub unsafe fn AppendRelativeNativePath(&self, relativeFilePath: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AppendRelativeNativePath)(self, relativeFilePath)
    }


    /// ```text
    /// /**
    ///      *  Accessor to a null terminated string which will specify
    ///      *  the file in a persistent manner for disk storage.
    ///      *
    ///      *  The character set of this attribute is undefined.  DO NOT TRY TO
    ///      *  INTERPRET IT AS HUMAN READABLE TEXT!
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString persistentDescriptor;`
    #[inline]
    pub unsafe fn GetPersistentDescriptor(&self, aPersistentDescriptor: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPersistentDescriptor)(self, aPersistentDescriptor)
    }


    /// ```text
    /// /**
    ///      *  Accessor to a null terminated string which will specify
    ///      *  the file in a persistent manner for disk storage.
    ///      *
    ///      *  The character set of this attribute is undefined.  DO NOT TRY TO
    ///      *  INTERPRET IT AS HUMAN READABLE TEXT!
    ///      */
    /// ```
    ///

    /// `[must_use] attribute ACString persistentDescriptor;`
    #[inline]
    pub unsafe fn SetPersistentDescriptor(&self, aPersistentDescriptor: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetPersistentDescriptor)(self, aPersistentDescriptor)
    }


    /// ```text
    /// /**
    ///      *  reveal
    ///      *
    ///      *  Ask the operating system to open the folder which contains
    ///      *  this file or folder. This routine only works on platforms which
    ///      *  support the ability to open a folder and is run async on Windows.
    ///      *  This routine must be called on the main.
    ///      */
    /// ```
    ///

    /// `[must_use] void reveal ();`
    #[inline]
    pub unsafe fn Reveal(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reveal)(self, )
    }


    /// ```text
    /// /**
    ///      *  launch
    ///      *
    ///      *  Ask the operating system to attempt to open the file.
    ///      *  this really just simulates "double clicking" the file on your platform.
    ///      *  This routine only works on platforms which support this functionality
    ///      *  and is run async on Windows.  This routine must be called on the
    ///      *  main thread.
    ///      */
    /// ```
    ///

    /// `[must_use] void launch ();`
    #[inline]
    pub unsafe fn Launch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Launch)(self, )
    }


    /// ```text
    /// /**
    ///      *  getRelativeDescriptor
    ///      *
    ///      *  Returns a relative file path in an opaque, XP format. It is therefore
    ///      *  not a native path.
    ///      *
    ///      *  The character set of the string returned from this function is
    ///      *  undefined.  DO NOT TRY TO INTERPRET IT AS HUMAN READABLE TEXT!
    ///      *
    ///      *   @param fromFile
    ///      *       the file from which the descriptor is relative.
    ///      *       Throws if fromFile is null.
    ///      */
    /// ```
    ///

    /// `[must_use] ACString getRelativeDescriptor (in nsIFile fromFile);`
    #[inline]
    pub unsafe fn GetRelativeDescriptor(&self, fromFile: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRelativeDescriptor)(self, fromFile, _retval)
    }


    /// ```text
    /// /**
    ///      *  setRelativeDescriptor
    ///      *
    ///      *  Initializes the file to the location relative to fromFile using
    ///      *  a string returned by getRelativeDescriptor.
    ///      *
    ///      *   @param fromFile
    ///      *       the file to which the descriptor is relative
    ///      *   @param relative
    ///      *       the relative descriptor obtained from getRelativeDescriptor
    ///      */
    /// ```
    ///

    /// `[must_use] void setRelativeDescriptor (in nsIFile fromFile, in ACString relativeDesc);`
    #[inline]
    pub unsafe fn SetRelativeDescriptor(&self, fromFile: *const nsIFile, relativeDesc: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetRelativeDescriptor)(self, fromFile, relativeDesc)
    }


    /// ```text
    /// /**
    ///      *  getRelativePath
    ///      *
    ///      *  Returns a relative file from 'fromFile' to this file as a UTF-8 string.
    ///      *  Going up the directory tree is represented via "../".  '/' is used as
    ///      *  the path segment separator.  This is not a native path, since it's UTF-8
    ///      *  encoded.
    ///      *
    ///      *   @param fromFile
    ///      *       the file from which the path is relative.
    ///      *       Throws if fromFile is null.
    ///      */
    /// ```
    ///

    /// `[must_use] AUTF8String getRelativePath (in nsIFile fromFile);`
    #[inline]
    pub unsafe fn GetRelativePath(&self, fromFile: *const nsIFile, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRelativePath)(self, fromFile, _retval)
    }


    /// ```text
    /// /**
    ///      *  setRelativePath
    ///      *
    ///      *  Initializes the file to the location relative to fromFile using
    ///      *  a string returned by getRelativePath.
    ///      *
    ///      *   @param fromFile
    ///      *       the file from which the path is relative
    ///      *   @param relative
    ///      *       the relative path obtained from getRelativePath
    ///      */
    /// ```
    ///

    /// `[must_use] void setRelativePath (in nsIFile fromFile, in AUTF8String relativeDesc);`
    #[inline]
    pub unsafe fn SetRelativePath(&self, fromFile: *const nsIFile, relativeDesc: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetRelativePath)(self, fromFile, relativeDesc)
    }


}


