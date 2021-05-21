//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIPromptService.idl
//


/// `interface nsIPromptService : nsISupports`
///

/// ```text
/// /**
///  * This is the interface to the embeddable prompt service; the service that
///  * implements nsIPrompt.  Its interface is designed to be just nsIPrompt, each
///  * method modified to take a parent window parameter.
///  *
///  * Accesskeys can be attached to buttons and checkboxes by inserting an &
///  * before the accesskey character in the checkbox message or button title.  For
///  * a real &, use && instead.  (A "button title" generally refers to the text
    ///  * label of a button.)
///  *
///  * One note: in all cases, the parent window parameter can be null.  However,
///  * these windows are all intended to have parents.  So when no parent is
///  * specified, the implementation should try hard to find a suitable foster
///  * parent.
///  *
///  * Implementations are free to choose how they present the various button
///  * types.  For example, while prompts that give the user a choice between OK
///  * and Cancel are required to return a boolean value indicating whether or not
///  * the user accepted the prompt (pressed OK) or rejected the prompt (pressed
    ///  * Cancel), the implementation of this interface could very well speak the
///  * prompt to the user instead of rendering any visual user-interface.  The
///  * standard button types are merely idioms used to convey the nature of the
///  * choice the user is to make.
///  *
///  * Because implementations of this interface may loosely interpret the various
///  * button types, it is advised that text messages passed to these prompts do
///  * not refer to the button types by name.  For example, it is inadvisable to
///  * tell the user to "Press OK to proceed."  Instead, such a prompt might be
///  * rewritten to ask the user: "Would you like to proceed?"
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPromptService {
    vtable: *const nsIPromptServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPromptService.
unsafe impl XpCom for nsIPromptService {
    const IID: nsIID = nsID(0x404ebfa2, 0xd8f4, 0x4c94,
        [0x84, 0x16, 0xe6, 0x5a, 0x55, 0xf9, 0xdf, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPromptService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPromptService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPromptServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIPromptService`.
    fn coerce_from(v: &nsIPromptService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPromptServiceCoerce for nsIPromptService {
    #[inline]
    fn coerce_from(v: &nsIPromptService) -> &Self {
        v
    }
}

impl nsIPromptService {
    /// Cast this `nsIPromptService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPromptServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPromptService {
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
impl<T: nsISupportsCoerce> nsIPromptServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPromptService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPromptService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPromptServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void alert (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
    pub Alert: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16) -> ::nserror::nsresult,

    /* void alertBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
    pub AlertBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16) -> ::nserror::nsresult,

    /* Promise asyncAlert (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncAlert: *const ::libc::c_void,

    /* void alertCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    pub AlertCheck: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool) -> ::nserror::nsresult,

    /* void alertCheckBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    pub AlertCheckBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncAlertCheck (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, in boolean aCheckState); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncAlertCheck: *const ::libc::c_void,

    /* boolean confirm (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText); */
    pub Confirm: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean confirmBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
    pub ConfirmBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncConfirm (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncConfirm: *const ::libc::c_void,

    /* boolean confirmCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    pub ConfirmCheck: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean confirmCheckBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState); */
    pub ConfirmCheckBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncConfirmCheck (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, in boolean aCheckState); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncConfirmCheck: *const ::libc::c_void,

    /* int32_t confirmEx (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState); */
    pub ConfirmEx: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aButtonFlags: u32, aButton0Title: *const i16, aButton1Title: *const i16, aButton2Title: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut int32_t) -> ::nserror::nsresult,

    /* int32_t confirmExBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState); */
    pub ConfirmExBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aButtonFlags: u32, aButton0Title: *const i16, aButton1Title: *const i16, aButton2Title: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut int32_t) -> ::nserror::nsresult,

    /* Promise asyncConfirmEx (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, in boolean aCheckState, [optional] in jsval aExtraArgs); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AsyncConfirmEx: *const ::libc::c_void,

    /* boolean prompt (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState); */
    pub Prompt: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aValue: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState); */
    pub PromptBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aValue: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncPrompt (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aValue, in wstring aCheckMsg, in boolean aCheckState); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncPrompt: *const ::libc::c_void,

    /* boolean promptUsernameAndPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    pub PromptUsernameAndPassword: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aUsername: *mut *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptUsernameAndPasswordBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    pub PromptUsernameAndPasswordBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aUsername: *mut *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncPromptUsernameAndPassword (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aUsername, in wstring aPassword, in wstring aCheckMsg, in boolean aCheckState); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncPromptUsernameAndPassword: *const ::libc::c_void,

    /* boolean promptPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    pub PromptPassword: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptPasswordBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState); */
    pub PromptPasswordBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncPromptPassword (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aPassword, in wstring aCheckMsg, in boolean aCheckState); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncPromptPassword: *const ::libc::c_void,

    /* boolean select (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList, out long aOutSelection); */
    pub Select: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aSelectList: *const thin_vec::ThinVec<::nsstring::nsString>, aOutSelection: *mut i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean selectBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList, out long aOutSelection); */
    pub SelectBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aSelectList: *const thin_vec::ThinVec<::nsstring::nsString>, aOutSelection: *mut i32, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncSelect (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncSelect: *const ::libc::c_void,

    /* boolean promptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
    pub PromptAuth: unsafe extern "system" fn (this: *const nsIPromptService, aParent: *const mozIDOMWindowProxy, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, checkboxLabel: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean promptAuthBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue); */
    pub PromptAuthBC: unsafe extern "system" fn (this: *const nsIPromptService, aBrowsingContext: *const libc::c_void, modalType: u32, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, checkboxLabel: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* Promise asyncPromptAuth (in BrowsingContext aBrowsingContext, in unsigned long modalType, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, in boolean checkValue); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub AsyncPromptAuth: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPromptService {
    /// ```text
    /// /**
    ///    * Button Flags
    ///    *
    ///    * The following flags are combined to form the aButtonFlags parameter passed
    ///    * to confirmEx.  See confirmEx for more information on how the flags may be
    ///    * combined.
    ///    */
    /// /**
    ///    * Button Position Flags
    ///    */
    /// ```
    ///

    pub const BUTTON_POS_0: i64 = 1;


    pub const BUTTON_POS_1: i64 = 256;


    pub const BUTTON_POS_2: i64 = 65536;

    /// ```text
    /// /**
    ///    * Button Title Flags (used to set the labels of buttons in the prompt)
    ///    */
    /// ```
    ///

    pub const BUTTON_TITLE_OK: i64 = 1;


    pub const BUTTON_TITLE_CANCEL: i64 = 2;


    pub const BUTTON_TITLE_YES: i64 = 3;


    pub const BUTTON_TITLE_NO: i64 = 4;


    pub const BUTTON_TITLE_SAVE: i64 = 5;


    pub const BUTTON_TITLE_DONT_SAVE: i64 = 6;


    pub const BUTTON_TITLE_REVERT: i64 = 7;


    pub const BUTTON_TITLE_IS_STRING: i64 = 127;

    /// ```text
    /// /**
    ///    * Button Default Flags (used to select which button is the default one)
    ///    */
    /// ```
    ///

    pub const BUTTON_POS_0_DEFAULT: i64 = 0;


    pub const BUTTON_POS_1_DEFAULT: i64 = 16777216;


    pub const BUTTON_POS_2_DEFAULT: i64 = 33554432;

    /// ```text
    /// /**
    ///    * Causes the buttons to be initially disabled.  They are enabled after a
    ///    * timeout expires.  The implementation may interpret this loosely as the
    ///    * intent is to ensure that the user does not click through a security dialog
    ///    * too quickly.  Strictly speaking, the implementation could choose to ignore
    ///    * this flag.
    ///    */
    /// ```
    ///

    pub const BUTTON_DELAY_ENABLE: i64 = 67108864;

    /// ```text
    /// /**
    ///    * Selects the standard set of OK/Cancel buttons.
    ///    */
    /// ```
    ///

    pub const STD_OK_CANCEL_BUTTONS: i64 = 513;

    /// ```text
    /// /**
    ///    * Selects the standard set of Yes/No buttons.
    ///    */
    /// ```
    ///

    pub const STD_YES_NO_BUTTONS: i64 = 1027;


    pub const MODAL_TYPE_CONTENT: i64 = 1;


    pub const MODAL_TYPE_TAB: i64 = 2;


    pub const MODAL_TYPE_WINDOW: i64 = 3;

    /// ```text
    /// /**
    ///    * Puts up an alert dialog with an OK button.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    */
    /// ```
    ///

    /// `void alert (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText);`
    #[inline]
    pub unsafe fn Alert(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).Alert)(self, aParent, aDialogTitle, aText)
    }


    /// ```text
    /// /**
    ///    * Like alert, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `void alertBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText);`
    #[inline]
    pub unsafe fn AlertBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).AlertBC)(self, aBrowsingContext, modalType, aDialogTitle, aText)
    }


    /// ```text
    /// /**
    ///    * Async version of alertBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    */
    /// ```
    ///

    /// `Promise asyncAlert (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText);`
    const _AsyncAlert: () = ();

    /// ```text
    /// /**
    ///    * Puts up an alert dialog with an OK button and a labeled checkbox.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aCheckMsg
    ///    *        Text to appear with the checkbox.
    ///    * @param aCheckState
    ///    *        Contains the initial checked state of the checkbox when this method
    ///    *        is called and the final checked state after this method returns.
    ///    */
    /// ```
    ///

    /// `void alertCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn AlertCheck(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AlertCheck)(self, aParent, aDialogTitle, aText, aCheckMsg, aCheckState)
    }


    /// ```text
    /// /**
    ///    * Like alertCheck, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `void alertCheckBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn AlertCheckBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).AlertCheckBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState)
    }


    /// ```text
    /// /**
    ///    * Async version of alertCheckBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { checked: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncAlertCheck (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, in boolean aCheckState);`
    const _AsyncAlertCheck: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog with OK and Cancel buttons.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    *
    ///    * @return true for OK, false for Cancel
    ///    */
    /// ```
    ///

    /// `boolean confirm (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText);`
    #[inline]
    pub unsafe fn Confirm(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Confirm)(self, aParent, aDialogTitle, aText, _retval)
    }


    /// ```text
    /// /**
    ///    * Like confirm, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean confirmBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText);`
    #[inline]
    pub unsafe fn ConfirmBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of confirmBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { ok: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncConfirm (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText);`
    const _AsyncConfirm: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog with OK and Cancel buttons and a labeled checkbox.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aCheckMsg
    ///    *        Text to appear with the checkbox.
    ///    * @param aCheckState
    ///    *        Contains the initial checked state of the checkbox when this method
    ///    *        is called and the final checked state after this method returns.
    ///    *
    ///    * @return true for OK, false for Cancel
    ///    */
    /// ```
    ///

    /// `boolean confirmCheck (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn ConfirmCheck(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmCheck)(self, aParent, aDialogTitle, aText, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Like confirmCheck, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean confirmCheckBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn ConfirmCheckBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmCheckBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of confirmCheckBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { ok: boolean, checked: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncConfirmCheck (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aCheckMsg, in boolean aCheckState);`
    const _AsyncConfirmCheck: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog with up to 3 buttons and an optional, labeled checkbox.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aButtonFlags
    ///    *        A combination of Button Flags.
    ///    * @param aButton0Title
    ///    *        Used when button 0 uses TITLE_IS_STRING
    ///    * @param aButton1Title
    ///    *        Used when button 1 uses TITLE_IS_STRING
    ///    * @param aButton2Title
    ///    *        Used when button 2 uses TITLE_IS_STRING
    ///    * @param aCheckMsg
    ///    *        Text to appear with the checkbox.  Null if no checkbox.
    ///    * @param aCheckState
    ///    *        Contains the initial checked state of the checkbox when this method
    ///    *        is called and the final checked state after this method returns.
    ///    *
    ///    * @return index of the button pressed.
    ///    *
    ///    * Buttons are numbered 0 - 2. The implementation can decide whether the
    ///    * sequence goes from right to left or left to right.  Button 0 is the
    ///    * default button unless one of the Button Default Flags is specified.
    ///    *
    ///    * A button may use a predefined title, specified by one of the Button Title
    ///    * Flags values.  Each title value can be multiplied by a position value to
    ///    * assign the title to a particular button.  If BUTTON_TITLE_IS_STRING is
    ///    * used for a button, the string parameter for that button will be used.  If
    ///    * the value for a button position is zero, the button will not be shown.
    ///    *
    ///    * In general, aButtonFlags is constructed per the following example:
    ///    *
    ///    *   aButtonFlags = (BUTTON_POS_0) * (BUTTON_TITLE_AAA) +
    ///    *                  (BUTTON_POS_1) * (BUTTON_TITLE_BBB) +
    ///    *                   BUTTON_POS_1_DEFAULT;
    ///    *
    ///    * where "AAA" and "BBB" correspond to one of the button titles.
    ///    */
    /// ```
    ///

    /// `int32_t confirmEx (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn ConfirmEx(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aButtonFlags: u32, aButton0Title: *const i16, aButton1Title: *const i16, aButton2Title: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmEx)(self, aParent, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Like confirmEx, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `int32_t confirmExBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn ConfirmExBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aButtonFlags: u32, aButton0Title: *const i16, aButton1Title: *const i16, aButton2Title: *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).ConfirmExBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aButtonFlags, aButton0Title, aButton1Title, aButton2Title, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of confirmExBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { checked: boolean, buttonNumClicked: int }
    ///    */
    /// ```
    ///

    /// `Promise asyncConfirmEx (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in unsigned long aButtonFlags, in wstring aButton0Title, in wstring aButton1Title, in wstring aButton2Title, in wstring aCheckMsg, in boolean aCheckState, [optional] in jsval aExtraArgs);`
    const _AsyncConfirmEx: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog with an edit field and an optional, labeled checkbox.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aValue
    ///    *        Contains the default value for the dialog field when this method
    ///    *        is called (null value is ok).  Upon return, if the user pressed
    ///    *        OK, then this parameter contains a newly allocated string value.
    ///    *        Otherwise, the parameter's value is unmodified.
    ///    * @param aCheckMsg
    ///    *        Text to appear with the checkbox.  If null, check box will not be shown.
    ///    * @param aCheckState
    ///    *        Contains the initial checked state of the checkbox when this method
    ///    *        is called and the final checked state after this method returns.
    ///    *
    ///    * @return true for OK, false for Cancel.
    ///    */
    /// ```
    ///

    /// `boolean prompt (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn Prompt(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aValue: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Prompt)(self, aParent, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Like prompt, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean promptBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aValue, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn PromptBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aValue: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aValue, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of promptBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { checked: boolean, value: string, ok: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncPrompt (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aValue, in wstring aCheckMsg, in boolean aCheckState);`
    const _AsyncPrompt: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog with an edit field, a password field, and an optional,
    ///    * labeled checkbox.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aUsername
    ///    *        Contains the default value for the username field when this method
    ///    *        is called (null value is ok).  Upon return, if the user pressed OK,
    ///    *        then this parameter contains a newly allocated string value.
    ///    *        Otherwise, the parameter's value is unmodified.
    ///    * @param aPassword
    ///    *        Contains the default value for the password field when this method
    ///    *        is called (null value is ok).  Upon return, if the user pressed OK,
    ///    *        then this parameter contains a newly allocated string value.
    ///    *        Otherwise, the parameter's value is unmodified.
    ///    * @param aCheckMsg
    ///    *        Text to appear with the checkbox.  If null, check box will not be shown.
    ///    * @param aCheckState
    ///    *        Contains the initial checked state of the checkbox when this method
    ///    *        is called and the final checked state after this method returns.
    ///    *
    ///    * @return true for OK, false for Cancel.
    ///    */
    /// ```
    ///

    /// `boolean promptUsernameAndPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn PromptUsernameAndPassword(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aUsername: *mut *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptUsernameAndPassword)(self, aParent, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Like promptUsernameAndPassword, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean promptUsernameAndPasswordBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aUsername, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn PromptUsernameAndPasswordBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aUsername: *mut *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptUsernameAndPasswordBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aUsername, aPassword, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of promptUsernameAndPasswordBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { checked: boolean, user: string, pass: string, ok: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncPromptUsernameAndPassword (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aUsername, in wstring aPassword, in wstring aCheckMsg, in boolean aCheckState);`
    const _AsyncPromptUsernameAndPassword: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog with a password field and an optional, labeled checkbox.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aPassword
    ///    *        Contains the default value for the password field when this method
    ///    *        is called (null value is ok).  Upon return, if the user pressed OK,
    ///    *        then this parameter contains a newly allocated string value.
    ///    *        Otherwise, the parameter's value is unmodified.
    ///    * @param aCheckMsg
    ///    *        Text to appear with the checkbox.  If null, check box will not be shown.
    ///    * @param aCheckState
    ///    *        Contains the initial checked state of the checkbox when this method
    ///    *        is called and the final checked state after this method returns.
    ///    *
    ///    * @return true for OK, false for Cancel.
    ///    */
    /// ```
    ///

    /// `boolean promptPassword (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn PromptPassword(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptPassword)(self, aParent, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Like promptPassword, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean promptPasswordBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, inout wstring aPassword, in wstring aCheckMsg, inout boolean aCheckState);`
    #[inline]
    pub unsafe fn PromptPasswordBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aPassword: *mut *const i16, aCheckMsg: *const i16, aCheckState: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptPasswordBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aPassword, aCheckMsg, aCheckState, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of promptPasswordBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { checked: boolean, pass: string, ok: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncPromptPassword (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in wstring aPassword, in wstring aCheckMsg, in boolean aCheckState);`
    const _AsyncPromptPassword: () = ();

    /// ```text
    /// /**
    ///    * Puts up a dialog box which has a list box of strings from which the user
    ///    * may make a single selection.
    ///    *
    ///    * @param aParent
    ///    *        The parent window or null.
    ///    * @param aDialogTitle
    ///    *        Text to appear in the title of the dialog.
    ///    * @param aText
    ///    *        Text to appear in the body of the dialog.
    ///    * @param aSelectList
    ///    *        The list of strings to display.
    ///    * @param aOutSelection
    ///    *        Contains the index of the selected item in the list when this
    ///    *        method returns true.
    ///    *
    ///    * @return true for OK, false for Cancel.
    ///    */
    /// ```
    ///

    /// `boolean select (in mozIDOMWindowProxy aParent, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList, out long aOutSelection);`
    #[inline]
    pub unsafe fn Select(&self, aParent: *const mozIDOMWindowProxy, aDialogTitle: *const i16, aText: *const i16, aSelectList: *const thin_vec::ThinVec<::nsstring::nsString>, aOutSelection: *mut i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Select)(self, aParent, aDialogTitle, aText, aSelectList, aOutSelection, _retval)
    }


    /// ```text
    /// /**
    ///    * Like select, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean selectBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList, out long aOutSelection);`
    #[inline]
    pub unsafe fn SelectBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aDialogTitle: *const i16, aText: *const i16, aSelectList: *const thin_vec::ThinVec<::nsstring::nsString>, aOutSelection: *mut i32, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SelectBC)(self, aBrowsingContext, modalType, aDialogTitle, aText, aSelectList, aOutSelection, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of selectBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { selected: int, ok: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncSelect (in BrowsingContext aBrowsingContext, in unsigned long modalType, in wstring aDialogTitle, in wstring aText, in Array<AString> aSelectList);`
    const _AsyncSelect: () = ();


    /// `boolean promptAuth (in mozIDOMWindowProxy aParent, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue);`
    #[inline]
    pub unsafe fn PromptAuth(&self, aParent: *const mozIDOMWindowProxy, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, checkboxLabel: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptAuth)(self, aParent, aChannel, level, authInfo, checkboxLabel, checkValue, _retval)
    }


    /// ```text
    /// /**
    ///    * Like promptAuth, but with a BrowsingContext as parent.
    ///    *
    ///    * @param aBrowsingContext
    ///    *        The browsing context the prompt should be opened for.
    ///    * @param modalType
    ///    *        Whether the prompt should be window, tab or content modal.
    ///    */
    /// ```
    ///

    /// `boolean promptAuthBC (in BrowsingContext aBrowsingContext, in unsigned long modalType, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, inout boolean checkValue);`
    #[inline]
    pub unsafe fn PromptAuthBC(&self, aBrowsingContext: *const libc::c_void, modalType: u32, aChannel: *const nsIChannel, level: uint32_t, authInfo: *const nsIAuthInformation, checkboxLabel: *const i16, checkValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PromptAuthBC)(self, aBrowsingContext, modalType, aChannel, level, authInfo, checkboxLabel, checkValue, _retval)
    }


    /// ```text
    /// /**
    ///    * Async version of promptAuthBC
    ///    *
    ///    * @return A promise which resolves when the prompt is dismissed.
    ///    *
    ///    * @resolves nsIPropertyBag { ok: boolean }
    ///    */
    /// ```
    ///

    /// `Promise asyncPromptAuth (in BrowsingContext aBrowsingContext, in unsigned long modalType, in nsIChannel aChannel, in uint32_t level, in nsIAuthInformation authInfo, in wstring checkboxLabel, in boolean checkValue);`
    const _AsyncPromptAuth: () = ();

}


