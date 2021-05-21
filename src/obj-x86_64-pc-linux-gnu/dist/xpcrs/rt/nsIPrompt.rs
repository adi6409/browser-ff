//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPrompt.idl
//


/// `interface nsIPrompt : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrompt {
    vtable: *const nsIPromptVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrompt.
unsafe impl XpCom for nsIPrompt {
    const IID: nsIID = nsID(0xa63f70c0, 0x148b, 0x11d3,
        [0x93, 0x33, 0x00, 0x10, 0x4b, 0xa0, 0xfd, 0x40]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrompt {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrompt.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPromptCoerce {
    /// Cheaply cast a value of this type from a `nsIPrompt`.
    fn coerce_from(v: &nsIPrompt) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPromptCoerce for nsIPrompt {
    #[inline]
    fn coerce_from(v: &nsIPrompt) -> &Self {
        v
    }
}

impl nsIPrompt {
    /// Cast this `nsIPrompt` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPromptCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrompt {
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
impl<T: nsISupportsCoerce> nsIPromptCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrompt) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrompt
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPromptVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void alert (in wstring dialogTitle, in wstring text); */
    pub Alert: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16) -> ::nserror::nsresult,

    /* void alertCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
    pub AlertCheck: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, checkMsg: *const i16, checkValue: *mut bool) -> ::nserror::nsresult,

    /* boolean confirm (in wstring dialogTitle, in wstring text); */
    pub Confirm: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean confirmCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue); */
    pub ConfirmCheck: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* int32_t confirmEx (in wstring dialogTitle, in wstring text, in unsigned long buttonFlags, in wstring button0Title, in wstring button1Title, in wstring button2Title, in wstring checkMsg, inout boolean checkValue); */
    pub ConfirmEx: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, buttonFlags: u32, button0Title: *const i16, button1Title: *const i16, button2Title: *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut int32_t) -> ::nserror::nsresult,

    /* boolean prompt (in wstring dialogTitle, in wstring text, inout wstring value, in wstring checkMsg, inout boolean checkValue); */
    pub Prompt: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, value: *mut *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptPassword (in wstring dialogTitle, in wstring text, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
    pub PromptPassword: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, password: *mut *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, inout wstring username, inout wstring password, in wstring checkMsg, inout boolean checkValue); */
    pub PromptUsernameAndPassword: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, username: *mut *const i16, password: *mut *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean select (in wstring dialogTitle, in wstring text, in Array<AString> selectList, out long outSelection); */
    pub Select: unsafe extern "system" fn (this: *const nsIPrompt, dialogTitle: *const i16, text: *const i16, selectList: *const thin_vec::ThinVec<::nsstring::nsString>, outSelection: *mut i32, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrompt {

    pub const BUTTON_POS_0: i64 = 1;


    pub const BUTTON_POS_1: i64 = 256;


    pub const BUTTON_POS_2: i64 = 65536;


    pub const BUTTON_TITLE_OK: i64 = 1;


    pub const BUTTON_TITLE_CANCEL: i64 = 2;


    pub const BUTTON_TITLE_YES: i64 = 3;


    pub const BUTTON_TITLE_NO: i64 = 4;


    pub const BUTTON_TITLE_SAVE: i64 = 5;


    pub const BUTTON_TITLE_DONT_SAVE: i64 = 6;


    pub const BUTTON_TITLE_REVERT: i64 = 7;


    pub const BUTTON_TITLE_IS_STRING: i64 = 127;


    pub const BUTTON_POS_0_DEFAULT: i64 = 0;


    pub const BUTTON_POS_1_DEFAULT: i64 = 16777216;


    pub const BUTTON_POS_2_DEFAULT: i64 = 33554432;


    pub const BUTTON_DELAY_ENABLE: i64 = 67108864;


    pub const STD_OK_CANCEL_BUTTONS: i64 = 513;


    pub const STD_YES_NO_BUTTONS: i64 = 1027;


    pub const MODAL_TYPE_CONTENT: i64 = 1;


    pub const MODAL_TYPE_TAB: i64 = 2;


    pub const MODAL_TYPE_WINDOW: i64 = 3;


    pub const MODAL_TYPE_INTERNAL_WINDOW: i64 = 4;


    /// `void alert (in wstring dialogTitle, in wstring text);`
    #[inline]
    pub unsafe fn Alert(&self, dialogTitle: *const i16, text: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).Alert)(self, dialogTitle, text)
    }



    /// `void alertCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue);`
    #[inline]
    pub unsafe fn AlertCheck(&self, dialogTitle: *const i16, text: *const i16, checkMsg: *const i16, checkValue: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AlertCheck)(self, dialogTitle, text, checkMsg, checkValue)
    }



    /// `boolean confirm (in wstring dialogTitle, in wstring text);`
    #[inline]
    pub unsafe fn Confirm(&self, dialogTitle: *const i16, text: *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Confirm)(self, dialogTitle, text, _retval)
    }



    /// `boolean confirmCheck (in wstring dialogTitle, in wstring text, in wstring checkMsg, inout boolean checkValue);`
    #[inline]
    pub unsafe fn ConfirmCheck(&self, dialogTitle: *const i16, text: *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmCheck)(self, dialogTitle, text, checkMsg, checkValue, _retval)
    }



    /// `int32_t confirmEx (in wstring dialogTitle, in wstring text, in unsigned long buttonFlags, in wstring button0Title, in wstring button1Title, in wstring button2Title, in wstring checkMsg, inout boolean checkValue);`
    #[inline]
    pub unsafe fn ConfirmEx(&self, dialogTitle: *const i16, text: *const i16, buttonFlags: u32, button0Title: *const i16, button1Title: *const i16, button2Title: *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmEx)(self, dialogTitle, text, buttonFlags, button0Title, button1Title, button2Title, checkMsg, checkValue, _retval)
    }



    /// `boolean prompt (in wstring dialogTitle, in wstring text, inout wstring value, in wstring checkMsg, inout boolean checkValue);`
    #[inline]
    pub unsafe fn Prompt(&self, dialogTitle: *const i16, text: *const i16, value: *mut *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Prompt)(self, dialogTitle, text, value, checkMsg, checkValue, _retval)
    }



    /// `boolean promptPassword (in wstring dialogTitle, in wstring text, inout wstring password, in wstring checkMsg, inout boolean checkValue);`
    #[inline]
    pub unsafe fn PromptPassword(&self, dialogTitle: *const i16, text: *const i16, password: *mut *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptPassword)(self, dialogTitle, text, password, checkMsg, checkValue, _retval)
    }



    /// `boolean promptUsernameAndPassword (in wstring dialogTitle, in wstring text, inout wstring username, inout wstring password, in wstring checkMsg, inout boolean checkValue);`
    #[inline]
    pub unsafe fn PromptUsernameAndPassword(&self, dialogTitle: *const i16, text: *const i16, username: *mut *const i16, password: *mut *const i16, checkMsg: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptUsernameAndPassword)(self, dialogTitle, text, username, password, checkMsg, checkValue, _retval)
    }



    /// `boolean select (in wstring dialogTitle, in wstring text, in Array<AString> selectList, out long outSelection);`
    #[inline]
    pub unsafe fn Select(&self, dialogTitle: *const i16, text: *const i16, selectList: *const thin_vec::ThinVec<::nsstring::nsString>, outSelection: *mut i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Select)(self, dialogTitle, text, selectList, outSelection, _retval)
    }


}


