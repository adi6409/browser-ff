//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIFocusManager.idl
//


/// `interface nsIFocusManager : nsISupports`
///

/// ```text
/// /**
///  * The focus manager deals with all focus related behaviour. Only one element
///  * in the entire application may have the focus at a time; this element
///  * receives any keyboard events. While there is only one application-wide
///  * focused element, each nsIDOMWindow maintains a reference to the element
///  * that would be focused if the window was active.
///  *
///  * If the window's reference is to a frame element (iframe, browser,
    ///  * editor), then the child window contains the element that is currently
///  * focused. If the window's reference is to a root element, then the root is
///  * focused. If a window's reference is null, then no element is focused, yet
///  * the window is still focused.
///  *
///  * The blur event is fired on an element when it loses the application focus.
///  * After this blur event, if the focus is moving away from a document, two
///  * additional blur events are fired on the old document and window containing
///  * the focus respectively.
///  *
///  * When a new document is focused, two focus events are fired on the new
///  * document and window respectively. Then the focus event is fired on an
///  * element when it gains the application focus.
///  *
///  * A special case is that the root element may be focused, yet does not
///  * receive the element focus and blur events. Instead a focus outline may be
///  * drawn around the document.
///  *
///  * Blur and focus events do not bubble as per the W3C DOM Events spec.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFocusManager {
    vtable: *const nsIFocusManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFocusManager.
