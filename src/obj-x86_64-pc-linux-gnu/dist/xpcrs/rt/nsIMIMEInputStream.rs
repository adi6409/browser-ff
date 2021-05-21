//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIMIMEInputStream.idl
//


/// `interface nsIMIMEInputStream : nsIInputStream`
///

/// ```text
/// /**
///  * The MIME stream separates headers and a datastream. It also allows
///  * automatic creation of the content-length header.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMIMEInputStream {
    vtable: *const nsIMIMEInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMIMEInputStream.
unsafe impl XpCom for nsIMIMEInputStream {
    const IID: nsIID = nsID(0xdcbce63c, 0x1dd1, 0x11b2,
        [0xb9, 0x4d, 0x91, 0xf6, 0xd4, 0x9a, 0x31, 0x61]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMIMEInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMIMEInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMIMEInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsIMIMEInputStream`.
    fn coerce_from(v: &nsIMIMEInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMIMEInputStreamCoerce for nsIMIMEInputStream {
    #[inline]
    fn coerce_from(v: &nsIMIMEInputStream) -> &Self {
        v
    }
}

impl nsIMIMEInputStream {
    /// Cast this `nsIMIMEInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMIMEInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMIMEInputStream {
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
impl<T: nsIInputStreamCoerce> nsIMIMEInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMIMEInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMIMEInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIInputStreamVTable,

    /* void addHeader (in string name, in string value); */
    pub AddHeader: unsafe extern "system" fn (this: *const nsIMIMEInputStream, name: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult,

    /* void visitHeaders (in nsIHttpHeaderVisitor visitor); */
    pub VisitHeaders: unsafe extern "system" fn (this: *const nsIMIMEInputStream, visitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult,

    /* void setData (in nsIInputStream stream); */
    pub SetData: unsafe extern "system" fn (this: *const nsIMIMEInputStream, stream: *const nsIInputStream) -> ::nserror::nsresult,

    /* readonly attribute nsIInputStream data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIMIMEInputStream, aData: *mut *const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMIMEInputStream {

    /// ```text
    /// /**
    ///      * Adds an additional header to the stream on the form "name: value". May
    ///      * not be called once the stream has been started to be read.
    ///      * @param name   name of the header
    ///      * @param value  value of the header
    ///      */
    /// ```
    ///

    /// `void addHeader (in string name, in string value);`
    #[inline]
    pub unsafe fn AddHeader(&self, name: *const libc::c_char, value: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).AddHeader)(self, name, value)
    }


    /// ```text
    /// /**
    ///      * Visits all headers which have been added via addHeader.  Calling
    ///      * addHeader while visiting request headers has undefined behavior.
    ///      *
    ///      * @param aVisitor
    ///      *        The header visitor instance.
    ///      */
    /// ```
    ///

    /// `void visitHeaders (in nsIHttpHeaderVisitor visitor);`
    #[inline]
    pub unsafe fn VisitHeaders(&self, visitor: *const nsIHttpHeaderVisitor) -> ::nserror::nsresult {
        ((*self.vtable).VisitHeaders)(self, visitor)
    }


    /// ```text
    /// /**
    ///      * Sets data-stream. May not be called once the stream has been started
    ///      * to be read.
    ///      * The cursor of the new stream should be located at the beginning of the
    ///      * stream if the implementation of the nsIMIMEInputStream also is used as
    ///      * an nsISeekableStream.
    ///      * @param stream  stream containing the data for the stream
    ///      */
    /// ```
    ///

    /// `void setData (in nsIInputStream stream);`
    #[inline]
    pub unsafe fn SetData(&self, stream: *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).SetData)(self, stream)
    }


    /// ```text
    /// /**
    ///      * Get the wrapped data stream
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIInputStream data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


}


