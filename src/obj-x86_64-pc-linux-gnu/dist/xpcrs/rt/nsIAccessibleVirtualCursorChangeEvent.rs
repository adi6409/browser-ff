//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleVirtualCursorChangeEvent.idl
//


/// `interface nsIAccessibleVirtualCursorChangeEvent : nsIAccessibleEvent`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleVirtualCursorChangeEvent {
    vtable: *const nsIAccessibleVirtualCursorChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleVirtualCursorChangeEvent.
unsafe impl XpCom for nsIAccessibleVirtualCursorChangeEvent {
    const IID: nsIID = nsID(0xa58693b1, 0x009e, 0x4cc9,
        [0xae, 0x93, 0x9c, 0x7d, 0x8f, 0x85, 0xcf, 0xdf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleVirtualCursorChangeEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleVirtualCursorChangeEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleVirtualCursorChangeEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleVirtualCursorChangeEvent`.
    fn coerce_from(v: &nsIAccessibleVirtualCursorChangeEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleVirtualCursorChangeEventCoerce for nsIAccessibleVirtualCursorChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleVirtualCursorChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleVirtualCursorChangeEvent {
    /// Cast this `nsIAccessibleVirtualCursorChangeEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleVirtualCursorChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleVirtualCursorChangeEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleVirtualCursorChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleVirtualCursorChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleVirtualCursorChangeEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleVirtualCursorChangeEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute nsIAccessible oldAccessible; */
    pub GetOldAccessible: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aOldAccessible: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute long oldStartOffset; */
    pub GetOldStartOffset: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aOldStartOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long oldEndOffset; */
    pub GetOldEndOffset: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aOldEndOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessible newAccessible; */
    pub GetNewAccessible: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aNewAccessible: *mut*const nsIAccessible) -> ::nserror::nsresult,

    /* readonly attribute long newStartOffset; */
    pub GetNewStartOffset: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aNewStartOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long newEndOffset; */
    pub GetNewEndOffset: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aNewEndOffset: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute short reason; */
    pub GetReason: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aReason: *mut i16) -> ::nserror::nsresult,

    /* readonly attribute short boundaryType; */
    pub GetBoundaryType: unsafe extern "system" fn (this: *const nsIAccessibleVirtualCursorChangeEvent, aBoundaryType: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleVirtualCursorChangeEvent {

    /// ```text
    /// /**
    ///    * Previous object pointed at by virtual cursor, null if none.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible oldAccessible;`
    #[inline]
    pub unsafe fn GetOldAccessible(&self, aOldAccessible: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetOldAccessible)(self, aOldAccessible)
    }


    /// ```text
    /// /**
    ///    * Previous start offset of pivot, otherwise -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long oldStartOffset;`
    #[inline]
    pub unsafe fn GetOldStartOffset(&self, aOldStartOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetOldStartOffset)(self, aOldStartOffset)
    }


    /// ```text
    /// /**
    ///    * Previous end offset of pivot, otherwise -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long oldEndOffset;`
    #[inline]
    pub unsafe fn GetOldEndOffset(&self, aOldEndOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetOldEndOffset)(self, aOldEndOffset)
    }


    /// ```text
    /// /**
    ///    * New object pointed at by virtual cursor, null if none.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessible newAccessible;`
    #[inline]
    pub unsafe fn GetNewAccessible(&self, aNewAccessible: *mut*const nsIAccessible) -> ::nserror::nsresult {
        ((*self.vtable).GetNewAccessible)(self, aNewAccessible)
    }


    /// ```text
    /// /**
    ///    * New start offset of pivot, otherwise -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long newStartOffset;`
    #[inline]
    pub unsafe fn GetNewStartOffset(&self, aNewStartOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNewStartOffset)(self, aNewStartOffset)
    }


    /// ```text
    /// /**
    ///    * New end offset of pivot, otherwise -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long newEndOffset;`
    #[inline]
    pub unsafe fn GetNewEndOffset(&self, aNewEndOffset: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNewEndOffset)(self, aNewEndOffset)
    }


    /// ```text
    /// /**
    ///    * Reason for virtual cursor move.
    ///    */
    /// ```
    ///

    /// `readonly attribute short reason;`
    #[inline]
    pub unsafe fn GetReason(&self, aReason: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetReason)(self, aReason)
    }


    /// ```text
    /// /**
    ///    * Text boundary type for movement, NO_BOUNDARY if none.
    ///    */
    /// ```
    ///

    /// `readonly attribute short boundaryType;`
    #[inline]
    pub unsafe fn GetBoundaryType(&self, aBoundaryType: *mut i16) -> ::nserror::nsresult {
        ((*self.vtable).GetBoundaryType)(self, aBoundaryType)
    }


}


