//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/threads/nsIRunnable.idl
//


/// `interface nsIRunnable : nsISupports`
///

/// ```text
/// /**
///  * Represents a task which can be dispatched to a thread for execution.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRunnable {
    vtable: *const nsIRunnableVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRunnable.
unsafe impl XpCom for nsIRunnable {
    const IID: nsIID = nsID(0x4a2abaf0, 0x6886, 0x11d3,
        [0x93, 0x82, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRunnable {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRunnable.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRunnableCoerce {
    /// Cheaply cast a value of this type from a `nsIRunnable`.
    fn coerce_from(v: &nsIRunnable) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRunnableCoerce for nsIRunnable {
    #[inline]
    fn coerce_from(v: &nsIRunnable) -> &Self {
        v
    }
}

impl nsIRunnable {
    /// Cast this `nsIRunnable` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRunnableCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRunnable {
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
impl<T: nsISupportsCoerce> nsIRunnableCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRunnable) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRunnable
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRunnableVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void run (); */
    pub Run: unsafe extern "system" fn (this: *const nsIRunnable) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRunnable {

    /// ```text
    /// /**
    ///      * The function implementing the task to be run.
    ///      */
    /// ```
    ///

    /// `void run ();`
    #[inline]
    pub unsafe fn Run(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Run)(self, )
    }


}


/// `interface nsIRunnablePriority : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRunnablePriority {
    vtable: *const nsIRunnablePriorityVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRunnablePriority.
unsafe impl XpCom for nsIRunnablePriority {
    const IID: nsIID = nsID(0xe75aa42a, 0x80a9, 0x11e6,
        [0xaf, 0xb5, 0xe8, 0x9d, 0x87, 0x34, 0x8e, 0x2c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRunnablePriority {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRunnablePriority.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRunnablePriorityCoerce {
    /// Cheaply cast a value of this type from a `nsIRunnablePriority`.
    fn coerce_from(v: &nsIRunnablePriority) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRunnablePriorityCoerce for nsIRunnablePriority {
    #[inline]
    fn coerce_from(v: &nsIRunnablePriority) -> &Self {
        v
    }
}

impl nsIRunnablePriority {
    /// Cast this `nsIRunnablePriority` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRunnablePriorityCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRunnablePriority {
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
impl<T: nsISupportsCoerce> nsIRunnablePriorityCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRunnablePriority) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRunnablePriority
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRunnablePriorityVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long priority; */
    pub GetPriority: unsafe extern "system" fn (this: *const nsIRunnablePriority, aPriority: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRunnablePriority {

    pub const PRIORITY_IDLE: i64 = 0;


    pub const PRIORITY_DEFERRED_TIMERS: i64 = 1;


    pub const PRIORITY_NORMAL: i64 = 3;


    pub const PRIORITY_MEDIUMHIGH: i64 = 4;


    pub const PRIORITY_INPUT_HIGH: i64 = 5;


    pub const PRIORITY_HIGH: i64 = 6;


    /// `readonly attribute unsigned long priority;`
    #[inline]
    pub unsafe fn GetPriority(&self, aPriority: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetPriority)(self, aPriority)
    }


}


/// `interface nsIRunnableIPCMessageType : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRunnableIPCMessageType {
    vtable: *const nsIRunnableIPCMessageTypeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRunnableIPCMessageType.
unsafe impl XpCom for nsIRunnableIPCMessageType {
    const IID: nsIID = nsID(0x3114c36c, 0xa482, 0x4c6e,
        [0x95, 0x23, 0x1d, 0xcf, 0xc6, 0xf6, 0x05, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRunnableIPCMessageType {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRunnableIPCMessageType.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRunnableIPCMessageTypeCoerce {
    /// Cheaply cast a value of this type from a `nsIRunnableIPCMessageType`.
    fn coerce_from(v: &nsIRunnableIPCMessageType) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRunnableIPCMessageTypeCoerce for nsIRunnableIPCMessageType {
    #[inline]
    fn coerce_from(v: &nsIRunnableIPCMessageType) -> &Self {
        v
    }
}

impl nsIRunnableIPCMessageType {
    /// Cast this `nsIRunnableIPCMessageType` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRunnableIPCMessageTypeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRunnableIPCMessageType {
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
impl<T: nsISupportsCoerce> nsIRunnableIPCMessageTypeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRunnableIPCMessageType) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRunnableIPCMessageType
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRunnableIPCMessageTypeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIRunnableIPCMessageType, aType: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRunnableIPCMessageType {


    /// `readonly attribute unsigned long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


}


