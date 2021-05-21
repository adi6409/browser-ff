//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsISerialEventTarget.idl
//


/// `interface nsISerialEventTarget : nsIEventTarget`
///

/// ```text
/// /**
///  * A serial event target is an event dispatching interface like
///  * nsIEventTarget. Runnables dispatched to an nsISerialEventTarget are required
///  * to execute serially. That is, two different runnables dispatched to the
///  * target should never be allowed to execute simultaneously. One exception to
///  * this rule is nested event loops. If a runnable spins a nested event loop,
///  * causing another runnable dispatched to the target to run, the target may
///  * still be considered "serial".
///  *
///  * Examples:
///  * - nsIThread is a serial event target.
///  * - Thread pools are not serial event targets.
///  * - However, one can "convert" a thread pool into an nsISerialEventTarget
///  *   by putting a TaskQueue in front of it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISerialEventTarget {
    vtable: *const nsISerialEventTargetVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISerialEventTarget.
unsafe impl XpCom for nsISerialEventTarget {
    const IID: nsIID = nsID(0x9f982380, 0x24b4, 0x49f3,
        [0x88, 0xf6, 0x45, 0xe2, 0x95, 0x20, 0x36, 0xc7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISerialEventTarget {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISerialEventTarget.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISerialEventTargetCoerce {
    /// Cheaply cast a value of this type from a `nsISerialEventTarget`.
    fn coerce_from(v: &nsISerialEventTarget) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISerialEventTargetCoerce for nsISerialEventTarget {
    #[inline]
    fn coerce_from(v: &nsISerialEventTarget) -> &Self {
        v
    }
}

impl nsISerialEventTarget {
    /// Cast this `nsISerialEventTarget` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISerialEventTargetCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISerialEventTarget {
    type Target = nsIEventTarget;
    #[inline]
    fn deref(&self) -> &nsIEventTarget {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIEventTargetCoerce> nsISerialEventTargetCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISerialEventTarget) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISerialEventTarget
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISerialEventTargetVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIEventTargetVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISerialEventTarget {


}


