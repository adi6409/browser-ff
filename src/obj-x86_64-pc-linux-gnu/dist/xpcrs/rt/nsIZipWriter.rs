//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/modules/libjar/zipwriter/nsIZipWriter.idl
//


/// `interface nsIZipWriter : nsISupports`
///

/// ```text
/// /**
///  * nsIZipWriter
///  *
///  * An interface for a zip archiver that can be used from script.
///  *
///  * The interface supports both a synchronous method of archiving data and a
///  * queueing system to allow operations to be prepared then run in sequence
///  * with notification after completion.
///  *
///  * Operations added to the queue do not get performed until performQueue is
///  * called at which point they will be performed in the order that they were
///  * added to the queue.
///  *
///  * Operations performed on the queue will throw any errors out to the
///  * observer.
///  *
///  * An attempt to perform a synchronous operation while the background queue
///  * is in progress will throw NS_ERROR_IN_PROGRESS.
///  *
///  * Entry names should use /'s as path separators and should not start with
///  * a /.
///  *
///  * It is not generally necessary to add directory entries in order to add file
///  * entries within them, however it is possible that some zip programs may
///  * experience problems what that.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIZipWriter {
    vtable: *const nsIZipWriterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIZipWriter.
unsafe impl XpCom for nsIZipWriter {
    const IID: nsIID = nsID(0x3ca10750, 0x797e, 0x4a22,
        [0xbc, 0xfe, 0x66, 0x17, 0x0b, 0x5e, 0x96, 0xdd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIZipWriter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIZipWriter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIZipWriterCoerce {
    /// Cheaply cast a value of this type from a `nsIZipWriter`.
    fn coerce_from(v: &nsIZipWriter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIZipWriterCoerce for nsIZipWriter {
    #[inline]
    fn coerce_from(v: &nsIZipWriter) -> &Self {
        v
    }
}

impl nsIZipWriter {
    /// Cast this `nsIZipWriter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIZipWriterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIZipWriter {
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
impl<T: nsISupportsCoerce> nsIZipWriterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIZipWriter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIZipWriter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIZipWriterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute ACString comment; */
    pub GetComment: unsafe extern "system" fn (this: *const nsIZipWriter, aComment: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString comment; */
    pub SetComment: unsafe extern "system" fn (this: *const nsIZipWriter, aComment: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean inQueue; */
    pub GetInQueue: unsafe extern "system" fn (this: *const nsIZipWriter, aInQueue: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIFile file; */
    pub GetFile: unsafe extern "system" fn (this: *const nsIZipWriter, aFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* void open (in nsIFile aFile, in int32_t aIoFlags); */
    pub Open: unsafe extern "system" fn (this: *const nsIZipWriter, aFile: *const nsIFile, aIoFlags: int32_t) -> ::nserror::nsresult,

    /* nsIZipEntry getEntry (in AUTF8String aZipEntry); */
    pub GetEntry: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, _retval: *mut*const nsIZipEntry) -> ::nserror::nsresult,

    /* boolean hasEntry (in AUTF8String aZipEntry); */
    pub HasEntry: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue); */
    pub AddEntryDirectory: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, aModTime: PRTime, aQueue: bool) -> ::nserror::nsresult,

    /* void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue); */
    pub AddEntryFile: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, aCompression: int32_t, aFile: *const nsIFile, aQueue: bool) -> ::nserror::nsresult,

    /* void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue); */
    pub AddEntryChannel: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, aModTime: PRTime, aCompression: int32_t, aChannel: *const nsIChannel, aQueue: bool) -> ::nserror::nsresult,

    /* void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue); */
    pub AddEntryStream: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, aModTime: PRTime, aCompression: int32_t, aStream: *const nsIInputStream, aQueue: bool) -> ::nserror::nsresult,

    /* void removeEntry (in AUTF8String aZipEntry, in boolean aQueue); */
    pub RemoveEntry: unsafe extern "system" fn (this: *const nsIZipWriter, aZipEntry: *const ::nsstring::nsACString, aQueue: bool) -> ::nserror::nsresult,

    /* void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext); */
    pub ProcessQueue: unsafe extern "system" fn (this: *const nsIZipWriter, aObserver: *const nsIRequestObserver, aContext: *const nsISupports) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIZipWriter) -> ::nserror::nsresult,

    /* void alignStoredFiles (in uint16_t aAlignSize); */
    pub AlignStoredFiles: unsafe extern "system" fn (this: *const nsIZipWriter, aAlignSize: uint16_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIZipWriter {
    /// ```text
    /// /**
    ///    * Some predefined compression levels
    ///    */
    /// ```
    ///

    pub const COMPRESSION_NONE: i64 = 0;


    pub const COMPRESSION_FASTEST: i64 = 1;


    pub const COMPRESSION_DEFAULT: i64 = 6;


    pub const COMPRESSION_BEST: i64 = 9;

    /// ```text
    /// /**
    ///    * Gets or sets the comment associated with the open zip file.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    */
    /// ```
    ///

    /// `attribute ACString comment;`
    #[inline]
    pub unsafe fn GetComment(&self, aComment: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetComment)(self, aComment)
    }


    /// ```text
    /// /**
    ///    * Gets or sets the comment associated with the open zip file.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    */
    /// ```
    ///

    /// `attribute ACString comment;`
    #[inline]
    pub unsafe fn SetComment(&self, aComment: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetComment)(self, aComment)
    }


    /// ```text
    /// /**
    ///    * Indicates that operations on the background queue are being performed.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean inQueue;`
    #[inline]
    pub unsafe fn GetInQueue(&self, aInQueue: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInQueue)(self, aInQueue)
    }


    /// ```text
    /// /**
    ///    * The file that the zipwriter is writing to.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIFile file;`
    #[inline]
    pub unsafe fn GetFile(&self, aFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///    * Opens a zip file.
    ///    *
    ///    * @param aFile the zip file to open
    ///    * @param aIoFlags the open flags for the zip file from prio.h
    ///    *
    ///    * @throws NS_ERROR_ALREADY_INITIALIZED if a zip file is already open
    ///    * @throws NS_ERROR_INVALID_ARG if aFile is null
    ///    * @throws NS_ERROR_FILE_NOT_FOUND if aFile does not exist and flags did
    ///    *  not allow for creation
    ///    * @throws NS_ERROR_FILE_CORRUPTED if the file does not contain zip markers
    ///    * @throws <other-error> on failure to open zip file (most likely corrupt
        ///    *  or unsupported form)
    ///    */
    /// ```
    ///

    /// `void open (in nsIFile aFile, in int32_t aIoFlags);`
    #[inline]
    pub unsafe fn Open(&self, aFile: *const nsIFile, aIoFlags: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, aFile, aIoFlags)
    }


    /// ```text
    /// /**
    ///    * Returns a nsIZipEntry describing a specified zip entry or null if there
    ///    * is no such entry in the zip file
    ///    *
    ///    * @param aZipEntry the path of the entry
    ///    */
    /// ```
    ///

    /// `nsIZipEntry getEntry (in AUTF8String aZipEntry);`
    #[inline]
    pub unsafe fn GetEntry(&self, aZipEntry: *const ::nsstring::nsACString, _retval: *mut*const nsIZipEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetEntry)(self, aZipEntry, _retval)
    }


