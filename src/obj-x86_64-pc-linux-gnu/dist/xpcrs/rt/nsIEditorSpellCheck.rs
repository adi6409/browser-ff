//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorSpellCheck.idl
//


/// `interface nsIEditorSpellCheck : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEditorSpellCheck {
    vtable: *const nsIEditorSpellCheckVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEditorSpellCheck.
unsafe impl XpCom for nsIEditorSpellCheck {
    const IID: nsIID = nsID(0xa171c25f, 0xe4a8, 0x4d08,
        [0xad, 0xef, 0xb7, 0x97, 0xe6, 0x37, 0x7b, 0xdc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEditorSpellCheck {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEditorSpellCheck.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEditorSpellCheckCoerce {
    /// Cheaply cast a value of this type from a `nsIEditorSpellCheck`.
    fn coerce_from(v: &nsIEditorSpellCheck) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEditorSpellCheckCoerce for nsIEditorSpellCheck {
    #[inline]
    fn coerce_from(v: &nsIEditorSpellCheck) -> &Self {
        v
    }
}

impl nsIEditorSpellCheck {
    /// Cast this `nsIEditorSpellCheck` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEditorSpellCheckCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEditorSpellCheck {
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
impl<T: nsISupportsCoerce> nsIEditorSpellCheckCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorSpellCheck) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEditorSpellCheck
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEditorSpellCheckVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean canSpellCheck (); */
    pub CanSpellCheck: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, _retval: *mut bool) -> ::nserror::nsresult,

    /* void InitSpellChecker (in nsIEditor editor, in boolean enableSelectionChecking, [optional] in nsIEditorSpellCheckCallback callback); */
    pub InitSpellChecker: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, editor: *const nsIEditor, enableSelectionChecking: bool, callback: *const nsIEditorSpellCheckCallback) -> ::nserror::nsresult,

    /* [can_run_script] AString GetNextMisspelledWord (); */
    pub GetNextMisspelledWord: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString GetSuggestedWord (); */
    pub GetSuggestedWord: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean CheckCurrentWord (in AString suggestedWord); */
    pub CheckCurrentWord: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, suggestedWord: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] void ReplaceWord (in AString misspelledWord, in AString replaceWord, in boolean allOccurrences); */
    pub ReplaceWord: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, misspelledWord: *const ::nsstring::nsAString, replaceWord: *const ::nsstring::nsAString, allOccurrences: bool) -> ::nserror::nsresult,

    /* void IgnoreWordAllOccurrences (in AString word); */
    pub IgnoreWordAllOccurrences: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, word: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void GetPersonalDictionary (); */
    pub GetPersonalDictionary: unsafe extern "system" fn (this: *const nsIEditorSpellCheck) -> ::nserror::nsresult,

    /* AString GetPersonalDictionaryWord (); */
    pub GetPersonalDictionaryWord: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void AddWordToDictionary (in AString word); */
    pub AddWordToDictionary: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, word: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void RemoveWordFromDictionary (in AString word); */
    pub RemoveWordFromDictionary: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, word: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* Array<ACString> GetDictionaryList (); */
    pub GetDictionaryList: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* ACString GetCurrentDictionary (); */
    pub GetCurrentDictionary: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void SetCurrentDictionary (in ACString dictionary); */
    pub SetCurrentDictionary: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, dictionary: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void UninitSpellChecker (); */
    pub UninitSpellChecker: unsafe extern "system" fn (this: *const nsIEditorSpellCheck) -> ::nserror::nsresult,

