//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextSelectionChangeEvent.idl
//


/// `interface nsIAccessibleTextSelectionChangeEvent : nsIAccessibleEvent`
///

/// ```text
/// /**
///  * Fired when the caret changes position in text.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTextSelectionChangeEvent {
    vtable: *const nsIAccessibleTextSelectionChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTextSelectionChangeEvent.
unsafe impl XpCom for nsIAccessibleTextSelectionChangeEvent {
    const IID: nsIID = nsID(0x011f98e2, 0x2beb, 0x4ec3,
        [0x97, 0xa5, 0xf1, 0x54, 0xf6, 0x24, 0xe1, 0x12]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTextSelectionChangeEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTextSelectionChangeEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTextSelectionChangeEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTextSelectionChangeEvent`.
    fn coerce_from(v: &nsIAccessibleTextSelectionChangeEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTextSelectionChangeEventCoerce for nsIAccessibleTextSelectionChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextSelectionChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleTextSelectionChangeEvent {
    /// Cast this `nsIAccessibleTextSelectionChangeEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTextSelectionChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTextSelectionChangeEvent {
    type Target = nsIAccessibleEvent;
    #[inline]
    fn deref(&self) -> &nsIAccessibleEvent {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIAccessibleEventCoerce> nsIAccessibleTextSelectionChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextSelectionChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTextSelectionChangeEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTextSelectionChangeEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute nsIArray selectionRanges; */
    pub GetSelectionRanges: unsafe extern "system" fn (this: *const nsIAccessibleTextSelectionChangeEvent, aSelectionRanges: *mut*const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTextSelectionChangeEvent {

    /// ```text
    /// /**
    ///    * Return an array of disjoint ranges for selected text within the
    ///    * source of this event.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIArray selectionRanges;`
    #[inline]
    pub unsafe fn GetSelectionRanges(&self, aSelectionRanges: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetSelectionRanges)(self, aSelectionRanges)
    }


}


