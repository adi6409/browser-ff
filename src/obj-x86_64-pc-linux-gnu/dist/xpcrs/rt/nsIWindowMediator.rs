//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpfe/appshell/nsIWindowMediator.idl
//


/// `interface nsIWindowMediator : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWindowMediator {
    vtable: *const nsIWindowMediatorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWindowMediator.
unsafe impl XpCom for nsIWindowMediator {
    const IID: nsIID = nsID(0xdf0da056, 0x357d, 0x427f,
        [0xba, 0xfd, 0xe6, 0xcb, 0xf1, 0x9c, 0x93, 0x81]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWindowMediator {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWindowMediator.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWindowMediatorCoerce {
    /// Cheaply cast a value of this type from a `nsIWindowMediator`.
    fn coerce_from(v: &nsIWindowMediator) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWindowMediatorCoerce for nsIWindowMediator {
    #[inline]
    fn coerce_from(v: &nsIWindowMediator) -> &Self {
        v
    }
}

impl nsIWindowMediator {
    /// Cast this `nsIWindowMediator` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWindowMediatorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWindowMediator {
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
impl<T: nsISupportsCoerce> nsIWindowMediatorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWindowMediator) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWindowMediator
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWindowMediatorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsISimpleEnumerator getEnumerator (in wstring aWindowType); */
    pub GetEnumerator: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindowType: *const i16, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* nsISimpleEnumerator getAppWindowEnumerator (in wstring aWindowType); */
    pub GetAppWindowEnumerator: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindowType: *const i16, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* nsISimpleEnumerator getZOrderAppWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack); */
    pub GetZOrderAppWindowEnumerator: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindowType: *const i16, aFrontToBack: bool, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult,

    /* mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType); */
    pub GetMostRecentWindow: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindowType: *const i16, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* mozIDOMWindowProxy getMostRecentBrowserWindow (); */
    pub GetMostRecentBrowserWindow: unsafe extern "system" fn (this: *const nsIWindowMediator, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType); */
    pub GetMostRecentNonPBWindow: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindowType: *const i16, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID); */
    pub GetOuterWindowWithId: unsafe extern "system" fn (this: *const nsIWindowMediator, aOuterWindowID: u64, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID); */
    pub GetCurrentInnerWindowWithId: unsafe extern "system" fn (this: *const nsIWindowMediator, aInnerWindowID: u64, _retval: *mut*const mozIDOMWindow) -> ::nserror::nsresult,

    /* [noscript] void registerWindow (in nsIAppWindow aWindow); */
    pub RegisterWindow: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindow: *const nsIAppWindow) -> ::nserror::nsresult,

