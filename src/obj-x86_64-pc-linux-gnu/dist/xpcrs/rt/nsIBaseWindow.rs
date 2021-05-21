//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIBaseWindow.idl
//


/// `typedef void *  nativeWindow;`
///


pub type nativeWindow = *const libc::c_void;


/// `interface nsIBaseWindow : nsISupports`
///

/// ```text
/// /**
///  * The nsIBaseWindow describes a generic window and basic operations that
///  * can be performed on it.  This is not to be a complete windowing interface
///  * but rather a common set that nearly all windowed objects support.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBaseWindow {
    vtable: *const nsIBaseWindowVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBaseWindow.
unsafe impl XpCom for nsIBaseWindow {
    const IID: nsIID = nsID(0xca635529, 0xa977, 0x4552,
        [0x9b, 0x8a, 0x66, 0x18, 0x7e, 0x54, 0xd8, 0x82]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBaseWindow {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBaseWindow.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBaseWindowCoerce {
    /// Cheaply cast a value of this type from a `nsIBaseWindow`.
    fn coerce_from(v: &nsIBaseWindow) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBaseWindowCoerce for nsIBaseWindow {
    #[inline]
    fn coerce_from(v: &nsIBaseWindow) -> &Self {
        v
    }
}

impl nsIBaseWindow {
    /// Cast this `nsIBaseWindow` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBaseWindowCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBaseWindow {
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
impl<T: nsISupportsCoerce> nsIBaseWindowCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBaseWindow) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBaseWindow
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBaseWindowVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void initWindow (in nativeWindow parentNativeWindow, in nsIWidget parentWidget, in long x, in long y, in long cx, in long cy); */
    /// Unable to generate binding because `native type nsIWidget unsupported`
    pub InitWindow: *const ::libc::c_void,

    /* void destroy (); */
    pub Destroy: unsafe extern "system" fn (this: *const nsIBaseWindow) -> ::nserror::nsresult,

    /* void setPosition (in long x, in long y); */
    pub SetPosition: unsafe extern "system" fn (this: *const nsIBaseWindow, x: i32, y: i32) -> ::nserror::nsresult,

    /* void setPositionDesktopPix (in long x, in long y); */
    pub SetPositionDesktopPix: unsafe extern "system" fn (this: *const nsIBaseWindow, x: i32, y: i32) -> ::nserror::nsresult,

    /* void getPosition (out long x, out long y); */
    pub GetPosition: unsafe extern "system" fn (this: *const nsIBaseWindow, x: *mut i32, y: *mut i32) -> ::nserror::nsresult,

    /* void setSize (in long cx, in long cy, in boolean fRepaint); */
    pub SetSize: unsafe extern "system" fn (this: *const nsIBaseWindow, cx: i32, cy: i32, fRepaint: bool) -> ::nserror::nsresult,

    /* void getSize (out long cx, out long cy); */
    pub GetSize: unsafe extern "system" fn (this: *const nsIBaseWindow, cx: *mut i32, cy: *mut i32) -> ::nserror::nsresult,

    /* void setPositionAndSize (in long x, in long y, in long cx, in long cy, in unsigned long flags); */
    pub SetPositionAndSize: unsafe extern "system" fn (this: *const nsIBaseWindow, x: i32, y: i32, cx: i32, cy: i32, flags: u32) -> ::nserror::nsresult,

    /* void getPositionAndSize (out long x, out long y, out long cx, out long cy); */
    pub GetPositionAndSize: unsafe extern "system" fn (this: *const nsIBaseWindow, x: *mut i32, y: *mut i32, cx: *mut i32, cy: *mut i32) -> ::nserror::nsresult,

    /* void repaint (in boolean force); */
    pub Repaint: unsafe extern "system" fn (this: *const nsIBaseWindow, force: bool) -> ::nserror::nsresult,

    /* [noscript] attribute nsIWidget parentWidget; */
    /// Unable to generate binding because `native type nsIWidget unsupported`
    pub GetParentWidget: *const ::libc::c_void,

    /* [noscript] attribute nsIWidget parentWidget; */
    /// Unable to generate binding because `native type nsIWidget unsupported`
    pub SetParentWidget: *const ::libc::c_void,

    /* attribute nativeWindow parentNativeWindow; */
    pub GetParentNativeWindow: unsafe extern "system" fn (this: *const nsIBaseWindow, aParentNativeWindow: *mut nativeWindow) -> ::nserror::nsresult,

    /* attribute nativeWindow parentNativeWindow; */
    pub SetParentNativeWindow: unsafe extern "system" fn (this: *const nsIBaseWindow, aParentNativeWindow: nativeWindow) -> ::nserror::nsresult,

    /* readonly attribute AString nativeHandle; */
    pub GetNativeHandle: unsafe extern "system" fn (this: *const nsIBaseWindow, aNativeHandle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute boolean visibility; */
    pub GetVisibility: unsafe extern "system" fn (this: *const nsIBaseWindow, aVisibility: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean visibility; */
    pub SetVisibility: unsafe extern "system" fn (this: *const nsIBaseWindow, aVisibility: bool) -> ::nserror::nsresult,

    /* attribute boolean enabled; */
    pub GetEnabled: unsafe extern "system" fn (this: *const nsIBaseWindow, aEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean enabled; */
    pub SetEnabled: unsafe extern "system" fn (this: *const nsIBaseWindow, aEnabled: bool) -> ::nserror::nsresult,

    /* [noscript] readonly attribute nsIWidget mainWidget; */
    /// Unable to generate binding because `native type nsIWidget unsupported`
    pub GetMainWidget: *const ::libc::c_void,

    /* readonly attribute double unscaledDevicePixelsPerCSSPixel; */
    pub GetUnscaledDevicePixelsPerCSSPixel: unsafe extern "system" fn (this: *const nsIBaseWindow, aUnscaledDevicePixelsPerCSSPixel: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double devicePixelsPerDesktopPixel; */
    pub GetDevicePixelsPerDesktopPixel: unsafe extern "system" fn (this: *const nsIBaseWindow, aDevicePixelsPerDesktopPixel: *mut libc::c_double) -> ::nserror::nsresult,

    /* void setFocus (); */
    pub SetFocus: unsafe extern "system" fn (this: *const nsIBaseWindow) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsIBaseWindow, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString title; */
    pub SetTitle: unsafe extern "system" fn (this: *const nsIBaseWindow, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBaseWindow {
    /// ```text
    /// /**
    /// 	 * The 'flags' argument to setPositionAndSize is a set of these bits.
    /// 	 */
    /// ```
    ///

    pub const eRepaint: i64 = 1;


    pub const eDelayResize: i64 = 2;


    /// `[noscript] void initWindow (in nativeWindow parentNativeWindow, in nsIWidget parentWidget, in long x, in long y, in long cx, in long cy);`
    const _InitWindow: () = ();


    /// `void destroy ();`
    #[inline]
    pub unsafe fn Destroy(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Destroy)(self, )
    }



    /// `void setPosition (in long x, in long y);`
    #[inline]
    pub unsafe fn SetPosition(&self, x: i32, y: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetPosition)(self, x, y)
    }



    /// `void setPositionDesktopPix (in long x, in long y);`
    #[inline]
    pub unsafe fn SetPositionDesktopPix(&self, x: i32, y: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetPositionDesktopPix)(self, x, y)
    }



    /// `void getPosition (out long x, out long y);`
    #[inline]
    pub unsafe fn GetPosition(&self, x: *mut i32, y: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPosition)(self, x, y)
    }



    /// `void setSize (in long cx, in long cy, in boolean fRepaint);`
    #[inline]
    pub unsafe fn SetSize(&self, cx: i32, cy: i32, fRepaint: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSize)(self, cx, cy, fRepaint)
    }



    /// `void getSize (out long cx, out long cy);`
    #[inline]
    pub unsafe fn GetSize(&self, cx: *mut i32, cy: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSize)(self, cx, cy)
    }



    /// `void setPositionAndSize (in long x, in long y, in long cx, in long cy, in unsigned long flags);`
    #[inline]
    pub unsafe fn SetPositionAndSize(&self, x: i32, y: i32, cx: i32, cy: i32, flags: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetPositionAndSize)(self, x, y, cx, cy, flags)
    }



    /// `void getPositionAndSize (out long x, out long y, out long cx, out long cy);`
    #[inline]
    pub unsafe fn GetPositionAndSize(&self, x: *mut i32, y: *mut i32, cx: *mut i32, cy: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPositionAndSize)(self, x, y, cx, cy)
    }


    /// ```text
    /// /**
    /// 	 * Tell the window to repaint itself
    /// 	 * @param aForce - if true, repaint immediately
    /// 	 *                 if false, the window may defer repainting as it sees fit.
    /// 	 */
    /// ```
    ///

    /// `void repaint (in boolean force);`
    #[inline]
    pub unsafe fn Repaint(&self, force: bool) -> ::nserror::nsresult {
        ((*self.vtable).Repaint)(self, force)
    }



    /// `[noscript] attribute nsIWidget parentWidget;`
    const _GetParentWidget: () = ();


    /// `[noscript] attribute nsIWidget parentWidget;`
    const _SetParentWidget: () = ();


    /// `attribute nativeWindow parentNativeWindow;`
    #[inline]
    pub unsafe fn GetParentNativeWindow(&self, aParentNativeWindow: *mut nativeWindow) -> ::nserror::nsresult {
        ((*self.vtable).GetParentNativeWindow)(self, aParentNativeWindow)
    }



    /// `attribute nativeWindow parentNativeWindow;`
    #[inline]
    pub unsafe fn SetParentNativeWindow(&self, aParentNativeWindow: nativeWindow) -> ::nserror::nsresult {
        ((*self.vtable).SetParentNativeWindow)(self, aParentNativeWindow)
    }



    /// `readonly attribute AString nativeHandle;`
    #[inline]
    pub unsafe fn GetNativeHandle(&self, aNativeHandle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNativeHandle)(self, aNativeHandle)
    }



