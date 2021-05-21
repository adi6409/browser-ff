//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIMutableArray.idl
//


/// `interface nsIMutableArray : nsIArrayExtensions`
///

/// ```text
/// /**
///  * nsIMutableArray
///  * A separate set of methods that will act on the array. Consumers of
///  * nsIArray should not QueryInterface to nsIMutableArray unless they
///  * own the array.
///  *
///  * As above, it is legal to add null elements to the array. Note also
///  * that null elements can be created as a side effect of
///  * insertElementAt(). Conversely, if insertElementAt() is never used,
///  * and null elements are never explicitly added to the array, then it
///  * is guaranteed that queryElementAt() will never return a null value.
///  *
///  * Any of these methods may throw NS_ERROR_OUT_OF_MEMORY when the
///  * array must grow to complete the call, but the allocation fails.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMutableArray {
    vtable: *const nsIMutableArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMutableArray.
unsafe impl XpCom for nsIMutableArray {
    const IID: nsIID = nsID(0xaf059da0, 0xc85b, 0x40ec,
        [0xaf, 0x07, 0xae, 0x4b, 0xfd, 0xc1, 0x92, 0xcc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMutableArray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMutableArray.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMutableArrayCoerce {
    /// Cheaply cast a value of this type from a `nsIMutableArray`.
    fn coerce_from(v: &nsIMutableArray) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMutableArrayCoerce for nsIMutableArray {
    #[inline]
    fn coerce_from(v: &nsIMutableArray) -> &Self {
        v
    }
}

impl nsIMutableArray {
    /// Cast this `nsIMutableArray` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMutableArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMutableArray {
    type Target = nsIArrayExtensions;
    #[inline]
    fn deref(&self) -> &nsIArrayExtensions {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIArrayExtensionsCoerce> nsIMutableArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMutableArray) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMutableArray
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMutableArrayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIArrayExtensionsVTable,

    /* void appendElement (in nsISupports element); */
    pub AppendElement: unsafe extern "system" fn (this: *const nsIMutableArray, element: *const nsISupports) -> ::nserror::nsresult,

    /* void removeElementAt (in unsigned long index); */
    pub RemoveElementAt: unsafe extern "system" fn (this: *const nsIMutableArray, index: u32) -> ::nserror::nsresult,

    /* void insertElementAt (in nsISupports element, in unsigned long index); */
    pub InsertElementAt: unsafe extern "system" fn (this: *const nsIMutableArray, element: *const nsISupports, index: u32) -> ::nserror::nsresult,

    /* void replaceElementAt (in nsISupports element, in unsigned long index); */
    pub ReplaceElementAt: unsafe extern "system" fn (this: *const nsIMutableArray, element: *const nsISupports, index: u32) -> ::nserror::nsresult,

    /* void clear (); */
    pub Clear: unsafe extern "system" fn (this: *const nsIMutableArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMutableArray {

    /// ```text
    /// /**
    ///      * appendElement()
    ///      *
    ///      * Append an element at the end of the array.
    ///      *
    ///      * @param element The element to append.
    ///      */
    /// ```
    ///

    /// `void appendElement (in nsISupports element);`
    #[inline]
    pub unsafe fn AppendElement(&self, element: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).AppendElement)(self, element)
    }


    /// ```text
    /// /**
    ///      * removeElementAt()
    ///      *
    ///      * Remove an element at a specific position, moving all elements
    ///      * stored at a higher position down one.
    ///      * To remove a specific element, use indexOf() to find the index
    ///      * first, then call removeElementAt().
    ///      *
    ///      * @param index the position of the item
    ///      *
    ///      */
    /// ```
    ///

    /// `void removeElementAt (in unsigned long index);`
    #[inline]
    pub unsafe fn RemoveElementAt(&self, index: u32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveElementAt)(self, index)
    }


    /// ```text
    /// /**
    ///      * insertElementAt()
    ///      *
    ///      * Insert an element at the given position, moving the element
    ///      * currently located in that position, and all elements in higher
    ///      * position, up by one.
    ///      *
    ///      * @param element The element to insert
    ///      * @param index   The position in the array:
    ///      *                If the position is lower than the current length
    ///      *                of the array, the elements at that position and
    ///      *                onwards are bumped one position up.
    ///      *                If the position is equal to the current length
    ///      *                of the array, the new element is appended.
    ///      *                An index lower than 0 or higher than the current
    ///      *                length of the array is invalid and will be ignored.
    ///      */
    /// ```
    ///

    /// `void insertElementAt (in nsISupports element, in unsigned long index);`
    #[inline]
    pub unsafe fn InsertElementAt(&self, element: *const nsISupports, index: u32) -> ::nserror::nsresult {
        ((*self.vtable).InsertElementAt)(self, element, index)
    }


    /// ```text
    /// /**
    ///      * replaceElementAt()
    ///      *
    ///      * Replace the element at the given position.
    ///      *
    ///      * @param element The new element to insert
    ///      * @param index   The position in the array
    ///      *                If the position is lower than the current length
    ///      *                of the array, an existing element will be replaced.
    ///      *                If the position is equal to the current length
    ///      *                of the array, the new element is appended.
    ///      *                If the position is higher than the current length
    ///      *                of the array, empty elements are appended followed
    ///      *                by the new element at the specified position.
    ///      *                An index lower than 0 is invalid and will be ignored.
    ///      */
    /// ```
    ///

    /// `void replaceElementAt (in nsISupports element, in unsigned long index);`
    #[inline]
    pub unsafe fn ReplaceElementAt(&self, element: *const nsISupports, index: u32) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceElementAt)(self, element, index)
    }


    /// ```text
    /// /**
    ///      * clear()
    ///      *
    ///      * clear the entire array, releasing all stored objects
    ///      */
    /// ```
    ///

    /// `void clear ();`
    #[inline]
    pub unsafe fn Clear(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, )
    }


}


