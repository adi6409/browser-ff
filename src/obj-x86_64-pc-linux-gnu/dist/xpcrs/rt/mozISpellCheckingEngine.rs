//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/extensions/spellcheck/idl/mozISpellCheckingEngine.idl
//


/// `interface mozISpellCheckingEngine : nsISupports`
///

/// ```text
/// /**
///  * This interface represents a SpellChecker.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozISpellCheckingEngine {
    vtable: *const mozISpellCheckingEngineVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozISpellCheckingEngine.
unsafe impl XpCom for mozISpellCheckingEngine {
    const IID: nsIID = nsID(0x8ba643a4, 0x7ddc, 0x4662,
        [0xb9, 0x76, 0x7e, 0xc1, 0x23, 0x84, 0x3f, 0x10]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozISpellCheckingEngine {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozISpellCheckingEngine.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozISpellCheckingEngineCoerce {
    /// Cheaply cast a value of this type from a `mozISpellCheckingEngine`.
    fn coerce_from(v: &mozISpellCheckingEngine) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozISpellCheckingEngineCoerce for mozISpellCheckingEngine {
    #[inline]
    fn coerce_from(v: &mozISpellCheckingEngine) -> &Self {
        v
    }
}

impl mozISpellCheckingEngine {
    /// Cast this `mozISpellCheckingEngine` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozISpellCheckingEngineCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozISpellCheckingEngine {
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
impl<T: nsISupportsCoerce> mozISpellCheckingEngineCoerce for T {
    #[inline]
    fn coerce_from(v: &mozISpellCheckingEngine) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozISpellCheckingEngine
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozISpellCheckingEngineVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute ACString dictionary; */
    pub GetDictionary: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, aDictionary: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString dictionary; */
    pub SetDictionary: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, aDictionary: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute mozIPersonalDictionary personalDictionary; */
    pub GetPersonalDictionary: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, aPersonalDictionary: *mut*const mozIPersonalDictionary) -> ::nserror::nsresult,

    /* attribute mozIPersonalDictionary personalDictionary; */
    pub SetPersonalDictionary: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, aPersonalDictionary: *const mozIPersonalDictionary) -> ::nserror::nsresult,

    /* Array<ACString> getDictionaryList (); */
    pub GetDictionaryList: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* boolean check (in AString word); */
    pub Check: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, word: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* Array<AString> suggest (in AString word); */
    pub Suggest: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, word: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* void loadDictionariesFromDir (in nsIFile dir); */
    pub LoadDictionariesFromDir: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, dir: *const nsIFile) -> ::nserror::nsresult,

    /* void addDirectory (in nsIFile dir); */
    pub AddDirectory: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, dir: *const nsIFile) -> ::nserror::nsresult,

    /* void removeDirectory (in nsIFile dir); */
    pub RemoveDirectory: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, dir: *const nsIFile) -> ::nserror::nsresult,

