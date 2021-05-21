//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/autocomplete/nsIAutoCompleteSimpleResult.idl
//


/// `interface nsIAutoCompleteSimpleResult : nsIAutoCompleteResult`
///

/// ```text
/// /**
///  * This class implements nsIAutoCompleteResult and provides simple methods
///  * for setting the value and result items. It can be used whenever some basic
///  * auto complete results are needed that can be pre-generated and filled into
///  * an array.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteSimpleResult {
    vtable: *const nsIAutoCompleteSimpleResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteSimpleResult.
unsafe impl XpCom for nsIAutoCompleteSimpleResult {
    const IID: nsIID = nsID(0x23de9c96, 0xbecb, 0x4d0d,
        [0xa9, 0xbb, 0x1d, 0x13, 0x1c, 0xe3, 0x61, 0xb5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteSimpleResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteSimpleResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteSimpleResultCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteSimpleResult`.
    fn coerce_from(v: &nsIAutoCompleteSimpleResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteSimpleResultCoerce for nsIAutoCompleteSimpleResult {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResult) -> &Self {
        v
    }
}

impl nsIAutoCompleteSimpleResult {
    /// Cast this `nsIAutoCompleteSimpleResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSimpleResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteSimpleResult {
    type Target = nsIAutoCompleteResult;
    #[inline]
    fn deref(&self) -> &nsIAutoCompleteResult {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIAutoCompleteResultCoerce> nsIAutoCompleteSimpleResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteSimpleResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteSimpleResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIAutoCompleteResultVTable,

    /* void setSearchString (in AString aSearchString); */
    pub SetSearchString: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setErrorDescription (in AString aErrorDescription); */
    pub SetErrorDescription: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aErrorDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setDefaultIndex (in long aDefaultIndex); */
    pub SetDefaultIndex: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aDefaultIndex: i32) -> ::nserror::nsresult,

    /* void setSearchResult (in unsigned short aSearchResult); */
    pub SetSearchResult: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aSearchResult: u16) -> ::nserror::nsresult,

    /* void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
    pub InsertMatchAt: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aIndex: i32, aValue: *const ::nsstring::nsAString, aComment: *const ::nsstring::nsAString, aImage: *const ::nsstring::nsAString, aStyle: *const ::nsstring::nsAString, aFinalCompleteValue: *const ::nsstring::nsAString, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel); */
    pub AppendMatch: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aValue: *const ::nsstring::nsAString, aComment: *const ::nsstring::nsAString, aImage: *const ::nsstring::nsAString, aStyle: *const ::nsstring::nsAString, aFinalCompleteValue: *const ::nsstring::nsAString, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void removeMatchAt (in long aIndex); */
    pub RemoveMatchAt: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aIndex: i32) -> ::nserror::nsresult,

    /* nsIAutoCompleteSimpleResultListener getListener (); */
    pub GetListener: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, _retval: *mut*const nsIAutoCompleteSimpleResultListener) -> ::nserror::nsresult,

    /* void setListener (in nsIAutoCompleteSimpleResultListener aListener); */
    pub SetListener: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResult, aListener: *const nsIAutoCompleteSimpleResultListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteSimpleResult {

    /// ```text
    /// /**
    ///    * A writer for the readonly attribute 'searchString' which should contain
    ///    * the string that the user typed.
    ///    */
    /// ```
    ///

    /// `void setSearchString (in AString aSearchString);`
    #[inline]
    pub unsafe fn SetSearchString(&self, aSearchString: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchString)(self, aSearchString)
    }


    /// ```text
    /// /**
    ///    * A writer for the readonly attribute 'errorDescription'.
    ///    */
    /// ```
    ///

