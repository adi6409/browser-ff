//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageFunction.idl
//


/// `interface mozIStorageFunction : nsISupports`
///

/// ```text
/// /**
///  * mozIStorageFunction is to be implemented by storage consumers that
///  * wish to receive callbacks during the request execution.
///  *
///  * SQL can apply functions to values from tables. Examples of
///  * such functions are MIN(a1,a2) or SQRT(num). Many functions are
///  * implemented in SQL engine.
///  *
///  * This interface allows consumers to implement their own,
///  * problem-specific functions.
///  * These functions can be called from triggers, too.
///  *
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageFunction {
    vtable: *const mozIStorageFunctionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageFunction.
unsafe impl XpCom for mozIStorageFunction {
    const IID: nsIID = nsID(0x9ff02465, 0x21cb, 0x49f3,
        [0xb9, 0x75, 0x7d, 0x5b, 0x38, 0xce, 0xec, 0x73]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageFunction {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageFunction.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageFunctionCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageFunction`.
    fn coerce_from(v: &mozIStorageFunction) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageFunctionCoerce for mozIStorageFunction {
    #[inline]
    fn coerce_from(v: &mozIStorageFunction) -> &Self {
        v
    }
}

impl mozIStorageFunction {
    /// Cast this `mozIStorageFunction` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageFunctionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageFunction {
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
impl<T: nsISupportsCoerce> mozIStorageFunctionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageFunction) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageFunction
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageFunctionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments); */
    pub OnFunctionCall: unsafe extern "system" fn (this: *const mozIStorageFunction, aFunctionArguments: *const mozIStorageValueArray, _retval: *mut*const nsIVariant) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageFunction {

    /// ```text
    /// /**
    ///    * onFunctionCall is called when execution of a custom
    ///    * function should occur.
    ///    *
    ///    * @param aNumArguments         The number of arguments
    ///    * @param aFunctionArguments    The arguments passed in to the function
    ///    *
    ///    * @returns any value as Variant type.
    ///    */
    /// ```
    ///

    /// `nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments);`
    #[inline]
    pub unsafe fn OnFunctionCall(&self, aFunctionArguments: *const mozIStorageValueArray, _retval: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).OnFunctionCall)(self, aFunctionArguments, _retval)
    }


}


