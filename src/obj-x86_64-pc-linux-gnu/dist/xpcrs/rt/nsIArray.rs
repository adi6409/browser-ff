//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIArray.idl
//


/// `interface nsIArray : nsISupports`
///

/// ```text
/// /**
///  * nsIArray
///  *
///  * An indexed collection of elements. Provides basic functionality for
///  * retrieving elements at a specific position, searching for
///  * elements. Indexes are zero-based, such that the last element in the
///  * array is stored at the index length-1.
///  *
///  * For an array which can be modified, see nsIMutableArray below.
///  *
///  * Neither interface makes any attempt to protect the individual
///  * elements from modification. The convention is that the elements of
///  * the array should not be modified. Documentation within a specific
///  * interface should describe variations from this convention.
///  *
///  * It is also convention that if an interface provides access to an
///  * nsIArray, that the array should not be QueryInterfaced to an
///  * nsIMutableArray for modification. If the interface in question had
///  * intended the array to be modified, it would have returned an
///  * nsIMutableArray!
///  *
///  * null is a valid entry in the array, and as such any nsISupports
///  * parameters may be null, except where noted.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIArray {
    vtable: *const nsIArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIArray.
unsafe impl XpCom for nsIArray {
    const IID: nsIID = nsID(0x114744d9, 0xc369, 0x456e,
        [0xb5, 0x5a, 0x52, 0xfe, 0x52, 0x88, 0x0d, 0x2d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIArray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIArray.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIArrayCoerce {
    /// Cheaply cast a value of this type from a `nsIArray`.
    fn coerce_from(v: &nsIArray) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIArrayCoerce for nsIArray {
    #[inline]
    fn coerce_from(v: &nsIArray) -> &Self {
        v
    }
}

impl nsIArray {
    /// Cast this `nsIArray` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIArray {
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
impl<T: nsISupportsCoerce> nsIArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIArray) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIArray
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIArrayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long length; */
    pub GetLength: unsafe extern "system" fn (this: *const nsIArray, aLength: *mut u32) -> ::nserror::nsresult,

    /* void queryElementAt (in unsigned long index, in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result); */
    pub QueryElementAt: unsafe extern "system" fn (this: *const nsIArray, index: u32, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* unsigned long indexOf (in unsigned long startIndex, in nsISupports element); */
    pub IndexOf: unsafe extern "system" fn (this: *const nsIArray, startIndex: u32, element: *const nsISupports, _retval: *mut u32) -> ::nserror::nsresult,

    /* [binaryname(ScriptedEnumerate),optional_argc] nsISimpleEnumerator enumerate ([optional] in nsIIDRef aElemIID); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub ScriptedEnumerate: *const ::libc::c_void,

    /* [noscript] nsISimpleEnumerator enumerateImpl (in nsIDRef aElemIID); */
    pub EnumerateImpl: unsafe extern "system" fn (this: *const nsIArray, aElemIID: *const nsID, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIArray {

    /// ```text
    /// /**
    ///      * length
    ///      *
    ///      * number of elements in the array.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }


    /// ```text
    /// /**
    ///      * queryElementAt()
    ///      *
    ///      * Retrieve a specific element of the array, and QueryInterface it
    ///      * to the specified interface. null is a valid result for
    ///      * this method, but exceptions are thrown in other circumstances
    ///      *
    ///      * @param index position of element
    ///      * @param uuid the IID of the requested interface
    ///      * @param result the object, QI'd to the requested interface
    ///      *
    ///      * @throws NS_ERROR_NO_INTERFACE when an entry exists at the
    ///      *         specified index, but the requested interface is not
    ///      *         available.
    ///      * @throws NS_ERROR_ILLEGAL_VALUE when index > length-1
    ///      *
    ///      */
    /// ```
    ///

    /// `void queryElementAt (in unsigned long index, in nsIIDRef uuid, [iid_is (uuid), retval] out nsQIResult result);`
    #[inline]
    pub unsafe fn QueryElementAt(&self, index: u32, uuid: *const nsIID, result: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).QueryElementAt)(self, index, uuid, result)
    }


    /// ```text
    /// /**
    ///      * indexOf()
    ///      *
    ///      * Get the position of a specific element. Note that since null is
    ///      * a valid input, exceptions are used to indicate that an element
    ///      * is not found.
    ///      *
    ///      * @param startIndex The initial element to search in the array
    ///      *                   To start at the beginning, use 0 as the
    ///      *                   startIndex
    ///      * @param element    The element you are looking for
    ///      * @returns a number >= startIndex which is the position of the
    ///      *          element in the array.
    ///      * @throws NS_ERROR_FAILURE if the element was not in the array.
    ///      */
    /// ```
    ///

    /// `unsigned long indexOf (in unsigned long startIndex, in nsISupports element);`
    #[inline]
    pub unsafe fn IndexOf(&self, startIndex: u32, element: *const nsISupports, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).IndexOf)(self, startIndex, element, _retval)
    }


    /// ```text
    /// /**
    ///      * enumerate the array
    ///      *
    ///      * @returns a new enumerator positioned at the start of the array
    ///      * @throws NS_ERROR_FAILURE if the array is empty (to make it easy
        ///      *         to detect errors), or NS_ERROR_OUT_OF_MEMORY if out of memory.
    ///      */
    /// ```
    ///

    /// `[binaryname(ScriptedEnumerate),optional_argc] nsISimpleEnumerator enumerate ([optional] in nsIIDRef aElemIID);`
    const _ScriptedEnumerate: () = ();


    /// `[noscript] nsISimpleEnumerator enumerateImpl (in nsIDRef aElemIID);`
    #[inline]
    pub unsafe fn EnumerateImpl(&self, aElemIID: *const nsID, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateImpl)(self, aElemIID, _retval)
    }


}