    /* [noscript] void unregisterWindow (in nsIAppWindow aWindow); */
    pub UnregisterWindow: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindow: *const nsIAppWindow) -> ::nserror::nsresult,

    /* [noscript] void updateWindowTimeStamp (in nsIAppWindow aWindow); */
    pub UpdateWindowTimeStamp: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindow: *const nsIAppWindow) -> ::nserror::nsresult,

    /* [noscript] boolean calculateZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow); */
    pub CalculateZPosition: unsafe extern "system" fn (this: *const nsIWindowMediator, inWindow: *const nsIAppWindow, inPosition: u32, inBelow: *const nsIWidget, outPosition: *mut u32, outBelow: *mut*const nsIWidget, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void setZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIAppWindow inBelow); */
    pub SetZPosition: unsafe extern "system" fn (this: *const nsIWindowMediator, inWindow: *const nsIAppWindow, inPosition: u32, inBelow: *const nsIAppWindow) -> ::nserror::nsresult,

    /* [noscript] uint32_t getZLevel (in nsIAppWindow aWindow); */
    pub GetZLevel: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindow: *const nsIAppWindow, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [noscript] void setZLevel (in nsIAppWindow aWindow, in uint32_t aZLevel); */
    pub SetZLevel: unsafe extern "system" fn (this: *const nsIWindowMediator, aWindow: *const nsIAppWindow, aZLevel: uint32_t) -> ::nserror::nsresult,

    /* void addListener (in nsIWindowMediatorListener aListener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsIWindowMediator, aListener: *const nsIWindowMediatorListener) -> ::nserror::nsresult,

    /* void removeListener (in nsIWindowMediatorListener aListener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsIWindowMediator, aListener: *const nsIWindowMediatorListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWindowMediator {

    pub const zLevelTop: i64 = 1;


    pub const zLevelBottom: i64 = 2;


    pub const zLevelBelow: i64 = 3;

    /// ```text
    /// /** Return an enumerator which iterates over all windows of type aWindowType
    ///     * from the oldest window to the youngest.
    ///     * @param  aWindowType the returned enumerator will enumerate only
    ///     *                     windows of this type. ("type" is the
        ///     *                     |windowtype| attribute of the XML <window> element.)
    ///     *                     If null, all windows will be enumerated.
    ///     * @return an enumerator of nsIDOMWindows.  Note that windows close
    ///     *         asynchronously in many cases, so windows returned from this
    ///     *         enumerator can have .closed set to true.  Caveat enumerator!
    ///     */
    /// ```
    ///

    /// `nsISimpleEnumerator getEnumerator (in wstring aWindowType);`
    #[inline]
    pub unsafe fn GetEnumerator(&self, aWindowType: *const i16, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetEnumerator)(self, aWindowType, _retval)
    }


    /// ```text
    /// /** Identical to getEnumerator except:
    ///     * @return an enumerator of nsIAppWindows
    ///   */
    /// ```
    ///

    /// `nsISimpleEnumerator getAppWindowEnumerator (in wstring aWindowType);`
    #[inline]
    pub unsafe fn GetAppWindowEnumerator(&self, aWindowType: *const i16, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetAppWindowEnumerator)(self, aWindowType, _retval)
    }


    /// ```text
    /// /** Return an enumerator which iterates over all windows of type aWindowType
    ///     * in their z (front-to-back) order. Note this interface makes
    ///     * no requirement that a window couldn't be revisited if windows
    ///     * are re-ordered while z-order enumerators are active.
    ///     * @param  aWindowType the returned enumerator will enumerate only
    ///     *                     windows of this type. ("type" is the
        ///     *                     |windowtype| attribute of the XML <window> element.)
    ///     *                     If null, all windows will be enumerated.
    ///     * @param  aFrontToBack if true, the enumerator enumerates windows in order
    ///     *                      from front to back. back to front if false.
    ///     * @return an enumerator of nsIAppWindows
    ///     */
    /// ```
    ///

    /// `nsISimpleEnumerator getZOrderAppWindowEnumerator (in wstring aWindowType, in boolean aFrontToBack);`
    #[inline]
    pub unsafe fn GetZOrderAppWindowEnumerator(&self, aWindowType: *const i16, aFrontToBack: bool, _retval: *mut *const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetZOrderAppWindowEnumerator)(self, aWindowType, aFrontToBack, _retval)
    }


    /// ```text
    /// /** This is a shortcut for simply fetching the first window in
    ///     * front to back order.
    ///     * @param  aWindowType return the topmost window of this type.
    ///     *                     ("type" is the |windowtype| attribute of
        ///     *                     the XML <window> element.)
    ///     *                     If null, return the topmost window of any type.
    ///     * @return the topmost window
    ///     */
    /// ```
    ///

    /// `mozIDOMWindowProxy getMostRecentWindow (in wstring aWindowType);`
    #[inline]
    pub unsafe fn GetMostRecentWindow(&self, aWindowType: *const i16, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetMostRecentWindow)(self, aWindowType, _retval)
    }


    /// ```text
    /// /** This is a shortcut for getMostRecentWindow('navigator:browser'), but
    ///     * if that fails it also tries 'navigator:geckoview' and 'mail:3pane'.
    ///     *
    ///     * @return the topmost browser window
    ///     */
    /// ```
    ///

    /// `mozIDOMWindowProxy getMostRecentBrowserWindow ();`
    #[inline]
    pub unsafe fn GetMostRecentBrowserWindow(&self, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetMostRecentBrowserWindow)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Same as getMostRecentWindow, but ignores private browsing
    ///    * windows.
    ///    */
    /// ```
    ///

    /// `mozIDOMWindowProxy getMostRecentNonPBWindow (in wstring aWindowType);`
    #[inline]
    pub unsafe fn GetMostRecentNonPBWindow(&self, aWindowType: *const i16, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetMostRecentNonPBWindow)(self, aWindowType, _retval)
    }


    /// ```text
    /// /**
    ///    * Return the outer window with the given ID, if any.  Can return null.
    ///    */
    /// ```
    ///

    /// `mozIDOMWindowProxy getOuterWindowWithId (in unsigned long long aOuterWindowID);`
    #[inline]
    pub unsafe fn GetOuterWindowWithId(&self, aOuterWindowID: u64, _retval: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetOuterWindowWithId)(self, aOuterWindowID, _retval)
    }


    /// ```text
    /// /**
    ///     * Return the inner window with the given current window ID, if any.
    ///     * Can return null if no inner window with the ID exists or if it's not
    ///     * a current inner anymore.
    ///     */
    /// ```
    ///

    /// `mozIDOMWindow getCurrentInnerWindowWithId (in unsigned long long aInnerWindowID);`
    #[inline]
    pub unsafe fn GetCurrentInnerWindowWithId(&self, aInnerWindowID: u64, _retval: *mut*const mozIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentInnerWindowWithId)(self, aInnerWindowID, _retval)
    }


    /// ```text
    /// /** Add the window to the list of known windows. Listeners (see
        ///     * addListener) will be notified through their onOpenWindow method.
    ///     * @param aWindow the window to add
    ///     */
    /// ```
    ///

    /// `[noscript] void registerWindow (in nsIAppWindow aWindow);`
    #[inline]
    pub unsafe fn RegisterWindow(&self, aWindow: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).RegisterWindow)(self, aWindow)
    }


    /// ```text
    /// /** Remove the window from the list of known windows. Listeners (see
        ///     * addListener) will be be notified through their onCloseWindow method.
    ///     * @param aWindow the window to remove
    ///     */
    /// ```
    ///

    /// `[noscript] void unregisterWindow (in nsIAppWindow aWindow);`
    #[inline]
    pub unsafe fn UnregisterWindow(&self, aWindow: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterWindow)(self, aWindow)
    }


    /// ```text
    /// /** Call this method when a window gains focus. It's a primitive means of
    ///     * determining the most recent window. It's no longer necessary and it
    ///     * really should be removed.
    ///     * @param aWindow the window which has gained focus
    ///     */
    /// ```
    ///

    /// `[noscript] void updateWindowTimeStamp (in nsIAppWindow aWindow);`
    #[inline]
    pub unsafe fn UpdateWindowTimeStamp(&self, aWindow: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).UpdateWindowTimeStamp)(self, aWindow)
    }


    /// ```text
    /// /** A window wants to be moved in z-order. Calculate whether and how
    ///     * it should be constrained. Note this method is advisory only:
    ///     * it changes nothing either in WindowMediator's internal state
    ///     * or with the window.
    ///     * Note it compares the nsIAppWindow to nsIWidgets. A pure interface
    ///     * would use all nsIAppWindows. But we expect this to be called from
    ///     * callbacks originating in native window code. They are expected to
    ///     * hand us comparison values which are pulled from general storage
    ///     * in the native widget, and may not correspond to an nsIWidget at all.
    ///     * For that reason this interface requires only objects one step
    ///     * removed from the native window (nsIWidgets), and its implementation
    ///     * must be very understanding of what may be completely invalid
    ///     * pointers in those parameters.
    ///     *
    ///     * @param inWindow the window in question
    ///     * @param inPosition requested position
    ///     *                   values: zLevelTop: topmost window. zLevelBottom: bottom.
    ///     *                   zLevelBelow: below ioBelow. (the value of ioBelow will
        ///     *                   be ignored for zLevelTop and Bottom.)
    ///     * @param inBelow if inPosition==zLevelBelow, the window
    ///     *                 below which inWindow wants to be placed. Otherwise this
    ///     *                 variable is ignored.
    ///     * @param outPosition constrained position, values like inPosition.
    ///     * @param outBelow if outPosition==zLevelBelow, the window
    ///     *                 below which inWindow should be placed. Otherwise this
    ///     *                 this value will be null.
    ///     * @return PR_TRUE if the position returned is different from
    ///     *         the position given.
    ///     */
    /// ```
    ///

    /// `[noscript] boolean calculateZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIWidget inBelow, out unsigned long outPosition, out nsIWidget outBelow);`
    #[inline]
    pub unsafe fn CalculateZPosition(&self, inWindow: *const nsIAppWindow, inPosition: u32, inBelow: *const nsIWidget, outPosition: *mut u32, outBelow: *mut*const nsIWidget, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CalculateZPosition)(self, inWindow, inPosition, inBelow, outPosition, outBelow, _retval)
    }


    /// ```text
    /// /** A window has been positioned behind another. Inform WindowMediator
    ///     * @param inWindow the window in question
    ///     * @param inPosition new position. values:
    ///     *                   zLevelTop: topmost window.
    ///     *                   zLevelBottom: bottom.
    ///     *                   zLevelBelow: below inBelow. (inBelow is ignored
        ///     *                                for other values of inPosition.)
    ///     * @param inBelow the window inWindow is behind, if zLevelBelow
    ///     */
    /// ```
    ///

    /// `[noscript] void setZPosition (in nsIAppWindow inWindow, in unsigned long inPosition, in nsIAppWindow inBelow);`
    #[inline]
    pub unsafe fn SetZPosition(&self, inWindow: *const nsIAppWindow, inPosition: u32, inBelow: *const nsIAppWindow) -> ::nserror::nsresult {
        ((*self.vtable).SetZPosition)(self, inWindow, inPosition, inBelow)
    }


    /// ```text
    /// /** Return the window's Z level (as defined in nsIAppWindow).
    ///     * @param aWindow the window in question
    ///     * @return aWindow's z level
    ///     */
    /// ```
    ///

    /// `[noscript] uint32_t getZLevel (in nsIAppWindow aWindow);`
    #[inline]
    pub unsafe fn GetZLevel(&self, aWindow: *const nsIAppWindow, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetZLevel)(self, aWindow, _retval)
    }


    /// ```text
    /// /** Set the window's Z level (as defined in nsIAppWindow). The implementation
    ///     * will reposition the window as necessary to match its new Z level.
    ///     * The implementation will assume a window's Z level to be
    ///     * nsIAppWindow::normalZ until it has been informed of a different level.
    ///     * @param aWindow the window in question
    ///     * @param aZLevel the window's new Z level
    ///     */
    /// ```
    ///

    /// `[noscript] void setZLevel (in nsIAppWindow aWindow, in uint32_t aZLevel);`
    #[inline]
    pub unsafe fn SetZLevel(&self, aWindow: *const nsIAppWindow, aZLevel: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetZLevel)(self, aWindow, aZLevel)
    }


    /// ```text
    /// /** Register a listener for window status changes.
    ///     * keeps strong ref? (to be decided)
    ///     * @param aListener the listener to register
    ///     */
    /// ```
    ///

    /// `void addListener (in nsIWindowMediatorListener aListener);`
    #[inline]
    pub unsafe fn AddListener(&self, aListener: *const nsIWindowMediatorListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, aListener)
    }


    /// ```text
    /// /** Unregister a listener of window status changes.
    ///     * @param aListener the listener to unregister
    ///     */
    /// ```
    ///

    /// `void removeListener (in nsIWindowMediatorListener aListener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, aListener: *const nsIWindowMediatorListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, aListener)
    }


}


