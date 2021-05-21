//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/browser/nsIEmbeddingSiteWindow.idl
//


/// `interface nsIEmbeddingSiteWindow : nsISupports`
///

/// ```text
/// /**
///  * The nsIEmbeddingSiteWindow is implemented by the embedder to provide
///  * Gecko with the means to call up to the host to resize the window,
///  * hide or show it and set/get its title.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEmbeddingSiteWindow {
    vtable: *const nsIEmbeddingSiteWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEmbeddingSiteWindow.
unsafe impl XpCom for nsIEmbeddingSiteWindow {
    const IID: nsIID = nsID(0x0b976267, 0x4aaa, 0x4f36,
        [0xa2, 0xd4, 0x27, 0xb5, 0xca, 0x8d, 0x73, 0xbb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEmbeddingSiteWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEmbeddingSiteWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEmbeddingSiteWindowCoerce {
    /// Cheaply cast a value of this type from a `nsIEmbeddingSiteWindow`.
    fn coerce_from(v: &nsIEmbeddingSiteWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEmbeddingSiteWindowCoerce for nsIEmbeddingSiteWindow {
    #[inline]
    fn coerce_from(v: &nsIEmbeddingSiteWindow) -> &Self {
        v
    }
}

impl nsIEmbeddingSiteWindow {
    /// Cast this `nsIEmbeddingSiteWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEmbeddingSiteWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEmbeddingSiteWindow {
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
impl<T: nsISupportsCoerce> nsIEmbeddingSiteWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEmbeddingSiteWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEmbeddingSiteWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEmbeddingSiteWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy); */
    pub SetDimensions: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, flags: u32, x: i32, y: i32, cx: i32, cy: i32) -> ::nserror::nsresult,

    /* void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy); */
    pub GetDimensions: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, flags: u32, x: *mut i32, y: *mut i32, cx: *mut i32, cy: *mut i32) -> ::nserror::nsresult,

    /* void setFocus (); */
    pub SetFocus: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow) -> ::nserror::nsresult,

    /* attribute boolean visibility; */
    pub GetVisibility: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, aVisibility: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean visibility; */
    pub SetVisibility: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, aVisibility: bool) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub SetTitle: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [noscript] readonly attribute voidPtr siteWindow; */
    pub GetSiteWindow: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow, aSiteWindow: *mut *mut libc::c_void) -> ::nserror::nsresult,

    /* void blur (); */
    pub Blur: unsafe extern "system" fn (this: *const nsIEmbeddingSiteWindow) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEmbeddingSiteWindow {
    /// ```text
    /// /**
    ///      * Flag indicates that position of the top left corner of the outer area
    ///      * is required/specified.
    ///      *
    ///      * @see setDimensions
    ///      * @see getDimensions
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_POSITION: i64 = 1;

    /// ```text
    /// /**
    ///      * Flag indicates that the size of the inner area is required/specified.
    ///      *
    ///      * @note The inner and outer flags are mutually exclusive and it is
    ///      *       invalid to combine them.
    ///      *
    ///      * @see setDimensions
    ///      * @see getDimensions
    ///      * @see DIM_FLAGS_SIZE_OUTER
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_SIZE_INNER: i64 = 2;

    /// ```text
    /// /**
    ///      * Flag indicates that the size of the outer area is required/specified.
    ///      *
    ///      * @see setDimensions
    ///      * @see getDimensions
    ///      * @see DIM_FLAGS_SIZE_INNER
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_SIZE_OUTER: i64 = 4;

    /// ```text
    /// /**
    ///      * Flag indicates that the x parameter should be ignored.
    ///      *
    ///      * @see setDimensions
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_IGNORE_X: i64 = 8;

    /// ```text
    /// /**
    ///      * Flag indicates that the y parameter should be ignored.
    ///      *
    ///      * @see setDimensions
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_IGNORE_Y: i64 = 16;

    /// ```text
    /// /**
    ///      * Flag indicates that the cx parameter should be ignored.
    ///      *
    ///      * @see setDimensions
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_IGNORE_CX: i64 = 32;

    /// ```text
    /// /**
    ///      * Flag indicates that the cy parameter should be ignored.
    ///      *
    ///      * @see setDimensions
    ///      */
    /// ```
    ///

    pub const DIM_FLAGS_IGNORE_CY: i64 = 64;

    /// ```text
    /// /**
    ///      * Sets the dimensions for the window; the position & size. The
    ///      * flags to indicate what the caller wants to set and whether the size
    ///      * refers to the inner or outer area. The inner area refers to just
    ///      * the embedded area, wheras the outer area can also include any
    ///      * surrounding chrome, window frame, title bar, and so on.
    ///      *
    ///      * @param flags  Combination of position, inner and outer size flags.
    ///      *               The ignore flags are telling the parent to use the
    ///      *               current values for those dimensions and ignore the
    ///      *               corresponding parameters the child sends.
    ///      * @param x      Left hand corner of the outer area.
    ///      * @param y      Top corner of the outer area.
    ///      * @param cx     Width of the inner or outer area.
    ///      * @param cy     Height of the inner or outer area.
    ///      *
    ///      * @return <code>NS_OK</code> if operation was performed correctly;
    ///      *         <code>NS_ERROR_UNEXPECTED</code> if window could not be
    ///      *           destroyed;
    ///      *         <code>NS_ERROR_INVALID_ARG</code> for bad flag combination
    ///      *           or illegal dimensions.
    ///      *
    ///      * @see getDimensions
    ///      * @see DIM_FLAGS_POSITION
    ///      * @see DIM_FLAGS_SIZE_OUTER
    ///      * @see DIM_FLAGS_SIZE_INNER
    ///      */
    /// ```
    ///

    /// `void setDimensions (in unsigned long flags, in long x, in long y, in long cx, in long cy);`
    #[inline]
    pub unsafe fn SetDimensions(&self, flags: u32, x: i32, y: i32, cx: i32, cy: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetDimensions)(self, flags, x, y, cx, cy)
    }


    /// ```text
    /// /**
    ///      * Gets the dimensions of the window. The caller may pass
    ///      * <CODE>nullptr</CODE> for any value it is uninterested in receiving.
    ///      *
    ///      * @param flags  Combination of position, inner and outer size flag .
    ///      * @param x      Left hand corner of the outer area; or <CODE>nullptr</CODE>.
    ///      * @param y      Top corner of the outer area; or <CODE>nullptr</CODE>.
    ///      * @param cx     Width of the inner or outer area; or <CODE>nullptr</CODE>.
    ///      * @param cy     Height of the inner or outer area; or <CODE>nullptr</CODE>.
    ///      *
    ///      * @see setDimensions
    ///      * @see DIM_FLAGS_POSITION
    ///      * @see DIM_FLAGS_SIZE_OUTER
    ///      * @see DIM_FLAGS_SIZE_INNER
    ///      */
    /// ```
    ///

    /// `void getDimensions (in unsigned long flags, out long x, out long y, out long cx, out long cy);`
    #[inline]
    pub unsafe fn GetDimensions(&self, flags: u32, x: *mut i32, y: *mut i32, cx: *mut i32, cy: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetDimensions)(self, flags, x, y, cx, cy)
    }


    /// ```text
    /// /**
    ///      * Give the window focus.
    ///      */
    /// ```
    ///

    /// `void setFocus ();`
    #[inline]
    pub unsafe fn SetFocus(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetFocus)(self, )
    }


    /// ```text
    /// /**
    ///      * Visibility of the window.
    ///      */
    /// ```
    ///

    /// `attribute boolean visibility;`
    #[inline]
    pub unsafe fn GetVisibility(&self, aVisibility: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetVisibility)(self, aVisibility)
    }


    /// ```text
    /// /**
    ///      * Visibility of the window.
    ///      */
    /// ```
    ///

    /// `attribute boolean visibility;`
    #[inline]
    pub unsafe fn SetVisibility(&self, aVisibility: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetVisibility)(self, aVisibility)
    }


    /// ```text
    /// /**
    ///      * Title of the window.
    ///      */
    /// ```
    ///

    /// `attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///      * Title of the window.
    ///      */
    /// ```
    ///

    /// `attribute AString title;`
    #[inline]
    pub unsafe fn SetTitle(&self, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///      * Native window for the site's window. The implementor should copy the
    ///      * native window object into the address supplied by the caller. The
    ///      * type of the native window that the address refers to is  platform
    ///      * and OS specific as follows:
    ///      *
    ///      * <ul>
    ///      *   <li>On Win32 it is an <CODE>HWND</CODE>.</li>
    ///      *   <li>On MacOS this is a <CODE>WindowPtr</CODE>.</li>
    ///      *   <li>On GTK this is a <CODE>GtkWidget*</CODE>.</li>
    ///      * </ul>
    ///      */
    /// ```
    ///

    /// `[noscript] readonly attribute voidPtr siteWindow;`
    #[inline]
    pub unsafe fn GetSiteWindow(&self, aSiteWindow: *mut *mut libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSiteWindow)(self, aSiteWindow)
    }


    /// ```text
    /// /**
    ///      * Blur the window. This should unfocus the window and send an onblur event.
    ///      */
    /// ```
    ///

    /// `void blur ();`
    #[inline]
    pub unsafe fn Blur(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Blur)(self, )
    }


}


