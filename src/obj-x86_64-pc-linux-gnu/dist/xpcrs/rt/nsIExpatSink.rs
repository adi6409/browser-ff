//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/parser/htmlparser/nsIExpatSink.idl
//


/// `interface nsIExpatSink : nsISupports`
///

/// ```text
/// /**
///  * This interface should be implemented by any content sink that wants
///  * to get output from expat and do something with it; in other words,
///  * by any sink that handles some sort of XML dialect.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIExpatSink {
    vtable: *const nsIExpatSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIExpatSink.
unsafe impl XpCom for nsIExpatSink {
    const IID: nsIID = nsID(0x01f681af, 0x0f22, 0x4725,
        [0xa9, 0x14, 0x0d, 0x39, 0x61, 0x14, 0xda, 0xf0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIExpatSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIExpatSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIExpatSinkCoerce {
    /// Cheaply cast a value of this type from a `nsIExpatSink`.
    fn coerce_from(v: &nsIExpatSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIExpatSinkCoerce for nsIExpatSink {
    #[inline]
    fn coerce_from(v: &nsIExpatSink) -> &Self {
        v
    }
}

impl nsIExpatSink {
    /// Cast this `nsIExpatSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIExpatSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIExpatSink {
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
impl<T: nsISupportsCoerce> nsIExpatSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIExpatSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIExpatSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIExpatSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void HandleStartElement (in wstring aName, [array, size_is (aAttsCount)] in wstring aAtts, in unsigned long aAttsCount, in unsigned long aLineNumber, in unsigned long aColumnNumber); */
    pub HandleStartElement: unsafe extern "system" fn (this: *const nsIExpatSink, aName: *const i16, aAtts: *mut *const i16, aAttsCount: u32, aLineNumber: u32, aColumnNumber: u32) -> ::nserror::nsresult,

    /* void HandleEndElement (in wstring aName); */
    pub HandleEndElement: unsafe extern "system" fn (this: *const nsIExpatSink, aName: *const i16) -> ::nserror::nsresult,

    /* void HandleComment (in wstring aCommentText); */
    pub HandleComment: unsafe extern "system" fn (this: *const nsIExpatSink, aCommentText: *const i16) -> ::nserror::nsresult,

    /* void HandleCDataSection ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
    pub HandleCDataSection: unsafe extern "system" fn (this: *const nsIExpatSink, aData: *const i16, aLength: u32) -> ::nserror::nsresult,

    /* void HandleDoctypeDecl (in AString aSubset, in AString aName, in AString aSystemId, in AString aPublicId, in nsISupports aCatalogData); */
    pub HandleDoctypeDecl: unsafe extern "system" fn (this: *const nsIExpatSink, aSubset: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aSystemId: *const ::nsstring::nsAString, aPublicId: *const ::nsstring::nsAString, aCatalogData: *const nsISupports) -> ::nserror::nsresult,

    /* void HandleCharacterData ([size_is (aLength)] in wstring aData, in unsigned long aLength); */
    pub HandleCharacterData: unsafe extern "system" fn (this: *const nsIExpatSink, aData: *const i16, aLength: u32) -> ::nserror::nsresult,

    /* void HandleProcessingInstruction (in wstring aTarget, in wstring aData); */
    pub HandleProcessingInstruction: unsafe extern "system" fn (this: *const nsIExpatSink, aTarget: *const i16, aData: *const i16) -> ::nserror::nsresult,

    /* void HandleXMLDeclaration (in wstring aVersion, in wstring aEncoding, in long aStandalone); */
    pub HandleXMLDeclaration: unsafe extern "system" fn (this: *const nsIExpatSink, aVersion: *const i16, aEncoding: *const i16, aStandalone: i32) -> ::nserror::nsresult,

    /* boolean ReportError (in wstring aErrorText, in wstring aSourceText, in nsIScriptError aError); */
    pub ReportError: unsafe extern "system" fn (this: *const nsIExpatSink, aErrorText: *const i16, aSourceText: *const i16, aError: *const nsIScriptError, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIExpatSink {

    /// ```text
    /// /**
    ///    * Called to handle the opening tag of an element.
    ///    * @param aName the fully qualified tagname of the element
    ///    * @param aAtts the array of attribute names and values.  There are
    ///    *        aAttsCount/2 names and aAttsCount/2 values, so the total number of
    ///    *        elements in the array is aAttsCount.  The names and values
    ///    *        alternate.  Thus, if we number attributes starting with 0,
    ///    *        aAtts[2*k] is the name of the k-th attribute and aAtts[2*k+1] is
    ///    *        the value of that attribute  Both explicitly specified attributes
    ///    *        and attributes that are defined to have default values in a DTD are
    ///    *        present in aAtts.
    ///    * @param aAttsCount the number of elements in aAtts.
    ///    * @param aLineNumber the line number of the start tag in the data stream.
    ///    * @param aColumnNumber the column number of the start tag in the data stream.
    ///    */
    /// ```
    ///

    /// `void HandleStartElement (in wstring aName, [array, size_is (aAttsCount)] in wstring aAtts, in unsigned long aAttsCount, in unsigned long aLineNumber, in unsigned long aColumnNumber);`
    #[inline]
    pub unsafe fn HandleStartElement(&self, aName: *const i16, aAtts: *mut *const i16, aAttsCount: u32, aLineNumber: u32, aColumnNumber: u32) -> ::nserror::nsresult {
        ((*self.vtable).HandleStartElement)(self, aName, aAtts, aAttsCount, aLineNumber, aColumnNumber)
    }


    /// ```text
    /// /**
    ///    * Called to handle the closing tag of an element.
    ///    * @param aName the fully qualified tagname of the element
    ///    */
    /// ```
    ///

    /// `void HandleEndElement (in wstring aName);`
    #[inline]
    pub unsafe fn HandleEndElement(&self, aName: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).HandleEndElement)(self, aName)
    }


    /// ```text
    /// /**
    ///    * Called to handle a comment
    ///    * @param aCommentText the text of the comment (not including the
        ///    *        "<!--" and "-->")
    ///    */
    /// ```
    ///

    /// `void HandleComment (in wstring aCommentText);`
    #[inline]
    pub unsafe fn HandleComment(&self, aCommentText: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).HandleComment)(self, aCommentText)
    }


    /// ```text
    /// /**
    ///    * Called to handle a CDATA section
    ///    * @param aData the text in the CDATA section.  This is null-terminated.
    ///    * @param aLength the length of the aData string
    ///    */
    /// ```
    ///

    /// `void HandleCDataSection ([size_is (aLength)] in wstring aData, in unsigned long aLength);`
    #[inline]
    pub unsafe fn HandleCDataSection(&self, aData: *const i16, aLength: u32) -> ::nserror::nsresult {
        ((*self.vtable).HandleCDataSection)(self, aData, aLength)
    }


    /// ```text
    /// /**
    ///    * Called to handle the doctype declaration
    ///    */
    /// ```
    ///

    /// `void HandleDoctypeDecl (in AString aSubset, in AString aName, in AString aSystemId, in AString aPublicId, in nsISupports aCatalogData);`
    #[inline]
    pub unsafe fn HandleDoctypeDecl(&self, aSubset: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aSystemId: *const ::nsstring::nsAString, aPublicId: *const ::nsstring::nsAString, aCatalogData: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).HandleDoctypeDecl)(self, aSubset, aName, aSystemId, aPublicId, aCatalogData)
    }


    /// ```text
    /// /**
    ///    * Called to handle character data.  Note that this does NOT get
    ///    * called for the contents of CDATA sections.
    ///    * @param aData the data to handle.  aData is NOT NULL-TERMINATED.
    ///    * @param aLength the length of the aData string
    ///    */
    /// ```
    ///

    /// `void HandleCharacterData ([size_is (aLength)] in wstring aData, in unsigned long aLength);`
    #[inline]
    pub unsafe fn HandleCharacterData(&self, aData: *const i16, aLength: u32) -> ::nserror::nsresult {
        ((*self.vtable).HandleCharacterData)(self, aData, aLength)
    }


    /// ```text
    /// /**
    ///    * Called to handle a processing instruction
    ///    * @param aTarget the PI target (e.g. xml-stylesheet)
    ///    * @param aData all the rest of the data in the PI
    ///    */
    /// ```
    ///

    /// `void HandleProcessingInstruction (in wstring aTarget, in wstring aData);`
    #[inline]
    pub unsafe fn HandleProcessingInstruction(&self, aTarget: *const i16, aData: *const i16) -> ::nserror::nsresult {
        ((*self.vtable).HandleProcessingInstruction)(self, aTarget, aData)
    }


    /// ```text
    /// /**
    ///    * Handle the XML Declaration.
    ///    *
    ///    * @param aVersion    The version string, can be null if not specified.
    ///    * @param aEncoding   The encoding string, can be null if not specified.
    ///    * @param aStandalone -1, 0, or 1 indicating respectively that there was no
    ///    *                    standalone parameter in the declaration, that it was
    ///    *                    given as no, or that it was given as yes.
    ///    */
    /// ```
    ///

    /// `void HandleXMLDeclaration (in wstring aVersion, in wstring aEncoding, in long aStandalone);`
    #[inline]
    pub unsafe fn HandleXMLDeclaration(&self, aVersion: *const i16, aEncoding: *const i16, aStandalone: i32) -> ::nserror::nsresult {
        ((*self.vtable).HandleXMLDeclaration)(self, aVersion, aEncoding, aStandalone)
    }


    /// ```text
    /// /**
    ///    * Ask the content sink if the expat driver should log an error to the console.
    ///    *
    ///    * @param aErrorText  Error message to pass to content sink.
    ///    * @param aSourceText Source text of the document we're parsing.
    ///    * @param aError      Script error object with line number & column number
    ///    *
    ///    * @retval True if the expat driver should report the error.
    ///    */
    /// ```
    ///

    /// `boolean ReportError (in wstring aErrorText, in wstring aSourceText, in nsIScriptError aError);`
    #[inline]
    pub unsafe fn ReportError(&self, aErrorText: *const i16, aSourceText: *const i16, aError: *const nsIScriptError, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ReportError)(self, aErrorText, aSourceText, aError, _retval)
    }


}


