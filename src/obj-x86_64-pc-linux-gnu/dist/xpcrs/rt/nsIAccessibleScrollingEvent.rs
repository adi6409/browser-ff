//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleScrollingEvent.idl
//


/// `interface nsIAccessibleScrollingEvent : nsIAccessibleEvent`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleScrollingEvent {
    vtable: *const nsIAccessibleScrollingEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleScrollingEvent.
unsafe impl XpCom for nsIAccessibleScrollingEvent {
    const IID: nsIID = nsID(0xf75f0b32, 0x5342, 0x4d60,
        [0xb1, 0xa5, 0xb7, 0xbd, 0x68, 0x88, 0xee, 0xf5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleScrollingEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleScrollingEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleScrollingEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleScrollingEvent`.
    fn coerce_from(v: &nsIAccessibleScrollingEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleScrollingEventCoerce for nsIAccessibleScrollingEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleScrollingEvent) -> &Self {
        v
    }
}

impl nsIAccessibleScrollingEvent {
    /// Cast this `nsIAccessibleScrollingEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleScrollingEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleScrollingEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleScrollingEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleScrollingEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleScrollingEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleScrollingEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute unsigned long scrollX; */
    pub GetScrollX: unsafe extern "system" fn (this: *const nsIAccessibleScrollingEvent, aScrollX: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long scrollY; */
    pub GetScrollY: unsafe extern "system" fn (this: *const nsIAccessibleScrollingEvent, aScrollY: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long maxScrollX; */
    pub GetMaxScrollX: unsafe extern "system" fn (this: *const nsIAccessibleScrollingEvent, aMaxScrollX: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long maxScrollY; */
    pub GetMaxScrollY: unsafe extern "system" fn (this: *const nsIAccessibleScrollingEvent, aMaxScrollY: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleScrollingEvent {

    /// ```text
    /// /**
    ///    * New X scroll position within a scrollable container in device pixels.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long scrollX;`
    #[inline]
    pub unsafe fn GetScrollX(&self, aScrollX: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollX)(self, aScrollX)
    }


    /// ```text
    /// /**
    ///    * New Y scroll position within a scrollable container in device pixels.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long scrollY;`
    #[inline]
    pub unsafe fn GetScrollY(&self, aScrollY: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetScrollY)(self, aScrollY)
    }


    /// ```text
    /// /**
    ///   * Max X scroll position within a scrollable container in device pixels.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long maxScrollX;`
    #[inline]
    pub unsafe fn GetMaxScrollX(&self, aMaxScrollX: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxScrollX)(self, aMaxScrollX)
    }


    /// ```text
    /// /**
    ///    * Max Y scroll position within a scrollable container in device pixels.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long maxScrollY;`
    #[inline]
    pub unsafe fn GetMaxScrollY(&self, aMaxScrollY: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxScrollY)(self, aMaxScrollY)
    }


}


