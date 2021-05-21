//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleCaretMoveEvent.idl
//


/// `interface nsIAccessibleCaretMoveEvent : nsIAccessibleEvent`
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
pub struct nsIAccessibleCaretMoveEvent {
    vtable: *const nsIAccessibleCaretMoveEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleCaretMoveEvent.
unsafe impl XpCom for nsIAccessibleCaretMoveEvent {
    const IID: nsIID = nsID(0xed1982e4, 0x57d7, 0x41a8,
        [0x8c, 0xd8, 0x90, 0x23, 0xf8, 0x09, 0x38, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleCaretMoveEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleCaretMoveEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleCaretMoveEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleCaretMoveEvent`.
    fn coerce_from(v: &nsIAccessibleCaretMoveEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleCaretMoveEventCoerce for nsIAccessibleCaretMoveEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCaretMoveEvent) -> &Self {
        v
    }
}

impl nsIAccessibleCaretMoveEvent {
    /// Cast this `nsIAccessibleCaretMoveEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleCaretMoveEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleCaretMoveEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleCaretMoveEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleCaretMoveEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleCaretMoveEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleCaretMoveEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute long caretOffset; */
    pub GetCaretOffset: unsafe extern "system" fn (this: *const nsIAccessibleCaretMoveEvent, aCaretOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute bool isSelectionCollapsed; */
    pub GetIsSelectionCollapsed: unsafe extern "system" fn (this: *const nsIAccessibleCaretMoveEvent, aIsSelectionCollapsed: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleCaretMoveEvent {

    /// ```text
    /// /**
    ///    * Return caret offset.
    ///    */
    /// ```
    ///

    /// `readonly attribute long caretOffset;`
    #[inline]
    pub unsafe fn GetCaretOffset(&self, aCaretOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetCaretOffset)(self, aCaretOffset)
    }


    /// ```text
    /// /**
    ///    * Return true if there is no selection.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool isSelectionCollapsed;`
    #[inline]
    pub unsafe fn GetIsSelectionCollapsed(&self, aIsSelectionCollapsed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSelectionCollapsed)(self, aIsSelectionCollapsed)
    }


}


