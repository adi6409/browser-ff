//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISimpleEnumerator.idl
//


/// `interface nsIJSEnumerator : nsISupports`
///

/// ```text
/// /**
///  * Used to enumerate over elements defined by its implementor.
///  * Although hasMoreElements() can be called independently of getNext(),
///  * getNext() must be pre-ceeded by a call to hasMoreElements(). There is
///  * no way to "reset" an enumerator, once you obtain one.
///  *
///  * @version 1.0
///  */
/// /**
///  * A wrapper for an nsISimpleEnumerator instance which implements the
///  * JavaScript iteration protocol.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIJSEnumerator {
    vtable: *const nsIJSEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIJSEnumerator.
unsafe impl XpCom for nsIJSEnumerator {
    const IID: nsIID = nsID(0x4432e8ae, 0xd4d3, 0x42a6,
        [0xa4, 0xd1, 0x82, 0x9f, 0x1c, 0x29, 0x51, 0x2b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIJSEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIJSEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIJSEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsIJSEnumerator`.
    fn coerce_from(v: &nsIJSEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIJSEnumeratorCoerce for nsIJSEnumerator {
    #[inline]
    fn coerce_from(v: &nsIJSEnumerator) -> &Self {
        v
    }
}

impl nsIJSEnumerator {
    /// Cast this `nsIJSEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIJSEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIJSEnumerator {
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
impl<T: nsISupportsCoerce> nsIJSEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIJSEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIJSEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIJSEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [symbol] nsIJSEnumerator iterator (); */
    pub Iterator: unsafe extern "system" fn (this: *const nsIJSEnumerator, _retval: *mut *const nsIJSEnumerator) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval next (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub Next: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIJSEnumerator {


    /// `[symbol] nsIJSEnumerator iterator ();`
    #[inline]
    pub unsafe fn Iterator(&self, _retval: *mut *const nsIJSEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Iterator)(self, _retval)
    }



    /// `[implicit_jscontext] jsval next ();`
    const _Next: () = ();

}


/// `interface nsISimpleEnumeratorBase : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISimpleEnumeratorBase {
    vtable: *const nsISimpleEnumeratorBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISimpleEnumeratorBase.
unsafe impl XpCom for nsISimpleEnumeratorBase {
    const IID: nsIID = nsID(0x796f340d, 0x0a2a, 0x490b,
        [0x9c, 0x60, 0x64, 0x07, 0x65, 0xe9, 0x97, 0x82]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISimpleEnumeratorBase {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISimpleEnumeratorBase.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISimpleEnumeratorBaseCoerce {
    /// Cheaply cast a value of this type from a `nsISimpleEnumeratorBase`.
    fn coerce_from(v: &nsISimpleEnumeratorBase) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISimpleEnumeratorBaseCoerce for nsISimpleEnumeratorBase {
    #[inline]
    fn coerce_from(v: &nsISimpleEnumeratorBase) -> &Self {
        v
    }
}

impl nsISimpleEnumeratorBase {
    /// Cast this `nsISimpleEnumeratorBase` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISimpleEnumeratorBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISimpleEnumeratorBase {
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
impl<T: nsISupportsCoerce> nsISimpleEnumeratorBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISimpleEnumeratorBase) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISimpleEnumeratorBase
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISimpleEnumeratorBaseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [symbol] nsIJSEnumerator iterator (); */
    pub Iterator: unsafe extern "system" fn (this: *const nsISimpleEnumeratorBase, _retval: *mut *const nsIJSEnumerator) -> ::nserror::nsresult,

    /* nsIJSEnumerator entries (in nsIIDRef aIface); */
    pub Entries: unsafe extern "system" fn (this: *const nsISimpleEnumeratorBase, aIface: *const nsIID, _retval: *mut *const nsIJSEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISimpleEnumeratorBase {

    /// ```text
    /// /**
    ///    * Returns a JavaScript iterator for all remaining entries in the enumerator.
    ///    * Each entry is typically queried to the appropriate interface for the
    ///    * enumerator.
    ///    */
    /// ```
    ///

    /// `[symbol] nsIJSEnumerator iterator ();`
    #[inline]
    pub unsafe fn Iterator(&self, _retval: *mut *const nsIJSEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Iterator)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns JavaScript iterator for all remaining entries in the enumerator.
    ///    * Each entry is queried only to the supplied interface. If any element
    ///    * fails to query to that interface, the error is propagated to the caller.
    ///    */
    /// ```
    ///

    /// `nsIJSEnumerator entries (in nsIIDRef aIface);`
    #[inline]
    pub unsafe fn Entries(&self, aIface: *const nsIID, _retval: *mut *const nsIJSEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).Entries)(self, aIface, _retval)
    }


}


/// `interface nsISimpleEnumerator : nsISimpleEnumeratorBase`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISimpleEnumerator {
    vtable: *const nsISimpleEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISimpleEnumerator.
unsafe impl XpCom for nsISimpleEnumerator {
    const IID: nsIID = nsID(0xd1899240, 0xf9d2, 0x11d2,
        [0xbd, 0xd6, 0x00, 0x00, 0x64, 0x65, 0x73, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISimpleEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISimpleEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISimpleEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsISimpleEnumerator`.
    fn coerce_from(v: &nsISimpleEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISimpleEnumeratorCoerce for nsISimpleEnumerator {
    #[inline]
    fn coerce_from(v: &nsISimpleEnumerator) -> &Self {
        v
    }
}

impl nsISimpleEnumerator {
    /// Cast this `nsISimpleEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISimpleEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISimpleEnumerator {
    type Target = nsISimpleEnumeratorBase;
    #[inline]
    fn deref(&self) -> &nsISimpleEnumeratorBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISimpleEnumeratorBaseCoerce> nsISimpleEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISimpleEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISimpleEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISimpleEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISimpleEnumeratorBaseVTable,

    /* boolean hasMoreElements (); */
    pub HasMoreElements: unsafe extern "system" fn (this: *const nsISimpleEnumerator, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsISupports getNext (); */
    pub GetNext: unsafe extern "system" fn (this: *const nsISimpleEnumerator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISimpleEnumerator {

    /// ```text
    /// /**
    ///    * Called to determine whether or not the enumerator has
    ///    * any elements that can be returned via getNext(). This method
    ///    * is generally used to determine whether or not to initiate or
    ///    * continue iteration over the enumerator, though it can be
    ///    * called without subsequent getNext() calls. Does not affect
    ///    * internal state of enumerator.
    ///    *
    ///    * @see getNext()
    ///    * @return true if there are remaining elements in the enumerator.
    ///    *         false if there are no more elements in the enumerator.
    ///    */
    /// ```
    ///

    /// `boolean hasMoreElements ();`
    #[inline]
    pub unsafe fn HasMoreElements(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMoreElements)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to retrieve the next element in the enumerator. The "next"
    ///    * element is the first element upon the first call. Must be
    ///    * pre-ceeded by a call to hasMoreElements() which returns PR_TRUE.
    ///    * This method is generally called within a loop to iterate over
    ///    * the elements in the enumerator.
    ///    *
    ///    * @see hasMoreElements()
    ///    * @throws NS_ERROR_FAILURE if there are no more elements
    ///    *                          to enumerate.
    ///    * @return the next element in the enumeration.
    ///    */
    /// ```
    ///

    /// `nsISupports getNext ();`
    #[inline]
    pub unsafe fn GetNext(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetNext)(self, _retval)
    }


}


