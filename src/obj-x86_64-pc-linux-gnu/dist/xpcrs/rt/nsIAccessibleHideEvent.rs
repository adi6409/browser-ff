//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleHideEvent.idl
//


/// `interface nsIAccessibleHideEvent : nsIAccessibleEvent`
///

/// ```text
/// /**
///  * Fired when a accessible and its subtree are removed from the tree.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleHideEvent {
    vtable: *const nsIAccessibleHideEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleHideEvent.
unsafe impl XpCom for nsIAccessibleHideEvent {
    const IID: nsIID = nsID(0x2051709a, 0x4e0d, 0x4be5,
        [0x87, 0x3d, 0xb4, 0x9d, 0x1d, 0xee, 0x35, 0xfa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleHideEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleHideEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleHideEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleHideEvent`.
    fn coerce_from(v: &nsIAccessibleHideEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleHideEventCoerce for nsIAccessibleHideEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHideEvent) -> &Self {
        v
    }
}

impl nsIAccessibleHideEvent {
    /// Cast this `nsIAccessibleHideEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleHideEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleHideEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleHideEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleHideEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleHideEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleHideEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute nsIAccessible targetParent; */
    pub GetTargetParent: unsafe extern "system" fn (this: *const nsIAccessibleHideEvent, aTargetParent: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible targetNextSibling; */
    pub GetTargetNextSibling: unsafe extern "system" fn (this: *const nsIAccessibleHideEvent, aTargetNextSibling: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible targetPrevSibling; */
    pub GetTargetPrevSibling: unsafe extern "system" fn (this: *const nsIAccessibleHideEvent, aTargetPrevSibling: *mut*const nsIAccessible) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleHideEvent {

    /// ```text
    /// /**
    ///    * Return an accessible that was a parent of the target.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible targetParent;`
    #[inline]
    pub unsafe fn GetTargetParent(&self, aTargetParent: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetParent)(self, aTargetParent)
    }


    /// ```text
    /// /**
    ///    * Return an accessible that was a next sibling of the target
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible targetNextSibling;`
    #[inline]
    pub unsafe fn GetTargetNextSibling(&self, aTargetNextSibling: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetNextSibling)(self, aTargetNextSibling)
    }


    /// ```text
    /// /**
    ///    * Return an accessible that was a parent of the target
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible targetPrevSibling;`
    #[inline]
    pub unsafe fn GetTargetPrevSibling(&self, aTargetPrevSibling: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetPrevSibling)(self, aTargetPrevSibling)
    }


}


