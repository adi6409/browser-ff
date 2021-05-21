//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIMessageManager.idl
//


/// `interface nsIMessageSender : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMessageSender {
    vtable: *const nsIMessageSenderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMessageSender.
unsafe impl XpCom for nsIMessageSender {
    const IID: nsIID = nsID(0xbb5d79e4, 0xe73c, 0x45e7,
        [0x96, 0x51, 0x4d, 0x71, 0x8f, 0x4b, 0x99, 0x4c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMessageSender {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMessageSender.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMessageSenderCoerce {
    /// Cheaply cast a value of this type from a `nsIMessageSender`.
    fn coerce_from(v: &nsIMessageSender) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMessageSenderCoerce for nsIMessageSender {
    #[inline]
    fn coerce_from(v: &nsIMessageSender) -> &Self {
        v
    }
}

impl nsIMessageSender {
    /// Cast this `nsIMessageSender` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMessageSenderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMessageSender {
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
impl<T: nsISupportsCoerce> nsIMessageSenderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMessageSender) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMessageSender
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMessageSenderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMessageSender {


}


/// `interface nsIInProcessContentFrameMessageManager : nsIMessageSender`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInProcessContentFrameMessageManager {
    vtable: *const nsIInProcessContentFrameMessageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInProcessContentFrameMessageManager.
unsafe impl XpCom for nsIInProcessContentFrameMessageManager {
    const IID: nsIID = nsID(0xb39a3324, 0xb574, 0x4f85,
        [0x8c, 0xdb, 0x27, 0x4d, 0x04, 0xf8, 0x07, 0xef]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInProcessContentFrameMessageManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInProcessContentFrameMessageManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInProcessContentFrameMessageManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIInProcessContentFrameMessageManager`.
    fn coerce_from(v: &nsIInProcessContentFrameMessageManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInProcessContentFrameMessageManagerCoerce for nsIInProcessContentFrameMessageManager {
    #[inline]
    fn coerce_from(v: &nsIInProcessContentFrameMessageManager) -> &Self {
        v
    }
}

impl nsIInProcessContentFrameMessageManager {
    /// Cast this `nsIInProcessContentFrameMessageManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInProcessContentFrameMessageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInProcessContentFrameMessageManager {
    type Target = nsIMessageSender;
    #[inline]
    fn deref(&self) -> &nsIMessageSender {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIMessageSenderCoerce> nsIInProcessContentFrameMessageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInProcessContentFrameMessageManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInProcessContentFrameMessageManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInProcessContentFrameMessageManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIMessageSenderVTable,

    /* [notxpcom] nsIContent getOwnerContent (); */
    pub GetOwnerContent: unsafe extern "system" fn (this: *const nsIInProcessContentFrameMessageManager) -> *const nsIContent,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInProcessContentFrameMessageManager {


    /// `[notxpcom] nsIContent getOwnerContent ();`
    #[inline]
    pub unsafe fn GetOwnerContent(&self, ) -> *const nsIContent {
        ((*self.vtable).GetOwnerContent)(self, )
    }


}


