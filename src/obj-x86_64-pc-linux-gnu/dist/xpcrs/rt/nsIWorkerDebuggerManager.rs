//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/workers/nsIWorkerDebuggerManager.idl
//


/// `interface nsIWorkerDebuggerManagerListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWorkerDebuggerManagerListener {
    vtable: *const nsIWorkerDebuggerManagerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWorkerDebuggerManagerListener.
unsafe impl XpCom for nsIWorkerDebuggerManagerListener {
    const IID: nsIID = nsID(0xd2aa74ee, 0x6b98, 0x4d5d,
        [0x81, 0x73, 0x4e, 0x23, 0x42, 0x2d, 0xaf, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWorkerDebuggerManagerListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWorkerDebuggerManagerListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWorkerDebuggerManagerListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWorkerDebuggerManagerListener`.
    fn coerce_from(v: &nsIWorkerDebuggerManagerListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWorkerDebuggerManagerListenerCoerce for nsIWorkerDebuggerManagerListener {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManagerListener) -> &Self {
        v
    }
}

impl nsIWorkerDebuggerManagerListener {
    /// Cast this `nsIWorkerDebuggerManagerListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerManagerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWorkerDebuggerManagerListener {
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
impl<T: nsISupportsCoerce> nsIWorkerDebuggerManagerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManagerListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWorkerDebuggerManagerListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWorkerDebuggerManagerListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onRegister (in nsIWorkerDebugger debugger); */
    pub OnRegister: unsafe extern "system" fn (this: *const nsIWorkerDebuggerManagerListener, debugger: *const nsIWorkerDebugger) -> ::nserror::nsresult,

    /* void onUnregister (in nsIWorkerDebugger debugger); */
    pub OnUnregister: unsafe extern "system" fn (this: *const nsIWorkerDebuggerManagerListener, debugger: *const nsIWorkerDebugger) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWorkerDebuggerManagerListener {


    /// `void onRegister (in nsIWorkerDebugger debugger);`
    #[inline]
    pub unsafe fn OnRegister(&self, debugger: *const nsIWorkerDebugger) -> ::nserror::nsresult {
        ((*self.vtable).OnRegister)(self, debugger)
    }



    /// `void onUnregister (in nsIWorkerDebugger debugger);`
    #[inline]
    pub unsafe fn OnUnregister(&self, debugger: *const nsIWorkerDebugger) -> ::nserror::nsresult {
        ((*self.vtable).OnUnregister)(self, debugger)
    }


}


/// `interface nsIWorkerDebuggerManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWorkerDebuggerManager {
    vtable: *const nsIWorkerDebuggerManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWorkerDebuggerManager.
unsafe impl XpCom for nsIWorkerDebuggerManager {
    const IID: nsIID = nsID(0x056d7918, 0xdc86, 0x452a,
        [0xb4, 0xe6, 0x86, 0xda, 0x34, 0x05, 0xf0, 0x15]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWorkerDebuggerManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWorkerDebuggerManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWorkerDebuggerManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIWorkerDebuggerManager`.
    fn coerce_from(v: &nsIWorkerDebuggerManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWorkerDebuggerManagerCoerce for nsIWorkerDebuggerManager {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManager) -> &Self {
        v
    }
}

impl nsIWorkerDebuggerManager {
    /// Cast this `nsIWorkerDebuggerManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWorkerDebuggerManager {
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
impl<T: nsISupportsCoerce> nsIWorkerDebuggerManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWorkerDebuggerManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWorkerDebuggerManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator getWorkerDebuggerEnumerator (); */
    pub GetWorkerDebuggerEnumerator: unsafe extern "system" fn (this: *const nsIWorkerDebuggerManager, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* void addListener (in nsIWorkerDebuggerManagerListener listener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIWorkerDebuggerManager, listener: *const nsIWorkerDebuggerManagerListener) -> ::nserror::nsresult,

    /* void removeListener (in nsIWorkerDebuggerManagerListener listener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIWorkerDebuggerManager, listener: *const nsIWorkerDebuggerManagerListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWorkerDebuggerManager {


    /// `nsISimpleEnumerator getWorkerDebuggerEnumerator ();`
    #[inline]
    pub unsafe fn GetWorkerDebuggerEnumerator(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetWorkerDebuggerEnumerator)(self, _retval)
    }



    /// `void addListener (in nsIWorkerDebuggerManagerListener listener);`
    #[inline]
    pub unsafe fn AddListener(&self, listener: *const nsIWorkerDebuggerManagerListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, listener)
    }



    /// `void removeListener (in nsIWorkerDebuggerManagerListener listener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, listener: *const nsIWorkerDebuggerManagerListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, listener)
    }


}


