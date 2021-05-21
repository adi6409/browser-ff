//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIDirectoryEnumerator.idl
//


/// `interface nsIDirectoryEnumerator : nsISimpleEnumerator`
///

/// ```text
/// /**
///  * This interface provides a means for enumerating the contents of a directory.
///  * It is similar to nsISimpleEnumerator except the retrieved entries are QI'ed
///  * to nsIFile, and there is a mechanism for closing the directory when the
///  * enumeration is complete.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirectoryEnumerator {
    vtable: *const nsIDirectoryEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirectoryEnumerator.
unsafe impl XpCom for nsIDirectoryEnumerator {
    const IID: nsIID = nsID(0x31f7f4ae, 0x6916, 0x4f2d,
        [0xa8, 0x1e, 0x92, 0x6a, 0x4e, 0x30, 0x22, 0xee]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirectoryEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirectoryEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirectoryEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsIDirectoryEnumerator`.
    fn coerce_from(v: &nsIDirectoryEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirectoryEnumeratorCoerce for nsIDirectoryEnumerator {
    #[inline]
    fn coerce_from(v: &nsIDirectoryEnumerator) -> &Self {
        v
    }
}

impl nsIDirectoryEnumerator {
    /// Cast this `nsIDirectoryEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirectoryEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirectoryEnumerator {
    type Target = nsISimpleEnumerator;
    #[inline]
    fn deref(&self) -> &nsISimpleEnumerator {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISimpleEnumeratorCoerce> nsIDirectoryEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirectoryEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirectoryEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISimpleEnumeratorVTable,

    /* readonly attribute nsIFile nextFile; */
    pub GetNextFile: unsafe extern "system" fn (this: *const nsIDirectoryEnumerator, aNextFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const nsIDirectoryEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirectoryEnumerator {

    /// ```text
    /// /**
    ///    * Retrieves the next file in the sequence. The "nextFile" element is the
    ///    * first element upon the first call. This attribute is null if there is no
    ///    * next element.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIFile nextFile;`
    #[inline]
    pub unsafe fn GetNextFile(&self, aNextFile: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetNextFile)(self, aNextFile)
    }


    /// ```text
    /// /**
    ///    * Closes the directory being enumerated, releasing the system resource.
    ///    * @throws NS_OK if the call succeeded and the directory was closed.
    ///    *         NS_ERROR_FAILURE if the directory close failed.
    ///    *         It is safe to call this function many times.
    ///    */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


}


