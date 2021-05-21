//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/simpledb/nsISDBCallbacks.idl
//


/// `interface nsISDBCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISDBCallback {
    vtable: *const nsISDBCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISDBCallback.
unsafe impl XpCom for nsISDBCallback {
    const IID: nsIID = nsID(0x8cbd576c, 0xc6bf, 0x42fd,
        [0x96, 0xee, 0x3b, 0x82, 0x4d, 0xaf, 0xe1, 0xd4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISDBCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISDBCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISDBCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISDBCallback`.
    fn coerce_from(v: &nsISDBCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISDBCallbackCoerce for nsISDBCallback {
    #[inline]
    fn coerce_from(v: &nsISDBCallback) -> &Self {
        v
    }
}

impl nsISDBCallback {
    /// Cast this `nsISDBCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISDBCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISDBCallback {
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
impl<T: nsISupportsCoerce> nsISDBCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISDBCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISDBCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISDBCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onComplete (in nsISDBRequest aRequest); */
    pub OnComplete: unsafe extern "system" fn (this: *const nsISDBCallback, aRequest: *const nsISDBRequest) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISDBCallback {


    /// `void onComplete (in nsISDBRequest aRequest);`
    #[inline]
    pub unsafe fn OnComplete(&self, aRequest: *const nsISDBRequest) -> ::nserror::nsresult {
        ((*self.vtable).OnComplete)(self, aRequest)
    }


}


/// `interface nsISDBCloseCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISDBCloseCallback {
    vtable: *const nsISDBCloseCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISDBCloseCallback.
unsafe impl XpCom for nsISDBCloseCallback {
    const IID: nsIID = nsID(0xe0821d43, 0x62b9, 0x40fe,
        [0x99, 0xf8, 0xff, 0x9a, 0xb3, 0x18, 0x4c, 0xbf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISDBCloseCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISDBCloseCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISDBCloseCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISDBCloseCallback`.
    fn coerce_from(v: &nsISDBCloseCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISDBCloseCallbackCoerce for nsISDBCloseCallback {
    #[inline]
    fn coerce_from(v: &nsISDBCloseCallback) -> &Self {
        v
    }
}

impl nsISDBCloseCallback {
    /// Cast this `nsISDBCloseCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISDBCloseCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISDBCloseCallback {
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
impl<T: nsISupportsCoerce> nsISDBCloseCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISDBCloseCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISDBCloseCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISDBCloseCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onClose (in nsISDBConnection aConnection); */
    pub OnClose: unsafe extern "system" fn (this: *const nsISDBCloseCallback, aConnection: *const nsISDBConnection) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISDBCloseCallback {


    /// `void onClose (in nsISDBConnection aConnection);`
    #[inline]
    pub unsafe fn OnClose(&self, aConnection: *const nsISDBConnection) -> ::nserror::nsresult {
        ((*self.vtable).OnClose)(self, aConnection)
    }


}


