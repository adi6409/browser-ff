//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIPaperMargin.idl
//


/// `interface nsIPaperMargin : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPaperMargin {
    vtable: *const nsIPaperMarginVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPaperMargin.
unsafe impl XpCom for nsIPaperMargin {
    const IID: nsIID = nsID(0x0858d1a7, 0xb646, 0x4b15,
        [0xa1, 0xe8, 0x7e, 0xb5, 0xab, 0x57, 0x2d, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPaperMargin {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPaperMargin.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPaperMarginCoerce {
    /// Cheaply cast a value of this type from a `nsIPaperMargin`.
    fn coerce_from(v: &nsIPaperMargin) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPaperMarginCoerce for nsIPaperMargin {
    #[inline]
    fn coerce_from(v: &nsIPaperMargin) -> &Self {
        v
    }
}

impl nsIPaperMargin {
    /// Cast this `nsIPaperMargin` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPaperMarginCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPaperMargin {
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
impl<T: nsISupportsCoerce> nsIPaperMarginCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPaperMargin) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPaperMargin
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPaperMarginVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute double top; */
    pub GetTop: unsafe extern "system" fn (this: *const nsIPaperMargin, aTop: *mut libc::c_double) -> ::nserror::nsresult,

    /* [infallible] readonly attribute double right; */
    pub GetRight: unsafe extern "system" fn (this: *const nsIPaperMargin, aRight: *mut libc::c_double) -> ::nserror::nsresult,

    /* [infallible] readonly attribute double bottom; */
    pub GetBottom: unsafe extern "system" fn (this: *const nsIPaperMargin, aBottom: *mut libc::c_double) -> ::nserror::nsresult,

    /* [infallible] readonly attribute double left; */
    pub GetLeft: unsafe extern "system" fn (this: *const nsIPaperMargin, aLeft: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPaperMargin {


    /// `[infallible] readonly attribute double top;`
    #[inline]
    pub unsafe fn GetTop(&self) -> libc::c_double {
        let mut result = <libc::c_double as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetTop)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute double right;`
    #[inline]
    pub unsafe fn GetRight(&self) -> libc::c_double {
        let mut result = <libc::c_double as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetRight)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute double bottom;`
    #[inline]
    pub unsafe fn GetBottom(&self) -> libc::c_double {
        let mut result = <libc::c_double as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetBottom)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute double left;`
    #[inline]
    pub unsafe fn GetLeft(&self) -> libc::c_double {
        let mut result = <libc::c_double as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLeft)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


}