    /// ```text
    /// /**
    ///    * Checks whether the zipfile contains an entry specified by zipEntry.
    ///    *
    ///    * @param aZipEntry the path of the entry
    ///    */
    /// ```
    ///

    /// `boolean hasEntry (in AUTF8String aZipEntry);`
    #[inline]
    pub unsafe fn HasEntry(&self, aZipEntry: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasEntry)(self, aZipEntry, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds a new directory entry to the zip file. If aZipEntry does not end with
    ///    * "/" then it will be added.
    ///    *
    ///    * @param aZipEntry the path of the directory entry
    ///    * @param aModTime the modification time of the entry in microseconds
    ///    * @param aQueue adds the operation to the background queue. Will be
    ///    *        performed when processQueue is called.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_FILE_ALREADY_EXISTS if the path already exists in the
    ///    *  file
    ///    * @throws NS_ERROR_IN_PROGRESS if another operation is currently in progress
    ///    * @throws NS_ERROR_INVALID_ARG if aModTime is older than 1980-1-1
    ///    */
    /// ```
    ///

    /// `void addEntryDirectory (in AUTF8String aZipEntry, in PRTime aModTime, in boolean aQueue);`
    #[inline]
    pub unsafe fn AddEntryDirectory(&self, aZipEntry: *const ::nsstring::nsACString, aModTime: PRTime, aQueue: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddEntryDirectory)(self, aZipEntry, aModTime, aQueue)
    }


    /// ```text
    /// /**
    ///    * Adds a new file or directory to the zip file. If the specified file is
    ///    * a directory then this will be equivalent to a call to
    ///    * addEntryDirectory(aZipEntry, aFile.lastModifiedTime, aQueue)
    ///    *
    ///    * @param aZipEntry the path of the file entry
    ///    * @param aCompression the compression level, 0 is no compression, 9 is best
    ///    * @param aFile the file to get the data and modification time from
    ///    * @param aQueue adds the operation to the background queue. Will be
    ///    *        performed when processQueue is called.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_FILE_ALREADY_EXISTS if the path already exists in the zip
    ///    * @throws NS_ERROR_IN_PROGRESS if another operation is currently in progress
    ///    * @throws NS_ERROR_FILE_NOT_FOUND if file does not exist
    ///    */
    /// ```
    ///

