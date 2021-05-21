//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/shell/nsIShellService.idl
//


/// `interface nsIShellService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIShellService {
    vtable: *const nsIShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIShellService.
unsafe impl XpCom for nsIShellService {
    const IID: nsIID = nsID(0x2d1a95e4, 0x5bd8, 0x4eeb,
        [0xb0, 0xa8, 0xc1, 0x45, 0x5f, 0xd2, 0xa3, 0x57]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIShellService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIShellService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIShellServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIShellService`.
    fn coerce_from(v: &nsIShellService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIShellServiceCoerce for nsIShellService {
    #[inline]
    fn coerce_from(v: &nsIShellService) -> &Self {
        v
    }
}

impl nsIShellService {
    /// Cast this `nsIShellService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIShellService {
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
impl<T: nsISupportsCoerce> nsIShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIShellService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIShellService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIShellServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean isDefaultBrowser ([optional] in boolean aForAllTypes); */
    pub IsDefaultBrowser: unsafe extern "system" fn (this: *const nsIShellService, aForAllTypes: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers); */
    pub SetDefaultBrowser: unsafe extern "system" fn (this: *const nsIShellService, aClaimAllTypes: bool, aForAllUsers: bool) -> ::nserror::nsresult,

    /* void setDesktopBackground (in Element aElement, in long aPosition, in ACString aImageName); */
    pub SetDesktopBackground: unsafe extern "system" fn (this: *const nsIShellService, aElement: *const libc::c_void, aPosition: i32, aImageName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute unsigned long desktopBackgroundColor; */
    pub GetDesktopBackgroundColor: unsafe extern "system" fn (this: *const nsIShellService, aDesktopBackgroundColor: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long desktopBackgroundColor; */
    pub SetDesktopBackgroundColor: unsafe extern "system" fn (this: *const nsIShellService, aDesktopBackgroundColor: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIShellService {
    /// ```text
    /// /**
    ///    * Flags for positioning/sizing of the Desktop Background image.
    ///    */
    /// ```
    ///

    pub const BACKGROUND_TILE: i64 = 1;


    pub const BACKGROUND_STRETCH: i64 = 2;


    pub const BACKGROUND_CENTER: i64 = 3;


    pub const BACKGROUND_FILL: i64 = 4;


    pub const BACKGROUND_FIT: i64 = 5;


    pub const BACKGROUND_SPAN: i64 = 6;

    /// ```text
    /// /**
    ///    * Determines whether or not Firefox is the "Default Browser."
    ///    * This is simply whether or not Firefox is registered to handle
    ///    * http links.
    ///    *
    ///    * @param aForAllTypes  true if the check should be made for HTTP and HTML.
    ///    *                      false if the check should be made for HTTP only.
    ///    *                      This parameter may be ignored on some platforms.
    ///    */
    /// ```
    ///

    /// `boolean isDefaultBrowser ([optional] in boolean aForAllTypes);`
    #[inline]
    pub unsafe fn IsDefaultBrowser(&self, aForAllTypes: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDefaultBrowser)(self, aForAllTypes, _retval)
    }


    /// ```text
    /// /**
    ///    * Registers Firefox as the "Default Browser."
    ///    *
    ///    * @param aClaimAllTypes Register Firefox as the handler for
    ///    *                       additional protocols (ftp, chrome etc)
    ///    *                       and web documents (.html, .xhtml etc).
    ///    * @param aForAllUsers   Whether or not Firefox should attempt
    ///    *                       to become the default browser for all
    ///    *                       users on a multi-user system.
    ///    */
    /// ```
    ///

    /// `void setDefaultBrowser (in boolean aClaimAllTypes, in boolean aForAllUsers);`
    #[inline]
    pub unsafe fn SetDefaultBrowser(&self, aClaimAllTypes: bool, aForAllUsers: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultBrowser)(self, aClaimAllTypes, aForAllUsers)
    }


    /// ```text
    /// /**
    ///      * Sets the desktop background image using either the HTML <IMG>
    ///      * element supplied or the background image of the element supplied.
    ///      *
    ///      * @param aImageElement Either a HTML <IMG> element or an element with
    ///      *                      a background image from which to source the
    ///      *                      background image.
    ///      * @param aPosition     How to place the image on the desktop
    ///      * @param aImageName    The image name. Equivalent to the leaf name of the
    ///      *                      location.href.
    ///      */
    /// ```
    ///

    /// `void setDesktopBackground (in Element aElement, in long aPosition, in ACString aImageName);`
    #[inline]
    pub unsafe fn SetDesktopBackground(&self, aElement: *const libc::c_void, aPosition: i32, aImageName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDesktopBackground)(self, aElement, aPosition, aImageName)
    }


    /// ```text
    /// /**
    ///    * The desktop background color, visible when no background image is
    ///    * used, or if the background image is centered and does not fill the
    ///    * entire screen. A rgb value, where (r << 16 | g << 8 | b)
    ///    */
    /// ```
    ///

    /// `attribute unsigned long desktopBackgroundColor;`
    #[inline]
    pub unsafe fn GetDesktopBackgroundColor(&self, aDesktopBackgroundColor: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDesktopBackgroundColor)(self, aDesktopBackgroundColor)
    }


    /// ```text
    /// /**
    ///    * The desktop background color, visible when no background image is
    ///    * used, or if the background image is centered and does not fill the
    ///    * entire screen. A rgb value, where (r << 16 | g << 8 | b)
    ///    */
    /// ```
    ///

    /// `attribute unsigned long desktopBackgroundColor;`
    #[inline]
    pub unsafe fn SetDesktopBackgroundColor(&self, aDesktopBackgroundColor: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetDesktopBackgroundColor)(self, aDesktopBackgroundColor)
    }


}