    /// `attribute boolean visibility;`
    #[inline]
    pub unsafe fn GetVisibility(&self, aVisibility: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetVisibility)(self, aVisibility)
    }



    /// `attribute boolean visibility;`
    #[inline]
    pub unsafe fn SetVisibility(&self, aVisibility: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetVisibility)(self, aVisibility)
    }



    /// `attribute boolean enabled;`
    #[inline]
    pub unsafe fn GetEnabled(&self, aEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEnabled)(self, aEnabled)
    }



    /// `attribute boolean enabled;`
    #[inline]
    pub unsafe fn SetEnabled(&self, aEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEnabled)(self, aEnabled)
    }



    /// `[noscript] readonly attribute nsIWidget mainWidget;`
    const _GetMainWidget: () = ();


    /// `readonly attribute double unscaledDevicePixelsPerCSSPixel;`
    #[inline]
    pub unsafe fn GetUnscaledDevicePixelsPerCSSPixel(&self, aUnscaledDevicePixelsPerCSSPixel: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetUnscaledDevicePixelsPerCSSPixel)(self, aUnscaledDevicePixelsPerCSSPixel)
    }



    /// `readonly attribute double devicePixelsPerDesktopPixel;`
    #[inline]
    pub unsafe fn GetDevicePixelsPerDesktopPixel(&self, aDevicePixelsPerDesktopPixel: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDevicePixelsPerDesktopPixel)(self, aDevicePixelsPerDesktopPixel)
    }


    /// ```text
    /// /**
    /// 	* Give the window focus.
    /// 	*/
    /// ```
    ///

    /// `void setFocus ();`
    #[inline]
    pub unsafe fn SetFocus(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SetFocus)(self, )
    }



    /// `attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }



    /// `attribute AString title;`
    #[inline]
    pub unsafe fn SetTitle(&self, aTitle: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTitle)(self, aTitle)
    }


}


