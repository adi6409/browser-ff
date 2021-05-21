//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTableChangeEvent.idl
//


/// `interface nsIAccessibleTableChangeEvent : nsIAccessibleEvent`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTableChangeEvent {
    vtable: *const nsIAccessibleTableChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTableChangeEvent.
unsafe impl XpCom for nsIAccessibleTableChangeEvent {
    const IID: nsIID = nsID(0x9fb3a8a4, 0xd254, 0x43d3,
        [0x80, 0xa5, 0x20, 0xe1, 0x71, 0xd5, 0x2b, 0x21]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTableChangeEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTableChangeEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTableChangeEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTableChangeEvent`.
    fn coerce_from(v: &nsIAccessibleTableChangeEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTableChangeEventCoerce for nsIAccessibleTableChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleTableChangeEvent {
    /// Cast this `nsIAccessibleTableChangeEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTableChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTableChangeEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleTableChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTableChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTableChangeEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTableChangeEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute long rowOrColIndex; */
    pub GetRowOrColIndex: unsafe extern "system" fn (this: *const nsIAccessibleTableChangeEvent, aRowOrColIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long RowsOrColsCount; */
    pub GetRowsOrColsCount: unsafe extern "system" fn (this: *const nsIAccessibleTableChangeEvent, aRowsOrColsCount: *mut i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTableChangeEvent {

    /// ```text
    /// /**
    ///    * Return the row or column index.
    ///    */
    /// ```
    ///

    /// `readonly attribute long rowOrColIndex;`
    #[inline]
    pub unsafe fn GetRowOrColIndex(&self, aRowOrColIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowOrColIndex)(self, aRowOrColIndex)
    }


    /// ```text
    /// /**
    ///    * Return the number of rows or cols
    ///    */
    /// ```
    ///

    /// `readonly attribute long RowsOrColsCount;`
    #[inline]
    pub unsafe fn GetRowsOrColsCount(&self, aRowsOrColsCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRowsOrColsCount)(self, aRowsOrColsCount)
    }


}


