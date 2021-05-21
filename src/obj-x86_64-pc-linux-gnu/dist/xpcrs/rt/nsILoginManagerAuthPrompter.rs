//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/passwordmgr/nsILoginManagerAuthPrompter.idl
//


/// `interface nsILoginManagerAuthPrompter : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILoginManagerAuthPrompter {
    vtable: *const nsILoginManagerAuthPrompterVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILoginManagerAuthPrompter.
unsafe impl XpCom for nsILoginManagerAuthPrompter {
    const IID: nsIID = nsID(0x425f73b9, 0xb2db, 0x4e8a,
        [0x88, 0xc5, 0x9a, 0xc2, 0x51, 0x29, 0x34, 0xce]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILoginManagerAuthPrompter {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILoginManagerAuthPrompter.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILoginManagerAuthPrompterCoerce {
    /// Cheaply cast a value of this type from a `nsILoginManagerAuthPrompter`.
    fn coerce_from(v: &nsILoginManagerAuthPrompter) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILoginManagerAuthPrompterCoerce for nsILoginManagerAuthPrompter {
    #[inline]
    fn coerce_from(v: &nsILoginManagerAuthPrompter) -> &Self {
        v
    }
}

impl nsILoginManagerAuthPrompter {
    /// Cast this `nsILoginManagerAuthPrompter` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILoginManagerAuthPrompterCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILoginManagerAuthPrompter {
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
impl<T: nsISupportsCoerce> nsILoginManagerAuthPrompterCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILoginManagerAuthPrompter) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILoginManagerAuthPrompter
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILoginManagerAuthPrompterVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in nsIDOMWindow aWindow); */
    pub Init: unsafe extern "system" fn (this: *const nsILoginManagerAuthPrompter, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult,

    /* attribute Element browser; */
    pub GetBrowser: unsafe extern "system" fn (this: *const nsILoginManagerAuthPrompter, aBrowser: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Element browser; */
    pub SetBrowser: unsafe extern "system" fn (this: *const nsILoginManagerAuthPrompter, aBrowser: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Element openerBrowser; */
    pub GetOpenerBrowser: unsafe extern "system" fn (this: *const nsILoginManagerAuthPrompter, aOpenerBrowser: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Element openerBrowser; */
    pub SetOpenerBrowser: unsafe extern "system" fn (this: *const nsILoginManagerAuthPrompter, aOpenerBrowser: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILoginManagerAuthPrompter {

    /// ```text
    /// /**
    ///    * Initialize the prompter. Must be called before using other interfaces.
    ///    *
    ///    * @param aWindow
    ///    *        The window in which the user is doing some login-related action that's
    ///    *        resulting in a need to prompt them for something. The prompt
    ///    *        will be associated with this window (or, if a notification bar
        ///    *        is being used, topmost opener in some cases).
    ///    *
    ///    *        aWindow can be null if there is no associated window, e.g. in a JSM
    ///    *        or Sandbox. In this case there will be no checkbox to save the login
    ///    *        since the window is needed to know if this is a private context.
    ///    *
    ///    *        If this window is a content window, the corresponding window and browser
    ///    *        elements will be calculated. If this window is a chrome window, the
    ///    *        corresponding browser element needs to be set using setBrowser.
    ///    */
    /// ```
    ///

    /// `void init (in nsIDOMWindow aWindow);`
    #[inline]
    pub unsafe fn Init(&self, aWindow: *const nsIDOMWindow) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aWindow)
    }


    /// ```text
    /// /**
    ///    * The browser this prompter is being created for.
    ///    * This is required if the init function received a chrome window as argument.
    ///    */
    /// ```
    ///

    /// `attribute Element browser;`
    #[inline]
    pub unsafe fn GetBrowser(&self, aBrowser: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowser)(self, aBrowser)
    }


    /// ```text
    /// /**
    ///    * The browser this prompter is being created for.
    ///    * This is required if the init function received a chrome window as argument.
    ///    */
    /// ```
    ///

    /// `attribute Element browser;`
    #[inline]
    pub unsafe fn SetBrowser(&self, aBrowser: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetBrowser)(self, aBrowser)
    }


    /// ```text
    /// /**
    ///    * The opener browser that was used to open the window passed to init.
    ///    * The opener can be used to determine in which window the prompt
    ///    * should be shown.
    ///    */
    /// ```
    ///

    /// `attribute Element openerBrowser;`
    #[inline]
    pub unsafe fn GetOpenerBrowser(&self, aOpenerBrowser: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetOpenerBrowser)(self, aOpenerBrowser)
    }


    /// ```text
    /// /**
    ///    * The opener browser that was used to open the window passed to init.
    ///    * The opener can be used to determine in which window the prompt
    ///    * should be shown.
    ///    */
    /// ```
    ///

    /// `attribute Element openerBrowser;`
    #[inline]
    pub unsafe fn SetOpenerBrowser(&self, aOpenerBrowser: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetOpenerBrowser)(self, aOpenerBrowser)
    }


}


