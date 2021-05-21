//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/image/imgINotificationObserver.idl
//


/// `interface imgINotificationObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct imgINotificationObserver {
    vtable: *const imgINotificationObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for imgINotificationObserver.
unsafe impl XpCom for imgINotificationObserver {
    const IID: nsIID = nsID(0x03da5641, 0xa333, 0x454a,
        [0xa8, 0x59, 0x03, 0x6d, 0x0b, 0xb6, 0x83, 0xb7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for imgINotificationObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from imgINotificationObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait imgINotificationObserverCoerce {
    /// Cheaply cast a value of this type from a `imgINotificationObserver`.
    fn coerce_from(v: &imgINotificationObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl imgINotificationObserverCoerce for imgINotificationObserver {
    #[inline]
    fn coerce_from(v: &imgINotificationObserver) -> &Self {
        v
    }
}

impl imgINotificationObserver {
    /// Cast this `imgINotificationObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: imgINotificationObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for imgINotificationObserver {
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
impl<T: nsISupportsCoerce> imgINotificationObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &imgINotificationObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every imgINotificationObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct imgINotificationObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript,nostdcall,notxpcom] void notify (in imgIRequest aProxy, in long aType, [const] in nsIntRect aRect); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub Notify: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl imgINotificationObserver {

    pub const SIZE_AVAILABLE: i64 = 1;


    pub const FRAME_UPDATE: i64 = 2;


    pub const FRAME_COMPLETE: i64 = 3;


    pub const LOAD_COMPLETE: i64 = 4;


    pub const DECODE_COMPLETE: i64 = 5;


    pub const DISCARD: i64 = 6;


    pub const UNLOCKED_DRAW: i64 = 7;


    pub const IS_ANIMATED: i64 = 8;


    pub const HAS_TRANSPARENCY: i64 = 9;


    /// `[noscript,nostdcall,notxpcom] void notify (in imgIRequest aProxy, in long aType, [const] in nsIntRect aRect);`
    const _Notify: () = ();

}


