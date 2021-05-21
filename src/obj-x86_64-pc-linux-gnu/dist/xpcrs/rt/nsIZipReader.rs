//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/nsIZipReader.idl
//


/// `interface nsIZipEntry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIZipEntry {
    vtable: *const nsIZipEntryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIZipEntry.
unsafe impl XpCom for nsIZipEntry {
    const IID: nsIID = nsID(0xfad6f72f, 0x13d8, 0x4e26,
        [0x91, 0x73, 0x53, 0x00, 0x7a, 0x4a, 0xfe, 0x71]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIZipEntry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIZipEntry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIZipEntryCoerce {
    /// Cheaply cast a value of this type from a `nsIZipEntry`.
    fn coerce_from(v: &nsIZipEntry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIZipEntryCoerce for nsIZipEntry {
    #[inline]
    fn coerce_from(v: &nsIZipEntry) -> &Self {
        v
    }
}

impl nsIZipEntry {
    /// Cast this `nsIZipEntry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIZipEntryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIZipEntry {
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
impl<T: nsISupportsCoerce> nsIZipEntryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipEntry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIZipEntry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIZipEntryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short compression; */
    pub GetCompression: unsafe extern "system" fn (this: *const nsIZipEntry, aCompression: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute unsigned long size; */
    pub GetSize: unsafe extern "system" fn (this: *const nsIZipEntry, aSize: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long realSize; */
    pub GetRealSize: unsafe extern "system" fn (this: *const nsIZipEntry, aRealSize: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long CRC32; */
    pub GetCRC32: unsafe extern "system" fn (this: *const nsIZipEntry, aCRC32: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute boolean isDirectory; */
    pub GetIsDirectory: unsafe extern "system" fn (this: *const nsIZipEntry, aIsDirectory: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute PRTime lastModifiedTime; */
    pub GetLastModifiedTime: unsafe extern "system" fn (this: *const nsIZipEntry, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute boolean isSynthetic; */
    pub GetIsSynthetic: unsafe extern "system" fn (this: *const nsIZipEntry, aIsSynthetic: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long permissions; */
    pub GetPermissions: unsafe extern "system" fn (this: *const nsIZipEntry, aPermissions: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIZipEntry {

    /// ```text
    /// /**
    ///      * The type of compression used for the item.  The possible values and
    ///      * their meanings are defined in the zip file specification at
    ///      * http://www.pkware.com/business_and_developers/developer/appnote/
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned short compression;`
    #[inline]
    pub unsafe fn GetCompression(&self, aCompression: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetCompression)(self, aCompression)
    }


    /// ```text
    /// /**
    ///      * The compressed size of the data in the item.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long size;`
    #[inline]
    pub unsafe fn GetSize(&self, aSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetSize)(self, aSize)
    }


    /// ```text
    /// /**
    ///      * The uncompressed size of the data in the item.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long realSize;`
    #[inline]
    pub unsafe fn GetRealSize(&self, aRealSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetRealSize)(self, aRealSize)
    }


    /// ```text
    /// /**
    ///      * The CRC-32 hash of the file in the entry.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long CRC32;`
    #[inline]
    pub unsafe fn GetCRC32(&self, aCRC32: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetCRC32)(self, aCRC32)
    }


    /// ```text
    /// /**
    ///      * True if the name of the entry ends with '/' and false otherwise.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isDirectory;`
    #[inline]
    pub unsafe fn GetIsDirectory(&self, aIsDirectory: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsDirectory)(self, aIsDirectory)
    }


    /// ```text
    /// /**
    ///      * The time at which this item was last modified.
    ///      */
    /// ```
    ///

    /// `readonly attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn GetLastModifiedTime(&self, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModifiedTime)(self, aLastModifiedTime)
    }


    /// ```text
    /// /**
    ///      * Use this attribute to determine whether this item is an actual zip entry
    ///      * or is one synthesized for part of a real entry's path.  A synthesized
    ///      * entry represents a directory within the zip file which has no
    ///      * corresponding entry within the zip file.  For example, the entry for the
    ///      * directory foo/ in a zip containing exactly one entry for foo/bar.txt
    ///      * is synthetic.  If the zip file contains an actual entry for a directory,
    ///      * this attribute will be false for the nsIZipEntry for that directory.
    ///      * It is impossible for a file to be synthetic.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isSynthetic;`
    #[inline]
    pub unsafe fn GetIsSynthetic(&self, aIsSynthetic: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSynthetic)(self, aIsSynthetic)
    }


    /// ```text
    /// /**
    ///      * The UNIX style file permissions of this item.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long permissions;`
    #[inline]
    pub unsafe fn GetPermissions(&self, aPermissions: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissions)(self, aPermissions)
    }


}


/// `interface nsIZipReader : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIZipReader {
    vtable: *const nsIZipReaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIZipReader.
unsafe impl XpCom for nsIZipReader {
    const IID: nsIID = nsID(0x9ba4ef54, 0xe0a0, 0x4f65,
        [0x9d, 0x23, 0x12, 0x84, 0x82, 0x44, 0x88, 0x85]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIZipReader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIZipReader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIZipReaderCoerce {
    /// Cheaply cast a value of this type from a `nsIZipReader`.
    fn coerce_from(v: &nsIZipReader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIZipReaderCoerce for nsIZipReader {
    #[inline]
    fn coerce_from(v: &nsIZipReader) -> &Self {
        v
    }
}

impl nsIZipReader {
    /// Cast this `nsIZipReader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIZipReaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIZipReader {
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
impl<T: nsISupportsCoerce> nsIZipReaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipReader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIZipReader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIZipReaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void open (in nsIFile zipFile); */
    pub Open: unsafe extern "system" fn (this: *const nsIZipReader, zipFile: *const nsIFile) -> ::nserror::nsresult,

    /* void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry); */
    pub OpenInner: unsafe extern "system" fn (this: *const nsIZipReader, zipReader: *const nsIZipReader, zipEntry: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void openMemory (in voidPtr aData, in unsigned long aLength); */
    pub OpenMemory: unsafe extern "system" fn (this: *const nsIZipReader, aData: *const libc::c_void, aLength: u32) -> ::nserror::nsresult,

    /* readonly attribute nsIFile file; */
    pub GetFile: unsafe extern "system" fn (this: *const nsIZipReader, aFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIZipReader) -> ::nserror::nsresult,

    /* void test (in AUTF8String aEntryName); */
    pub Test: unsafe extern "system" fn (this: *const nsIZipReader, aEntryName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void extract (in AUTF8String zipEntry, in nsIFile outFile); */
    pub Extract: unsafe extern "system" fn (this: *const nsIZipReader, zipEntry: *const ::nsstring::nsACString, outFile: *const nsIFile) -> ::nserror::nsresult,

    /* nsIZipEntry getEntry (in AUTF8String zipEntry); */
    pub GetEntry: unsafe extern "system" fn (this: *const nsIZipReader, zipEntry: *const ::nsstring::nsACString, _retval: *mut *const nsIZipEntry) -> ::nserror::nsresult,

    /* boolean hasEntry (in AUTF8String zipEntry); */
    pub HasEntry: unsafe extern "system" fn (this: *const nsIZipReader, zipEntry: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern); */
    pub FindEntries: unsafe extern "system" fn (this: *const nsIZipReader, aPattern: *const ::nsstring::nsACString, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,

    /* nsIInputStream getInputStream (in AUTF8String zipEntry); */
    pub GetInputStream: unsafe extern "system" fn (this: *const nsIZipReader, zipEntry: *const ::nsstring::nsACString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,

    /* nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry); */
    pub GetInputStreamWithSpec: unsafe extern "system" fn (this: *const nsIZipReader, aJarSpec: *const ::nsstring::nsACString, zipEntry: *const ::nsstring::nsACString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIZipReader {

    /// ```text
    /// /**
    ///      * Opens a zip file for reading.
    ///      * It is allowed to open with another file,
    ///      * but it needs to be closed first with close().
    ///      */
    /// ```
    ///

    /// `void open (in nsIFile zipFile);`
    #[inline]
    pub unsafe fn Open(&self, zipFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, zipFile)
    }


    /// ```text
    /// /**
    ///      * Opens a zip file inside a zip file for reading.
    ///      */
    /// ```
    ///

    /// `void openInner (in nsIZipReader zipReader, in AUTF8String zipEntry);`
    #[inline]
    pub unsafe fn OpenInner(&self, zipReader: *const nsIZipReader, zipEntry: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OpenInner)(self, zipReader, zipEntry)
    }


    /// ```text
    /// /**
    ///      * Opens a zip file stored in memory; the file attribute will be null.
    ///      *
    ///      * The ZipReader does not copy or take ownership of this memory; the
    ///      * caller must ensure that it is valid and unmodified until the
    ///      * ZipReader is closed or destroyed, and must free the memory as
    ///      * appropriate afterwards.
    ///      */
    /// ```
    ///

    /// `void openMemory (in voidPtr aData, in unsigned long aLength);`
    #[inline]
    pub unsafe fn OpenMemory(&self, aData: *const libc::c_void, aLength: u32) -> ::nserror::nsresult {
        ((*self.vtable).OpenMemory)(self, aData, aLength)
    }


    /// ```text
    /// /**
    ///      * The file that represents the zip with which this zip reader was
    ///      * initialized.  This will be null if there is no underlying file.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIFile file;`
    #[inline]
    pub unsafe fn GetFile(&self, aFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///      * Closes a zip reader. Subsequent attempts to extract files or read from
    ///      * its input stream will result in an error.
    ///      *
    ///      * Subsequent attempts to access a nsIZipEntry obtained from this zip
    ///      * reader will cause unspecified behavior.
    ///      */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///      * Tests the integrity of the archive by performing a CRC check
    ///      * on each item expanded into memory.  If an entry is specified
    ///      * the integrity of only that item is tested.  If null (javascript)
    ///      * or ""_ns (c++) is passed in the integrity of all items
    ///      * in the archive are tested.
    ///      */
    /// ```
    ///

    /// `void test (in AUTF8String aEntryName);`
    #[inline]
    pub unsafe fn Test(&self, aEntryName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Test)(self, aEntryName)
    }


    /// ```text
    /// /**
    ///      * Extracts a zip entry into a local file specified by outFile.
    ///      * The entry must be stored in the zip in either uncompressed or
    ///      * DEFLATE-compressed format for the extraction to be successful.
    ///      * If the entry is a directory, the directory will be extracted
    ///      * non-recursively.
    ///      */
    /// ```
    ///

    /// `void extract (in AUTF8String zipEntry, in nsIFile outFile);`
    #[inline]
    pub unsafe fn Extract(&self, zipEntry: *const ::nsstring::nsACString, outFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Extract)(self, zipEntry, outFile)
    }


    /// ```text
    /// /**
    ///      * Returns a nsIZipEntry describing a specified zip entry.
    ///      */
    /// ```
    ///

    /// `nsIZipEntry getEntry (in AUTF8String zipEntry);`
    #[inline]
    pub unsafe fn GetEntry(&self, zipEntry: *const ::nsstring::nsACString, _retval: *mut *const nsIZipEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetEntry)(self, zipEntry, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks whether the zipfile contains an entry specified by entryName.
    ///      */
    /// ```
    ///

    /// `boolean hasEntry (in AUTF8String zipEntry);`
    #[inline]
    pub unsafe fn HasEntry(&self, zipEntry: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasEntry)(self, zipEntry, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns a string enumerator containing the matching entry names.
    ///      *
    ///      * @param aPattern
    ///      *   A regular expression used to find matching entries in the zip file.
    ///      *   Set this parameter to null (javascript) or ""_ns (c++) or "*"
    ///      *   to get all entries; otherwise, use the
    ///      *   following syntax:
    ///      *
    ///      *   o * matches anything
    ///      *   o ? matches one character
    ///      *   o $ matches the end of the string
    ///      *   o [abc] matches one occurrence of a, b, or c. The only character that
    ///      *           must be escaped inside the brackets is ].  ^ and - must never
///      *           appear in the first and second positions within the brackets,
///      *           respectively.  (In the former case, the behavior specified for
    ///      *           '[^az]' will happen.)
///      *   o [a-z] matches any character between a and z.  The characters a and z
///      *           must either both be letters or both be numbers, with the
///      *           character represented by 'a' having a lower ASCII value than
///      *           the character represented by 'z'.
///      *   o [^az] matches any character except a or z.  If ] is to appear inside
///      *           the brackets as a character to not match, it must be escaped.
///      *   o pat~pat2 returns matches to the pattern 'pat' which do not also match
///      *              the pattern 'pat2'.  This may be used to perform filtering
///      *              upon the results of one pattern to remove all matches which
///      *              also match another pattern.  For example, because '*'
///      *              matches any string and '*z*' matches any string containing a
///      *              'z', '*~*z*' will match all strings except those containing
///      *              a 'z'.  Note that a pattern may not use '~' multiple times,
///      *              so a string such as '*~*z*~*y*' is not a valid pattern.
///      *   o (foo|bar) will match either the pattern foo or the pattern bar.
///      *               Neither of the patterns foo or bar may use the 'pat~pat2'
///      *               syntax described immediately above.
///      *   o \ will escape a special character.  Escaping is required for all
///      *       special characters unless otherwise specified.
///      *   o All other characters match case-sensitively.
///      *
///      *   An aPattern not conforming to this syntax has undefined behavior.
///      *
///      * @throws NS_ERROR_ILLEGAL_VALUE on many but not all invalid aPattern
///      *                                values.
///      */
/// ```
///

/// `nsIUTF8StringEnumerator findEntries (in AUTF8String aPattern);`
#[inline]
pub unsafe fn FindEntries(&self, aPattern: *const ::nsstring::nsACString, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
((*self.vtable).FindEntries)(self, aPattern, _retval)
}


/// ```text
/// /**
///      * Returns an input stream containing the contents of the specified zip
///      * entry.
///      * @param zipEntry the name of the entry to open the stream from
///      */
/// ```
///

/// `nsIInputStream getInputStream (in AUTF8String zipEntry);`
#[inline]
pub unsafe fn GetInputStream(&self, zipEntry: *const ::nsstring::nsACString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
((*self.vtable).GetInputStream)(self, zipEntry, _retval)
}


/// ```text
/// /**
///      * Returns an input stream containing the contents of the specified zip
///      * entry. If the entry refers to a directory (ends with '/'), a directory stream
///      * is opened, otherwise the contents of the file entry is returned.
///      * @param aJarSpec the Spec of the URI for the JAR (only used for directory streams)
///      * @param zipEntry the name of the entry to open the stream from
///      */
/// ```
///

/// `nsIInputStream getInputStreamWithSpec (in AUTF8String aJarSpec, in AUTF8String zipEntry);`
#[inline]
pub unsafe fn GetInputStreamWithSpec(&self, aJarSpec: *const ::nsstring::nsACString, zipEntry: *const ::nsstring::nsACString, _retval: *mut*const nsIInputStream) -> ::nserror::nsresult {
((*self.vtable).GetInputStreamWithSpec)(self, aJarSpec, zipEntry, _retval)
}


}


/// `interface nsIZipReaderCache : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIZipReaderCache {
vtable: *const nsIZipReaderCacheVTable,

/// This field is a phantomdata to ensure that the VTable type and any
/// struct containing it is not safe to send across threads, as XPCOM is
/// generally not threadsafe.
///
/// XPCOM interfaces in general are not safe to send across threads.
__nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIZipReaderCache.
unsafe impl XpCom for nsIZipReaderCache {
const IID: nsIID = nsID(0x31179807, 0x9fcd, 0x46c4,
[0xbe, 0xfa, 0x2a, 0xde, 0x20, 0x9a, 0x39, 0x4b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIZipReaderCache {
#[inline]
unsafe fn addref(&self) {
self.AddRef();
}
#[inline]
unsafe fn release(&self) {
self.Release();
}
}

// This trait is implemented on all types which can be coerced to from nsIZipReaderCache.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIZipReaderCacheCoerce {
/// Cheaply cast a value of this type from a `nsIZipReaderCache`.
fn coerce_from(v: &nsIZipReaderCache) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIZipReaderCacheCoerce for nsIZipReaderCache {
#[inline]
fn coerce_from(v: &nsIZipReaderCache) -> &Self {
v
}
}

impl nsIZipReaderCache {
/// Cast this `nsIZipReaderCache` to one of its base interfaces.
#[inline]
pub fn coerce<T: nsIZipReaderCacheCoerce>(&self) -> &T {
T::coerce_from(self)
}
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIZipReaderCache {
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
impl<T: nsISupportsCoerce> nsIZipReaderCacheCoerce for T {
#[inline]
fn coerce_from(v: &nsIZipReaderCache) -> &Self {
T::coerce_from(v)
}
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIZipReaderCache
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIZipReaderCacheVTable {
/// We need to include the members from the base interface's vtable at the start
/// of the VTable definition.
pub __base: nsISupportsVTable,

/* void init (in unsigned long cacheSize); */
pub Init: unsafe extern "system" fn (this: *const nsIZipReaderCache, cacheSize: u32) -> ::nserror::nsresult,

/* nsIZipReader getZip (in nsIFile zipFile); */
pub GetZip: unsafe extern "system" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, _retval: *mut *const nsIZipReader) -> ::nserror::nsresult,

/* nsIZipReader getZipIfCached (in nsIFile zipFile); */
pub GetZipIfCached: unsafe extern "system" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, _retval: *mut *const nsIZipReader) -> ::nserror::nsresult,

/* bool isCached (in nsIFile zipFile); */
pub IsCached: unsafe extern "system" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult,

/* nsIZipReader getInnerZip (in nsIFile zipFile, in AUTF8String zipEntry); */
pub GetInnerZip: unsafe extern "system" fn (this: *const nsIZipReaderCache, zipFile: *const nsIFile, zipEntry: *const ::nsstring::nsACString, _retval: *mut *const nsIZipReader) -> ::nserror::nsresult,

/* PRFileDescStar getFd (in nsIFile zipFile); */
/// Unable to generate binding because `native type PRFileDesc unsupported`
pub GetFd: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIZipReaderCache {

/// ```text
/// /**
///      * Initializes a new zip reader cache.
///      * @param cacheSize - the number of released entries to maintain before
///      *   beginning to throw some out (note that the number of outstanding
///      *   entries can be much greater than this number -- this is the count
///      *   for those otherwise unused entries)
///      */
/// ```
///

/// `void init (in unsigned long cacheSize);`
#[inline]
pub unsafe fn Init(&self, cacheSize: u32) -> ::nserror::nsresult {
((*self.vtable).Init)(self, cacheSize)
}


/// ```text
/// /**
///      * Returns a (possibly shared) nsIZipReader for an nsIFile.
///      *
///      * If the zip reader for given file is not in the cache, a new zip reader
///      * is created, initialized, and opened (see nsIZipReader::init and
///      * nsIZipReader::open). Otherwise the previously created zip reader is
///      * returned.
///      *
///      * @note If someone called close() on the shared nsIZipReader, this method
///      *       will return the closed zip reader.
///      */
/// ```
///

/// `nsIZipReader getZip (in nsIFile zipFile);`
#[inline]
pub unsafe fn GetZip(&self, zipFile: *const nsIFile, _retval: *mut *const nsIZipReader) -> ::nserror::nsresult {
((*self.vtable).GetZip)(self, zipFile, _retval)
}


/// ```text
/// /**
///      * Like getZip(), returns a (possibly shared) nsIZipReader for an nsIFile,
///      * but if a zip reader for the given file is not in the cache, returns
///      * error NS_ERROR_CACHE_KEY_NOT_FOUND rather than creating a new reader.
///      *
///      * @note If someone called close() on the shared nsIZipReader, this method
///      *       will return the closed zip reader.
///      */
/// ```
///

/// `nsIZipReader getZipIfCached (in nsIFile zipFile);`
#[inline]
pub unsafe fn GetZipIfCached(&self, zipFile: *const nsIFile, _retval: *mut *const nsIZipReader) -> ::nserror::nsresult {
((*self.vtable).GetZipIfCached)(self, zipFile, _retval)
}


/// ```text
/// /**
///      * returns true if this zipreader already has this file cached
///      */
/// ```
///

/// `bool isCached (in nsIFile zipFile);`
#[inline]
pub unsafe fn IsCached(&self, zipFile: *const nsIFile, _retval: *mut bool) -> ::nserror::nsresult {
((*self.vtable).IsCached)(self, zipFile, _retval)
}


/// ```text
/// /**
///      * Returns a (possibly shared) nsIZipReader for a zip inside another zip
///      *
///      * See getZip
///      */
/// ```
///

/// `nsIZipReader getInnerZip (in nsIFile zipFile, in AUTF8String zipEntry);`
#[inline]
pub unsafe fn GetInnerZip(&self, zipFile: *const nsIFile, zipEntry: *const ::nsstring::nsACString, _retval: *mut *const nsIZipReader) -> ::nserror::nsresult {
((*self.vtable).GetInnerZip)(self, zipFile, zipEntry, _retval)
}


/// ```text
/// /**
///      * Returns the cached NSPR file descriptor of the file.
///      * Note: currently not supported on Windows platform.
///      */
/// ```
///

/// `PRFileDescStar getFd (in nsIFile zipFile);`
const _GetFd: () = ();

}


