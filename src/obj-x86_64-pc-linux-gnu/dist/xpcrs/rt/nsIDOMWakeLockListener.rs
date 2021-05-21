//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/power/nsIDOMWakeLockListener.idl
//


/// `interface nsIDOMMozWakeLockListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMMozWakeLockListener {
    vtable: *const nsIDOMMozWakeLockListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMMozWakeLockListener.
unsafe impl XpCom for nsIDOMMozWakeLockListener {
    const IID: nsIID = nsID(0x4e258af8, 0xcffb, 0x47bc,
        [0xb1, 0x6d, 0xe8, 0x24, 0x12, 0x43, 0x42, 0x6e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMMozWakeLockListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMMozWakeLockListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMMozWakeLockListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMMozWakeLockListener`.
    fn coerce_from(v: &nsIDOMMozWakeLockListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMMozWakeLockListenerCoerce for nsIDOMMozWakeLockListener {
    #[inline]
    fn coerce_from(v: &nsIDOMMozWakeLockListener) -> &Self {
        v
    }
}

impl nsIDOMMozWakeLockListener {
    /// Cast this `nsIDOMMozWakeLockListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMMozWakeLockListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMMozWakeLockListener {
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
impl<T: nsISupportsCoerce> nsIDOMMozWakeLockListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMMozWakeLockListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMMozWakeLockListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMMozWakeLockListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (in AString aTopic, in AString aState); */
    pub Callback: unsafe extern "system" fn (this: *const nsIDOMMozWakeLockListener, aTopic: *const ::nsstring::nsAString, aState: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMMozWakeLockListener {

    /// ```text
    /// /**
    ///    * The callback will be called when a lock topic changes its lock
    ///    * state.
    ///    *
    ///    * Possible states are:
    ///    *
    ///    *  - "unlocked" - nobody holds the wake lock.
    ///    *
    ///    *  - "locked-foreground" - at least one window holds the wake lock,
    ///    *    and it is visible.
    ///    *
    ///    *  - "locked-background" - at least one window holds the wake lock,
    ///    *    but all of them are hidden.
    ///    *
    ///    * @param aTopic The resource name related to the wake lock.
    ///    * @param aState The wake lock state
    ///    */
    /// ```
    ///

    /// `void callback (in AString aTopic, in AString aState);`
    #[inline]
    pub unsafe fn Callback(&self, aTopic: *const ::nsstring::nsAString, aState: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, aTopic, aState)
    }


}


