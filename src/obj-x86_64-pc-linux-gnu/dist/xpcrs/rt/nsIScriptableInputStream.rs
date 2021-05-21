//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIScriptableInputStream.idl
//


/// `interface nsIScriptableInputStream : nsISupports`
///

/// ```text
/// /**
///  * nsIScriptableInputStream provides scriptable access to an nsIInputStream
///  * instance.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptableInputStream {
    vtable: *const nsIScriptableInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptableInputStream.
unsafe impl XpCom for nsIScriptableInputStream {
    const IID: nsIID = nsID(0x3fce9015, 0x472a, 0x4080,
        [0xac, 0x3e, 0xcd, 0x87, 0x5d, 0xbe, 0x36, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptableInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptableInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptableInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptableInputStream`.
    fn coerce_from(v: &nsIScriptableInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptableInputStreamCoerce for nsIScriptableInputStream {
    #[inline]
    fn coerce_from(v: &nsIScriptableInputStream) -> &Self {
        v
    }
}

impl nsIScriptableInputStream {
    /// Cast this `nsIScriptableInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptableInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptableInputStream {
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
impl<T: nsISupportsCoerce> nsIScriptableInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptableInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptableInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptableInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIScriptableInputStream) -> ::nserror::nsresult,

    /* void init (in nsIInputStream aInputStream); */
    pub Init: unsafe extern "system" fn (this: *const nsIScriptableInputStream, aInputStream: *const nsIInputStream) -> ::nserror::nsresult,

    /* unsigned long long available (); */
    pub Available: unsafe extern "system" fn (this: *const nsIScriptableInputStream, _retval: *mut u64) -> ::nserror::nsresult,

    /* string read (in unsigned long aCount); */
    pub Read: unsafe extern "system" fn (this: *const nsIScriptableInputStream, aCount: u32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* ACString readBytes (in unsigned long aCount); */
    pub ReadBytes: unsafe extern "system" fn (this: *const nsIScriptableInputStream, aCount: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptableInputStream {

    /// ```text
    /// /**
    ///      * Closes the stream.
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
    ///      * Wrap the given nsIInputStream with this nsIScriptableInputStream.
    ///      *
    ///      * @param aInputStream parameter providing the stream to wrap
    ///      */
    /// ```
    ///

    /// `void init (in nsIInputStream aInputStream);`
    #[inline]
    pub unsafe fn Init(&self, aInputStream: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aInputStream)
    }


    /// ```text
    /// /**
    ///      * Return the number of bytes currently available in the stream
    ///      *
    ///      * @return the number of bytes
    ///      *
    ///      * @throws NS_BASE_STREAM_CLOSED if called after the stream has been closed
    ///      */
    /// ```
    ///

    /// `unsigned long long available ();`
    #[inline]
    pub unsafe fn Available(&self, _retval: *mut u64) -> ::nserror::nsresult {
        ((*self.vtable).Available)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Read data from the stream.
    ///      *
    ///      * WARNING: If the data contains a null byte, then this method will return
    ///      * a truncated string.
    ///      *
    ///      * @param aCount the maximum number of bytes to read
    ///      *
    ///      * @return the data, which will be an empty string if the stream is at EOF.
    ///      *
    ///      * @throws NS_BASE_STREAM_CLOSED if called after the stream has been closed
    ///      * @throws NS_ERROR_NOT_INITIALIZED if init was not called
    ///      */
    /// ```
    ///

    /// `string read (in unsigned long aCount);`
    #[inline]
    pub unsafe fn Read(&self, aCount: u32, _retval: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).Read)(self, aCount, _retval)
    }


    /// ```text
    /// /**
    ///      * Read data from the stream, including NULL bytes.
    ///      *
    ///      * @param aCount the maximum number of bytes to read.
    ///      *
    ///      * @return the data from the stream, which will be an empty string if EOF
    ///      *         has been reached.
    ///      *
    ///      * @throws NS_BASE_STREAM_WOULD_BLOCK if reading from the input stream
    ///      *         would block the calling thread (non-blocking mode only).
    ///      * @throws NS_ERROR_FAILURE if there are not enough bytes available to read
    ///      *         aCount amount of data.
    ///      */
    /// ```
    ///

    /// `ACString readBytes (in unsigned long aCount);`
    #[inline]
    pub unsafe fn ReadBytes(&self, aCount: u32, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ReadBytes)(self, aCount, _retval)
    }


}


