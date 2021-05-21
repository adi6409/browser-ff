//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIScreen.idl
//


/// `interface nsIScreen : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScreen {
    vtable: *const nsIScreenVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScreen.
unsafe impl XpCom for nsIScreen {
    const IID: nsIID = nsID(0x826e80c8, 0xd70f, 0x42e2,
        [0x8a, 0xa9, 0x82, 0xc0, 0x5f, 0x2a, 0x37, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScreen {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScreen.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScreenCoerce {
    /// Cheaply cast a value of this type from a `nsIScreen`.
    fn coerce_from(v: &nsIScreen) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScreenCoerce for nsIScreen {
    #[inline]
    fn coerce_from(v: &nsIScreen) -> &Self {
        v
    }
}

impl nsIScreen {
    /// Cast this `nsIScreen` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScreenCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScreen {
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
impl<T: nsISupportsCoerce> nsIScreenCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScreen) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScreen
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScreenVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void GetRect (out long left, out long top, out long width, out long height); */
    pub GetRect: unsafe extern "system" fn (this: *const nsIScreen, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void GetAvailRect (out long left, out long top, out long width, out long height); */
    pub GetAvailRect: unsafe extern "system" fn (this: *const nsIScreen, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void GetRectDisplayPix (out long left, out long top, out long width, out long height); */
    pub GetRectDisplayPix: unsafe extern "system" fn (this: *const nsIScreen, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height); */
    pub GetAvailRectDisplayPix: unsafe extern "system" fn (this: *const nsIScreen, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long pixelDepth; */
    pub GetPixelDepth: unsafe extern "system" fn (this: *const nsIScreen, aPixelDepth: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long colorDepth; */
    pub GetColorDepth: unsafe extern "system" fn (this: *const nsIScreen, aColorDepth: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute double contentsScaleFactor; */
    pub GetContentsScaleFactor: unsafe extern "system" fn (this: *const nsIScreen, aContentsScaleFactor: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double defaultCSSScaleFactor; */
    pub GetDefaultCSSScaleFactor: unsafe extern "system" fn (this: *const nsIScreen, aDefaultCSSScaleFactor: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute float dpi; */
    pub GetDpi: unsafe extern "system" fn (this: *const nsIScreen, aDpi: *mut libc::c_float) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScreen {

    /// ```text
    /// /**
    ///    * These report screen dimensions in (screen-specific) device pixels
    ///    */
    /// ```
    ///

    /// `void GetRect (out long left, out long top, out long width, out long height);`
    #[inline]
    pub unsafe fn GetRect(&self, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRect)(self, left, top, width, height)
    }



    /// `void GetAvailRect (out long left, out long top, out long width, out long height);`
    #[inline]
    pub unsafe fn GetAvailRect(&self, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetAvailRect)(self, left, top, width, height)
    }


    /// ```text
    /// /**
    ///    * And these report in desktop pixels
    ///    */
    /// ```
    ///

    /// `void GetRectDisplayPix (out long left, out long top, out long width, out long height);`
    #[inline]
    pub unsafe fn GetRectDisplayPix(&self, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRectDisplayPix)(self, left, top, width, height)
    }



    /// `void GetAvailRectDisplayPix (out long left, out long top, out long width, out long height);`
    #[inline]
    pub unsafe fn GetAvailRectDisplayPix(&self, left: *mut i32, top: *mut i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetAvailRectDisplayPix)(self, left, top, width, height)
    }



    /// `readonly attribute long pixelDepth;`
    #[inline]
    pub unsafe fn GetPixelDepth(&self, aPixelDepth: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPixelDepth)(self, aPixelDepth)
    }



    /// `readonly attribute long colorDepth;`
    #[inline]
    pub unsafe fn GetColorDepth(&self, aColorDepth: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetColorDepth)(self, aColorDepth)
    }


    /// ```text
    /// /**
    ///    * The number of device pixels per desktop pixel for this screen (for
        ///    * hidpi configurations where there may be multiple device pixels per
        ///    * desktop px and/or per CSS px).
    ///    *
    ///    * This seems poorly named (something like devicePixelsPerDesktopPixel
        ///    * would be more accurate/explicit), but given that it is exposed to
    ///    * front-end code and may also be used by add-ons, it's probably not
    ///    * worth the disruption of changing it.
    ///    *
    ///    * Returns 1.0 if HiDPI mode is disabled or unsupported, or if the
    ///    * host OS uses device pixels as its desktop pixel units (e.g. Windows 7 or
        ///    * GTK/X11). Per-monitor DPI is available in Windows 8.1+, GTK/Wayland or
    ///    * macOS.
    ///    */
    /// ```
    ///

    /// `readonly attribute double contentsScaleFactor;`
    #[inline]
    pub unsafe fn GetContentsScaleFactor(&self, aContentsScaleFactor: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetContentsScaleFactor)(self, aContentsScaleFactor)
    }


    /// ```text
    /// /**
    ///    * The default number of device pixels per unscaled CSS pixel for this
    ///    * screen. This is probably what contentsScaleFactor originally meant
    ///    * to be, prior to confusion between CSS pixels and desktop pixel units.
    ///    */
    /// ```
    ///

    /// `readonly attribute double defaultCSSScaleFactor;`
    #[inline]
    pub unsafe fn GetDefaultCSSScaleFactor(&self, aDefaultCSSScaleFactor: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultCSSScaleFactor)(self, aDefaultCSSScaleFactor)
    }


    /// ```text
    /// /**
    ///    * The DPI of the screen.
    ///    */
    /// ```
    ///

    /// `readonly attribute float dpi;`
    #[inline]
    pub unsafe fn GetDpi(&self, aDpi: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetDpi)(self, aDpi)
    }


}


