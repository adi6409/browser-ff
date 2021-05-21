//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIScreenManager.idl
//


/// `interface nsIScreenManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScreenManager {
    vtable: *const nsIScreenManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScreenManager.
unsafe impl XpCom for nsIScreenManager {
    const IID: nsIID = nsID(0xe8a96e60, 0x6b61, 0x4a14,
        [0xba, 0xcc, 0x53, 0x89, 0x16, 0x04, 0xb5, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScreenManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScreenManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScreenManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIScreenManager`.
    fn coerce_from(v: &nsIScreenManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScreenManagerCoerce for nsIScreenManager {
    #[inline]
    fn coerce_from(v: &nsIScreenManager) -> &Self {
        v
    }
}

impl nsIScreenManager {
    /// Cast this `nsIScreenManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScreenManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScreenManager {
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
impl<T: nsISupportsCoerce> nsIScreenManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScreenManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScreenManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScreenManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIScreen screenForRect (in long left, in long top, in long width, in long height); */
    pub ScreenForRect: unsafe extern "system" fn (this: *const nsIScreenManager, left: i32, top: i32, width: i32, height: i32, _retval: *mut *const nsIScreen) -> ::nserror::nsresult,

    /* readonly attribute nsIScreen primaryScreen; */
    pub GetPrimaryScreen: unsafe extern "system" fn (this: *const nsIScreenManager, aPrimaryScreen: *mut *const nsIScreen) -> ::nserror::nsresult,

    /* readonly attribute int64_t totalScreenPixels; */
    pub GetTotalScreenPixels: unsafe extern "system" fn (this: *const nsIScreenManager, aTotalScreenPixels: *mut int64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScreenManager {


    /// `nsIScreen screenForRect (in long left, in long top, in long width, in long height);`
    #[inline]
    pub unsafe fn ScreenForRect(&self, left: i32, top: i32, width: i32, height: i32, _retval: *mut *const nsIScreen) -> ::nserror::nsresult {
        ((*self.vtable).ScreenForRect)(self, left, top, width, height, _retval)
    }



    /// `readonly attribute nsIScreen primaryScreen;`
    #[inline]
    pub unsafe fn GetPrimaryScreen(&self, aPrimaryScreen: *mut *const nsIScreen) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryScreen)(self, aPrimaryScreen)
    }



    /// `readonly attribute int64_t totalScreenPixels;`
    #[inline]
    pub unsafe fn GetTotalScreenPixels(&self, aTotalScreenPixels: *mut int64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTotalScreenPixels)(self, aTotalScreenPixels)
    }


}


