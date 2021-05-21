//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/workers/nsIWorkerDebugger.idl
//


/// `interface nsIWorkerDebuggerListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWorkerDebuggerListener {
    vtable: *const nsIWorkerDebuggerListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWorkerDebuggerListener.
unsafe impl XpCom for nsIWorkerDebuggerListener {
    const IID: nsIID = nsID(0x9cf3b48e, 0x361d, 0x486a,
        [0x89, 0x17, 0x55, 0xcf, 0x8d, 0x00, 0xbb, 0x41]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWorkerDebuggerListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWorkerDebuggerListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWorkerDebuggerListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWorkerDebuggerListener`.
    fn coerce_from(v: &nsIWorkerDebuggerListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWorkerDebuggerListenerCoerce for nsIWorkerDebuggerListener {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerListener) -> &Self {
        v
    }
}

impl nsIWorkerDebuggerListener {
    /// Cast this `nsIWorkerDebuggerListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWorkerDebuggerListener {
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
impl<T: nsISupportsCoerce> nsIWorkerDebuggerListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebuggerListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWorkerDebuggerListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWorkerDebuggerListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onClose (); */
    pub OnClose: unsafe extern "system" fn (this: *const nsIWorkerDebuggerListener) -> ::nserror::nsresult,

    /* void onError (in AString filename, in unsigned long lineno, in AString message); */
    pub OnError: unsafe extern "system" fn (this: *const nsIWorkerDebuggerListener, filename: *const ::nsstring::nsAString, lineno: u32, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void onMessage (in AString message); */
    pub OnMessage: unsafe extern "system" fn (this: *const nsIWorkerDebuggerListener, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWorkerDebuggerListener {


    /// `void onClose ();`
    #[inline]
    pub unsafe fn OnClose(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnClose)(self, )
    }



    /// `void onError (in AString filename, in unsigned long lineno, in AString message);`
    #[inline]
    pub unsafe fn OnError(&self, filename: *const ::nsstring::nsAString, lineno: u32, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnError)(self, filename, lineno, message)
    }



    /// `void onMessage (in AString message);`
    #[inline]
    pub unsafe fn OnMessage(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnMessage)(self, message)
    }


}


/// `interface nsIWorkerDebugger : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWorkerDebugger {
    vtable: *const nsIWorkerDebuggerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWorkerDebugger.
unsafe impl XpCom for nsIWorkerDebugger {
    const IID: nsIID = nsID(0x22f93aa3, 0x8a05, 0x46be,
        [0x87, 0xe0, 0xfa, 0x93, 0xbf, 0x8a, 0x8e, 0xff]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWorkerDebugger {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWorkerDebugger.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWorkerDebuggerCoerce {
    /// Cheaply cast a value of this type from a `nsIWorkerDebugger`.
    fn coerce_from(v: &nsIWorkerDebugger) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWorkerDebuggerCoerce for nsIWorkerDebugger {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebugger) -> &Self {
        v
    }
}

impl nsIWorkerDebugger {
    /// Cast this `nsIWorkerDebugger` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWorkerDebuggerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWorkerDebugger {
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
impl<T: nsISupportsCoerce> nsIWorkerDebuggerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWorkerDebugger) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWorkerDebugger
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWorkerDebuggerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute bool isClosed; */
    pub GetIsClosed: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aIsClosed: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool isChrome; */
    pub GetIsChrome: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aIsChrome: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute bool isInitialized; */
    pub GetIsInitialized: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aIsInitialized: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIWorkerDebugger parent; */
    pub GetParent: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aParent: *mut *const nsIWorkerDebugger) -> ::nserror::nsresult,

    /* readonly attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString url; */
    pub GetUrl: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aUrl: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindow window; */
    pub GetWindow: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aWindow: *mut*const mozIDOMWindow) -> ::nserror::nsresult,

    /* readonly attribute Array<uint64_t> windowIDs; */
    pub GetWindowIDs: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aWindowIDs: *mut thin_vec::ThinVec<uint64_t>) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute unsigned long serviceWorkerID; */
    pub GetServiceWorkerID: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aServiceWorkerID: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AString id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIWorkerDebugger, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void initialize (in AString url); */
    pub Initialize: unsafe extern "system" fn (this: *const nsIWorkerDebugger, url: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [binaryname(PostMessageMoz)] void postMessage (in AString message); */
    pub PostMessageMoz: unsafe extern "system" fn (this: *const nsIWorkerDebugger, message: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void addListener (in nsIWorkerDebuggerListener listener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIWorkerDebugger, listener: *const nsIWorkerDebuggerListener) -> ::nserror::nsresult,

    /* void removeListener (in nsIWorkerDebuggerListener listener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIWorkerDebugger, listener: *const nsIWorkerDebuggerListener) -> ::nserror::nsresult,

    /* void setDebuggerReady (in boolean ready); */
    pub SetDebuggerReady: unsafe extern "system" fn (this: *const nsIWorkerDebugger, ready: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWorkerDebugger {

    pub const TYPE_DEDICATED: i64 = 0;


    pub const TYPE_SHARED: i64 = 1;


    pub const TYPE_SERVICE: i64 = 2;


    /// `readonly attribute bool isClosed;`
    #[inline]
    pub unsafe fn GetIsClosed(&self, aIsClosed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsClosed)(self, aIsClosed)
    }



    /// `readonly attribute bool isChrome;`
    #[inline]
    pub unsafe fn GetIsChrome(&self, aIsChrome: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsChrome)(self, aIsChrome)
    }



    /// `readonly attribute bool isInitialized;`
    #[inline]
    pub unsafe fn GetIsInitialized(&self, aIsInitialized: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsInitialized)(self, aIsInitialized)
    }



    /// `readonly attribute nsIWorkerDebugger parent;`
    #[inline]
    pub unsafe fn GetParent(&self, aParent: *mut *const nsIWorkerDebugger) -> ::nserror::nsresult {
        ((*self.vtable).GetParent)(self, aParent)
    }



    /// `readonly attribute unsigned long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `readonly attribute AString url;`
    #[inline]
    pub unsafe fn GetUrl(&self, aUrl: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUrl)(self, aUrl)
    }



    /// `readonly attribute mozIDOMWindow window;`
    #[inline]
    pub unsafe fn GetWindow(&self, aWindow: *mut*const mozIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetWindow)(self, aWindow)
    }



