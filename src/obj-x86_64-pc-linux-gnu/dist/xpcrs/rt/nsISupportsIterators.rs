//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsISupportsIterators.idl
//


/// `interface nsIOutputIterator : nsISupports`
///

/// ```text
/// /**
///    * ...
///    */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOutputIterator {
    vtable: *const nsIOutputIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOutputIterator.
unsafe impl XpCom for nsIOutputIterator {
    const IID: nsIID = nsID(0x7330650e, 0x1dd2, 0x11b2,
        [0xa0, 0xc2, 0x9f, 0xf8, 0x6e, 0xe9, 0x7b, 0xed]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOutputIterator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOutputIterator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOutputIteratorCoerce {
    /// Cheaply cast a value of this type from a `nsIOutputIterator`.
    fn coerce_from(v: &nsIOutputIterator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOutputIteratorCoerce for nsIOutputIterator {
    #[inline]
    fn coerce_from(v: &nsIOutputIterator) -> &Self {
        v
    }
}

impl nsIOutputIterator {
    /// Cast this `nsIOutputIterator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOutputIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOutputIterator {
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
impl<T: nsISupportsCoerce> nsIOutputIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOutputIterator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOutputIterator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOutputIteratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void putElement (in nsISupports anElementToPut); */
    pub PutElement: unsafe extern "system" fn (this: *const nsIOutputIterator, anElementToPut: *const nsISupports) -> ::nserror::nsresult,

    /* void stepForward (); */
    pub StepForward: unsafe extern "system" fn (this: *const nsIOutputIterator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOutputIterator {

    /// ```text
    /// /**
    ///        * Put |anElementToPut| into the underlying container or sequence at the position currently pointed to by this iterator.
    ///        * The iterator and the underlying container or sequence cooperate to |Release()|
    ///        * the replaced element, if any and if necessary, and to |AddRef()| the new element.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @param anElementToPut the element to place into the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `void putElement (in nsISupports anElementToPut);`
    #[inline]
    pub unsafe fn PutElement(&self, anElementToPut: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).PutElement)(self, anElementToPut)
    }


    /// ```text
    /// /**
    ///        * Advance this iterator to the next position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepForward ();`
    #[inline]
    pub unsafe fn StepForward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepForward)(self, )
    }


}


/// `interface nsIInputIterator : nsISupports`
///

/// ```text
/// /**
///    * ...
///    */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInputIterator {
    vtable: *const nsIInputIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInputIterator.
unsafe impl XpCom for nsIInputIterator {
    const IID: nsIID = nsID(0x85585e12, 0x1dd2, 0x11b2,
        [0xa9, 0x30, 0xf6, 0x92, 0x90, 0x58, 0x26, 0x9a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInputIterator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInputIterator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInputIteratorCoerce {
    /// Cheaply cast a value of this type from a `nsIInputIterator`.
    fn coerce_from(v: &nsIInputIterator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInputIteratorCoerce for nsIInputIterator {
    #[inline]
    fn coerce_from(v: &nsIInputIterator) -> &Self {
        v
    }
}

impl nsIInputIterator {
    /// Cast this `nsIInputIterator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInputIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInputIterator {
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
impl<T: nsISupportsCoerce> nsIInputIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInputIterator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInputIterator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInputIteratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub GetElement: unsafe extern "system" fn (this: *const nsIInputIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void stepForward (); */
    pub StepForward: unsafe extern "system" fn (this: *const nsIInputIterator) -> ::nserror::nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub IsEqualTo: unsafe extern "system" fn (this: *const nsIInputIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsISupports clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsIInputIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInputIterator {

    /// ```text
    /// /**
    ///        * Retrieve (and |AddRef()|) the element this iterator currently points to.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @result a new reference to the element this iterator currently points to (if any)
    ///        */
    /// ```
    ///

    /// `nsISupports getElement ();`
    #[inline]
    pub unsafe fn GetElement(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetElement)(self, _retval)
    }


    /// ```text
    /// /**
    ///        * Advance this iterator to the next position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepForward ();`
    #[inline]
    pub unsafe fn StepForward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepForward)(self, )
    }


    /// ```text
    /// /**
    ///        * Test if |anotherIterator| points to the same position in the underlying container or sequence.
    ///        *
    ///        * The result is undefined if |anotherIterator| was not created by or for the same underlying container or sequence.
    ///        *
    ///        * @param anotherIterator another iterator to compare against, created by or for the same underlying container or sequence
    ///        * @result true if |anotherIterator| points to the same position in the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `boolean isEqualTo (in nsISupports anotherIterator);`
    #[inline]
    pub unsafe fn IsEqualTo(&self, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsEqualTo)(self, anotherIterator, _retval)
    }


    /// ```text
    /// /**
    ///        * Create a new iterator pointing to the same position in the underlying container or sequence to which this iterator currently points.
    ///        * The returned iterator is suitable for use in a subsequent call to |isEqualTo()| against this iterator.
    ///        *
    ///        * @result a new iterator pointing at the same position in the same underlying container or sequence as this iterator
    ///        */
    /// ```
    ///

    /// `nsISupports clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


/// `interface nsIForwardIterator : nsISupports`
///

/// ```text
/// /**
///    * ...
///    */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIForwardIterator {
    vtable: *const nsIForwardIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIForwardIterator.
unsafe impl XpCom for nsIForwardIterator {
    const IID: nsIID = nsID(0x8da01646, 0x1dd2, 0x11b2,
        [0x98, 0xa7, 0xc7, 0x00, 0x90, 0x45, 0xbe, 0x7e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIForwardIterator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIForwardIterator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIForwardIteratorCoerce {
    /// Cheaply cast a value of this type from a `nsIForwardIterator`.
    fn coerce_from(v: &nsIForwardIterator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIForwardIteratorCoerce for nsIForwardIterator {
    #[inline]
    fn coerce_from(v: &nsIForwardIterator) -> &Self {
        v
    }
}

impl nsIForwardIterator {
    /// Cast this `nsIForwardIterator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIForwardIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIForwardIterator {
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
impl<T: nsISupportsCoerce> nsIForwardIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIForwardIterator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIForwardIterator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIForwardIteratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub GetElement: unsafe extern "system" fn (this: *const nsIForwardIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void putElement (in nsISupports anElementToPut); */
    pub PutElement: unsafe extern "system" fn (this: *const nsIForwardIterator, anElementToPut: *const nsISupports) -> ::nserror::nsresult,

    /* void stepForward (); */
    pub StepForward: unsafe extern "system" fn (this: *const nsIForwardIterator) -> ::nserror::nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub IsEqualTo: unsafe extern "system" fn (this: *const nsIForwardIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsISupports clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsIForwardIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIForwardIterator {

    /// ```text
    /// /**
    ///        * Retrieve (and |AddRef()|) the element this iterator currently points to.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @result a new reference to the element this iterator currently points to (if any)
    ///        */
    /// ```
    ///

    /// `nsISupports getElement ();`
    #[inline]
    pub unsafe fn GetElement(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetElement)(self, _retval)
    }


    /// ```text
    /// /**
    ///        * Put |anElementToPut| into the underlying container or sequence at the position currently pointed to by this iterator.
    ///        * The iterator and the underlying container or sequence cooperate to |Release()|
    ///        * the replaced element, if any and if necessary, and to |AddRef()| the new element.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @param anElementToPut the element to place into the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `void putElement (in nsISupports anElementToPut);`
    #[inline]
    pub unsafe fn PutElement(&self, anElementToPut: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).PutElement)(self, anElementToPut)
    }


    /// ```text
    /// /**
    ///        * Advance this iterator to the next position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepForward ();`
    #[inline]
    pub unsafe fn StepForward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepForward)(self, )
    }


    /// ```text
    /// /**
    ///        * Test if |anotherIterator| points to the same position in the underlying container or sequence.
    ///        *
    ///        * The result is undefined if |anotherIterator| was not created by or for the same underlying container or sequence.
    ///        *
    ///        * @param anotherIterator another iterator to compare against, created by or for the same underlying container or sequence
    ///        * @result true if |anotherIterator| points to the same position in the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `boolean isEqualTo (in nsISupports anotherIterator);`
    #[inline]
    pub unsafe fn IsEqualTo(&self, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsEqualTo)(self, anotherIterator, _retval)
    }


    /// ```text
    /// /**
    ///        * Create a new iterator pointing to the same position in the underlying container or sequence to which this iterator currently points.
    ///        * The returned iterator is suitable for use in a subsequent call to |isEqualTo()| against this iterator.
    ///        *
    ///        * @result a new iterator pointing at the same position in the same underlying container or sequence as this iterator
    ///        */
    /// ```
    ///

    /// `nsISupports clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


/// `interface nsIBidirectionalIterator : nsISupports`
///

/// ```text
/// /**
///    * ...
///    */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBidirectionalIterator {
    vtable: *const nsIBidirectionalIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBidirectionalIterator.
unsafe impl XpCom for nsIBidirectionalIterator {
    const IID: nsIID = nsID(0x948defaa, 0x1dd1, 0x11b2,
        [0x89, 0xf6, 0x8c, 0xe8, 0x1f, 0x5e, 0xbd, 0xa9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBidirectionalIterator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBidirectionalIterator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBidirectionalIteratorCoerce {
    /// Cheaply cast a value of this type from a `nsIBidirectionalIterator`.
    fn coerce_from(v: &nsIBidirectionalIterator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBidirectionalIteratorCoerce for nsIBidirectionalIterator {
    #[inline]
    fn coerce_from(v: &nsIBidirectionalIterator) -> &Self {
        v
    }
}

impl nsIBidirectionalIterator {
    /// Cast this `nsIBidirectionalIterator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBidirectionalIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBidirectionalIterator {
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
impl<T: nsISupportsCoerce> nsIBidirectionalIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBidirectionalIterator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBidirectionalIterator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBidirectionalIteratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub GetElement: unsafe extern "system" fn (this: *const nsIBidirectionalIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void putElement (in nsISupports anElementToPut); */
    pub PutElement: unsafe extern "system" fn (this: *const nsIBidirectionalIterator, anElementToPut: *const nsISupports) -> ::nserror::nsresult,

    /* void stepForward (); */
    pub StepForward: unsafe extern "system" fn (this: *const nsIBidirectionalIterator) -> ::nserror::nsresult,

    /* void stepBackward (); */
    pub StepBackward: unsafe extern "system" fn (this: *const nsIBidirectionalIterator) -> ::nserror::nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub IsEqualTo: unsafe extern "system" fn (this: *const nsIBidirectionalIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsISupports clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsIBidirectionalIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBidirectionalIterator {

    /// ```text
    /// /**
    ///        * Retrieve (and |AddRef()|) the element this iterator currently points to.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @result a new reference to the element this iterator currently points to (if any)
    ///        */
    /// ```
    ///

    /// `nsISupports getElement ();`
    #[inline]
    pub unsafe fn GetElement(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetElement)(self, _retval)
    }


    /// ```text
    /// /**
    ///        * Put |anElementToPut| into the underlying container or sequence at the position currently pointed to by this iterator.
    ///        * The iterator and the underlying container or sequence cooperate to |Release()|
    ///        * the replaced element, if any and if necessary, and to |AddRef()| the new element.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @param anElementToPut the element to place into the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `void putElement (in nsISupports anElementToPut);`
    #[inline]
    pub unsafe fn PutElement(&self, anElementToPut: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).PutElement)(self, anElementToPut)
    }


    /// ```text
    /// /**
    ///        * Advance this iterator to the next position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepForward ();`
    #[inline]
    pub unsafe fn StepForward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepForward)(self, )
    }


    /// ```text
    /// /**
    ///        * Move this iterator to the previous position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepBackward ();`
    #[inline]
    pub unsafe fn StepBackward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepBackward)(self, )
    }


    /// ```text
    /// /**
    ///        * Test if |anotherIterator| points to the same position in the underlying container or sequence.
    ///        *
    ///        * The result is undefined if |anotherIterator| was not created by or for the same underlying container or sequence.
    ///        *
    ///        * @param anotherIterator another iterator to compare against, created by or for the same underlying container or sequence
    ///        * @result true if |anotherIterator| points to the same position in the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `boolean isEqualTo (in nsISupports anotherIterator);`
    #[inline]
    pub unsafe fn IsEqualTo(&self, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsEqualTo)(self, anotherIterator, _retval)
    }


    /// ```text
    /// /**
    ///        * Create a new iterator pointing to the same position in the underlying container or sequence to which this iterator currently points.
    ///        * The returned iterator is suitable for use in a subsequent call to |isEqualTo()| against this iterator.
    ///        *
    ///        * @result a new iterator pointing at the same position in the same underlying container or sequence as this iterator
    ///        */
    /// ```
    ///

    /// `nsISupports clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


/// `interface nsIRandomAccessIterator : nsISupports`
///

/// ```text
/// /**
///    * ...
///    */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRandomAccessIterator {
    vtable: *const nsIRandomAccessIteratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRandomAccessIterator.
unsafe impl XpCom for nsIRandomAccessIterator {
    const IID: nsIID = nsID(0x9bd6fdb0, 0x1dd1, 0x11b2,
        [0x91, 0x01, 0xd1, 0x53, 0x75, 0x96, 0x82, 0x30]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRandomAccessIterator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRandomAccessIterator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRandomAccessIteratorCoerce {
    /// Cheaply cast a value of this type from a `nsIRandomAccessIterator`.
    fn coerce_from(v: &nsIRandomAccessIterator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRandomAccessIteratorCoerce for nsIRandomAccessIterator {
    #[inline]
    fn coerce_from(v: &nsIRandomAccessIterator) -> &Self {
        v
    }
}

impl nsIRandomAccessIterator {
    /// Cast this `nsIRandomAccessIterator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRandomAccessIteratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRandomAccessIterator {
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
impl<T: nsISupportsCoerce> nsIRandomAccessIteratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRandomAccessIterator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRandomAccessIterator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRandomAccessIteratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISupports getElement (); */
    pub GetElement: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* nsISupports getElementAt (in int32_t anOffset); */
    pub GetElementAt: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t, _retval: *mut *const nsISupports) -> ::nserror::nsresult,

    /* void putElement (in nsISupports anElementToPut); */
    pub PutElement: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, anElementToPut: *const nsISupports) -> ::nserror::nsresult,

    /* void putElementAt (in int32_t anOffset, in nsISupports anElementToPut); */
    pub PutElementAt: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t, anElementToPut: *const nsISupports) -> ::nserror::nsresult,

    /* void stepForward (); */
    pub StepForward: unsafe extern "system" fn (this: *const nsIRandomAccessIterator) -> ::nserror::nsresult,

    /* void stepForwardBy (in int32_t anOffset); */
    pub StepForwardBy: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t) -> ::nserror::nsresult,

    /* void stepBackward (); */
    pub StepBackward: unsafe extern "system" fn (this: *const nsIRandomAccessIterator) -> ::nserror::nsresult,

    /* void stepBackwardBy (in int32_t anOffset); */
    pub StepBackwardBy: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, anOffset: int32_t) -> ::nserror::nsresult,

    /* boolean isEqualTo (in nsISupports anotherIterator); */
    pub IsEqualTo: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsISupports clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsIRandomAccessIterator, _retval: *mut *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRandomAccessIterator {

    /// ```text
    /// /**
    ///        * Retrieve (and |AddRef()|) the element this iterator currently points to.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @result a new reference to the element this iterator currently points to (if any)
    ///        */
    /// ```
    ///

    /// `nsISupports getElement ();`
    #[inline]
    pub unsafe fn GetElement(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetElement)(self, _retval)
    }


    /// ```text
    /// /**
    ///        * Retrieve (and |AddRef()|) an element at some offset from where this iterator currently points.
    ///        * The offset may be negative.  |getElementAt(0)| is equivalent to |getElement()|.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @param anOffset a |0|-based offset from the position to which this iterator currently points
    ///        * @result a new reference to the indicated element (if any)
    ///        */
    /// ```
    ///

    /// `nsISupports getElementAt (in int32_t anOffset);`
    #[inline]
    pub unsafe fn GetElementAt(&self, anOffset: int32_t, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).GetElementAt)(self, anOffset, _retval)
    }


    /// ```text
    /// /**
    ///        * Put |anElementToPut| into the underlying container or sequence at the position currently pointed to by this iterator.
    ///        * The iterator and the underlying container or sequence cooperate to |Release()|
    ///        * the replaced element, if any and if necessary, and to |AddRef()| the new element.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @param anElementToPut the element to place into the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `void putElement (in nsISupports anElementToPut);`
    #[inline]
    pub unsafe fn PutElement(&self, anElementToPut: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).PutElement)(self, anElementToPut)
    }


    /// ```text
    /// /**
    ///        * Put |anElementToPut| into the underlying container or sequence at the position |anOffset| away from that currently pointed to by this iterator.
    ///        * The iterator and the underlying container or sequence cooperate to |Release()|
    ///        * the replaced element, if any and if necessary, and to |AddRef()| the new element.
    ///        * |putElementAt(0, obj)| is equivalent to |putElement(obj)|.
    ///        *
    ///        * The result is undefined if this iterator currently points outside the
    ///        * useful range of the underlying container or sequence.
    ///        *
    ///        * @param anOffset a |0|-based offset from the position to which this iterator currently points
    ///        * @param anElementToPut the element to place into the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `void putElementAt (in int32_t anOffset, in nsISupports anElementToPut);`
    #[inline]
    pub unsafe fn PutElementAt(&self, anOffset: int32_t, anElementToPut: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).PutElementAt)(self, anOffset, anElementToPut)
    }


    /// ```text
    /// /**
    ///        * Advance this iterator to the next position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepForward ();`
    #[inline]
    pub unsafe fn StepForward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepForward)(self, )
    }


    /// ```text
    /// /**
    ///        * Move this iterator by |anOffset| positions in the underlying container or sequence.
    ///        * |anOffset| may be negative.  |stepForwardBy(1)| is equivalent to |stepForward()|.
    ///        * |stepForwardBy(0)| is a no-op.
    ///        *
    ///        * @param anOffset a |0|-based offset from the position to which this iterator currently points
    ///        */
    /// ```
    ///

    /// `void stepForwardBy (in int32_t anOffset);`
    #[inline]
    pub unsafe fn StepForwardBy(&self, anOffset: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).StepForwardBy)(self, anOffset)
    }


