//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/xre/nsINativeAppSupport.idl
//


/// `interface nsINativeAppSupport : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeAppSupport {
    vtable: *const nsINativeAppSupportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeAppSupport.
unsafe impl XpCom for nsINativeAppSupport {
    const IID: nsIID = nsID(0x5fdf8480, 0x1f98, 0x11d4,
        [0x80, 0x77, 0x00, 0x60, 0x08, 0x11, 0xa9, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeAppSupport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeAppSupport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeAppSupportCoerce {
    /// Cheaply cast a value of this type from a `nsINativeAppSupport`.
    fn coerce_from(v: &nsINativeAppSupport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeAppSupportCoerce for nsINativeAppSupport {
    #[inline]
    fn coerce_from(v: &nsINativeAppSupport) -> &Self {
        v
    }
}

impl nsINativeAppSupport {
    /// Cast this `nsINativeAppSupport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeAppSupportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeAppSupport {
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
impl<T: nsISupportsCoerce> nsINativeAppSupportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeAppSupport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeAppSupport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeAppSupportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean start (); */
    pub Start: unsafe extern "system" fn (this: *const nsINativeAppSupport, _retval: *mut bool) -> ::nserror::nsresult,

    /* void enable (); */
    pub Enable: unsafe extern "system" fn (this: *const nsINativeAppSupport) -> ::nserror::nsresult,

    /* void onLastWindowClosing (); */
    pub OnLastWindowClosing: unsafe extern "system" fn (this: *const nsINativeAppSupport) -> ::nserror::nsresult,

    /* void ReOpen (); */
    pub ReOpen: unsafe extern "system" fn (this: *const nsINativeAppSupport) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeAppSupport {


    /// `boolean start ();`
    #[inline]
    pub unsafe fn Start(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Start)(self, _retval)
    }



    /// `void enable ();`
    #[inline]
    pub unsafe fn Enable(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Enable)(self, )
    }



    /// `void onLastWindowClosing ();`
    #[inline]
    pub unsafe fn OnLastWindowClosing(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnLastWindowClosing)(self, )
    }



    /// `void ReOpen ();`
    #[inline]
    pub unsafe fn ReOpen(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ReOpen)(self, )
    }


}