    /// `void addEntryFile (in AUTF8String aZipEntry, in int32_t aCompression, in nsIFile aFile, in boolean aQueue);`
    #[inline]
    pub unsafe fn AddEntryFile(&self, aZipEntry: *const ::nsstring::nsACString, aCompression: int32_t, aFile: *const nsIFile, aQueue: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddEntryFile)(self, aZipEntry, aCompression, aFile, aQueue)
    }


    /// ```text
    /// /**
    ///    * Adds data from a channel to the zip file. If the operation is performed
    ///    * on the queue then the channel will be opened asynchronously, otherwise
    ///    * the channel must support being opened synchronously.
    ///    *
    ///    * @param aZipEntry the path of the file entry
    ///    * @param aModTime the modification time of the entry in microseconds
    ///    * @param aCompression the compression level, 0 is no compression, 9 is best
    ///    * @param aChannel the channel to get the data from
    ///    * @param aQueue adds the operation to the background queue. Will be
    ///    *        performed when processQueue is called.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_FILE_ALREADY_EXISTS if the path already exists in the zip
    ///    * @throws NS_ERROR_IN_PROGRESS if another operation is currently in progress
    ///    * @throws NS_ERROR_INVALID_ARG if aModTime is older than 1980-1-1
    ///    */
    /// ```
    ///

    /// `void addEntryChannel (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIChannel aChannel, in boolean aQueue);`
    #[inline]
    pub unsafe fn AddEntryChannel(&self, aZipEntry: *const ::nsstring::nsACString, aModTime: PRTime, aCompression: int32_t, aChannel: *const nsIChannel, aQueue: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddEntryChannel)(self, aZipEntry, aModTime, aCompression, aChannel, aQueue)
    }


    /// ```text
    /// /**
    ///    * Adds data from an input stream to the zip file.
    ///    *
    ///    * @param aZipEntry the path of the file entry
    ///    * @param aModTime the modification time of the entry in microseconds
    ///    * @param aCompression the compression level, 0 is no compression, 9 is best
    ///    * @param aStream the input stream to get the data from
    ///    * @param aQueue adds the operation to the background queue. Will be
    ///    *        performed when processQueue is called.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_FILE_ALREADY_EXISTS if the path already exists in the zip
    ///    * @throws NS_ERROR_IN_PROGRESS if another operation is currently in progress
    ///    * @throws NS_ERROR_INVALID_ARG if aModTime is older than 1980-1-1
    ///    */
    /// ```
    ///

    /// `void addEntryStream (in AUTF8String aZipEntry, in PRTime aModTime, in int32_t aCompression, in nsIInputStream aStream, in boolean aQueue);`
    #[inline]
    pub unsafe fn AddEntryStream(&self, aZipEntry: *const ::nsstring::nsACString, aModTime: PRTime, aCompression: int32_t, aStream: *const nsIInputStream, aQueue: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddEntryStream)(self, aZipEntry, aModTime, aCompression, aStream, aQueue)
    }


    /// ```text
    /// /**
    ///    * Removes an existing entry from the zip file.
    ///    *
    ///    * @param aZipEntry the path of the entry to be removed
    ///    * @param aQueue adds the operation to the background queue. Will be
    ///    *        performed when processQueue is called.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_IN_PROGRESS if another operation is currently in progress
    ///    * @throws NS_ERROR_FILE_NOT_FOUND if no entry with the given path exists
    ///    * @throws <other-error> on failure to update the zip file
    ///    */
    /// ```
    ///

    /// `void removeEntry (in AUTF8String aZipEntry, in boolean aQueue);`
    #[inline]
    pub unsafe fn RemoveEntry(&self, aZipEntry: *const ::nsstring::nsACString, aQueue: bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoveEntry)(self, aZipEntry, aQueue)
    }


    /// ```text
    /// /**
    ///    * Processes all queued items until complete or some error occurs. The
    ///    * observer will be notified when the first operation starts and when the
    ///    * last operation completes. Any failures will be passed to the observer.
    ///    * The zip writer will be busy until the queue is complete or some error
    ///    * halted processing of the queue early. In the event of an early failure,
    ///    * remaining items will stay in the queue and calling processQueue will
    ///    * continue.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_IN_PROGRESS if the queue is already in progress
    ///    */
    /// ```
    ///

    /// `void processQueue (in nsIRequestObserver aObserver, in nsISupports aContext);`
    #[inline]
    pub unsafe fn ProcessQueue(&self, aObserver: *const nsIRequestObserver, aContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).ProcessQueue)(self, aObserver, aContext)
    }


    /// ```text
    /// /**
    ///    * Closes the zip file.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED if no zip file has been opened
    ///    * @throws NS_ERROR_IN_PROGRESS if another operation is currently in progress
    ///    * @throws <other-error> on failure to complete the zip file
    ///    */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///    * Make all stored(uncompressed) files align to given alignment size.
    ///    *
    ///    * @param aAlignSize is the alignment size, valid values from 2 to 32768, and
    ///             must be power of 2.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if aAlignSize is invalid
    ///    * @throws <other-error> on failure to update the zip file
    ///    */
    /// ```
    ///

    /// `void alignStoredFiles (in uint16_t aAlignSize);`
    #[inline]
    pub unsafe fn AlignStoredFiles(&self, aAlignSize: uint16_t) -> ::nserror::nsresult {
        ((*self.vtable).AlignStoredFiles)(self, aAlignSize)
    }


}