    /// `readonly attribute Array<uint64_t> windowIDs;`
    #[inline]
    pub unsafe fn GetWindowIDs(&self, aWindowIDs: *mut thin_vec::ThinVec<uint64_t>) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowIDs)(self, aWindowIDs)
    }



    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }



    /// `readonly attribute unsigned long serviceWorkerID;`
    #[inline]
    pub unsafe fn GetServiceWorkerID(&self, aServiceWorkerID: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetServiceWorkerID)(self, aServiceWorkerID)
    }



    /// `readonly attribute AString id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }



    /// `void initialize (in AString url);`
    #[inline]
    pub unsafe fn Initialize(&self, url: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Initialize)(self, url)
    }



    /// `[binaryname(PostMessageMoz)] void postMessage (in AString message);`
    #[inline]
    pub unsafe fn PostMessageMoz(&self, message: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).PostMessageMoz)(self, message)
    }



    /// `void addListener (in nsIWorkerDebuggerListener listener);`
    #[inline]
    pub unsafe fn AddListener(&self, listener: *const nsIWorkerDebuggerListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, listener)
    }



    /// `void removeListener (in nsIWorkerDebuggerListener listener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, listener: *const nsIWorkerDebuggerListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, listener)
    }



    /// `void setDebuggerReady (in boolean ready);`
    #[inline]
    pub unsafe fn SetDebuggerReady(&self, ready: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDebuggerReady)(self, ready)
    }


}


