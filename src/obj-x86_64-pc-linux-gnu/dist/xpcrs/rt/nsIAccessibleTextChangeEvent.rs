//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleTextChangeEvent.idl
//


/// `interface nsIAccessibleTextChangeEvent : nsIAccessibleEvent`
///

/// ```text
/// /**
///  * Fired when an accessible's text changes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleTextChangeEvent {
    vtable: *const nsIAccessibleTextChangeEventVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleTextChangeEvent.
unsafe impl XpCom for nsIAccessibleTextChangeEvent {
    const IID: nsIID = nsID(0x1fcc0dfa, 0x93e6, 0x48f4,
        [0xbb, 0xd4, 0xf8, 0x0e, 0xb1, 0xd9, 0xf2, 0xe6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleTextChangeEvent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleTextChangeEvent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleTextChangeEventCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleTextChangeEvent`.
    fn coerce_from(v: &nsIAccessibleTextChangeEvent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleTextChangeEventCoerce for nsIAccessibleTextChangeEvent {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextChangeEvent) -> &Self {
        v
    }
}

impl nsIAccessibleTextChangeEvent {
    /// Cast this `nsIAccessibleTextChangeEvent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleTextChangeEventCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleTextChangeEvent {
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
impl<T: nsIAccessibleEventCoerce> nsIAccessibleTextChangeEventCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleTextChangeEvent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleTextChangeEvent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleTextChangeEventVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAccessibleEventVTable,

    /* readonly attribute long start; */
    pub GetStart: unsafe extern "system" fn (this: *const nsIAccessibleTextChangeEvent, aStart: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long length; */
    pub GetLength: unsafe extern "system" fn (this: *const nsIAccessibleTextChangeEvent, aLength: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute boolean isInserted; */
    pub GetIsInserted: unsafe extern "system" fn (this: *const nsIAccessibleTextChangeEvent, aIsInserted: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString modifiedText; */
    pub GetModifiedText: unsafe extern "system" fn (this: *const nsIAccessibleTextChangeEvent, aModifiedText: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleTextChangeEvent {

    /// ```text
    /// /**
    ///    * Returns offset of changed text in accessible.
    ///    */
    /// ```
    ///

    /// `readonly attribute long start;`
    #[inline]
    pub unsafe fn GetStart(&self, aStart: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetStart)(self, aStart)
    }


    /// ```text
    /// /**
    ///    * Returns length of changed text.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long length;`
    #[inline]
    pub unsafe fn GetLength(&self, aLength: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetLength)(self, aLength)
    }


    /// ```text
    /// /**
    ///    * Returns true if text was inserted, otherwise false.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isInserted;`
    #[inline]
    pub unsafe fn GetIsInserted(&self, aIsInserted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInserted)(self, aIsInserted)
    }


    /// ```text
    /// /**
    ///    * The inserted or removed text
    ///    */
    /// ```
    ///

    /// `readonly attribute AString modifiedText;`
    #[inline]
    pub unsafe fn GetModifiedText(&self, aModifiedText: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetModifiedText)(self, aModifiedText)
    }


}


