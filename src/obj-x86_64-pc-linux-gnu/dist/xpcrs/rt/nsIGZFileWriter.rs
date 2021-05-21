//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIGZFileWriter.idl
//


/// `interface nsIGZFileWriter : nsISupports`
///

/// ```text
/// /**
///  * A simple interface for writing to a .gz file.
///  *
///  * Note that the file that this interface produces has a different format than
///  * what you'd get if you compressed your data as a gzip stream and dumped the
///  * result to a file.
///  *
///  * The standard gunzip tool cannot decompress a raw gzip stream, but can handle
///  * the files produced by this interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGZFileWriter {
    vtable: *const nsIGZFileWriterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGZFileWriter.
unsafe impl XpCom for nsIGZFileWriter {
    const IID: nsIID = nsID(0x6bd5642c, 0x1b90, 0x4499,
        [0xba, 0x4b, 0x19, 0x9f, 0x27, 0xef, 0xab, 0xa5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGZFileWriter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGZFileWriter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGZFileWriterCoerce {
    /// Cheaply cast a value of this type from a `nsIGZFileWriter`.
    fn coerce_from(v: &nsIGZFileWriter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGZFileWriterCoerce for nsIGZFileWriter {
    #[inline]
    fn coerce_from(v: &nsIGZFileWriter) -> &Self {
        v
    }
}

impl nsIGZFileWriter {
    /// Cast this `nsIGZFileWriter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGZFileWriterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGZFileWriter {
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
impl<T: nsISupportsCoerce> nsIGZFileWriterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGZFileWriter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGZFileWriter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGZFileWriterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void init (in nsIFile file); */
    pub Init: unsafe extern "system" fn (this: *const nsIGZFileWriter, file: *const nsIFile) -> ::nserror::nsresult,

    /* [must_use,noscript] void initANSIFileDesc (in FILE file); */
    /// Unable to generate binding because `native type FILE unsupported`
    pub InitANSIFileDesc: *const ::libc::c_void,

    /* [must_use] void write (in AUTF8String str); */
    pub Write: unsafe extern "system" fn (this: *const nsIGZFileWriter, str: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void finish (); */
    pub Finish: unsafe extern "system" fn (this: *const nsIGZFileWriter) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGZFileWriter {

    /// ```text
    /// /**
    ///    * Initialize this object.  We'll write our gzip'ed data to the given file,
    ///    * overwriting its contents if the file exists.
    ///    *
    ///    * init() will return an error if called twice.  It's an error to call any
    ///    * other method on this interface without first calling init().
    ///    */
    /// ```
    ///

    /// `[must_use] void init (in nsIFile file);`
    #[inline]
    pub unsafe fn Init(&self, file: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, file)
    }


    /// ```text
    /// /**
    ///    * Alternate version of init() for use when the file is already opened;
    ///    * e.g., with a FileDescriptor passed over IPC.
    ///    */
    /// ```
    ///

    /// `[must_use,noscript] void initANSIFileDesc (in FILE file);`
    const _InitANSIFileDesc: () = ();

    /// ```text
    /// /**
    ///    * Write the given string to the file.
    ///    */
    /// ```
    ///

    /// `[must_use] void write (in AUTF8String str);`
    #[inline]
    pub unsafe fn Write(&self, str: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Write)(self, str)
    }


    /// ```text
    /// /**
    ///    * Close this nsIGZFileWriter.  Classes implementing nsIGZFileWriter will run
    ///    * this method when the underlying object is destroyed, so it's not strictly
    ///    * necessary to explicitly call it from your code.
    ///    *
    ///    * It's an error to call this method twice, and it's an error to call write()
    ///    * after finish() has been called.
    ///    */
    /// ```
    ///

    /// `void finish ();`
    #[inline]
    pub unsafe fn Finish(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Finish)(self, )
    }


}


