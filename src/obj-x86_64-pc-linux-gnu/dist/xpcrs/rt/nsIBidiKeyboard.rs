//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIBidiKeyboard.idl
//


/// `interface nsIBidiKeyboard : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBidiKeyboard {
    vtable: *const nsIBidiKeyboardVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBidiKeyboard.
unsafe impl XpCom for nsIBidiKeyboard {
    const IID: nsIID = nsID(0x288dae24, 0x76e2, 0x43a3,
        [0xbe, 0xfe, 0x9d, 0x9f, 0xab, 0xe8, 0x01, 0x4e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBidiKeyboard {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBidiKeyboard.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBidiKeyboardCoerce {
    /// Cheaply cast a value of this type from a `nsIBidiKeyboard`.
    fn coerce_from(v: &nsIBidiKeyboard) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBidiKeyboardCoerce for nsIBidiKeyboard {
    #[inline]
    fn coerce_from(v: &nsIBidiKeyboard) -> &Self {
        v
    }
}

impl nsIBidiKeyboard {
    /// Cast this `nsIBidiKeyboard` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBidiKeyboardCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBidiKeyboard {
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
impl<T: nsISupportsCoerce> nsIBidiKeyboardCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBidiKeyboard) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBidiKeyboard
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBidiKeyboardVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void reset (); */
    pub Reset: unsafe extern "system" fn (this: *const nsIBidiKeyboard) -> ::nserror::nsresult,

    /* boolean isLangRTL (); */
    pub IsLangRTL: unsafe extern "system" fn (this: *const nsIBidiKeyboard, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean haveBidiKeyboards; */
    pub GetHaveBidiKeyboards: unsafe extern "system" fn (this: *const nsIBidiKeyboard, aHaveBidiKeyboards: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBidiKeyboard {

    /// ```text
    /// /**
    ///    * Inspects the installed keyboards and resets the bidi keyboard state
    ///    */
    /// ```
    ///

    /// `void reset ();`
    #[inline]
    pub unsafe fn Reset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, )
    }


    /// ```text
    /// /**
    ///    * Determines if the current keyboard language is right-to-left
    ///    * @throws NS_ERROR_FAILURE if no right-to-left keyboards are installed
    ///    */
    /// ```
    ///

    /// `boolean isLangRTL ();`
    #[inline]
    pub unsafe fn IsLangRTL(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsLangRTL)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Determines whether the system has at least one keyboard of each direction
    ///    * installed.
    ///    *
    ///    * @throws NS_ERROR_NOT_IMPLEMENTED if the widget layer does not provide this
    ///    * information.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean haveBidiKeyboards;`
    #[inline]
    pub unsafe fn GetHaveBidiKeyboards(&self, aHaveBidiKeyboards: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHaveBidiKeyboards)(self, aHaveBidiKeyboards)
    }


}


