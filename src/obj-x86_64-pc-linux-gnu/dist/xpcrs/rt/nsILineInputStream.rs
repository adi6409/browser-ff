//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsILineInputStream.idl
//


/// `interface nsILineInputStream : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILineInputStream {
    vtable: *const nsILineInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILineInputStream.
unsafe impl XpCom for nsILineInputStream {
    const IID: nsIID = nsID(0xc97b466c, 0x1e6e, 0x4773,
        [0xa4, 0xab, 0x2b, 0x2b, 0x31, 0x90, 0xa7, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILineInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILineInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILineInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsILineInputStream`.
    fn coerce_from(v: &nsILineInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILineInputStreamCoerce for nsILineInputStream {
    #[inline]
    fn coerce_from(v: &nsILineInputStream) -> &Self {
        v
    }
}

impl nsILineInputStream {
    /// Cast this `nsILineInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILineInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILineInputStream {
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
impl<T: nsISupportsCoerce> nsILineInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILineInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILineInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILineInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean readLine (out ACString aLine); */
    pub ReadLine: unsafe extern "system" fn (this: *const nsILineInputStream, aLine: *mut ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILineInputStream {

    /// ```text
    /// /**
    ///    * Read a single line from the stream, where a line is a
    ///    * possibly zero length sequence of 8bit chars terminated by a
    ///    * CR, LF, CRLF, LFCR, or eof.
    ///    * The line terminator is not returned.
    ///    * @retval false
    ///    *         End of file. This line is the last line of the file
    ///    *         (aLine is valid).
    ///    * @retval true
    ///    *         The file contains further lines.
    ///    * @note Do not mix readLine with other read functions.
    ///    *       Doing so can cause various problems and is not supported.
    ///    */
    /// ```
    ///

    /// `boolean readLine (out ACString aLine);`
    #[inline]
    pub unsafe fn ReadLine(&self, aLine: *mut ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ReadLine)(self, aLine, _retval)
    }


}


