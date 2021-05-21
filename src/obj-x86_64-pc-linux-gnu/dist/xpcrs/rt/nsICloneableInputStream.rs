//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsICloneableInputStream.idl
//


/// `interface nsICloneableInputStream : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICloneableInputStream {
    vtable: *const nsICloneableInputStreamVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICloneableInputStream.
unsafe impl XpCom for nsICloneableInputStream {
    const IID: nsIID = nsID(0x8149be1f, 0x44d3, 0x4f14,
        [0x8b, 0x65, 0xa5, 0x7a, 0x5f, 0xbb, 0xeb, 0x97]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICloneableInputStream {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICloneableInputStream.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICloneableInputStreamCoerce {
    /// Cheaply cast a value of this type from a `nsICloneableInputStream`.
    fn coerce_from(v: &nsICloneableInputStream) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICloneableInputStreamCoerce for nsICloneableInputStream {
    #[inline]
    fn coerce_from(v: &nsICloneableInputStream) -> &Self {
        v
    }
}

impl nsICloneableInputStream {
    /// Cast this `nsICloneableInputStream` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICloneableInputStreamCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICloneableInputStream {
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
impl<T: nsISupportsCoerce> nsICloneableInputStreamCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICloneableInputStream) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICloneableInputStream
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICloneableInputStreamVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute boolean cloneable; */
    pub GetCloneable: unsafe extern "system" fn (this: *const nsICloneableInputStream, aCloneable: *mut bool) -> ::nserror::nsresult,

    /* nsIInputStream clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsICloneableInputStream, _retval: *mut *const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICloneableInputStream {


    /// `[infallible] readonly attribute boolean cloneable;`
    #[inline]
    pub unsafe fn GetCloneable(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCloneable)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `nsIInputStream clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


/// `interface nsICloneableInputStreamWithRange : nsICloneableInputStream`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICloneableInputStreamWithRange {
    vtable: *const nsICloneableInputStreamWithRangeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICloneableInputStreamWithRange.
unsafe impl XpCom for nsICloneableInputStreamWithRange {
    const IID: nsIID = nsID(0xece853c3, 0xaded, 0x4cef,
        [0x8f, 0x51, 0x0d, 0x14, 0x93, 0xd6, 0x0b, 0xd5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICloneableInputStreamWithRange {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICloneableInputStreamWithRange.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICloneableInputStreamWithRangeCoerce {
    /// Cheaply cast a value of this type from a `nsICloneableInputStreamWithRange`.
    fn coerce_from(v: &nsICloneableInputStreamWithRange) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICloneableInputStreamWithRangeCoerce for nsICloneableInputStreamWithRange {
    #[inline]
    fn coerce_from(v: &nsICloneableInputStreamWithRange) -> &Self {
        v
    }
}

impl nsICloneableInputStreamWithRange {
    /// Cast this `nsICloneableInputStreamWithRange` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICloneableInputStreamWithRangeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICloneableInputStreamWithRange {
    type Target = nsICloneableInputStream;
    #[inline]
    fn deref(&self) -> &nsICloneableInputStream {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsICloneableInputStreamCoerce> nsICloneableInputStreamWithRangeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICloneableInputStreamWithRange) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICloneableInputStreamWithRange
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICloneableInputStreamWithRangeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsICloneableInputStreamVTable,

    /* nsIInputStream cloneWithRange (in uint64_t start, in uint64_t length); */
    pub CloneWithRange: unsafe extern "system" fn (this: *const nsICloneableInputStreamWithRange, start: uint64_t, length: uint64_t, _retval: *mut *const nsIInputStream) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICloneableInputStreamWithRange {


    /// `nsIInputStream cloneWithRange (in uint64_t start, in uint64_t length);`
    #[inline]
    pub unsafe fn CloneWithRange(&self, start: uint64_t, length: uint64_t, _retval: *mut *const nsIInputStream) -> ::nserror::nsresult {
        ((*self.vtable).CloneWithRange)(self, start, length, _retval)
    }


}


