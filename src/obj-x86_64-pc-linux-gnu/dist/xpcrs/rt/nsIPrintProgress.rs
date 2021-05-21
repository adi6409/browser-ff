//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/printing/nsIPrintProgress.idl
//


/// `interface nsIPrintProgress : nsIWebProgressListener`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrintProgress {
    vtable: *const nsIPrintProgressVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrintProgress.
unsafe impl XpCom for nsIPrintProgress {
    const IID: nsIID = nsID(0x05f4fb88, 0xe568, 0x4d35,
        [0xb3, 0x94, 0xce, 0x0a, 0xa3, 0xee, 0xa6, 0xfc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrintProgress {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrintProgress.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrintProgressCoerce {
    /// Cheaply cast a value of this type from a `nsIPrintProgress`.
    fn coerce_from(v: &nsIPrintProgress) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrintProgressCoerce for nsIPrintProgress {
    #[inline]
    fn coerce_from(v: &nsIPrintProgress) -> &Self {
        v
    }
}

impl nsIPrintProgress {
    /// Cast this `nsIPrintProgress` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrintProgressCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrintProgress {
    type Target = nsIWebProgressListener;
    #[inline]
    fn deref(&self) -> &nsIWebProgressListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIWebProgressListenerCoerce> nsIPrintProgressCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrintProgress) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrintProgress
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrintProgressVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIWebProgressListenerVTable,

    /* void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen); */
    pub OpenProgressDialog: unsafe extern "system" fn (this: *const nsIPrintProgress, parent: *const mozIDOMWindowProxy, dialogURL: *const libc::c_char, parameters: *const nsISupports, openDialogObserver: *const nsIObserver, notifyOnOpen: *mut bool) -> ::nserror::nsresult,

    /* void closeProgressDialog (in boolean forceClose); */
    pub CloseProgressDialog: unsafe extern "system" fn (this: *const nsIPrintProgress, forceClose: bool) -> ::nserror::nsresult,

    /* void registerListener (in nsIWebProgressListener listener); */
    pub RegisterListener: unsafe extern "system" fn (this: *const nsIPrintProgress, listener: *const nsIWebProgressListener) -> ::nserror::nsresult,

    /* void unregisterListener (in nsIWebProgressListener listener); */
    pub UnregisterListener: unsafe extern "system" fn (this: *const nsIPrintProgress, listener: *const nsIWebProgressListener) -> ::nserror::nsresult,

    /* void doneIniting (); */
    pub DoneIniting: unsafe extern "system" fn (this: *const nsIPrintProgress) -> ::nserror::nsresult,

    /* nsIPrompt getPrompter (); */
    pub GetPrompter: unsafe extern "system" fn (this: *const nsIPrintProgress, _retval: *mut*const nsIPrompt) -> ::nserror::nsresult,

    /* attribute boolean processCanceledByUser; */
    pub GetProcessCanceledByUser: unsafe extern "system" fn (this: *const nsIPrintProgress, aProcessCanceledByUser: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean processCanceledByUser; */
    pub SetProcessCanceledByUser: unsafe extern "system" fn (this: *const nsIPrintProgress, aProcessCanceledByUser: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrintProgress {


    /// `void openProgressDialog (in mozIDOMWindowProxy parent, in string dialogURL, in nsISupports parameters, in nsIObserver openDialogObserver, out boolean notifyOnOpen);`
    #[inline]
    pub unsafe fn OpenProgressDialog(&self, parent: *const mozIDOMWindowProxy, dialogURL: *const libc::c_char, parameters: *const nsISupports, openDialogObserver: *const nsIObserver, notifyOnOpen: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OpenProgressDialog)(self, parent, dialogURL, parameters, openDialogObserver, notifyOnOpen)
    }



    /// `void closeProgressDialog (in boolean forceClose);`
    #[inline]
    pub unsafe fn CloseProgressDialog(&self, forceClose: bool) -> ::nserror::nsresult {
        ((*self.vtable).CloseProgressDialog)(self, forceClose)
    }



    /// `void registerListener (in nsIWebProgressListener listener);`
    #[inline]
    pub unsafe fn RegisterListener(&self, listener: *const nsIWebProgressListener) -> ::nserror::nsresult {
        ((*self.vtable).RegisterListener)(self, listener)
    }



    /// `void unregisterListener (in nsIWebProgressListener listener);`
    #[inline]
    pub unsafe fn UnregisterListener(&self, listener: *const nsIWebProgressListener) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterListener)(self, listener)
    }



    /// `void doneIniting ();`
    #[inline]
    pub unsafe fn DoneIniting(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DoneIniting)(self, )
    }



    /// `nsIPrompt getPrompter ();`
    #[inline]
    pub unsafe fn GetPrompter(&self, _retval: *mut*const nsIPrompt) -> ::nserror::nsresult {
        ((*self.vtable).GetPrompter)(self, _retval)
    }



    /// `attribute boolean processCanceledByUser;`
    #[inline]
    pub unsafe fn GetProcessCanceledByUser(&self, aProcessCanceledByUser: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetProcessCanceledByUser)(self, aProcessCanceledByUser)
    }



    /// `attribute boolean processCanceledByUser;`
    #[inline]
    pub unsafe fn SetProcessCanceledByUser(&self, aProcessCanceledByUser: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetProcessCanceledByUser)(self, aProcessCanceledByUser)
    }


}