    /// ```text
    /// /**
    ///        * Move this iterator to the previous position in the underlying container or sequence.
    ///        */
    /// ```
    ///

    /// `void stepBackward ();`
    #[inline]
    pub unsafe fn StepBackward(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StepBackward)(self, )
    }


    /// ```text
    /// /**
    ///        * Move this iterator backwards by |anOffset| positions in the underlying container or sequence.
    ///        * |anOffset| may be negative.  |stepBackwardBy(1)| is equivalent to |stepBackward()|.
    ///        * |stepBackwardBy(n)| is equivalent to |stepForwardBy(-n)|.  |stepBackwardBy(0)| is a no-op.
    ///        *
    ///        * @param anOffset a |0|-based offset from the position to which this iterator currently points
    ///        */
    /// ```
    ///

    /// `void stepBackwardBy (in int32_t anOffset);`
    #[inline]
    pub unsafe fn StepBackwardBy(&self, anOffset: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).StepBackwardBy)(self, anOffset)
    }


    /// ```text
    /// /**
    ///        * Test if |anotherIterator| points to the same position in the underlying container or sequence.
    ///        *
    ///        * The result is undefined if |anotherIterator| was not created by or for the same underlying container or sequence.
    ///        *
    ///        * @param anotherIterator another iterator to compare against, created by or for the same underlying container or sequence
    ///        * @result true if |anotherIterator| points to the same position in the underlying container or sequence
    ///        */
    /// ```
    ///

    /// `boolean isEqualTo (in nsISupports anotherIterator);`
    #[inline]
    pub unsafe fn IsEqualTo(&self, anotherIterator: *const nsISupports, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsEqualTo)(self, anotherIterator, _retval)
    }


    /// ```text
    /// /**
    ///        * Create a new iterator pointing to the same position in the underlying container or sequence to which this iterator currently points.
    ///        * The returned iterator is suitable for use in a subsequent call to |isEqualTo()| against this iterator.
    ///        *
    ///        * @result a new iterator pointing at the same position in the same underlying container or sequence as this iterator
    ///        */
    /// ```
    ///

    /// `nsISupports clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


