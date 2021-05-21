//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsISupportsPriority.idl
//


/// `interface nsISupportsPriority : nsISupports`
///

/// ```text
/// /**
///  * This interface exposes the general notion of a scheduled object with a
///  * integral priority value.  Following UNIX conventions, smaller (and possibly
    ///  * negative) values have higher priority.
///  *
///  * This interface does not strictly define what happens when the priority of an
///  * object is changed.  An implementation of this interface is free to define
///  * the side-effects of changing the priority of an object.  In some cases,
///  * changing the priority of an object may be disallowed (resulting in an
    ///  * exception being thrown) or may simply be ignored.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISupportsPriority {
    vtable: *const nsISupportsPriorityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISupportsPriority.
unsafe impl XpCom for nsISupportsPriority {
    const IID: nsIID = nsID(0xaa578b44, 0xabd5, 0x4c19,
        [0x8b, 0x14, 0x36, 0xd4, 0xde, 0x6f, 0xdc, 0x36]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISupportsPriority {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISupportsPriority.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISupportsPriorityCoerce {
    /// Cheaply cast a value of this type from a `nsISupportsPriority`.
    fn coerce_from(v: &nsISupportsPriority) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISupportsPriorityCoerce for nsISupportsPriority {
    #[inline]
    fn coerce_from(v: &nsISupportsPriority) -> &Self {
        v
    }
}

impl nsISupportsPriority {
    /// Cast this `nsISupportsPriority` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISupportsPriorityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISupportsPriority {
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
impl<T: nsISupportsCoerce> nsISupportsPriorityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISupportsPriority) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISupportsPriority
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISupportsPriorityVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute long priority; */
    pub GetPriority: unsafe extern "system" fn (this: *const nsISupportsPriority, aPriority: *mut i32) -> ::nserror::nsresult,

    /* attribute long priority; */
    pub SetPriority: unsafe extern "system" fn (this: *const nsISupportsPriority, aPriority: i32) -> ::nserror::nsresult,

    /* void adjustPriority (in long delta); */
    pub AdjustPriority: unsafe extern "system" fn (this: *const nsISupportsPriority, delta: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISupportsPriority {
    /// ```text
    /// /**
    ///    * Typical priority values.
    ///    */
    /// ```
    ///

    pub const PRIORITY_HIGHEST: i64 = -20;


    pub const PRIORITY_HIGH: i64 = -10;


    pub const PRIORITY_NORMAL: i64 = 0;


    pub const PRIORITY_LOW: i64 = 10;


    pub const PRIORITY_LOWEST: i64 = 20;

    /// ```text
    /// /**
    ///    * This attribute may be modified to change the priority of this object.  The
    ///    * implementation of this interface is free to truncate a given priority
    ///    * value to whatever limits are appropriate.  Typically, this attribute is
    ///    * initialized to PRIORITY_NORMAL, but implementations may choose to assign a
    ///    * different initial value.
    ///    */
    /// ```
    ///

    /// `attribute long priority;`
    #[inline]
    pub unsafe fn GetPriority(&self, aPriority: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPriority)(self, aPriority)
    }


    /// ```text
    /// /**
    ///    * This attribute may be modified to change the priority of this object.  The
    ///    * implementation of this interface is free to truncate a given priority
    ///    * value to whatever limits are appropriate.  Typically, this attribute is
    ///    * initialized to PRIORITY_NORMAL, but implementations may choose to assign a
    ///    * different initial value.
    ///    */
    /// ```
    ///

    /// `attribute long priority;`
    #[inline]
    pub unsafe fn SetPriority(&self, aPriority: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetPriority)(self, aPriority)
    }


    /// ```text
    /// /**
    ///    * This method adjusts the priority attribute by a given delta.  It helps
    ///    * reduce the amount of coding required to increment or decrement the value
    ///    * of the priority attribute.
    ///    */
    /// ```
    ///

    /// `void adjustPriority (in long delta);`
    #[inline]
    pub unsafe fn AdjustPriority(&self, delta: i32) -> ::nserror::nsresult {
        ((*self.vtable).AdjustPriority)(self, delta)
    }


}