    /// `void setErrorDescription (in AString aErrorDescription);`
    #[inline]
    pub unsafe fn SetErrorDescription(&self, aErrorDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetErrorDescription)(self, aErrorDescription)
    }


    /// ```text
    /// /**
    ///    * A writer for the readonly attribute 'defaultIndex' which should contain
    ///    * the index of the list that will be selected by default (normally 0).
    ///    */
    /// ```
    ///

    /// `void setDefaultIndex (in long aDefaultIndex);`
    #[inline]
    pub unsafe fn SetDefaultIndex(&self, aDefaultIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultIndex)(self, aDefaultIndex)
    }


    /// ```text
    /// /**
    ///    * A writer for the readonly attribute 'searchResult' which should contain
    ///    * one of the constants nsIAutoCompleteResult.RESULT_* indicating the success
    ///    * of the search.
    ///    */
    /// ```
    ///

    /// `void setSearchResult (in unsigned short aSearchResult);`
    #[inline]
    pub unsafe fn SetSearchResult(&self, aSearchResult: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchResult)(self, aSearchResult)
    }


    /// ```text
    /// /**
    ///    * Inserts a match consisting of the given value, comment, image, style and
    ///    * the value to use for defaultIndex completion at a given position.
    ///    * @param aIndex
    ///    *        The index to insert at
    ///    * @param aValue
    ///    *        The value to autocomplete to
    ///    * @param aComment
    ///    *        Comment shown in the autocomplete widget to describe this match
    ///    * @param aImage
    ///    *        Image shown in the autocomplete widget for this match.
    ///    * @param aStyle
    ///    *        Describes how to style the match in the autocomplete widget
    ///    * @param aFinalCompleteValue
    ///    *        Value used when the user confirms selecting this match. If not
    ///    *        provided, aValue will be used.
    ///    */
    /// ```
    ///

    /// `void insertMatchAt (in long aIndex, in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel);`
    #[inline]
    pub unsafe fn InsertMatchAt(&self, aIndex: i32, aValue: *const ::nsstring::nsAString, aComment: *const ::nsstring::nsAString, aImage: *const ::nsstring::nsAString, aStyle: *const ::nsstring::nsAString, aFinalCompleteValue: *const ::nsstring::nsAString, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).InsertMatchAt)(self, aIndex, aValue, aComment, aImage, aStyle, aFinalCompleteValue, aLabel)
    }


    /// ```text
    /// /**
    ///    * Appends a match consisting of the given value, comment, image, style and
    ///    * the value to use for defaultIndex completion.
    ///    * @param aValue
    ///    *        The value to autocomplete to
    ///    * @param aComment
    ///    *        Comment shown in the autocomplete widget to describe this match
    ///    * @param aImage
    ///    *        Image shown in the autocomplete widget for this match.
    ///    * @param aStyle
    ///    *        Describes how to style the match in the autocomplete widget
    ///    * @param aFinalCompleteValue
    ///    *        Value used when the user confirms selecting this match. If not
    ///    *        provided, aValue will be used.
    ///    */
    /// ```
    ///

    /// `void appendMatch (in AString aValue, in AString aComment, [optional] in AString aImage, [optional] in AString aStyle, [optional] in AString aFinalCompleteValue, [optional] in AString aLabel);`
    #[inline]
    pub unsafe fn AppendMatch(&self, aValue: *const ::nsstring::nsAString, aComment: *const ::nsstring::nsAString, aImage: *const ::nsstring::nsAString, aStyle: *const ::nsstring::nsAString, aFinalCompleteValue: *const ::nsstring::nsAString, aLabel: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AppendMatch)(self, aValue, aComment, aImage, aStyle, aFinalCompleteValue, aLabel)
    }


    /// ```text
    /// /**
    ///    * Removes an existing match.
    ///    * @note this is different from removeValueAt, since it's not a consequence of
    ///    * a user action, and as such it won't notify onValueRemoved.
    ///    */
    /// ```
    ///

    /// `void removeMatchAt (in long aIndex);`
    #[inline]
    pub unsafe fn RemoveMatchAt(&self, aIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).RemoveMatchAt)(self, aIndex)
    }


    /// ```text
    /// /**
    ///    * Gets the listener for changes in the result.
    ///    */
    /// ```
    ///

    /// `nsIAutoCompleteSimpleResultListener getListener ();`
    #[inline]
    pub unsafe fn GetListener(&self, _retval: *mut*const nsIAutoCompleteSimpleResultListener) -> ::nserror::nsresult {
        ((*self.vtable).GetListener)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Sets a listener for changes in the result.
    ///    */
    /// ```
    ///

    /// `void setListener (in nsIAutoCompleteSimpleResultListener aListener);`
    #[inline]
    pub unsafe fn SetListener(&self, aListener: *const nsIAutoCompleteSimpleResultListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, aListener)
    }


}


/// `interface nsIAutoCompleteSimpleResultListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAutoCompleteSimpleResultListener {
    vtable: *const nsIAutoCompleteSimpleResultListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAutoCompleteSimpleResultListener.
unsafe impl XpCom for nsIAutoCompleteSimpleResultListener {
    const IID: nsIID = nsID(0x004efdc5, 0x1989, 0x4874,
        [0x8a, 0x7a, 0x34, 0x5b, 0xf2, 0xfa, 0x33, 0xaf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAutoCompleteSimpleResultListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAutoCompleteSimpleResultListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAutoCompleteSimpleResultListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIAutoCompleteSimpleResultListener`.
    fn coerce_from(v: &nsIAutoCompleteSimpleResultListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAutoCompleteSimpleResultListenerCoerce for nsIAutoCompleteSimpleResultListener {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResultListener) -> &Self {
        v
    }
}

impl nsIAutoCompleteSimpleResultListener {
    /// Cast this `nsIAutoCompleteSimpleResultListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAutoCompleteSimpleResultListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAutoCompleteSimpleResultListener {
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
impl<T: nsISupportsCoerce> nsIAutoCompleteSimpleResultListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAutoCompleteSimpleResultListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAutoCompleteSimpleResultListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAutoCompleteSimpleResultListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue); */
    pub OnValueRemoved: unsafe extern "system" fn (this: *const nsIAutoCompleteSimpleResultListener, aResult: *const nsIAutoCompleteSimpleResult, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAutoCompleteSimpleResultListener {

    /// ```text
    /// /**
    ///    * Dispatched after a value is removed from the result.
    ///    * @param aResult
    ///    *        The result from which aValue has been removed.
    ///    * @param aValue
    ///    *        The removed value.
    ///    */
    /// ```
    ///

    /// `void onValueRemoved (in nsIAutoCompleteSimpleResult aResult, in AString aValue);`
    #[inline]
    pub unsafe fn OnValueRemoved(&self, aResult: *const nsIAutoCompleteSimpleResult, aValue: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnValueRemoved)(self, aResult, aValue)
    }


}


