//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsISound.idl
//


/// `interface nsISound : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISound {
    vtable: *const nsISoundVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISound.
unsafe impl XpCom for nsISound {
    const IID: nsIID = nsID(0xc3c28d92, 0xa17f, 0x43df,
        [0x97, 0x6d, 0x4e, 0xea, 0xe6, 0xf9, 0x95, 0xfc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISound {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISound.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISoundCoerce {
    /// Cheaply cast a value of this type from a `nsISound`.
    fn coerce_from(v: &nsISound) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISoundCoerce for nsISound {
    #[inline]
    fn coerce_from(v: &nsISound) -> &Self {
        v
    }
}

impl nsISound {
    /// Cast this `nsISound` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISoundCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISound {
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
impl<T: nsISupportsCoerce> nsISoundCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISound) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISound
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISoundVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void play (in nsIURL aURL); */
    pub Play: unsafe extern "system" fn (this: *const nsISound, aURL: *const nsIURL) -> ::nserror::nsresult,

    /* void beep (); */
    pub Beep: unsafe extern "system" fn (this: *const nsISound) -> ::nserror::nsresult,

    /* void init (); */
    pub Init: unsafe extern "system" fn (this: *const nsISound) -> ::nserror::nsresult,

    /* void playEventSound (in unsigned long aEventId); */
    pub PlayEventSound: unsafe extern "system" fn (this: *const nsISound, aEventId: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISound {
    /// ```text
    /// /**
    ///    * In some situations, playEventSound will be called.  Then, each
    ///    * implementations will play a system sound for the event if it's necessary.
    ///    *
    ///    * NOTE: Don't change these values because they are used in
    ///    * nsPIPromptService.idl. So, if they are changed, that makes big impact for
    ///    * the embedders.
    ///    */
    /// ```
    ///

    pub const EVENT_NEW_MAIL_RECEIVED: i64 = 0;


    pub const EVENT_ALERT_DIALOG_OPEN: i64 = 1;


    pub const EVENT_CONFIRM_DIALOG_OPEN: i64 = 2;


    pub const EVENT_PROMPT_DIALOG_OPEN: i64 = 3;


    pub const EVENT_SELECT_DIALOG_OPEN: i64 = 4;


    pub const EVENT_MENU_EXECUTE: i64 = 5;


    pub const EVENT_MENU_POPUP: i64 = 6;


    pub const EVENT_EDITOR_MAX_LEN: i64 = 7;


    /// `void play (in nsIURL aURL);`
    #[inline]
    pub unsafe fn Play(&self, aURL: *const nsIURL) -> ::nserror::nsresult {
        ((*self.vtable).Play)(self, aURL)
    }



    /// `void beep ();`
    #[inline]
    pub unsafe fn Beep(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Beep)(self, )
    }


    /// ```text
    /// /**
    ///     * Not strictly necessary, but avoids delay before first sound.
    ///     * The various methods on nsISound call Init() if they need to.
    /// 	*/
    /// ```
    ///

    /// `void init ();`
    #[inline]
    pub unsafe fn Init(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, )
    }



    /// `void playEventSound (in unsigned long aEventId);`
    #[inline]
    pub unsafe fn PlayEventSound(&self, aEventId: u32) -> ::nserror::nsresult {
        ((*self.vtable).PlayEventSound)(self, aEventId)
    }


}


