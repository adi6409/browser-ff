//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/ds/nsIStringEnumerator.idl
//


/// `interface nsIStringEnumeratorBase : nsISupports`
///

/// ```text
/// /**
///  * Used to enumerate over an ordered list of strings.
///  */
/// /**
///  * Base class for C++-implemented string iterators. JS implementors need not
///  * be queryable to it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStringEnumeratorBase {
    vtable: *const nsIStringEnumeratorBaseVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStringEnumeratorBase.
unsafe impl XpCom for nsIStringEnumeratorBase {
    const IID: nsIID = nsID(0xf5213d15, 0xa4d1, 0x4fb7,
        [0x8a, 0x48, 0xd6, 0x9c, 0xcb, 0x7f, 0xb0, 0xeb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStringEnumeratorBase {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStringEnumeratorBase.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStringEnumeratorBaseCoerce {
    /// Cheaply cast a value of this type from a `nsIStringEnumeratorBase`.
    fn coerce_from(v: &nsIStringEnumeratorBase) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStringEnumeratorBaseCoerce for nsIStringEnumeratorBase {
    #[inline]
    fn coerce_from(v: &nsIStringEnumeratorBase) -> &Self {
        v
    }
}

impl nsIStringEnumeratorBase {
    /// Cast this `nsIStringEnumeratorBase` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStringEnumeratorBaseCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStringEnumeratorBase {
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
impl<T: nsISupportsCoerce> nsIStringEnumeratorBaseCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringEnumeratorBase) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStringEnumeratorBase
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStringEnumeratorBaseVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [binaryname(StringIterator),symbol] nsIJSEnumerator iterator (); */
    pub StringIterator: unsafe extern "system" fn (this: *const nsIStringEnumeratorBase, _retval: *mut*const nsIJSEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStringEnumeratorBase {


    /// `[binaryname(StringIterator),symbol] nsIJSEnumerator iterator ();`
    #[inline]
    pub unsafe fn StringIterator(&self, _retval: *mut*const nsIJSEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).StringIterator)(self, _retval)
    }


}


/// `interface nsIStringEnumerator : nsIStringEnumeratorBase`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIStringEnumerator {
    vtable: *const nsIStringEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIStringEnumerator.
unsafe impl XpCom for nsIStringEnumerator {
    const IID: nsIID = nsID(0x50d3ef6c, 0x9380, 0x4f06,
        [0x9f, 0xb2, 0x95, 0x48, 0x8f, 0x7d, 0x14, 0x1c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIStringEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIStringEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIStringEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsIStringEnumerator`.
    fn coerce_from(v: &nsIStringEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIStringEnumeratorCoerce for nsIStringEnumerator {
    #[inline]
    fn coerce_from(v: &nsIStringEnumerator) -> &Self {
        v
    }
}

impl nsIStringEnumerator {
    /// Cast this `nsIStringEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIStringEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIStringEnumerator {
    type Target = nsIStringEnumeratorBase;
    #[inline]
    fn deref(&self) -> &nsIStringEnumeratorBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStringEnumeratorBaseCoerce> nsIStringEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIStringEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIStringEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIStringEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStringEnumeratorBaseVTable,

    /* boolean hasMore (); */
    pub HasMore: unsafe extern "system" fn (this: *const nsIStringEnumerator, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getNext (); */
    pub GetNext: unsafe extern "system" fn (this: *const nsIStringEnumerator, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIStringEnumerator {


    /// `boolean hasMore ();`
    #[inline]
    pub unsafe fn HasMore(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMore)(self, _retval)
    }



    /// `AString getNext ();`
    #[inline]
    pub unsafe fn GetNext(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNext)(self, _retval)
    }


}


/// `interface nsIUTF8StringEnumerator : nsIStringEnumeratorBase`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUTF8StringEnumerator {
    vtable: *const nsIUTF8StringEnumeratorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUTF8StringEnumerator.
unsafe impl XpCom for nsIUTF8StringEnumerator {
    const IID: nsIID = nsID(0x9bdf1010, 0x3695, 0x4907,
        [0x95, 0xed, 0x83, 0xd0, 0x41, 0x0e, 0xc3, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUTF8StringEnumerator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUTF8StringEnumerator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUTF8StringEnumeratorCoerce {
    /// Cheaply cast a value of this type from a `nsIUTF8StringEnumerator`.
    fn coerce_from(v: &nsIUTF8StringEnumerator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUTF8StringEnumeratorCoerce for nsIUTF8StringEnumerator {
    #[inline]
    fn coerce_from(v: &nsIUTF8StringEnumerator) -> &Self {
        v
    }
}

impl nsIUTF8StringEnumerator {
    /// Cast this `nsIUTF8StringEnumerator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUTF8StringEnumeratorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUTF8StringEnumerator {
    type Target = nsIStringEnumeratorBase;
    #[inline]
    fn deref(&self) -> &nsIStringEnumeratorBase {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStringEnumeratorBaseCoerce> nsIUTF8StringEnumeratorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUTF8StringEnumerator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUTF8StringEnumerator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUTF8StringEnumeratorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStringEnumeratorBaseVTable,

    /* boolean hasMore (); */
    pub HasMore: unsafe extern "system" fn (this: *const nsIUTF8StringEnumerator, _retval: *mut bool) -> ::nserror::nsresult,

    /* AUTF8String getNext (); */
    pub GetNext: unsafe extern "system" fn (this: *const nsIUTF8StringEnumerator, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUTF8StringEnumerator {


    /// `boolean hasMore ();`
    #[inline]
    pub unsafe fn HasMore(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasMore)(self, _retval)
    }



    /// `AUTF8String getNext ();`
    #[inline]
    pub unsafe fn GetNext(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNext)(self, _retval)
    }


}