unsafe impl XpCom for nsIFocusManager {
    const IID: nsIID = nsID(0x86e1f1e1, 0x365d, 0x493b,
        [0xb5, 0x2a, 0xa6, 0x49, 0xf3, 0xf3, 0x11, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFocusManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFocusManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFocusManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIFocusManager`.
    fn coerce_from(v: &nsIFocusManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFocusManagerCoerce for nsIFocusManager {
    #[inline]
    fn coerce_from(v: &nsIFocusManager) -> &Self {
        v
    }
}

impl nsIFocusManager {
    /// Cast this `nsIFocusManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFocusManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFocusManager {
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
impl<T: nsISupportsCoerce> nsIFocusManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFocusManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFocusManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFocusManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute mozIDOMWindowProxy activeWindow; */
    pub GetActiveWindow: unsafe extern "system" fn (this: *const nsIFocusManager, aActiveWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute BrowsingContext activeBrowsingContext; */
    pub GetActiveBrowsingContext: unsafe extern "system" fn (this: *const nsIFocusManager, aActiveBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute mozIDOMWindowProxy focusedWindow; */
    pub GetFocusedWindow: unsafe extern "system" fn (this: *const nsIFocusManager, aFocusedWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* attribute mozIDOMWindowProxy focusedWindow; */
    pub SetFocusedWindow: unsafe extern "system" fn (this: *const nsIFocusManager, aFocusedWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute BrowsingContext focusedContentBrowsingContext; */
    pub GetFocusedContentBrowsingContext: unsafe extern "system" fn (this: *const nsIFocusManager, aFocusedContentBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute Element focusedElement; */
    pub GetFocusedElement: unsafe extern "system" fn (this: *const nsIFocusManager, aFocusedElement: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* uint32_t getLastFocusMethod (in mozIDOMWindowProxy window); */
    pub GetLastFocusMethod: unsafe extern "system" fn (this: *const nsIFocusManager, window: *const mozIDOMWindowProxy, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [can_run_script] void setFocus (in Element aElement, in unsigned long aFlags); */
    pub SetFocus: unsafe extern "system" fn (this: *const nsIFocusManager, aElement: *const libc::c_void, aFlags: u32) -> ::nserror::nsresult,

    /* Element moveFocus (in mozIDOMWindowProxy aWindow, in Element aStartElement, in unsigned long aType, in unsigned long aFlags); */
    pub MoveFocus: unsafe extern "system" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy, aStartElement: *const libc::c_void, aType: u32, aFlags: u32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void clearFocus (in mozIDOMWindowProxy aWindow); */
    pub ClearFocus: unsafe extern "system" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* Element getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow); */
    pub GetFocusedElementForWindow: unsafe extern "system" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy, aDeep: bool, aFocusedWindow: *mut*const mozIDOMWindowProxy, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void moveCaretToFocus (in mozIDOMWindowProxy aWindow); */
    pub MoveCaretToFocus: unsafe extern "system" fn (this: *const nsIFocusManager, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* boolean elementIsFocusable (in Element aElement, in unsigned long aFlags); */
    pub ElementIsFocusable: unsafe extern "system" fn (this: *const nsIFocusManager, aElement: *const libc::c_void, aFlags: u32, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFocusManager {

    pub const FLAG_RAISE: i64 = 1;

    /// ```text
    /// /**
    ///    * Do not scroll the element to focus into view.
    ///    */
    /// ```
    ///

    pub const FLAG_NOSCROLL: i64 = 2;

    /// ```text
    /// /**
    ///    * If attempting to change focus in a window that is not focused, do not
    ///    * switch focus to that window. Instead, just update the focus within that
    ///    * window and leave the application focus as is. This flag will have no
    ///    * effect if a child window is focused and an attempt is made to adjust the
    ///    * focus in an ancestor, as the frame must be switched in this case.
    ///    */
    /// ```
    ///

    pub const FLAG_NOSWITCHFRAME: i64 = 4;

    /// ```text
    /// /**
    ///    * This flag is only used when passed to moveFocus. If set, focus is never
    ///    * moved to the parent frame of the starting element's document, instead
    ///    * iterating around to the beginning of that document again. Child frames
    ///    * are navigated as normal.
    ///    */
    /// ```
    ///

    pub const FLAG_NOPARENTFRAME: i64 = 8;

    /// ```text
    /// /**
    ///    * This flag is used for window and element focus operations to signal
    ///    * wether the caller is system or non system.
    ///    */
    /// ```
    ///

    pub const FLAG_NONSYSTEMCALLER: i64 = 16;

    /// ```text
    /// /**
    ///    * Focus is changing due to a mouse operation, for instance the mouse was
    ///    * clicked on an element.
    ///    */
    /// ```
    ///

    pub const FLAG_BYMOUSE: i64 = 4096;

    /// ```text
    /// /**
    ///    * Focus is changing due to a key operation, for instance pressing the tab
    ///    * key. This flag would normally be passed when MOVEFOCUS_FORWARD or
    ///    * MOVEFOCUS_BACKWARD is used.
    ///    */
    /// ```
    ///

    pub const FLAG_BYKEY: i64 = 8192;

    /// ```text
    /// /**
    ///    * Focus is changing due to a call to MoveFocus. This flag will be implied
    ///    * when MoveFocus is called except when one of the other mechanisms (mouse
        ///    * or key) is specified, or when the type is MOVEFOCUS_ROOT or
    ///    * MOVEFOCUS_CARET.
    ///    */
    /// ```
    ///

    pub const FLAG_BYMOVEFOCUS: i64 = 16384;

    /// ```text
    /// /**
    ///    * Always show the focus ring or other indicator of focus, regardless of
    ///    * other state.
    ///    */
    /// ```
    ///

    pub const FLAG_SHOWRING: i64 = 1048576;

    /// ```text
    /// /**
    ///    * Focus is changing due to a touch operation that generated a mouse event.
    ///    * Normally used in conjunction with FLAG_BYMOUSE.
    ///    */
    /// ```
    ///

    pub const FLAG_BYTOUCH: i64 = 2097152;


    pub const FLAG_BYLONGPRESS: i64 = 8388608;

    /// ```text
    /// /** move focus forward one element, used when pressing TAB */
    /// ```
    ///

    pub const MOVEFOCUS_FORWARD: i64 = 1;

    /// ```text
    /// /** move focus backward one element, used when pressing Shift+TAB */
    /// ```
    ///

    pub const MOVEFOCUS_BACKWARD: i64 = 2;

    /// ```text
    /// /** move focus forward to the next frame document, used when pressing F6 */
    /// ```
    ///

    pub const MOVEFOCUS_FORWARDDOC: i64 = 3;

    /// ```text
    /// /** move focus forward to the previous frame document, used when pressing Shift+F6 */
    /// ```
    ///

    pub const MOVEFOCUS_BACKWARDDOC: i64 = 4;

    /// ```text
    /// /** move focus to the first focusable element */
    /// ```
    ///

    pub const MOVEFOCUS_FIRST: i64 = 5;

    /// ```text
    /// /** move focus to the last focusable element */
    /// ```
    ///

    pub const MOVEFOCUS_LAST: i64 = 6;

    /// ```text
    /// /** move focus to the root element in the document */
    /// ```
    ///

    pub const MOVEFOCUS_ROOT: i64 = 7;

    /// ```text
    /// /** move focus to a link at the position of the caret. This is a special value used to
    ///    *  focus links as the caret moves over them in caret browsing mode.
    ///    */
    /// ```
    ///

    pub const MOVEFOCUS_CARET: i64 = 8;

    /// ```text
    /// /** move focus to the first focusable document */
    /// ```
    ///

    pub const MOVEFOCUS_FIRSTDOC: i64 = 9;

    /// ```text
    /// /** move focus to the last focusable document */
    /// ```
    ///

    pub const MOVEFOCUS_LASTDOC: i64 = 10;

    /// ```text
    /// /**
    ///    * The most active (frontmost) window, or null if no window that is part of
    ///    * the application is active. Do not use outside the parent process.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy activeWindow;`
    #[inline]
    pub unsafe fn GetActiveWindow(&self, aActiveWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveWindow)(self, aActiveWindow)
    }


    /// ```text
    /// /**
    ///    * In the parent process: The BrowsingContext corresponding to activeWindow.
    ///    * In content processes: The top-level Web content browsing context that
    ///    * focus is in if the application is active and focus is in Web content.
    ///    */
    /// ```
    ///

    /// `readonly attribute BrowsingContext activeBrowsingContext;`
    #[inline]
    pub unsafe fn GetActiveBrowsingContext(&self, aActiveBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetActiveBrowsingContext)(self, aActiveBrowsingContext)
    }


    /// ```text
    /// /**
    ///    * The child window within the activeWindow that is focused. This will
    ///    * always be activeWindow, a child window of activeWindow or null if no
    ///    * child window is focused. Setting the focusedWindow changes the focused
    ///    * window and raises the toplevel window it is in. If the current focus
    ///    * within the new focusedWindow is a frame element, then the focusedWindow
    ///    * will actually be set to the child window and the current element within
    ///    * that set as the focused element. This process repeats downwards until a
    ///    * non-frame element is found.
    ///    * The setter for this attribute defaults to CallerType::System.
    ///    */
    /// ```
    ///

    /// `attribute mozIDOMWindowProxy focusedWindow;`
    #[inline]
    pub unsafe fn GetFocusedWindow(&self, aFocusedWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedWindow)(self, aFocusedWindow)
    }


    /// ```text
    /// /**
    ///    * The child window within the activeWindow that is focused. This will
    ///    * always be activeWindow, a child window of activeWindow or null if no
    ///    * child window is focused. Setting the focusedWindow changes the focused
    ///    * window and raises the toplevel window it is in. If the current focus
    ///    * within the new focusedWindow is a frame element, then the focusedWindow
    ///    * will actually be set to the child window and the current element within
    ///    * that set as the focused element. This process repeats downwards until a
    ///    * non-frame element is found.
    ///    * The setter for this attribute defaults to CallerType::System.
    ///    */
    /// ```
    ///

    /// `attribute mozIDOMWindowProxy focusedWindow;`
    #[inline]
    pub unsafe fn SetFocusedWindow(&self, aFocusedWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).SetFocusedWindow)(self, aFocusedWindow)
    }


    /// ```text
    /// /**
    ///    * Parent-process only: The content BrowsingContext that currently has focus,
    ///    * if any. Note this can be different from activeBrowsingContext in the case
    ///    * of subframes.
    ///    */
    /// ```
    ///

    /// `readonly attribute BrowsingContext focusedContentBrowsingContext;`
    #[inline]
    pub unsafe fn GetFocusedContentBrowsingContext(&self, aFocusedContentBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedContentBrowsingContext)(self, aFocusedContentBrowsingContext)
    }


    /// ```text
    /// /**
    ///    * The element that is currently focused. This will always be an element
    ///    * within the document loaded in focusedWindow or null if no element in that
    ///    * document is focused.
    ///    */
    /// ```
    ///

    /// `readonly attribute Element focusedElement;`
    #[inline]
    pub unsafe fn GetFocusedElement(&self, aFocusedElement: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedElement)(self, aFocusedElement)
    }


    /// ```text
    /// /**
    ///    * Returns the method that was used to focus the element in window. This
    ///    * will either be 0, FLAG_BYMOUSE or FLAG_BYKEY. If window is null, then
    ///    * the current focusedWindow will be used by default. This has the result
    ///    * of retrieving the method that was used to focus the currently focused
    ///    * element.
    ///    */
    /// ```
    ///

    /// `uint32_t getLastFocusMethod (in mozIDOMWindowProxy window);`
    #[inline]
    pub unsafe fn GetLastFocusMethod(&self, window: *const mozIDOMWindowProxy, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLastFocusMethod)(self, window, _retval)
    }


    /// ```text
    /// /**
    ///    * Changes the focused element reference within the window containing
    ///    * aElement to aElement or potentially redirects it to an anonymous
    ///    * descendant of it (e.g., for `<input type="number">` the focus is redirected
        ///    * to its descendant `<input type="text">`).
    ///    */
    /// ```
    ///

    /// `[can_run_script] void setFocus (in Element aElement, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn SetFocus(&self, aElement: *const libc::c_void, aFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetFocus)(self, aElement, aFlags)
    }


    /// ```text
    /// /**
    ///    * Move the focus to another element. If aStartElement is specified, then
    ///    * movement is done relative to aStartElement. If aStartElement is null,
    ///    * then movement is done relative to the currently focused element. If no
    ///    * element is focused, focus the first focusable element within the
    ///    * document (or the last focusable element if aType is MOVEFOCUS_END). This
    ///    * method is equivalent to setting the focusedElement to the new element.
    ///    *
    ///    * Specifying aStartElement and using MOVEFOCUS_LAST is not currently
    ///    * implemented.
    ///    *
    ///    * If no element is found, and aType is either MOVEFOCUS_ROOT or
    ///    * MOVEFOCUS_CARET, then the focus is cleared. If aType is any other value,
    ///    * the focus is not changed.
    ///    *
    ///    * Returns the element that was focused (see setFocus). The return value
    ///    * may be null if focus was moved into a child process.
    ///    */
    /// ```
    ///

    /// `Element moveFocus (in mozIDOMWindowProxy aWindow, in Element aStartElement, in unsigned long aType, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn MoveFocus(&self, aWindow: *const mozIDOMWindowProxy, aStartElement: *const libc::c_void, aType: u32, aFlags: u32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).MoveFocus)(self, aWindow, aStartElement, aType, aFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Clears the focused element within aWindow. If the current focusedWindow
    ///    * is a descendant of aWindow, sets the current focusedWindow to aWindow.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if aWindow is null
    ///    */
    /// ```
    ///

    /// `void clearFocus (in mozIDOMWindowProxy aWindow);`
    #[inline]
    pub unsafe fn ClearFocus(&self, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).ClearFocus)(self, aWindow)
    }


    /// ```text
    /// /**
    ///    * Returns the currently focused element within aWindow. If aWindow is equal
    ///    * to the current value of focusedWindow, then the returned element will be
    ///    * the application-wide focused element (the value of focusedElement). The
    ///    * return value will be null if no element is focused.
    ///    *
    ///    * If aDeep is true, then child frames are traversed and the return value
    ///    * may be the element within a child descendant window that is focused. If
    ///    * aDeep if false, then the return value will be the frame element if the
    ///    * focus is in a child frame.
    ///    *
    ///    * aFocusedWindow will be set to the currently focused descendant window of
    ///    * aWindow, or to aWindow if aDeep is false. This will be set even if no
    ///    * element is focused.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if aWindow is null
    ///    */
    /// ```
    ///

    /// `Element getFocusedElementForWindow (in mozIDOMWindowProxy aWindow, in boolean aDeep, out mozIDOMWindowProxy aFocusedWindow);`
    #[inline]
    pub unsafe fn GetFocusedElementForWindow(&self, aWindow: *const mozIDOMWindowProxy, aDeep: bool, aFocusedWindow: *mut*const mozIDOMWindowProxy, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetFocusedElementForWindow)(self, aWindow, aDeep, aFocusedWindow, _retval)
    }


    /// ```text
    /// /**
    ///    * Moves the selection caret within aWindow to the current focus.
    ///    */
    /// ```
    ///

    /// `void moveCaretToFocus (in mozIDOMWindowProxy aWindow);`
    #[inline]
    pub unsafe fn MoveCaretToFocus(&self, aWindow: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).MoveCaretToFocus)(self, aWindow)
    }


    /// ```text
    /// /***
    ///    * Check if given element (or potentially a descendant, see setFocus) is
    ///    * focusable.
    ///    */
    /// ```
    ///

    /// `boolean elementIsFocusable (in Element aElement, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn ElementIsFocusable(&self, aElement: *const libc::c_void, aFlags: u32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ElementIsFocusable)(self, aElement, aFlags, _retval)
    }


}


