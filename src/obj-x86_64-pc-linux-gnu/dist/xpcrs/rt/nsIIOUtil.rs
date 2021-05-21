//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIIOUtil.idl
//


/// `interface nsIIOUtil : nsISupports`
///

/// ```text
/// /**
///  * nsIIOUtil provdes various xpcom/io-related utility methods.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIIOUtil {
    vtable: *const nsIIOUtilVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIIOUtil.
unsafe impl XpCom for nsIIOUtil {
    const IID: nsIID = nsID(0xe8152f7f, 0x4209, 0x4c63,
        [0xad, 0x23, 0xc3, 0xd2, 0xaa, 0x0c, 0x5a, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIIOUtil {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIIOUtil.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIIOUtilCoerce {
    /// Cheaply cast a value of this type from a `nsIIOUtil`.
    fn coerce_from(v: &nsIIOUtil) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIIOUtilCoerce for nsIIOUtil {
    #[inline]
    fn coerce_from(v: &nsIIOUtil) -> &Self {
        v
    }
}

impl nsIIOUtil {
    /// Cast this `nsIIOUtil` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIIOUtilCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIIOUtil {
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
impl<T: nsISupportsCoerce> nsIIOUtilCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIIOUtil) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIIOUtil
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIIOUtilVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean inputStreamIsBuffered (in nsIInputStream aStream); */
    pub InputStreamIsBuffered: unsafe extern "system" fn (this: *const nsIIOUtil, aStream: *const nsIInputStream, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean outputStreamIsBuffered (in nsIOutputStream aStream); */
    pub OutputStreamIsBuffered: unsafe extern "system" fn (this: *const nsIIOUtil, aStream: *const nsIOutputStream, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIIOUtil {

    /// ```text
    /// /**
    ///    * Test whether an input stream is buffered.  See nsStreamUtils.h
    ///    * documentation for NS_InputStreamIsBuffered for the definition of
    ///    * "buffered" used here and for edge-case behavior.
    ///    *
    ///    * @throws NS_ERROR_INVALID_POINTER if null is passed in.
    ///    */
    /// ```
    ///

    /// `boolean inputStreamIsBuffered (in nsIInputStream aStream);`
    #[inline]
    pub unsafe fn InputStreamIsBuffered(&self, aStream: *const nsIInputStream, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).InputStreamIsBuffered)(self, aStream, _retval)
    }


    /// ```text
    /// /**
    ///    * Test whether an output stream is buffered.  See nsStreamUtils.h
    ///    * documentation for NS_OutputStreamIsBuffered for the definition of
    ///    * "buffered" used here and for edge-case behavior.
    ///    *
    ///    * @throws NS_ERROR_INVALID_POINTER if null is passed in.
    ///    */
    /// ```
    ///

    /// `boolean outputStreamIsBuffered (in nsIOutputStream aStream);`
    #[inline]
    pub unsafe fn OutputStreamIsBuffered(&self, aStream: *const nsIOutputStream, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OutputStreamIsBuffered)(self, aStream, _retval)
    }


}


