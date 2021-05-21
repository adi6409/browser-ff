//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/extensions/spellcheck/idl/mozIPersonalDictionary.idl
//


/// `interface mozIPersonalDictionary : nsISupports`
///

/// ```text
/// /**
///  * This interface represents a Personal Dictionary.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIPersonalDictionary {
    vtable: *const mozIPersonalDictionaryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIPersonalDictionary.
unsafe impl XpCom for mozIPersonalDictionary {
    const IID: nsIID = nsID(0x7ef52eaf, 0xb7e1, 0x462b,
        [0x87, 0xe2, 0x5d, 0x1d, 0xba, 0xca, 0x90, 0x48]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIPersonalDictionary {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIPersonalDictionary.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIPersonalDictionaryCoerce {
    /// Cheaply cast a value of this type from a `mozIPersonalDictionary`.
    fn coerce_from(v: &mozIPersonalDictionary) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIPersonalDictionaryCoerce for mozIPersonalDictionary {
    #[inline]
    fn coerce_from(v: &mozIPersonalDictionary) -> &Self {
        v
    }
}

impl mozIPersonalDictionary {
    /// Cast this `mozIPersonalDictionary` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIPersonalDictionaryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIPersonalDictionary {
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
impl<T: nsISupportsCoerce> mozIPersonalDictionaryCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIPersonalDictionary) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIPersonalDictionary
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIPersonalDictionaryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void load (); */
    pub Load: unsafe extern "system" fn (this: *const mozIPersonalDictionary) -> ::nserror::nsresult,

    /* void save (); */
    pub Save: unsafe extern "system" fn (this: *const mozIPersonalDictionary) -> ::nserror::nsresult,

    /* readonly attribute nsIStringEnumerator wordList; */
    pub GetWordList: unsafe extern "system" fn (this: *const mozIPersonalDictionary, aWordList: *mut*const nsIStringEnumerator) -> ::nserror::nsresult,

    /* boolean check (in AString word); */
    pub Check: unsafe extern "system" fn (this: *const mozIPersonalDictionary, word: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void addWord (in AString word); */
    pub AddWord: unsafe extern "system" fn (this: *const mozIPersonalDictionary, word: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeWord (in AString word); */
    pub RemoveWord: unsafe extern "system" fn (this: *const mozIPersonalDictionary, word: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void ignoreWord (in AString word); */
    pub IgnoreWord: unsafe extern "system" fn (this: *const mozIPersonalDictionary, word: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void endSession (); */
    pub EndSession: unsafe extern "system" fn (this: *const mozIPersonalDictionary) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIPersonalDictionary {

    /// ```text
    /// /**
    ///    * Load the dictionary
    ///    */
    /// ```
    ///

    /// `void load ();`
    #[inline]
    pub unsafe fn Load(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Load)(self, )
    }


    /// ```text
    /// /**
    ///    * Save the dictionary
    ///    */
    /// ```
    ///

    /// `void save ();`
    #[inline]
    pub unsafe fn Save(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Save)(self, )
    }


    /// ```text
    /// /**
    ///    * Get the (lexicographically sorted) list of words
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIStringEnumerator wordList;`
    #[inline]
    pub unsafe fn GetWordList(&self, aWordList: *mut*const nsIStringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetWordList)(self, aWordList)
    }


    /// ```text
    /// /**
    ///    * Check a unicode string
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
    ///    * Add a word to the personal dictionary
    ///    */
    /// ```
    ///

    /// `void addWord (in AString word);`
    #[inline]
    pub unsafe fn AddWord(&self, word: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AddWord)(self, word)
    }


    /// ```text
    /// /**
    ///    * Remove a word from the personal dictionary
    ///    */
    /// ```
    ///

    /// `void removeWord (in AString word);`
    #[inline]
    pub unsafe fn RemoveWord(&self, word: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveWord)(self, word)
    }


    /// ```text
    /// /**
    ///    * Add a word to the ignore all list
    ///    */
    /// ```
    ///

    /// `void ignoreWord (in AString word);`
    #[inline]
    pub unsafe fn IgnoreWord(&self, word: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).IgnoreWord)(self, word)
    }


    /// ```text
    /// /**
    ///    * Clear the ignore list
    ///    */
    /// ```
    ///

    /// `void endSession ();`
    #[inline]
    pub unsafe fn EndSession(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EndSession)(self, )
    }


}


