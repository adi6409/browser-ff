//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIRegion.idl
//


/// `interface nsIRegion : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRegion {
    vtable: *const nsIRegionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRegion.
unsafe impl XpCom for nsIRegion {
    const IID: nsIID = nsID(0x21e6d094, 0xe016, 0x41a4,
        [0x80, 0xcd, 0x76, 0xd2, 0xe2, 0x08, 0x71, 0xaa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRegion {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRegion.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRegionCoerce {
    /// Cheaply cast a value of this type from a `nsIRegion`.
    fn coerce_from(v: &nsIRegion) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRegionCoerce for nsIRegion {
    #[inline]
    fn coerce_from(v: &nsIRegion) -> &Self {
        v
    }
}

impl nsIRegion {
    /// Cast this `nsIRegion` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRegionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRegion {
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
impl<T: nsISupportsCoerce> nsIRegionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRegion) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRegion
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRegionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString current; */
    pub GetCurrent: unsafe extern "system" fn (this: *const nsIRegion, aCurrent: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString home; */
    pub GetHome: unsafe extern "system" fn (this: *const nsIRegion, aHome: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRegion {

    /// ```text
    /// /**
    ///    * The users current region.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString current;`
    #[inline]
    pub unsafe fn GetCurrent(&self, aCurrent: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrent)(self, aCurrent)
    }


    /// ```text
    /// /**
    ///    * The users current home region.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString home;`
    #[inline]
    pub unsafe fn GetHome(&self, aHome: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetHome)(self, aHome)
    }


}


