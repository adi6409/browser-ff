//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIWindowWatcher.idl
//


/// `interface nsIWindowWatcher : nsISupports`
///

/// ```text
/// /**
///  * nsIWindowWatcher is the keeper of Gecko/DOM Windows. It maintains
///  * a list of open top-level windows, and allows some operations on them.
///
///  * Usage notes:
///
///  *   This component has an |activeWindow| property. Clients may expect
///  * this property to be always current, so to properly integrate this component
///  * the application will need to keep it current by setting the property
///  * as the active window changes.
///  *   This component should not keep a (XPCOM) reference to any windows;
///  * the implementation will claim no ownership. Windows must notify
///  * this component when they are created or destroyed, so only a weak
///  * reference is kept. Note that there is no interface for such notifications
///  * (not a public one, anyway). This is taken care of both in Mozilla and
///  * by common embedding code. Embedding clients need do nothing special
///  * about that requirement.
///  *   This component must be initialized at application startup by calling
///  * setWindowCreator.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWindowWatcher {
    vtable: *const nsIWindowWatcherVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWindowWatcher.
unsafe impl XpCom for nsIWindowWatcher {
    const IID: nsIID = nsID(0x641fe945, 0x6902, 0x4b3f,
        [0x87, 0xc2, 0x0d, 0xae, 0xf3, 0x24, 0x99, 0xb3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWindowWatcher {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWindowWatcher.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWindowWatcherCoerce {
    /// Cheaply cast a value of this type from a `nsIWindowWatcher`.
    fn coerce_from(v: &nsIWindowWatcher) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWindowWatcherCoerce for nsIWindowWatcher {
    #[inline]
    fn coerce_from(v: &nsIWindowWatcher) -> &Self {
        v
    }
}

impl nsIWindowWatcher {
    /// Cast this `nsIWindowWatcher` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWindowWatcherCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWindowWatcher {
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
impl<T: nsISupportsCoerce> nsIWindowWatcherCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowWatcher) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWindowWatcher
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWindowWatcherVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in nsISupports aArguments); */
    pub OpenWindow: unsafe extern "system" fn (this: *const nsIWindowWatcher, aParent: *const mozIDOMWindowProxy, aUrl: *const ::nsstring::nsACString, aName: *const ::nsstring::nsACString, aFeatures: *const ::nsstring::nsACString, aArguments: *const nsISupports, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* void registerNotification (in nsIObserver aObserver); */
    pub RegisterNotification: unsafe extern "system" fn (this: *const nsIWindowWatcher, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* void unregisterNotification (in nsIObserver aObserver); */
    pub UnregisterNotification: unsafe extern "system" fn (this: *const nsIWindowWatcher, aObserver: *const nsIObserver) -> ::nserror::nsresult,

    /* nsISimpleEnumerator getWindowEnumerator (); */
    pub GetWindowEnumerator: unsafe extern "system" fn (this: *const nsIWindowWatcher, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent); */
    pub GetNewPrompter: unsafe extern "system" fn (this: *const nsIWindowWatcher, aParent: *const mozIDOMWindowProxy, _retval: *mut*const nsIPrompt) -> ::nserror::nsresult,

    /* nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent); */
    pub GetNewAuthPrompter: unsafe extern "system" fn (this: *const nsIWindowWatcher, aParent: *const mozIDOMWindowProxy, _retval: *mut*const nsIAuthPrompt) -> ::nserror::nsresult,

    /* void setWindowCreator (in nsIWindowCreator creator); */
    pub SetWindowCreator: unsafe extern "system" fn (this: *const nsIWindowWatcher, creator: *const nsIWindowCreator) -> ::nserror::nsresult,

    /* boolean hasWindowCreator (); */
    pub HasWindowCreator: unsafe extern "system" fn (this: *const nsIWindowWatcher, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow); */
    pub GetChromeForWindow: unsafe extern "system" fn (this: *const nsIWindowWatcher, aWindow: *const mozIDOMWindowProxy, _retval: *mut*const nsIWebBrowserChrome) -> ::nserror::nsresult,

    /* mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow); */
    pub GetWindowByName: unsafe extern "system" fn (this: *const nsIWindowWatcher, aTargetName: *const ::nsstring::nsAString, aCurrentWindow: *const mozIDOMWindowProxy, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy activeWindow; */
    pub GetActiveWindow: unsafe extern "system" fn (this: *const nsIWindowWatcher, aActiveWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWindowWatcher {

    /// ```text
    /// /** Create a new window. It will automatically be added to our list
    ///       (via addWindow()).
    ///       @param aParent parent window, if any. Null if no parent.  If it is
    ///              impossible to get to an nsIWebBrowserChrome from aParent, this
    ///              method will effectively act as if aParent were null.
    ///       @param aURL url to which to open the new window. Must already be
    ///              escaped, if applicable. can be null.
    ///       @param aName window name from JS window.open. can be null.  If a window
    ///              with this name already exists, the openWindow call may just load
    ///              aUrl in it (if aUrl is not null) and return it.
    ///       @param aFeatures window features from JS window.open. can be null.
    ///       @param aArguments extra argument(s) to the new window, to be attached
    ///              as the |arguments| property. An nsIArray will be
    ///              unwound into multiple arguments (but not recursively!).
    ///              can be null.
    ///       @return the new window
    ///
    ///       @note This method may examine the JS context stack for purposes of
    ///             determining the security context to use for the search for a given
    ///             window named aName.
    ///       @note This method should try to set the default charset for the new
    ///             window to the default charset of aParent.  This is not guaranteed,
    ///             however.
    ///       @note This method may dispatch a "toplevel-window-ready" notification
    ///             via nsIObserverService if the window did not already exist.
    ///   */
    /// ```
    ///

    /// `mozIDOMWindowProxy openWindow (in mozIDOMWindowProxy aParent, in ACString aUrl, in ACString aName, in ACString aFeatures, in nsISupports aArguments);`
    #[inline]
    pub unsafe fn OpenWindow(&self, aParent: *const mozIDOMWindowProxy, aUrl: *const ::nsstring::nsACString, aName: *const ::nsstring::nsACString, aFeatures: *const ::nsstring::nsACString, aArguments: *const nsISupports, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).OpenWindow)(self, aParent, aUrl, aName, aFeatures, aArguments, _retval)
    }


    /// ```text
    /// /** Clients of this service can register themselves to be notified
    ///       when a window is opened or closed (added to or removed from this
        ///       service). This method adds an aObserver to the list of objects
    ///       to be notified.
    ///       @param aObserver the object to be notified when windows are
    ///                        opened or closed. Its Observe method will be
    ///                        called with the following parameters:
    ///
    ///       aObserver::Observe interprets its parameters so:
    ///       aSubject the window being opened or closed, sent as an nsISupports
    ///                which can be QIed to an nsIDOMWindow.
    ///       aTopic   a wstring, either "domwindowopened" or "domwindowclosed".
    ///       someData not used.
    ///   */
    /// ```
    ///

    /// `void registerNotification (in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn RegisterNotification(&self, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).RegisterNotification)(self, aObserver)
    }


    /// ```text
    /// /** Clients of this service can register themselves to be notified
    ///       when a window is opened or closed (added to or removed from this
        ///       service). This method removes an aObserver from the list of objects
    ///       to be notified.
    ///       @param aObserver the observer to be removed.
    ///   */
    /// ```
    ///

    /// `void unregisterNotification (in nsIObserver aObserver);`
    #[inline]
    pub unsafe fn UnregisterNotification(&self, aObserver: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterNotification)(self, aObserver)
    }


    /// ```text
    /// /** Get an iterator for currently open windows in the order they were opened,
    ///       guaranteeing that each will be visited exactly once.
    ///       @return an enumerator which will itself return nsISupports objects which
    ///               can be QIed to an nsIDOMWindow
    ///   */
    /// ```
    ///

    /// `nsISimpleEnumerator getWindowEnumerator ();`
    #[inline]
    pub unsafe fn GetWindowEnumerator(&self, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowEnumerator)(self, _retval)
    }


    /// ```text
    /// /** Return a newly created nsIPrompt implementation.
    ///       @param aParent the parent window used for posing alerts. can be null.
    ///       @return a new nsIPrompt object
    ///   */
    /// ```
    ///

    /// `nsIPrompt getNewPrompter (in mozIDOMWindowProxy aParent);`
    #[inline]
    pub unsafe fn GetNewPrompter(&self, aParent: *const mozIDOMWindowProxy, _retval: *mut*const nsIPrompt) -> ::nserror::nsresult {
        ((*self.vtable).GetNewPrompter)(self, aParent, _retval)
    }


    /// ```text
    /// /** Return a newly created nsIAuthPrompt implementation.
    ///       @param aParent the parent window used for posing alerts. can be null.
    ///       @return a new nsIAuthPrompt object
    ///   */
    /// ```
    ///

    /// `nsIAuthPrompt getNewAuthPrompter (in mozIDOMWindowProxy aParent);`
    #[inline]
    pub unsafe fn GetNewAuthPrompter(&self, aParent: *const mozIDOMWindowProxy, _retval: *mut*const nsIAuthPrompt) -> ::nserror::nsresult {
        ((*self.vtable).GetNewAuthPrompter)(self, aParent, _retval)
    }


    /// ```text
    /// /** Set the window creator callback. It must be filled in by the app.
    ///       openWindow will use it to create new windows.
    ///       @param creator the callback. if null, the callback will be cleared
    ///                      and window creation capabilities lost.
    ///   */
    /// ```
    ///

    /// `void setWindowCreator (in nsIWindowCreator creator);`
    #[inline]
    pub unsafe fn SetWindowCreator(&self, creator: *const nsIWindowCreator) -> ::nserror::nsresult {
        ((*self.vtable).SetWindowCreator)(self, creator)
    }


    /// ```text
    /// /** Returns true if a window creator callback has been set, false otherwise.
    ///   */
    /// ```
    ///

    /// `boolean hasWindowCreator ();`
    #[inline]
    pub unsafe fn HasWindowCreator(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasWindowCreator)(self, _retval)
    }


    /// ```text
    /// /** Retrieve the chrome window mapped to the given DOM window. Window
    ///       Watcher keeps a list of all top-level DOM windows currently open,
    ///       along with their corresponding chrome interfaces. Since DOM Windows
    ///       lack a (public) means of retrieving their corresponding chrome,
    ///       this method will do that.
    ///       @param aWindow the DOM window whose chrome window the caller needs
    ///       @return the corresponding chrome window
    ///   */
    /// ```
    ///

    /// `nsIWebBrowserChrome getChromeForWindow (in mozIDOMWindowProxy aWindow);`
    #[inline]
    pub unsafe fn GetChromeForWindow(&self, aWindow: *const mozIDOMWindowProxy, _retval: *mut*const nsIWebBrowserChrome) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeForWindow)(self, aWindow, _retval)
    }


    /// ```text
    /// /**
    ///       Retrieve an existing window (or frame).
    ///       @param aTargetName the window name
    ///       @param aCurrentWindow a starting point in the window hierarchy to
    ///                             begin the search.  If null, each toplevel window
    ///                             will be searched.
    ///
    ///       Note: This method will search all open windows for any window or
    ///       frame with the given window name. Make sure you understand the
    ///       security implications of this before using this method!
    ///   */
    /// ```
    ///

    /// `mozIDOMWindowProxy getWindowByName (in AString aTargetName, in mozIDOMWindowProxy aCurrentWindow);`
    #[inline]
    pub unsafe fn GetWindowByName(&self, aTargetName: *const ::nsstring::nsAString, aCurrentWindow: *const mozIDOMWindowProxy, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetWindowByName)(self, aTargetName, aCurrentWindow, _retval)
    }


    /// ```text
    /// /**
    ///       Retrieves the active window from the focus manager.
    ///   */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy activeWindow;`
    #[inline]
    pub unsafe fn GetActiveWindow(&self, aActiveWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveWindow)(self, aActiveWindow)
    }


}


