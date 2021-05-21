//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleEditableText.idl
//


/// `interface nsIAccessibleEditableText : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleEditableText {
    vtable: *const nsIAccessibleEditableTextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleEditableText.
unsafe impl XpCom for nsIAccessibleEditableText {
    const IID: nsIID = nsID(0x28915cca, 0x3366, 0x4034,
        [0xba, 0x1d, 0xb7, 0xaf, 0xb9, 0xb3, 0x76, 0x39]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleEditableText {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleEditableText.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleEditableTextCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleEditableText`.
    fn coerce_from(v: &nsIAccessibleEditableText) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleEditableTextCoerce for nsIAccessibleEditableText {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEditableText) -> &Self {
        v
    }
}

impl nsIAccessibleEditableText {
    /// Cast this `nsIAccessibleEditableText` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleEditableTextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleEditableText {
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
impl<T: nsISupportsCoerce> nsIAccessibleEditableTextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleEditableText) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleEditableText
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleEditableTextVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setTextContents (in AString text); */
    pub SetTextContents: unsafe extern "system" fn (this: *const nsIAccessibleEditableText, text: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void insertText (in AString text, in long position); */
    pub InsertText: unsafe extern "system" fn (this: *const nsIAccessibleEditableText, text: *const ::nsstring::nsAString, position: i32) -> ::nserror::nsresult,

    /* void copyText (in long startPos, in long endPos); */
    pub CopyText: unsafe extern "system" fn (this: *const nsIAccessibleEditableText, startPos: i32, endPos: i32) -> ::nserror::nsresult,

    /* void cutText (in long startPos, in long endPos); */
    pub CutText: unsafe extern "system" fn (this: *const nsIAccessibleEditableText, startPos: i32, endPos: i32) -> ::nserror::nsresult,

    /* void deleteText (in long startPos, in long endPos); */
    pub DeleteText: unsafe extern "system" fn (this: *const nsIAccessibleEditableText, startPos: i32, endPos: i32) -> ::nserror::nsresult,

    /* [can_run_script] void pasteText (in long position); */
    pub PasteText: unsafe extern "system" fn (this: *const nsIAccessibleEditableText, position: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleEditableText {

    /// ```text
    /// /**
    ///    * Replaces the text represented by this object by the given text.
    ///    */
    /// ```
    ///

    /// `void setTextContents (in AString text);`
    #[inline]
    pub unsafe fn SetTextContents(&self, text: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetTextContents)(self, text)
    }


    /// ```text
    /// /**
    ///    * Inserts text at the specified position.
    ///    *
    ///    * @param text - text that is inserted.
    ///    * @param position - index at which to insert the text.
    ///    */
    /// ```
    ///

    /// `void insertText (in AString text, in long position);`
    #[inline]
    pub unsafe fn InsertText(&self, text: *const ::nsstring::nsAString, position: i32) -> ::nserror::nsresult {
        ((*self.vtable).InsertText)(self, text, position)
    }


    /// ```text
    /// /**
    ///    * Copies the text range into the clipboard.
    ///    *
    ///    * @param startPos - start index of the text to moved into the clipboard.
    ///    * @param endPos - end index of the text to moved into the clipboard.
    ///    */
    /// ```
    ///

    /// `void copyText (in long startPos, in long endPos);`
    #[inline]
    pub unsafe fn CopyText(&self, startPos: i32, endPos: i32) -> ::nserror::nsresult {
        ((*self.vtable).CopyText)(self, startPos, endPos)
    }


    /// ```text
    /// /**
    ///    * Deletes a range of text and copies it to the clipboard.
    ///    *
    ///    * @param startPos - start index of the text to be deleted.
    ///    * @param endOffset - end index of the text to be deleted.
    ///    */
    /// ```
    ///

    /// `void cutText (in long startPos, in long endPos);`
    #[inline]
    pub unsafe fn CutText(&self, startPos: i32, endPos: i32) -> ::nserror::nsresult {
        ((*self.vtable).CutText)(self, startPos, endPos)
    }


    /// ```text
    /// /**
    ///    * Deletes a range of text.
    ///    *
    ///    * @param startPos - start index of the text to be deleted.
    ///    * @param endPos - end index of the text to be deleted.
    ///    */
    /// ```
    ///

    /// `void deleteText (in long startPos, in long endPos);`
    #[inline]
    pub unsafe fn DeleteText(&self, startPos: i32, endPos: i32) -> ::nserror::nsresult {
        ((*self.vtable).DeleteText)(self, startPos, endPos)
    }


    /// ```text
    /// /**
    ///    * Pastes text from the clipboard.
    ///    *
    ///    * @param position - index at which to insert the text from the system
    ///    *                   clipboard into the text represented by this object.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void pasteText (in long position);`
    #[inline]
    pub unsafe fn PasteText(&self, position: i32) -> ::nserror::nsresult {
        ((*self.vtable).PasteText)(self, position)
    }


}


