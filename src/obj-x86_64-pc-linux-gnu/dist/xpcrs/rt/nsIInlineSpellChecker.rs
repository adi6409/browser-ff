//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/spellchecker/nsIInlineSpellChecker.idl
//


/// `interface nsIInlineSpellChecker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIInlineSpellChecker {
    vtable: *const nsIInlineSpellCheckerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIInlineSpellChecker.
unsafe impl XpCom for nsIInlineSpellChecker {
    const IID: nsIID = nsID(0xb7b7a77c, 0x40c4, 0x4196,
        [0xb0, 0xb7, 0xb0, 0x33, 0x82, 0x43, 0xb3, 0xfe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIInlineSpellChecker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIInlineSpellChecker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIInlineSpellCheckerCoerce {
    /// Cheaply cast a value of this type from a `nsIInlineSpellChecker`.
    fn coerce_from(v: &nsIInlineSpellChecker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIInlineSpellCheckerCoerce for nsIInlineSpellChecker {
    #[inline]
    fn coerce_from(v: &nsIInlineSpellChecker) -> &Self {
        v
    }
}

impl nsIInlineSpellChecker {
    /// Cast this `nsIInlineSpellChecker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIInlineSpellCheckerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIInlineSpellChecker {
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
impl<T: nsISupportsCoerce> nsIInlineSpellCheckerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIInlineSpellChecker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIInlineSpellChecker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIInlineSpellCheckerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIEditorSpellCheck spellChecker; */
    pub GetSpellChecker: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aSpellChecker: *mut*const nsIEditorSpellCheck) -> ::nserror::nsresult,

    /* void init (in nsIEditor aEditor); */
    pub Init: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aEditor: *const nsIEditor) -> ::nserror::nsresult,

    /* void cleanup (in boolean aDestroyingFrames); */
    pub Cleanup: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aDestroyingFrames: bool) -> ::nserror::nsresult,

    /* attribute boolean enableRealTimeSpell; */
    pub GetEnableRealTimeSpell: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aEnableRealTimeSpell: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean enableRealTimeSpell; */
    pub SetEnableRealTimeSpell: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aEnableRealTimeSpell: bool) -> ::nserror::nsresult,

    /* void spellCheckRange (in Range aSelection); */
    pub SpellCheckRange: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aSelection: *const libc::c_void) -> ::nserror::nsresult,

    /* Range getMisspelledWord (in Node aNode, in long aOffset); */
    pub GetMisspelledWord: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aNode: *const libc::c_void, aOffset: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void replaceWord (in Node aNode, in long aOffset, in AString aNewword); */
    pub ReplaceWord: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aNode: *const libc::c_void, aOffset: i32, aNewword: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void addWordToDictionary (in AString aWord); */
    pub AddWordToDictionary: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aWord: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeWordFromDictionary (in AString aWord); */
    pub RemoveWordFromDictionary: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aWord: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void ignoreWord (in AString aWord); */
    pub IgnoreWord: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aWord: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void ignoreWords (in Array<AString> aWordsToIgnore); */
    pub IgnoreWords: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aWordsToIgnore: *const thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* void updateCurrentDictionary (); */
    pub UpdateCurrentDictionary: unsafe extern "system" fn (this: *const nsIInlineSpellChecker) -> ::nserror::nsresult,

    /* readonly attribute boolean spellCheckPending; */
    pub GetSpellCheckPending: unsafe extern "system" fn (this: *const nsIInlineSpellChecker, aSpellCheckPending: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIInlineSpellChecker {


    /// `readonly attribute nsIEditorSpellCheck spellChecker;`
    #[inline]
    pub unsafe fn GetSpellChecker(&self, aSpellChecker: *mut*const nsIEditorSpellCheck) -> ::nserror::nsresult {
        ((*self.vtable).GetSpellChecker)(self, aSpellChecker)
    }



    /// `void init (in nsIEditor aEditor);`
    #[inline]
    pub unsafe fn Init(&self, aEditor: *const nsIEditor) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aEditor)
    }



    /// `void cleanup (in boolean aDestroyingFrames);`
    #[inline]
    pub unsafe fn Cleanup(&self, aDestroyingFrames: bool) -> ::nserror::nsresult {
        ((*self.vtable).Cleanup)(self, aDestroyingFrames)
    }



    /// `attribute boolean enableRealTimeSpell;`
    #[inline]
    pub unsafe fn GetEnableRealTimeSpell(&self, aEnableRealTimeSpell: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetEnableRealTimeSpell)(self, aEnableRealTimeSpell)
    }



    /// `attribute boolean enableRealTimeSpell;`
    #[inline]
    pub unsafe fn SetEnableRealTimeSpell(&self, aEnableRealTimeSpell: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetEnableRealTimeSpell)(self, aEnableRealTimeSpell)
    }



    /// `void spellCheckRange (in Range aSelection);`
    #[inline]
    pub unsafe fn SpellCheckRange(&self, aSelection: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SpellCheckRange)(self, aSelection)
    }



    /// `Range getMisspelledWord (in Node aNode, in long aOffset);`
    #[inline]
    pub unsafe fn GetMisspelledWord(&self, aNode: *const libc::c_void, aOffset: i32, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetMisspelledWord)(self, aNode, aOffset, _retval)
    }



    /// `[can_run_script] void replaceWord (in Node aNode, in long aOffset, in AString aNewword);`
    #[inline]
    pub unsafe fn ReplaceWord(&self, aNode: *const libc::c_void, aOffset: i32, aNewword: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceWord)(self, aNode, aOffset, aNewword)
    }



    /// `void addWordToDictionary (in AString aWord);`
    #[inline]
    pub unsafe fn AddWordToDictionary(&self, aWord: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddWordToDictionary)(self, aWord)
    }



    /// `void removeWordFromDictionary (in AString aWord);`
    #[inline]
    pub unsafe fn RemoveWordFromDictionary(&self, aWord: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWordFromDictionary)(self, aWord)
    }



    /// `void ignoreWord (in AString aWord);`
    #[inline]
    pub unsafe fn IgnoreWord(&self, aWord: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).IgnoreWord)(self, aWord)
    }



    /// `void ignoreWords (in Array<AString> aWordsToIgnore);`
    #[inline]
    pub unsafe fn IgnoreWords(&self, aWordsToIgnore: *const thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).IgnoreWords)(self, aWordsToIgnore)
    }



    /// `void updateCurrentDictionary ();`
    #[inline]
    pub unsafe fn UpdateCurrentDictionary(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpdateCurrentDictionary)(self, )
    }



    /// `readonly attribute boolean spellCheckPending;`
    #[inline]
    pub unsafe fn GetSpellCheckPending(&self, aSpellCheckPending: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSpellCheckPending)(self, aSpellCheckPending)
    }


}


