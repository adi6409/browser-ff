//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIHapticFeedback.idl
//


/// `interface nsIHapticFeedback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHapticFeedback {
    vtable: *const nsIHapticFeedbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHapticFeedback.
unsafe impl XpCom for nsIHapticFeedback {
    const IID: nsIID = nsID(0x91917c98, 0xa8f3, 0x4c98,
        [0x8f, 0x10, 0x4a, 0xfb, 0x87, 0x2f, 0x54, 0xc7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHapticFeedback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHapticFeedback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHapticFeedbackCoerce {
    /// Cheaply cast a value of this type from a `nsIHapticFeedback`.
    fn coerce_from(v: &nsIHapticFeedback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHapticFeedbackCoerce for nsIHapticFeedback {
    #[inline]
    fn coerce_from(v: &nsIHapticFeedback) -> &Self {
        v
    }
}

impl nsIHapticFeedback {
    /// Cast this `nsIHapticFeedback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHapticFeedbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHapticFeedback {
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
impl<T: nsISupportsCoerce> nsIHapticFeedbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHapticFeedback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHapticFeedback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHapticFeedbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void performSimpleAction (in long isLongPress); */
    pub PerformSimpleAction: unsafe extern "system" fn (this: *const nsIHapticFeedback, isLongPress: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHapticFeedback {

    pub const ShortPress: i64 = 0;


    pub const LongPress: i64 = 1;

    /// ```text
    /// /**
    ///      * Perform haptic feedback
    ///      *
    ///      * @param isLongPress
    ///      *        indicate whether feedback is for a long press (vs. short press)
    ///      */
    /// ```
    ///

    /// `void performSimpleAction (in long isLongPress);`
    #[inline]
    pub unsafe fn PerformSimpleAction(&self, isLongPress: i32) -> ::nserror::nsresult {
        ((*self.vtable).PerformSimpleAction)(self, isLongPress)
    }


}


