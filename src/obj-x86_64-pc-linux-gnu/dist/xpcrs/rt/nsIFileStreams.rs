//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIFileStreams.idl
//


/// `interface nsIFileInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * An input stream that allows you to read from a file.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileInputStream {
    vtable: *const nsIFileInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileInputStream.
unsafe impl XpCom for nsIFileInputStream {
    const IID: nsIID = nsID(0xe3d56a20, 0xc7ec, 0x11d3,
        [0x8c, 0xda, 0x00, 0x60, 0xb0, 0xfc, 0x14, 0xa3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIFileInputStream`.
    fn coerce_from(v: &nsIFileInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileInputStreamCoerce for nsIFileInputStream {
    #[inline]
    fn coerce_from(v: &nsIFileInputStream) -> &Self {
        v
    }
}

impl nsIFileInputStream {
    /// Cast this `nsIFileInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileInputStream {
    type Target = nsIInputStream;
    #[inline]
    fn deref(&self) -> &nsIInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIInputStreamCoerce> nsIFileInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    pub Init: unsafe extern "system" fn (this: *const nsIFileInputStream, file: *const nsIFile, ioFlags: i32, perm: i32, behaviorFlags: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileInputStream {
    /// ```text
    /// /**
    ///      * If this is set, the file will close automatically when the end of the
    ///      * file is reached.
    ///      */
    /// ```
    ///

    pub const CLOSE_ON_EOF: i64 = 4;

    /// ```text
    /// /**
    ///      * If this is set, the file will be reopened whenever we reach the start of
    ///      * the file, either by doing a Seek(0, NS_SEEK_CUR), or by doing a relative
    ///      * seek that happen to reach the beginning of the file. If the file is
    ///      * already open and the seek occurs, it will happen naturally.  (The file
        ///      * will only be reopened if it is closed for some reason.)
    ///      */
    /// ```
    ///

    pub const REOPEN_ON_REWIND: i64 = 8;

    /// ```text
    /// /**
    ///      * If this is set, the file will be opened (i.e., a call to
        ///      * PR_Open done) only when we do an actual operation on the stream,
    ///      * or more specifically, when one of the following is called:
    ///      *   - Seek
    ///      *   - Tell
    ///      *   - SetEOF
    ///      *   - Available
    ///      *   - Read
    ///      *   - ReadLine
    ///      *
    ///      * DEFER_OPEN is useful if we use the stream on a background
    ///      * thread, so that the opening and possible |stat|ing of the file
    ///      * happens there as well.
    ///      *
    ///      * @note Using this flag results in the file not being opened
    ///      *       during the call to Init.  This means that any errors that might
    ///      *       happen when this flag is not set would happen during the
    ///      *       first read.  Also, the file is not locked when Init is called,
    ///      *       so it might be deleted before we try to read from it.
    ///      */
    /// ```
    ///

    pub const DEFER_OPEN: i64 = 16;

    /// ```text
    /// /**
    ///      * This flag has no effect and is totally ignored on any platform except
    ///      * Windows since this is the default behavior on POSIX systems. On Windows
    ///      * if this flag is set then the stream is opened in a special mode that
    ///      * allows the OS to delete the file from disk just like POSIX.
    ///      */
    /// ```
    ///

    pub const SHARE_DELETE: i64 = 32;

    /// ```text
    /// /**
    ///      * @param file          file to read from
    ///      * @param ioFlags       file open flags listed in prio.h (see
        ///      *                      PR_Open documentation) or -1 to open the
    ///      *                      file in default mode (PR_RDONLY).
    ///      * @param perm          file mode bits listed in prio.h or -1 to
    ///      *                      use the default value (0)
    ///      * @param behaviorFlags flags specifying various behaviors of the class
    ///      *        (see enumerations in the class)
    ///      */
    /// ```
    ///

    /// `void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags);`
    #[inline]
    pub unsafe fn Init(&self, file: *const nsIFile, ioFlags: i32, perm: i32, behaviorFlags: i32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, file, ioFlags, perm, behaviorFlags)
    }


}


/// `interface nsIFileOutputStream : nsIOutputStream`
///

/// ```text
/// /**
///  * An output stream that lets you stream to a file.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileOutputStream {
    vtable: *const nsIFileOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileOutputStream.
unsafe impl XpCom for nsIFileOutputStream {
    const IID: nsIID = nsID(0xe734cac9, 0x1295, 0x4e6f,
        [0x96, 0x84, 0x3a, 0xc4, 0xe1, 0xf9, 0x10, 0x63]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIFileOutputStream`.
    fn coerce_from(v: &nsIFileOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileOutputStreamCoerce for nsIFileOutputStream {
    #[inline]
    fn coerce_from(v: &nsIFileOutputStream) -> &Self {
        v
    }
}

impl nsIFileOutputStream {
    /// Cast this `nsIFileOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileOutputStream {
    type Target = nsIOutputStream;
    #[inline]
    fn deref(&self) -> &nsIOutputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIOutputStreamCoerce> nsIFileOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIOutputStreamVTable,

    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    pub Init: unsafe extern "system" fn (this: *const nsIFileOutputStream, file: *const nsIFile, ioFlags: i32, perm: i32, behaviorFlags: i32) -> ::nserror::nsresult,

    /* [noscript] void preallocate (in long long length); */
    pub Preallocate: unsafe extern "system" fn (this: *const nsIFileOutputStream, length: i64) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileOutputStream {
    /// ```text
    /// /**
    ///      * See the same constant in nsIFileInputStream. The deferred open will
    ///      * be performed when one of the following is called:
    ///      *   - Seek
    ///      *   - Tell
    ///      *   - SetEOF
    ///      *   - Write
    ///      *   - Flush
    ///      *
    ///      * @note Using this flag results in the file not being opened
    ///      *       during the call to Init.  This means that any errors that might
    ///      *       happen when this flag is not set would happen during the
    ///      *       first write, and if the file is to be created, then it will not
    ///      *       appear on the disk until the first write.
    ///      */
    /// ```
    ///

    pub const DEFER_OPEN: i64 = 1;

    /// ```text
    /// /**
    ///      * @param file          file to write to
    ///      * @param ioFlags       file open flags listed in prio.h (see
        ///      *                      PR_Open documentation) or -1 to open the
    ///      *                      file in default mode (PR_WRONLY |
        ///      *                      PR_CREATE_FILE | PR_TRUNCATE)
    ///      * @param perm          file mode bits listed in prio.h or -1 to
    ///      *                      use the default permissions (0664)
    ///      * @param behaviorFlags flags specifying various behaviors of the class
    ///      *        (currently none supported)
    ///      */
    /// ```
    ///

    /// `void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags);`
    #[inline]
    pub unsafe fn Init(&self, file: *const nsIFile, ioFlags: i32, perm: i32, behaviorFlags: i32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, file, ioFlags, perm, behaviorFlags)
    }


    /// ```text
    /// /**
    ///      * @param length        asks the operating system to allocate storage for
    ///      *                      this file of at least |length| bytes long, and
    ///      *                      set the file length to the corresponding size.
    ///      * @throws NS_ERROR_FAILURE if the preallocation fails.
    ///      * @throws NS_ERROR_NOT_INITIALIZED if the file is not opened.
    ///      */
    /// ```
    ///

    /// `[noscript] void preallocate (in long long length);`
    #[inline]
    pub unsafe fn Preallocate(&self, length: i64) -> ::nserror::nsresult {
        ((*self.vtable).Preallocate)(self, length)
    }


}


/// `interface nsIFileStream : nsISupports`
///

/// ```text
/// /**
///  * A stream that allows you to read from a file or stream to a file.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileStream {
    vtable: *const nsIFileStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileStream.
unsafe impl XpCom for nsIFileStream {
    const IID: nsIID = nsID(0x82cf605a, 0x8393, 0x4550,
        [0x83, 0xab, 0x43, 0xcd, 0x55, 0x78, 0xe0, 0x06]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIFileStream`.
    fn coerce_from(v: &nsIFileStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileStreamCoerce for nsIFileStream {
    #[inline]
    fn coerce_from(v: &nsIFileStream) -> &Self {
        v
    }
}

impl nsIFileStream {
    /// Cast this `nsIFileStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileStream {
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
impl<T: nsISupportsCoerce> nsIFileStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags); */
    pub Init: unsafe extern "system" fn (this: *const nsIFileStream, file: *const nsIFile, ioFlags: i32, perm: i32, behaviorFlags: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileStream {
    /// ```text
    /// /**
    ///      * See the same constant in nsIFileInputStream. The deferred open will
    ///      * be performed when one of the following is called:
    ///      *   - Seek
    ///      *   - Tell
    ///      *   - SetEOF
    ///      *   - Available
    ///      *   - Read
    ///      *   - Flush
    ///      *   - Write
    ///      *   - GetSize
    ///      *   - GetLastModified
    ///      *
    ///      * @note Using this flag results in the file not being opened
    ///      *       during the call to Init.  This means that any errors that might
    ///      *       happen when this flag is not set would happen during the
    ///      *       first read or write. The file is not locked when Init is called,
    ///      *       so it might be deleted before we try to read from it and if the
    ///      *       file is to be created, then it will not appear on the disk until
    ///      *       the first write.
    ///      */
    /// ```
    ///

    pub const DEFER_OPEN: i64 = 1;

    /// ```text
    /// /**
    ///      * @param file          file to read from or stream to
    ///      * @param ioFlags       file open flags listed in prio.h (see
        ///      *                      PR_Open documentation) or -1 to open the
    ///      *                      file in default mode (PR_RDWR).
    ///      * @param perm          file mode bits listed in prio.h or -1 to
    ///      *                      use the default value (0)
    ///      * @param behaviorFlags flags specifying various behaviors of the class
    ///      *        (see enumerations in the class)
    ///      */
    /// ```
    ///

    /// `void init (in nsIFile file, in long ioFlags, in long perm, in long behaviorFlags);`
    #[inline]
    pub unsafe fn Init(&self, file: *const nsIFile, ioFlags: i32, perm: i32, behaviorFlags: i32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, file, ioFlags, perm, behaviorFlags)
    }


}


/// `interface nsIFileMetadata : nsISupports`
///

/// ```text
/// /**
///  * An interface that allows you to get some metadata like file size and
///  * file last modified time. These methods and attributes can throw
///  * NS_BASE_STREAM_WOULD_BLOCK in case the informations are not available yet.
///  * If this happens, consider the use of nsIAsyncFileMetadata.
///  *
///  * If using nsIAsyncFileMetadata, you should retrieve any data via this
///  * interface before taking any action that might consume the underlying stream.
///  * For example, once Available(), Read(), or nsIAsyncInputStream::AsyncWait()
///  * are invoked, these methods may return NS_BASE_STREAM_CLOSED.  This will
///  * happen when using RemoteLazyInputStream with an underlying file stream, for
///  * example.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileMetadata {
    vtable: *const nsIFileMetadataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileMetadata.
unsafe impl XpCom for nsIFileMetadata {
    const IID: nsIID = nsID(0x07f679e4, 0x9601, 0x4bd1,
        [0xb5, 0x10, 0xcd, 0x38, 0x52, 0xed, 0xb8, 0x81]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileMetadata {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileMetadata.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileMetadataCoerce {
    /// Cheaply cast a value of this type from a `nsIFileMetadata`.
    fn coerce_from(v: &nsIFileMetadata) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileMetadataCoerce for nsIFileMetadata {
    #[inline]
    fn coerce_from(v: &nsIFileMetadata) -> &Self {
        v
    }
}

impl nsIFileMetadata {
    /// Cast this `nsIFileMetadata` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileMetadataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileMetadata {
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
impl<T: nsISupportsCoerce> nsIFileMetadataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileMetadata) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileMetadata
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileMetadataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long long size; */
    pub GetSize: unsafe extern "system" fn (this: *const nsIFileMetadata, aSize: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long lastModified; */
    pub GetLastModified: unsafe extern "system" fn (this: *const nsIFileMetadata, aLastModified: *mut i64) -> ::nserror::nsresult,

    /* [noscript] PRFileDescPtr getFileDescriptor (); */
    /// Unable to generate binding because `native type PRFileDesc unsupported`
    pub GetFileDescriptor: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileMetadata {

    /// ```text
    /// /**
    ///      * File size in bytes.
    ///      */
    /// ```
    ///

    /// `readonly attribute long long size;`
    #[inline]
    pub unsafe fn GetSize(&self, aSize: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetSize)(self, aSize)
    }


    /// ```text
    /// /**
    ///      * File last modified time in milliseconds from midnight (00:00:00),
    ///      * January 1, 1970 Greenwich Mean Time (GMT).
    ///      */
    /// ```
    ///

    /// `readonly attribute long long lastModified;`
    #[inline]
    pub unsafe fn GetLastModified(&self, aLastModified: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModified)(self, aLastModified)
    }


    /// ```text
    /// /**
    ///      * The internal file descriptor. It can be used for memory mapping of the
    ///      * underlying file. Please use carefully! If this returns
    ///      * NS_BASE_STREAM_WOULD_BLOCK, consider the use of nsIAsyncFileMetadata.
    ///      */
    /// ```
    ///

    /// `[noscript] PRFileDescPtr getFileDescriptor ();`
    const _GetFileDescriptor: () = ();

}


/// `interface nsIAsyncFileMetadata : nsIFileMetadata`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAsyncFileMetadata {
    vtable: *const nsIAsyncFileMetadataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAsyncFileMetadata.
unsafe impl XpCom for nsIAsyncFileMetadata {
    const IID: nsIID = nsID(0xde15b80b, 0x29ba, 0x4b7f,
        [0x92, 0x20, 0xa3, 0xd7, 0x5b, 0x17, 0xae, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAsyncFileMetadata {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAsyncFileMetadata.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAsyncFileMetadataCoerce {
    /// Cheaply cast a value of this type from a `nsIAsyncFileMetadata`.
    fn coerce_from(v: &nsIAsyncFileMetadata) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAsyncFileMetadataCoerce for nsIAsyncFileMetadata {
    #[inline]
    fn coerce_from(v: &nsIAsyncFileMetadata) -> &Self {
        v
    }
}

impl nsIAsyncFileMetadata {
    /// Cast this `nsIAsyncFileMetadata` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAsyncFileMetadataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAsyncFileMetadata {
    type Target = nsIFileMetadata;
    #[inline]
    fn deref(&self) -> &nsIFileMetadata {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIFileMetadataCoerce> nsIAsyncFileMetadataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAsyncFileMetadata) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAsyncFileMetadata
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAsyncFileMetadataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIFileMetadataVTable,

    /* void asyncFileMetadataWait (in nsIFileMetadataCallback aCallback, in nsIEventTarget aEventTarget); */
    pub AsyncFileMetadataWait: unsafe extern "system" fn (this: *const nsIAsyncFileMetadata, aCallback: *const nsIFileMetadataCallback, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAsyncFileMetadata {

    /// ```text
    /// /**
    ///      * Asynchronously wait for the object to be ready.
    ///      *
    ///      * @param aCallback The callback will be used when the stream is ready to
    ///      *                  return File metadata. Use a nullptr to cancel a
    ///      *                  previous operation.
    ///      *
    ///      * @param aEventTarget The event target where aCallback will be executed.
    ///      *                     If aCallback is passed, aEventTarget cannot be null.
    ///      */
    /// ```
    ///

    /// `void asyncFileMetadataWait (in nsIFileMetadataCallback aCallback, in nsIEventTarget aEventTarget);`
    #[inline]
    pub unsafe fn AsyncFileMetadataWait(&self, aCallback: *const nsIFileMetadataCallback, aEventTarget: *const nsIEventTarget) -> ::nserror::nsresult {
        ((*self.vtable).AsyncFileMetadataWait)(self, aCallback, aEventTarget)
    }


}


/// `interface nsIFileMetadataCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFileMetadataCallback {
    vtable: *const nsIFileMetadataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFileMetadataCallback.
unsafe impl XpCom for nsIFileMetadataCallback {
    const IID: nsIID = nsID(0xd01c7ead, 0x7ba3, 0x4726,
        [0xb3, 0x99, 0x61, 0x8e, 0xc8, 0xec, 0x70, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFileMetadataCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFileMetadataCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFileMetadataCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIFileMetadataCallback`.
    fn coerce_from(v: &nsIFileMetadataCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFileMetadataCallbackCoerce for nsIFileMetadataCallback {
    #[inline]
    fn coerce_from(v: &nsIFileMetadataCallback) -> &Self {
        v
    }
}

impl nsIFileMetadataCallback {
    /// Cast this `nsIFileMetadataCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFileMetadataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFileMetadataCallback {
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
impl<T: nsISupportsCoerce> nsIFileMetadataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFileMetadataCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFileMetadataCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFileMetadataCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onFileMetadataReady (in nsIAsyncFileMetadata aObject); */
    pub OnFileMetadataReady: unsafe extern "system" fn (this: *const nsIFileMetadataCallback, aObject: *const nsIAsyncFileMetadata) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFileMetadataCallback {

    /// ```text
    /// /**
    ///  * This is a companion interface for
    ///  * nsIAsyncFileMetadata::asyncFileMetadataWait.
    ///  */
    /// /**
    ///      * Called to indicate that the nsIFileMetadata object is ready.
    ///      */
    /// ```
    ///

    /// `void onFileMetadataReady (in nsIAsyncFileMetadata aObject);`
    #[inline]
    pub unsafe fn OnFileMetadataReady(&self, aObject: *const nsIAsyncFileMetadata) -> ::nserror::nsresult {
        ((*self.vtable).OnFileMetadataReady)(self, aObject)
    }


}


