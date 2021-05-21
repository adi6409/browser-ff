//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierPrefixSet.idl
//


/// `interface nsIUrlClassifierPrefixSet : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierPrefixSet {
    vtable: *const nsIUrlClassifierPrefixSetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierPrefixSet.
unsafe impl XpCom for nsIUrlClassifierPrefixSet {
    const IID: nsIID = nsID(0x3d8579f0, 0x75fa, 0x4e00,
        [0xba, 0x41, 0x38, 0x66, 0x1d, 0x5b, 0x5d, 0x17]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierPrefixSet {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierPrefixSet.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierPrefixSetCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierPrefixSet`.
    fn coerce_from(v: &nsIUrlClassifierPrefixSet) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierPrefixSetCoerce for nsIUrlClassifierPrefixSet {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierPrefixSet) -> &Self {
        v
    }
}

impl nsIUrlClassifierPrefixSet {
    /// Cast this `nsIUrlClassifierPrefixSet` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierPrefixSetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierPrefixSet {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierPrefixSetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierPrefixSet) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierPrefixSet
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierPrefixSetVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in ACString aName); */
    pub Init: unsafe extern "system" fn (this: *const nsIUrlClassifierPrefixSet, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setPrefixes ([array, size_is (aLength), const] in unsigned long aPrefixes, in unsigned long aLength); */
    pub SetPrefixes: unsafe extern "system" fn (this: *const nsIUrlClassifierPrefixSet, aPrefixes: *const u32, aLength: u32) -> ::nserror::nsresult,

    /* void getPrefixes (out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long aPrefixes); */
    pub GetPrefixes: unsafe extern "system" fn (this: *const nsIUrlClassifierPrefixSet, aCount: *mut u32, aPrefixes: *mut *mut u32) -> ::nserror::nsresult,

    /* boolean contains (in unsigned long aPrefix); */
    pub Contains: unsafe extern "system" fn (this: *const nsIUrlClassifierPrefixSet, aPrefix: u32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean isEmpty (); */
    pub IsEmpty: unsafe extern "system" fn (this: *const nsIUrlClassifierPrefixSet, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierPrefixSet {


    /// `void init (in ACString aName);`
    #[inline]
    pub unsafe fn Init(&self, aName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aName)
    }



    /// `void setPrefixes ([array, size_is (aLength), const] in unsigned long aPrefixes, in unsigned long aLength);`
    #[inline]
    pub unsafe fn SetPrefixes(&self, aPrefixes: *const u32, aLength: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPrefixes)(self, aPrefixes, aLength)
    }



    /// `void getPrefixes (out unsigned long aCount, [array, size_is (aCount), retval] out unsigned long aPrefixes);`
    #[inline]
    pub unsafe fn GetPrefixes(&self, aCount: *mut u32, aPrefixes: *mut *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrefixes)(self, aCount, aPrefixes)
    }



    /// `boolean contains (in unsigned long aPrefix);`
    #[inline]
    pub unsafe fn Contains(&self, aPrefix: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Contains)(self, aPrefix, _retval)
    }



    /// `boolean isEmpty ();`
    #[inline]
    pub unsafe fn IsEmpty(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsEmpty)(self, _retval)
    }


}


