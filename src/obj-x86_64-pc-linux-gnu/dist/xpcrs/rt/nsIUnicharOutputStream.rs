//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIUnicharOutputStream.idl
//


/// `interface nsIUnicharOutputStream : nsISupports`
///

/// ```text
/// /**
///  * An interface that allows writing unicode data.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUnicharOutputStream {
    vtable: *const nsIUnicharOutputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUnicharOutputStream.
unsafe impl XpCom for nsIUnicharOutputStream {
    const IID: nsIID = nsID(0x2d00b1bb, 0x8b21, 0x4a63,
        [0xbc, 0xc6, 0x72, 0x13, 0xf5, 0x13, 0xac, 0x2e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUnicharOutputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUnicharOutputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUnicharOutputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIUnicharOutputStream`.
    fn coerce_from(v: &nsIUnicharOutputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUnicharOutputStreamCoerce for nsIUnicharOutputStream {
    #[inline]
    fn coerce_from(v: &nsIUnicharOutputStream) -> &Self {
        v
    }
}

impl nsIUnicharOutputStream {
    /// Cast this `nsIUnicharOutputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUnicharOutputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUnicharOutputStream {
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
impl<T: nsISupportsCoerce> nsIUnicharOutputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUnicharOutputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUnicharOutputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUnicharOutputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean write (in unsigned long aCount, [array, size_is (aCount), const] in char16_t c); */
    pub Write: unsafe extern "system" fn (this: *const nsIUnicharOutputStream, aCount: u32, c: *const char16_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean writeString (in AString str); */
    pub WriteString: unsafe extern "system" fn (this: *const nsIUnicharOutputStream, str: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void flush (); */
    pub Flush: unsafe extern "system" fn (this: *const nsIUnicharOutputStream) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIUnicharOutputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUnicharOutputStream {

    /// ```text
    /// /**
    ///      * Write a single character to the stream. When writing many characters,
    ///      * prefer the string-taking write method.
    ///      *
    ///      * @retval true The character was written successfully
    ///      * @retval false Not all bytes of the character could be written.
    ///      */
    /// ```
    ///

    /// `boolean write (in unsigned long aCount, [array, size_is (aCount), const] in char16_t c);`
    #[inline]
    pub unsafe fn Write(&self, aCount: u32, c: *const char16_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Write)(self, aCount, c, _retval)
    }


    /// ```text
    /// /**
    ///      * Write a string to the stream.
    ///      *
    ///      * @retval true The string was written successfully
    ///      * @retval false Not all bytes of the string could be written.
    ///      */
    /// ```
    ///

    /// `boolean writeString (in AString str);`
    #[inline]
    pub unsafe fn WriteString(&self, str: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WriteString)(self, str, _retval)
    }


    /// ```text
    /// /**
    ///      * Flush the stream. This finishes the conversion and writes any bytes that
    ///      * finish the current byte sequence.
    ///      *
    ///      * It does NOT flush the underlying stream.
    ///      */
    /// ```
    ///

    /// `void flush ();`
    #[inline]
    pub unsafe fn Flush(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Flush)(self, )
    }


    /// ```text
    /// /**
    ///      * Close the stream and free associated resources. This also closes the
    ///      * underlying stream.
    ///      */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


}