    /* void addDictionary (in AString lang, in nsIURI file); */
    pub AddDictionary: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, lang: *const ::nsstring::nsAString, file: *const nsIURI) -> ::nserror::nsresult,

    /* bool removeDictionary (in AString lang, in nsIURI file); */
    pub RemoveDictionary: unsafe extern "system" fn (this: *const mozISpellCheckingEngine, lang: *const ::nsstring::nsAString, file: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozISpellCheckingEngine {

    /// ```text
    /// /**
    ///    * The name of the current dictionary. Is either a value from
    ///    * getDictionaryList or the empty string if no dictionary is selected.
    ///    * Setting this attribute to a value not in getDictionaryList will throw
    ///    * NS_ERROR_FILE_NOT_FOUND.
    ///    *
    ///    * If the dictionary is changed to no dictionary (the empty string), an
    ///    * observer is allowed to set another dictionary before it returns.
    ///    */
    /// ```
    ///

    /// `attribute ACString dictionary;`
    #[inline]
    pub unsafe fn GetDictionary(&self, aDictionary: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDictionary)(self, aDictionary)
    }


    /// ```text
    /// /**
    ///    * The name of the current dictionary. Is either a value from
    ///    * getDictionaryList or the empty string if no dictionary is selected.
    ///    * Setting this attribute to a value not in getDictionaryList will throw
    ///    * NS_ERROR_FILE_NOT_FOUND.
    ///    *
    ///    * If the dictionary is changed to no dictionary (the empty string), an
    ///    * observer is allowed to set another dictionary before it returns.
    ///    */
    /// ```
    ///

    /// `attribute ACString dictionary;`
    #[inline]
    pub unsafe fn SetDictionary(&self, aDictionary: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDictionary)(self, aDictionary)
    }


    /// ```text
    /// /**
    ///    * the personal dictionary
    ///    */
    /// ```
    ///

    /// `attribute mozIPersonalDictionary personalDictionary;`
    #[inline]
    pub unsafe fn GetPersonalDictionary(&self, aPersonalDictionary: *mut*const mozIPersonalDictionary) -> ::nserror::nsresult {
        ((*self.vtable).GetPersonalDictionary)(self, aPersonalDictionary)
    }


    /// ```text
    /// /**
    ///    * the personal dictionary
    ///    */
    /// ```
    ///

    /// `attribute mozIPersonalDictionary personalDictionary;`
    #[inline]
    pub unsafe fn SetPersonalDictionary(&self, aPersonalDictionary: *const mozIPersonalDictionary) -> ::nserror::nsresult {
        ((*self.vtable).SetPersonalDictionary)(self, aPersonalDictionary)
    }


    /// ```text
    /// /**
    ///    * Get the list of dictionaries
    ///    */
    /// ```
    ///

    /// `Array<ACString> getDictionaryList ();`
    #[inline]
    pub unsafe fn GetDictionaryList(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetDictionaryList)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * check a word
    ///    */
    /// ```
    ///

    /// `boolean check (in AString word);`
    #[inline]
    pub unsafe fn Check(&self, word: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Check)(self, word, _retval)
    }


    /// ```text
    /// /**
    ///    * get a list of suggestions for a misspelled word
    ///    */
    /// ```
    ///

    /// `Array<AString> suggest (in AString word);`
    #[inline]
    pub unsafe fn Suggest(&self, word: *const ::nsstring::nsAString, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).Suggest)(self, word, _retval)
    }


    /// ```text
    /// /**
    ///    * Load dictionaries from the specified dir
    ///    */
    /// ```
    ///

    /// `void loadDictionariesFromDir (in nsIFile dir);`
    #[inline]
    pub unsafe fn LoadDictionariesFromDir(&self, dir: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).LoadDictionariesFromDir)(self, dir)
    }


    /// ```text
    /// /**
    ///    * Add dictionaries from a directory to the spell checker
    ///    */
    /// ```
    ///

    /// `void addDirectory (in nsIFile dir);`
    #[inline]
    pub unsafe fn AddDirectory(&self, dir: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).AddDirectory)(self, dir)
    }


    /// ```text
    /// /**
    ///    * Remove dictionaries from a directory from the spell checker
    ///    */
    /// ```
    ///

    /// `void removeDirectory (in nsIFile dir);`
    #[inline]
    pub unsafe fn RemoveDirectory(&self, dir: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDirectory)(self, dir)
    }


    /// ```text
    /// /**
    ///    * Add a dictionary with the given language code and source URI. The URI
    ///    * must point to an affix file, with the ".aff" extension. The word list
    ///    * file must be in the same directory, with the same base name, and the
    ///    * ".dic" extension.
    ///    */
    /// ```
    ///

    /// `void addDictionary (in AString lang, in nsIURI file);`
    #[inline]
    pub unsafe fn AddDictionary(&self, lang: *const ::nsstring::nsAString, file: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).AddDictionary)(self, lang, file)
    }


    /// ```text
    /// /**
    ///    * Remove a dictionary with the given language code and path. If the path does
    ///    * not match that of the current entry with the given language code, it is not
    ///    * removed.
    ///    *
    ///    * @returns True if the dictionary was found and removed.
    ///    */
    /// ```
    ///

    /// `bool removeDictionary (in AString lang, in nsIURI file);`
    #[inline]
    pub unsafe fn RemoveDictionary(&self, lang: *const ::nsstring::nsAString, file: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoveDictionary)(self, lang, file, _retval)
    }


}


