//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIPipe.idl
//


/// `interface nsIPipe : nsISupports`
///

/// ```text
/// /**
///  * nsIPipe represents an in-process buffer that can be read using nsIInputStream
///  * and written using nsIOutputStream.  The reader and writer of a pipe do not
///  * have to be on the same thread.  As a result, the pipe is an ideal mechanism
///  * to bridge data exchange between two threads.  For example, a worker thread
///  * might write data to a pipe from which the main thread will read.
///  *
///  * Each end of the pipe can be either blocking or non-blocking.  Recall that a
///  * non-blocking stream will return NS_BASE_STREAM_WOULD_BLOCK if it cannot be
///  * read or written to without blocking the calling thread.  For example, if you
///  * try to read from an empty pipe that has not yet been closed, then if that
///  * pipe's input end is non-blocking, then the read call will fail immediately
///  * with NS_BASE_STREAM_WOULD_BLOCK as the error condition.  However, if that
///  * pipe's input end is blocking, then the read call will not return until the
///  * pipe has data or until the pipe is closed.  This example presumes that the
///  * pipe is being filled asynchronously on some background thread.
///  *
///  * The pipe supports nsIAsyncInputStream and nsIAsyncOutputStream, which give
///  * the user of a non-blocking pipe the ability to wait for the pipe to become
///  * ready again.  For example, in the case of an empty non-blocking pipe, the
///  * user can call AsyncWait on the input end of the pipe to be notified when
///  * the pipe has data to read (or when the pipe becomes closed).
///  *
///  * NS_NewPipe2 and NS_NewPipe provide convenient pipe constructors.  In most
///  * cases nsIPipe is not actually used.  It is usually enough to just get
///  * references to the pipe's input and output end.  In which case, the pipe is
///  * automatically closed when the respective pipe ends are released.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPipe {
    vtable: *const nsIPipeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPipe.
unsafe impl XpCom for nsIPipe {
    const IID: nsIID = nsID(0x25d0de93, 0x685e, 0x4ea4,
        [0x95, 0xd3, 0xd8, 0x84, 0xe3, 0x1d, 0xf6, 0x3c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPipe {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPipe.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPipeCoerce {
    /// Cheaply cast a value of this type from a `nsIPipe`.
    fn coerce_from(v: &nsIPipe) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPipeCoerce for nsIPipe {
    #[inline]
    fn coerce_from(v: &nsIPipe) -> &Self {
        v
    }
}

impl nsIPipe {
    /// Cast this `nsIPipe` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPipeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPipe {
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
impl<T: nsISupportsCoerce> nsIPipeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPipe) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPipe
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPipeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount); */
    pub Init: unsafe extern "system" fn (this: *const nsIPipe, nonBlockingInput: bool, nonBlockingOutput: bool, segmentSize: u32, segmentCount: u32) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIAsyncInputStream inputStream; */
    pub GetInputStream: unsafe extern "system" fn (this: *const nsIPipe, aInputStream: *mut*const nsIAsyncInputStream) -> ::nserror::nsresult,

    /* [must_use] readonly attribute nsIAsyncOutputStream outputStream; */
    pub GetOutputStream: unsafe extern "system" fn (this: *const nsIPipe, aOutputStream: *mut*const nsIAsyncOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPipe {

    /// ```text
    /// /**
    ///      * initialize this pipe
    ///      *
    ///      * @param nonBlockingInput
    ///      *        true specifies non-blocking input stream behavior
    ///      * @param nonBlockingOutput
    ///      *        true specifies non-blocking output stream behavior
    ///      * @param segmentSize
    ///      *        specifies the segment size in bytes (pass 0 to use default value)
    ///      * @param segmentCount
    ///      *        specifies the max number of segments (pass 0 to use default
        ///      *        value).   Passing UINT32_MAX here causes the pipe to have
    ///      *        "infinite" space.  This mode can be useful in some cases, but
    ///      *        should always be used with caution.  The default value for this
    ///      *        parameter is a finite value.
    ///      */
    /// ```
    ///

    /// `[must_use] void init (in boolean nonBlockingInput, in boolean nonBlockingOutput, in unsigned long segmentSize, in unsigned long segmentCount);`
    #[inline]
    pub unsafe fn Init(&self, nonBlockingInput: bool, nonBlockingOutput: bool, segmentSize: u32, segmentCount: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, nonBlockingInput, nonBlockingOutput, segmentSize, segmentCount)
    }


    /// ```text
    /// /**
    ///      * The pipe's input end, which also implements nsISearchableInputStream.
    ///      * Getting fails if the pipe hasn't been initialized.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIAsyncInputStream inputStream;`
    #[inline]
    pub unsafe fn GetInputStream(&self, aInputStream: *mut*const nsIAsyncInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetInputStream)(self, aInputStream)
    }


    /// ```text
    /// /**
    ///      * The pipe's output end. Getting fails if the pipe hasn't been
    ///      * initialized.
    ///      */
    /// ```
    ///

    /// `[must_use] readonly attribute nsIAsyncOutputStream outputStream;`
    #[inline]
    pub unsafe fn GetOutputStream(&self, aOutputStream: *mut*const nsIAsyncOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetOutputStream)(self, aOutputStream)
    }


}


/// `interface nsISearchableInputStream : nsISupports`
///

/// ```text
/// /**
///  * XXX this interface doesn't really belong in here.  It is here because
///  * currently nsPipeInputStream is the only implementation of this interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISearchableInputStream {
    vtable: *const nsISearchableInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISearchableInputStream.
unsafe impl XpCom for nsISearchableInputStream {
    const IID: nsIID = nsID(0x8c39ef62, 0xf7c9, 0x11d4,
        [0x98, 0xf5, 0x00, 0x10, 0x83, 0x01, 0x0e, 0x9b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISearchableInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISearchableInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISearchableInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsISearchableInputStream`.
    fn coerce_from(v: &nsISearchableInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISearchableInputStreamCoerce for nsISearchableInputStream {
    #[inline]
    fn coerce_from(v: &nsISearchableInputStream) -> &Self {
        v
    }
}

impl nsISearchableInputStream {
    /// Cast this `nsISearchableInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISearchableInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISearchableInputStream {
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
impl<T: nsISupportsCoerce> nsISearchableInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISearchableInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISearchableInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISearchableInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo); */
    pub Search: unsafe extern "system" fn (this: *const nsISearchableInputStream, forString: *const libc::c_char, ignoreCase: bool, found: *mut bool, offsetSearchedTo: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISearchableInputStream {

    /// ```text
    /// /**
    ///      * Searches for a string in the input stream. Since the stream has a notion
    ///      * of EOF, it is possible that the string may at some time be in the
    ///      * buffer, but is is not currently found up to some offset. Consequently,
    ///      * both the found and not found cases return an offset:
    ///      *    if found, return offset where it was found
    ///      *    if not found, return offset of the first byte not searched
    ///      * In the case the stream is at EOF and the string is not found, the first
    ///      * byte not searched will correspond to the length of the buffer.
    ///      */
    /// ```
    ///

    /// `void search (in string forString, in boolean ignoreCase, out boolean found, out unsigned long offsetSearchedTo);`
    #[inline]
    pub unsafe fn Search(&self, forString: *const libc::c_char, ignoreCase: bool, found: *mut bool, offsetSearchedTo: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).Search)(self, forString, ignoreCase, found, offsetSearchedTo)
    }


}


