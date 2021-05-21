//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIAppShellService.idl
//


/// `interface nsIAppShellService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAppShellService {
    vtable: *const nsIAppShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAppShellService.
unsafe impl XpCom for nsIAppShellService {
    const IID: nsIID = nsID(0x19266025, 0x354c, 0x4bb9,
        [0x98, 0x6b, 0x34, 0x83, 0xb2, 0xb1, 0xcd, 0xef]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAppShellService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAppShellService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAppShellServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIAppShellService`.
    fn coerce_from(v: &nsIAppShellService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAppShellServiceCoerce for nsIAppShellService {
    #[inline]
    fn coerce_from(v: &nsIAppShellService) -> &Self {
        v
    }
}

impl nsIAppShellService {
    /// Cast this `nsIAppShellService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAppShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAppShellService {
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
impl<T: nsISupportsCoerce> nsIAppShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAppShellService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAppShellService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAppShellServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIAppWindow createTopLevelWindow (in nsIAppWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight); */
    pub CreateTopLevelWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aParent: *const nsIAppWindow, aUrl: *const nsIURI, aChromeMask: uint32_t, aInitialWidth: i32, aInitialHeight: i32, _retval: *mut*const nsIAppWindow) -> ::nserror::nsresult,

    /* nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome, [optional] in uint32_t aChromeMask); */
    pub CreateWindowlessBrowser: unsafe extern "system" fn (this: *const nsIAppShellService, aIsChrome: bool, aChromeMask: uint32_t, _retval: *mut*const nsIWindowlessBrowser) -> ::nserror::nsresult,

    /* [noscript] void createHiddenWindow (); */
    pub CreateHiddenWindow: unsafe extern "system" fn (this: *const nsIAppShellService) -> ::nserror::nsresult,

    /* void destroyHiddenWindow (); */
    pub DestroyHiddenWindow: unsafe extern "system" fn (this: *const nsIAppShellService) -> ::nserror::nsresult,

    /* [noscript] void setScreenId (in uint32_t aScreenId); */
    pub SetScreenId: unsafe extern "system" fn (this: *const nsIAppShellService, aScreenId: uint32_t) -> ::nserror::nsresult,

    /* readonly attribute nsIAppWindow hiddenWindow; */
    pub GetHiddenWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aHiddenWindow: *mut*const nsIAppWindow) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy hiddenDOMWindow; */
    pub GetHiddenDOMWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aHiddenDOMWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute boolean applicationProvidedHiddenWindow; */
    pub GetApplicationProvidedHiddenWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aApplicationProvidedHiddenWindow: *mut bool) -> ::nserror::nsresult,

    /* void registerTopLevelWindow (in nsIAppWindow aWindow); */
    pub RegisterTopLevelWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aWindow: *const nsIAppWindow) -> ::nserror::nsresult,

    /* void unregisterTopLevelWindow (in nsIAppWindow aWindow); */
    pub UnregisterTopLevelWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aWindow: *const nsIAppWindow) -> ::nserror::nsresult,

    /* readonly attribute boolean hasHiddenWindow; */
    pub GetHasHiddenWindow: unsafe extern "system" fn (this: *const nsIAppShellService, aHasHiddenWindow: *mut bool) -> ::nserror::nsresult,

    /* bool startEventLoopLagTracking (); */
    pub StartEventLoopLagTracking: unsafe extern "system" fn (this: *const nsIAppShellService, _retval: *mut bool) -> ::nserror::nsresult,

    /* void stopEventLoopLagTracking (); */
    pub StopEventLoopLagTracking: unsafe extern "system" fn (this: *const nsIAppShellService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAppShellService {
    /// ```text
    /// /**
    ///    * Create a window, which will be initially invisible.
    ///    * @param aParent the parent window.  Can be null.
    ///    * @param aUrl the contents of the new window.
    ///    * @param aChromeMask chrome flags affecting the kind of OS border
    ///    *                    given to the window. see nsIWebBrowserChrome for
    ///    *                    bit/flag definitions.
    ///    * @param aCallbacks interface providing C++ hooks for window initialization
    ///    *                   before the window is made visible.  Can be null.
    ///    *                   Deprecated.
    ///    * @param aInitialWidth width, in pixels, of the window.  Width of window
    ///    *                      at creation.  Can be overridden by the "width"
    ///    *                      tag in the XUL.  Set to NS_SIZETOCONTENT to force
    ///    *                      the window to wrap to its contents.
    ///    * @param aInitialHeight like aInitialWidth, but subtly different.
    ///    */
    /// ```
    ///

    pub const SIZE_TO_CONTENT: i64 = -1;


    /// `nsIAppWindow createTopLevelWindow (in nsIAppWindow aParent, in nsIURI aUrl, in uint32_t aChromeMask, in long aInitialWidth, in long aInitialHeight);`
    #[inline]
    pub unsafe fn CreateTopLevelWindow(&self, aParent: *const nsIAppWindow, aUrl: *const nsIURI, aChromeMask: uint32_t, aInitialWidth: i32, aInitialHeight: i32, _retval: *mut*const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).CreateTopLevelWindow)(self, aParent, aUrl, aChromeMask, aInitialWidth, aInitialHeight, _retval)
    }


    /// ```text
    /// /**
    ///    * This is the constructor for creating an invisible DocShell.
    ///    * It is used to simulate DOM windows without an actual physical
    ///    * representation.
    ///    * @param aIsChrome Set true if you want to use it for chrome content.
    ///    * @param aChromeMask Used to specify chrome flags that should be set on the
    ///    *                    window. See nsIWebBrowserChrome for flag definitions.
    ///    */
    /// ```
    ///

    /// `nsIWindowlessBrowser createWindowlessBrowser ([optional] in bool aIsChrome, [optional] in uint32_t aChromeMask);`
    #[inline]
    pub unsafe fn CreateWindowlessBrowser(&self, aIsChrome: bool, aChromeMask: uint32_t, _retval: *mut*const nsIWindowlessBrowser) -> ::nserror::nsresult {
        ((*self.vtable).CreateWindowlessBrowser)(self, aIsChrome, aChromeMask, _retval)
    }



    /// `[noscript] void createHiddenWindow ();`
    #[inline]
    pub unsafe fn CreateHiddenWindow(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CreateHiddenWindow)(self, )
    }



    /// `void destroyHiddenWindow ();`
    #[inline]
    pub unsafe fn DestroyHiddenWindow(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DestroyHiddenWindow)(self, )
    }


    /// ```text
    /// /**
    ///    * B2G multi-screen support. When open another top-level window on b2g,
    ///    * a screen ID is needed for identifying which screen this window is
    ///    * opened to.
    ///    * @param aScreenId Differentiate screens of windows. It is platform-
    ///    *                  specific due to the hardware limitation for now.
    ///    */
    /// ```
    ///

    /// `[noscript] void setScreenId (in uint32_t aScreenId);`
    #[inline]
    pub unsafe fn SetScreenId(&self, aScreenId: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetScreenId)(self, aScreenId)
    }


    /// ```text
    /// /**
    ///    * Return the (singleton) application hidden window, automatically created
    ///    * and maintained by this AppShellService.
    ///    * @param aResult the hidden window.  Do not unhide hidden window.
    ///    *                Do not taunt hidden window.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAppWindow hiddenWindow;`
    #[inline]
    pub unsafe fn GetHiddenWindow(&self, aHiddenWindow: *mut*const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetHiddenWindow)(self, aHiddenWindow)
    }


    /// ```text
    /// /**
    ///    * Return the (singleton) application hidden window, automatically created
    ///    * and maintained by this AppShellService.
    ///    * @param aResult the hidden window.  Do not unhide hidden window.
    ///    *                Do not taunt hidden window.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy hiddenDOMWindow;`
    #[inline]
    pub unsafe fn GetHiddenDOMWindow(&self, aHiddenDOMWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetHiddenDOMWindow)(self, aHiddenDOMWindow)
    }


    /// ```text
    /// /**
    ///    * Return true if the application hidden window was provided by the
    ///    * application. If it wasn't, the default hidden window was used. This will
    ///    * usually be false on all non-mac platforms.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean applicationProvidedHiddenWindow;`
    #[inline]
    pub unsafe fn GetApplicationProvidedHiddenWindow(&self, aApplicationProvidedHiddenWindow: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetApplicationProvidedHiddenWindow)(self, aApplicationProvidedHiddenWindow)
    }


    /// ```text
    /// /**
    ///    * Add a window to the application's registry of windows.  These windows
    ///    * are generally shown in the Windows taskbar, and the application
    ///    * knows it can't quit until it's out of registered windows.
    ///    * @param aWindow the window to register
    ///    * @note When this method is successful, it fires the global notification
    ///    *       "xul-window-registered"
    ///    */
    /// ```
    ///

    /// `void registerTopLevelWindow (in nsIAppWindow aWindow);`
    #[inline]
    pub unsafe fn RegisterTopLevelWindow(&self, aWindow: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).RegisterTopLevelWindow)(self, aWindow)
    }


    /// ```text
    /// /**
    ///    * Remove a window from the application's window registry. Note that
    ///    * this method won't automatically attempt to quit the app when
    ///    * the last window is unregistered. For that, see Quit().
    ///    * @param aWindow you see the pattern
    ///    */
    /// ```
    ///

    /// `void unregisterTopLevelWindow (in nsIAppWindow aWindow);`
    #[inline]
    pub unsafe fn UnregisterTopLevelWindow(&self, aWindow: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterTopLevelWindow)(self, aWindow)
    }


    /// ```text
    /// /**
    ///    * Whether the hidden window has been lazily created.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean hasHiddenWindow;`
    #[inline]
    pub unsafe fn GetHasHiddenWindow(&self, aHasHiddenWindow: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasHiddenWindow)(self, aHasHiddenWindow)
    }


    /// ```text
    /// /**
    ///    * Start/stop tracking lags in the event loop.
    ///    * If the event loop gets unresponsive, a "event-loop-lag" notification
    ///    * is sent. Note that calling `startEventLoopLagTracking` when tracking
    ///    * is already enabled has no effect.
    ///    * @return true if tracking succeeded.
    ///    */
    /// ```
    ///

    /// `bool startEventLoopLagTracking ();`
    #[inline]
    pub unsafe fn StartEventLoopLagTracking(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).StartEventLoopLagTracking)(self, _retval)
    }



    /// `void stopEventLoopLagTracking ();`
    #[inline]
    pub unsafe fn StopEventLoopLagTracking(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopEventLoopLagTracking)(self, )
    }


}