    /* void setFilterType (in unsigned long filterType); */
    pub SetFilterType: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, filterType: u32) -> ::nserror::nsresult,

    /* void UpdateCurrentDictionary ([optional] in nsIEditorSpellCheckCallback callback); */
    pub UpdateCurrentDictionary: unsafe extern "system" fn (this: *const nsIEditorSpellCheck, callback: *const nsIEditorSpellCheckCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEditorSpellCheck {

    pub const FILTERTYPE_NORMAL: i64 = 1;


    pub const FILTERTYPE_MAIL: i64 = 2;

    /// ```text
    /// /**
    ///    * Returns true if we can enable spellchecking. If there are no available
    ///    * dictionaries, this will return false.
    ///    */
    /// ```
    ///

    /// `boolean canSpellCheck ();`
    #[inline]
    pub unsafe fn CanSpellCheck(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanSpellCheck)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Turns on the spell checker for the given editor. enableSelectionChecking
    ///    * set means that we only want to check the current selection in the editor,
    ///    * (this controls the behavior of GetNextMisspelledWord). For spellchecking
    ///    * clients with no modal UI (such as inline spellcheckers), this flag doesn't
    ///    * matter.  Initialization is asynchronous and is not complete until the given
    ///    * callback is called.
    ///    */
    /// ```
    ///

    /// `void InitSpellChecker (in nsIEditor editor, in boolean enableSelectionChecking, [optional] in nsIEditorSpellCheckCallback callback);`
    #[inline]
    pub unsafe fn InitSpellChecker(&self, editor: *const nsIEditor, enableSelectionChecking: bool, callback: *const nsIEditorSpellCheckCallback) -> ::nserror::nsresult {
        ((*self.vtable).InitSpellChecker)(self, editor, enableSelectionChecking, callback)
    }


    /// ```text
    /// /**
    ///    * When interactively spell checking the document, this will return the
    ///    * value of the next word that is misspelled. This also computes the
    ///    * suggestions which you can get by calling GetSuggestedWord.
    ///    *
    ///    * @see mozSpellChecker::GetNextMisspelledWord
    ///    */
    /// ```
    ///

    /// `[can_run_script] AString GetNextMisspelledWord ();`
    #[inline]
    pub unsafe fn GetNextMisspelledWord(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetNextMisspelledWord)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Used to get suggestions for the last word that was checked and found to
    ///    * be misspelled. The first call will give you the first (best) suggestion.
    ///    * Subsequent calls will iterate through all the suggestions, allowing you
    ///    * to build a list. When there are no more suggestions, an empty string
    ///    * (not a null pointer) will be returned.
    ///    *
    ///    * @see mozSpellChecker::GetSuggestedWord
    ///    */
    /// ```
    ///

    /// `AString GetSuggestedWord ();`
    #[inline]
    pub unsafe fn GetSuggestedWord(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSuggestedWord)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Check a given word. In spite of the name, this function checks the word
    ///    * you give it, returning true if the word is misspelled. If the word is
    ///    * misspelled, it will compute the suggestions which you can get from
    ///    * GetSuggestedWord().
    ///    *
    ///    * @see mozSpellChecker::CheckCurrentWord
    ///    */
    /// ```
    ///

    /// `boolean CheckCurrentWord (in AString suggestedWord);`
    #[inline]
    pub unsafe fn CheckCurrentWord(&self, suggestedWord: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckCurrentWord)(self, suggestedWord, _retval)
    }


    /// ```text
    /// /**
    ///    * Use when modally checking the document to replace a word.
    ///    *
    ///    * @see mozSpellChecker::CheckCurrentWord
    ///    */
    /// ```
    ///

    /// `[can_run_script] void ReplaceWord (in AString misspelledWord, in AString replaceWord, in boolean allOccurrences);`
    #[inline]
    pub unsafe fn ReplaceWord(&self, misspelledWord: *const ::nsstring::nsAString, replaceWord: *const ::nsstring::nsAString, allOccurrences: bool) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceWord)(self, misspelledWord, replaceWord, allOccurrences)
    }


    /// ```text
    /// /**
    ///    * @see mozSpellChecker::IgnoreAll
    ///    */
    /// ```
    ///

    /// `void IgnoreWordAllOccurrences (in AString word);`
    #[inline]
    pub unsafe fn IgnoreWordAllOccurrences(&self, word: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).IgnoreWordAllOccurrences)(self, word)
    }


    /// ```text
    /// /**
    ///    * Fills an internal list of words added to the personal dictionary. These
    ///    * words can be retrieved using GetPersonalDictionaryWord()
    ///    *
    ///    * @see mozSpellChecker::GetPersonalDictionary
    ///    * @see GetPersonalDictionaryWord
    ///    */
    /// ```
    ///

    /// `void GetPersonalDictionary ();`
    #[inline]
    pub unsafe fn GetPersonalDictionary(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).GetPersonalDictionary)(self, )
    }


    /// ```text
    /// /**
    ///    * Used after you call GetPersonalDictionary() to iterate through all the
    ///    * words added to the personal dictionary. Will return the empty string when
    ///    * there are no more words.
    ///    */
    /// ```
    ///

    /// `AString GetPersonalDictionaryWord ();`
    #[inline]
    pub unsafe fn GetPersonalDictionaryWord(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetPersonalDictionaryWord)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds a word to the current personal dictionary.
    ///    *
    ///    * @see mozSpellChecker::AddWordToDictionary
    ///    */
    /// ```
    ///

    /// `void AddWordToDictionary (in AString word);`
    #[inline]
    pub unsafe fn AddWordToDictionary(&self, word: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddWordToDictionary)(self, word)
    }


    /// ```text
    /// /**
    ///    * Removes a word from the current personal dictionary.
    ///    *
    ///    * @see mozSpellChecker::RemoveWordFromPersonalDictionary
    ///    */
    /// ```
    ///

    /// `void RemoveWordFromDictionary (in AString word);`
    #[inline]
    pub unsafe fn RemoveWordFromDictionary(&self, word: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWordFromDictionary)(self, word)
    }


    /// ```text
    /// /**
    ///    * Retrieves a list of the currently available dictionaries. The strings will
    ///    * typically be language IDs, like "en-US".
    ///    *
    ///    * @see mozISpellCheckingEngine::GetDictionaryList
    ///    */
    /// ```
    ///

    /// `Array<ACString> GetDictionaryList ();`
    #[inline]
    pub unsafe fn GetDictionaryList(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetDictionaryList)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * @see mozSpellChecker::GetCurrentDictionary
    ///    */
    /// ```
    ///

    /// `ACString GetCurrentDictionary ();`
    #[inline]
    pub unsafe fn GetCurrentDictionary(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentDictionary)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * @see mozSpellChecker::SetCurrentDictionary
    ///    */
    /// ```
    ///

    /// `void SetCurrentDictionary (in ACString dictionary);`
    #[inline]
    pub unsafe fn SetCurrentDictionary(&self, dictionary: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentDictionary)(self, dictionary)
    }


    /// ```text
    /// /**
    ///    * Call this to free up the spell checking object. It will also save the
    ///    * current selected language as the default for future use.
    ///    *
    ///    * If you have called CanSpellCheck but not InitSpellChecker, you can still
    ///    * call this function to clear the cached spell check object, and no
    ///    * preference saving will happen.
    ///    */
    /// ```
    ///

    /// `void UninitSpellChecker ();`
    #[inline]
    pub unsafe fn UninitSpellChecker(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UninitSpellChecker)(self, )
    }


    /// ```text
    /// /**
    ///    * Used to filter the content (for example, to skip blockquotes in email from
        ///    * spellchecking. Call this before calling InitSpellChecker; calling it
        ///    * after initialization will have no effect.
        ///    */
        /// ```
        ///

        /// `void setFilterType (in unsigned long filterType);`
        #[inline]
        pub unsafe fn SetFilterType(&self, filterType: u32) -> ::nserror::nsresult {
            ((*self.vtable).SetFilterType)(self, filterType)
        }


        /// ```text
        /// /**
        ///    * Update the dictionary in use to be sure it corresponds to what the editor
        ///    * needs.  The update is asynchronous and is not complete until the given
        ///    * callback is called.
        ///    */
        /// ```
        ///

        /// `void UpdateCurrentDictionary ([optional] in nsIEditorSpellCheckCallback callback);`
        #[inline]
        pub unsafe fn UpdateCurrentDictionary(&self, callback: *const nsIEditorSpellCheckCallback) -> ::nserror::nsresult {
            ((*self.vtable).UpdateCurrentDictionary)(self, callback)
        }


    }


    /// `interface nsIEditorSpellCheckCallback : nsISupports`
    ///


    // The actual type definition for the interface. This struct has methods
    // declared on it which will call through its vtable. You never want to pass
    // this type around by value, always pass it behind a reference.

    #[repr(C)]
    pub struct nsIEditorSpellCheckCallback {
        vtable: *const nsIEditorSpellCheckCallbackVTable,

        /// This field is a phantomdata to ensure that the VTable type and any
        /// struct containing it is not safe to send across threads, as XPCOM is
        /// generally not threadsafe.
        ///
        /// XPCOM interfaces in general are not safe to send across threads.
        __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
    }

    // Implementing XpCom for an interface exposes its IID, which allows for easy
    // use of the `.query_interface<T>` helper method. This also defines that
    // method for nsIEditorSpellCheckCallback.
    unsafe impl XpCom for nsIEditorSpellCheckCallback {
        const IID: nsIID = nsID(0x5f0a4bab, 0x8538, 0x4074,
            [0x89, 0xd3, 0x2f, 0x0e, 0x86, 0x6a, 0x1c, 0x0b]);
    }

    // We need to implement the RefCounted trait so we can be used with `RefPtr`.
    // This trait teaches `RefPtr` how to manage our memory.
    unsafe impl RefCounted for nsIEditorSpellCheckCallback {
        #[inline]
        unsafe fn addref(&self) {
            self.AddRef();
        }
        #[inline]
        unsafe fn release(&self) {
            self.Release();
        }
    }

    // This trait is implemented on all types which can be coerced to from nsIEditorSpellCheckCallback.
    // It is used in the implementation of `fn coerce<T>`. We hide it from the
    // documentation, because it clutters it up a lot.
    #[doc(hidden)]
    pub trait nsIEditorSpellCheckCallbackCoerce {
        /// Cheaply cast a value of this type from a `nsIEditorSpellCheckCallback`.
        fn coerce_from(v: &nsIEditorSpellCheckCallback) -> &Self;
    }

    // The trivial implementation: We can obviously coerce ourselves to ourselves.
    impl nsIEditorSpellCheckCallbackCoerce for nsIEditorSpellCheckCallback {
        #[inline]
        fn coerce_from(v: &nsIEditorSpellCheckCallback) -> &Self {
            v
        }
    }

    impl nsIEditorSpellCheckCallback {
        /// Cast this `nsIEditorSpellCheckCallback` to one of its base interfaces.
        #[inline]
        pub fn coerce<T: nsIEditorSpellCheckCallbackCoerce>(&self) -> &T {
            T::coerce_from(self)
        }
    }

    // Every interface struct type implements `Deref` to its base interface. This
    // causes methods on the base interfaces to be directly avaliable on the
    // object. For example, you can call `.AddRef` or `.QueryInterface` directly
    // on any interface which inherits from `nsISupports`.
    impl ::std::ops::Deref for nsIEditorSpellCheckCallback {
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
    impl<T: nsISupportsCoerce> nsIEditorSpellCheckCallbackCoerce for T {
        #[inline]
        fn coerce_from(v: &nsIEditorSpellCheckCallback) -> &Self {
            T::coerce_from(v)
        }
    }

    // This struct represents the interface's VTable. A pointer to a statically
    // allocated version of this struct is at the beginning of every nsIEditorSpellCheckCallback
    // object. It contains one pointer field for each method in the interface. In
    // the case where we can't generate a binding for a method, we include a void
    // pointer.
    #[doc(hidden)]
    #[repr(C)]
    pub struct nsIEditorSpellCheckCallbackVTable {
        /// We need to include the members from the base interface's vtable at the start
        /// of the VTable definition.
        pub __base: nsISupportsVTable,

        /* void editorSpellCheckDone (); */
        pub EditorSpellCheckDone: unsafe extern "system" fn (this: *const nsIEditorSpellCheckCallback) -> ::nserror::nsresult,
    }


    // The implementations of the function wrappers which are exposed to rust code.
    // Call these methods rather than manually calling through the VTable struct.
    impl nsIEditorSpellCheckCallback {


        /// `void editorSpellCheckDone ();`
        #[inline]
        pub unsafe fn EditorSpellCheckDone(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).EditorSpellCheckDone)(self, )
        }


    }


