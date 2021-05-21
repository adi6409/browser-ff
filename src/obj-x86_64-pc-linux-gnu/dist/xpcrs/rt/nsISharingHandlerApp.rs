//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/exthandler/nsISharingHandlerApp.idl
//


/// `interface nsISharingHandlerApp : nsIHandlerApp`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISharingHandlerApp {
    vtable: *const nsISharingHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISharingHandlerApp.
unsafe impl XpCom for nsISharingHandlerApp {
    const IID: nsIID = nsID(0x7111f769, 0x53ec, 0x41fd,
        [0xb3, 0x14, 0x61, 0x36, 0x61, 0xd5, 0xb6, 0xba]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISharingHandlerApp {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISharingHandlerApp.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISharingHandlerAppCoerce {
    /// Cheaply cast a value of this type from a `nsISharingHandlerApp`.
    fn coerce_from(v: &nsISharingHandlerApp) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISharingHandlerAppCoerce for nsISharingHandlerApp {
    #[inline]
    fn coerce_from(v: &nsISharingHandlerApp) -> &Self {
        v
    }
}

impl nsISharingHandlerApp {
    /// Cast this `nsISharingHandlerApp` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISharingHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISharingHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHandlerAppCoerce> nsISharingHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISharingHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISharingHandlerApp
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISharingHandlerAppVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHandlerAppVTable,

    /* void share (in AString data, [optional] in AString title); */
    pub Share: unsafe extern "system" fn (this: *const nsISharingHandlerApp, data: *const ::nsstring::nsAString, title: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISharingHandlerApp {


    /// `void share (in AString data, [optional] in AString title);`
    #[inline]
    pub unsafe fn Share(&self, data: *const ::nsstring::nsAString, title: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Share)(self, data, title)
    }


}


