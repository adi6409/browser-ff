//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleService.idl
//


/// `interface nsIConsoleService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIConsoleService {
    vtable: *const nsIConsoleServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIConsoleService.
unsafe impl XpCom for nsIConsoleService {
    const IID: nsIID = nsID(0x0eb81d20, 0xc37e, 0x42d4,
        [0x82, 0xa8, 0xca, 0x9a, 0xe9, 0x6b, 0xdf, 0x52]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIConsoleService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIConsoleService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIConsoleServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIConsoleService`.
    fn coerce_from(v: &nsIConsoleService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIConsoleServiceCoerce for nsIConsoleService {
    #[inline]
    fn coerce_from(v: &nsIConsoleService) -> &Self {
        v
    }
}

impl nsIConsoleService {
    /// Cast this `nsIConsoleService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIConsoleServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIConsoleService {
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
impl<T: nsISupportsCoerce> nsIConsoleServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIConsoleService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIConsoleServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void logMessage (in nsIConsoleMessage message); */
    pub LogMessage: unsafe extern "system" fn (this: *const nsIConsoleService, message: *const nsIConsoleMessage) -> ::nserror::nsresult,

    /* void logStringMessage (in wstring message); */
    pub LogStringMessage: unsafe extern "system" fn (this: *const nsIConsoleService, message: *const i16) -> ::nserror::nsresult,

    /* Array<nsIConsoleMessage> getMessageArray (); */
    pub GetMessageArray: unsafe extern "system" fn (this: *const nsIConsoleService, _retval: *mut thin_vec::ThinVec<RefPtr<nsIConsoleMessage>>) -> ::nserror::nsresult,

    /* void registerListener (in nsIConsoleListener listener); */
    pub RegisterListener: unsafe extern "system" fn (this: *const nsIConsoleService, listener: *const nsIConsoleListener) -> ::nserror::nsresult,

    /* void unregisterListener (in nsIConsoleListener listener); */
    pub UnregisterListener: unsafe extern "system" fn (this: *const nsIConsoleService, listener: *const nsIConsoleListener) -> ::nserror::nsresult,

    /* void reset (); */
    pub Reset: unsafe extern "system" fn (this: *const nsIConsoleService) -> ::nserror::nsresult,

    /* void resetWindow (in uint64_t windowInnerId); */
    pub ResetWindow: unsafe extern "system" fn (this: *const nsIConsoleService, windowInnerId: uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIConsoleService {


    /// `void logMessage (in nsIConsoleMessage message);`
    #[inline]
    pub unsafe fn LogMessage(&self, message: *const nsIConsoleMessage) -> ::nserror::nsresult {
        ((*self.vtable).LogMessage)(self, message)
    }


    /// ```text
    /// /**
    ///      * Convenience method for logging simple messages.
    ///      */
    /// ```
    ///

    /// `void logStringMessage (in wstring message);`
    #[inline]
    pub unsafe fn LogStringMessage(&self, message: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).LogStringMessage)(self, message)
    }


    /// ```text
    /// /**
    ///      * Get an array of all the messages logged so far.
    ///      */
    /// ```
    ///

    /// `Array<nsIConsoleMessage> getMessageArray ();`
    #[inline]
    pub unsafe fn GetMessageArray(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsIConsoleMessage>>) -> ::nserror::nsresult {
        ((*self.vtable).GetMessageArray)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * To guard against stack overflows from listeners that could log
    ///      * messages (it's easy to do this inadvertently from listeners
        ///      * implemented in JavaScript), we don't call any listeners when
    ///      * another error is already being logged.
    ///      */
    /// ```
    ///

    /// `void registerListener (in nsIConsoleListener listener);`
    #[inline]
    pub unsafe fn RegisterListener(&self, listener: *const nsIConsoleListener) -> ::nserror::nsresult {
        ((*self.vtable).RegisterListener)(self, listener)
    }


    /// ```text
    /// /**
    ///      * Each registered listener should also be unregistered.
    ///      */
    /// ```
    ///

    /// `void unregisterListener (in nsIConsoleListener listener);`
    #[inline]
    pub unsafe fn UnregisterListener(&self, listener: *const nsIConsoleListener) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterListener)(self, listener)
    }


    /// ```text
    /// /**
    ///      * Clear the message buffer (e.g. for privacy reasons).
    ///      */
    /// ```
    ///

    /// `void reset ();`
    #[inline]
    pub unsafe fn Reset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, )
    }


    /// ```text
    /// /**
    ///      * Clear the message buffer for a given window.
    ///      */
    /// ```
    ///

    /// `void resetWindow (in uint64_t windowInnerId);`
    #[inline]
    pub unsafe fn ResetWindow(&self, windowInnerId: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).ResetWindow)(self, windowInnerId)
    }


}


