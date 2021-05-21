//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageRow.idl
//


/// `interface mozIStorageRow : mozIStorageValueArray`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageRow {
    vtable: *const mozIStorageRowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageRow.
unsafe impl XpCom for mozIStorageRow {
    const IID: nsIID = nsID(0x62d1b6bd, 0xcbfe, 0x4f9b,
        [0xae, 0xe1, 0x0e, 0xad, 0x4a, 0xf4, 0xe6, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageRow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageRow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageRowCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageRow`.
    fn coerce_from(v: &mozIStorageRow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageRowCoerce for mozIStorageRow {
    #[inline]
    fn coerce_from(v: &mozIStorageRow) -> &Self {
        v
    }
}

impl mozIStorageRow {
    /// Cast this `mozIStorageRow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageRowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageRow {
    type Target = mozIStorageValueArray;
    #[inline]
    fn deref(&self) -> &mozIStorageValueArray {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: mozIStorageValueArrayCoerce> mozIStorageRowCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageRow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageRow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageRowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: mozIStorageValueArrayVTable,

    /* nsIVariant getResultByIndex (in unsigned long aIndex); */
    pub GetResultByIndex: unsafe extern "system" fn (this: *const mozIStorageRow, aIndex: u32, _retval: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* nsIVariant getResultByName (in AUTF8String aName); */
    pub GetResultByName: unsafe extern "system" fn (this: *const mozIStorageRow, aName: *const ::nsstring::nsACString, _retval: *mut*const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageRow {

    /// ```text
    /// /**
    ///    * Obtains the result of a given column specified by aIndex.
    ///    *
    ///    * @param aIndex
    ///    *        Zero-based index of the result to get from the tuple.
    ///    * @returns the result of the specified column.
    ///    */
    /// ```
    ///

    /// `nsIVariant getResultByIndex (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetResultByIndex(&self, aIndex: u32, _retval: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetResultByIndex)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Obtains the result of a given column specified by aName.
    ///    *
    ///    * @param aName
    ///    *        Name of the result to get from the tuple.
    ///    * @returns the result of the specified column.
    ///    * @note The name of a result column is the value of the "AS" clause for that
    ///    *       column.  If there is no AS clause then the name of the column is
    ///    *       unspecified and may change from one release to the next.
    ///    */
    /// ```
    ///

    /// `nsIVariant getResultByName (in AUTF8String aName);`
    #[inline]
    pub unsafe fn GetResultByName(&self, aName: *const ::nsstring::nsACString, _retval: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetResultByName)(self, aName, _retval)
    }


}


