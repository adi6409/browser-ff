//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBindingParamsArray.idl
//


/// `interface mozIStorageBindingParamsArray : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageBindingParamsArray {
    vtable: *const mozIStorageBindingParamsArrayVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageBindingParamsArray.
unsafe impl XpCom for mozIStorageBindingParamsArray {
    const IID: nsIID = nsID(0x67eea5c3, 0x4881, 0x41ff,
        [0xb0, 0xfe, 0x09, 0xf2, 0x35, 0x6a, 0xea, 0xdb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageBindingParamsArray {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageBindingParamsArray.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageBindingParamsArrayCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageBindingParamsArray`.
    fn coerce_from(v: &mozIStorageBindingParamsArray) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageBindingParamsArrayCoerce for mozIStorageBindingParamsArray {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParamsArray) -> &Self {
        v
    }
}

impl mozIStorageBindingParamsArray {
    /// Cast this `mozIStorageBindingParamsArray` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageBindingParamsArrayCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageBindingParamsArray {
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
impl<T: nsISupportsCoerce> mozIStorageBindingParamsArrayCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageBindingParamsArray) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageBindingParamsArray
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageBindingParamsArrayVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* mozIStorageBindingParams newBindingParams (); */
    pub NewBindingParams: unsafe extern "system" fn (this: *const mozIStorageBindingParamsArray, _retval: *mut*const mozIStorageBindingParams) -> ::nserror::nsresult,

    /* void addParams (in mozIStorageBindingParams aParameters); */
    pub AddParams: unsafe extern "system" fn (this: *const mozIStorageBindingParamsArray, aParameters: *const mozIStorageBindingParams) -> ::nserror::nsresult,

    /* readonly attribute unsigned long length; */
    pub GetLength: unsafe extern "system" fn (this: *const mozIStorageBindingParamsArray, aLength: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageBindingParamsArray {

    /// ```text
    /// /**
    ///    * Creates a new mozIStorageBindingParams object that can be added to this
    ///    * array.
    ///    *
    ///    * @return a mozIStorageBindingParams object that can be used to specify
    ///    *         parameters that need to be bound.
    ///    */
    /// ```
    ///

    /// `mozIStorageBindingParams newBindingParams ();`
    #[inline]
    pub unsafe fn NewBindingParams(&self, _retval: *mut*const mozIStorageBindingParams) -> ::nserror::nsresult {
        ((*self.vtable).NewBindingParams)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds the parameters to the end of this array.
    ///    *
    ///    * @param aParameters
    ///    *        The parameters to add to this array.
    ///    */
    /// ```
    ///

    /// `void addParams (in mozIStorageBindingParams aParameters);`
    #[inline]
    pub unsafe fn AddParams(&self, aParameters: *const mozIStorageBindingParams) -> ::nserror::nsresult {
        ((*self.vtable).AddParams)(self, aParameters)
    }


    /// ```text
    /// /**
    ///    * The number of mozIStorageBindingParams this object contains.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }


}


