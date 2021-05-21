//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/serializers/nsIDocumentEncoder.idl
//


/// `interface nsIDocumentEncoderNodeFixup : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocumentEncoderNodeFixup {
    vtable: *const nsIDocumentEncoderNodeFixupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocumentEncoderNodeFixup.
unsafe impl XpCom for nsIDocumentEncoderNodeFixup {
    const IID: nsIID = nsID(0x3d9371d8, 0xa2ad, 0x403e,
        [0x8b, 0x0e, 0x88, 0x85, 0xad, 0x35, 0x62, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocumentEncoderNodeFixup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocumentEncoderNodeFixup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocumentEncoderNodeFixupCoerce {
    /// Cheaply cast a value of this type from a `nsIDocumentEncoderNodeFixup`.
    fn coerce_from(v: &nsIDocumentEncoderNodeFixup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocumentEncoderNodeFixupCoerce for nsIDocumentEncoderNodeFixup {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoderNodeFixup) -> &Self {
        v
    }
}

impl nsIDocumentEncoderNodeFixup {
    /// Cast this `nsIDocumentEncoderNodeFixup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocumentEncoderNodeFixupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocumentEncoderNodeFixup {
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
impl<T: nsISupportsCoerce> nsIDocumentEncoderNodeFixupCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoderNodeFixup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocumentEncoderNodeFixup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocumentEncoderNodeFixupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Node fixupNode (in Node aNode, out boolean aSerializeCloneKids); */
    pub FixupNode: unsafe extern "system" fn (this: *const nsIDocumentEncoderNodeFixup, aNode: *const libc::c_void, aSerializeCloneKids: *mut bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocumentEncoderNodeFixup {

    /// ```text
    /// /**
    ///    * Create a fixed up version of a node. This method is called before
    ///    * each node in a document is about to be persisted. The implementor
    ///    * may return a new node with fixed up attributes or null. If null is
    ///    * returned the node should be used as-is.
    ///    * @param aNode Node to fixup.
    ///    * @param [OUT] aSerializeCloneKids True if the document encoder should
    ///    * apply recursive serialization to the children of the fixed up node
    ///    * instead of the children of the original node.
    ///    * @return The resulting fixed up node.
    ///    */
    /// ```
    ///

    /// `Node fixupNode (in Node aNode, out boolean aSerializeCloneKids);`
    #[inline]
    pub unsafe fn FixupNode(&self, aNode: *const libc::c_void, aSerializeCloneKids: *mut bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).FixupNode)(self, aNode, aSerializeCloneKids, _retval)
    }


}


/// `interface nsIDocumentEncoder : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocumentEncoder {
    vtable: *const nsIDocumentEncoderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocumentEncoder.
unsafe impl XpCom for nsIDocumentEncoder {
    const IID: nsIID = nsID(0x21f112df, 0xd96f, 0x47da,
        [0xbf, 0xcb, 0x53, 0x31, 0x27, 0x30, 0x03, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocumentEncoder {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocumentEncoder.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocumentEncoderCoerce {
    /// Cheaply cast a value of this type from a `nsIDocumentEncoder`.
    fn coerce_from(v: &nsIDocumentEncoder) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocumentEncoderCoerce for nsIDocumentEncoder {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoder) -> &Self {
        v
    }
}

impl nsIDocumentEncoder {
    /// Cast this `nsIDocumentEncoder` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocumentEncoderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocumentEncoder {
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
impl<T: nsISupportsCoerce> nsIDocumentEncoderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentEncoder) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocumentEncoder
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocumentEncoderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in Document aDocument, in AString aMimeType, in unsigned long aFlags); */
    pub Init: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aDocument: *const libc::c_void, aMimeType: *const ::nsstring::nsAString, aFlags: u32) -> ::nserror::nsresult,

    /* [noscript] void nativeInit (in Document aDocument, in AString aMimeType, in unsigned long aFlags); */
    pub NativeInit: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aDocument: *const libc::c_void, aMimeType: *const ::nsstring::nsAString, aFlags: u32) -> ::nserror::nsresult,

    /* void setSelection (in Selection aSelection); */
    pub SetSelection: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aSelection: *const libc::c_void) -> ::nserror::nsresult,

    /* void setRange (in Range aRange); */
    pub SetRange: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aRange: *const libc::c_void) -> ::nserror::nsresult,

    /* void setNode (in Node aNode); */
    pub SetNode: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aNode: *const libc::c_void) -> ::nserror::nsresult,

    /* void setContainerNode (in Node aContainer); */
    pub SetContainerNode: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aContainer: *const libc::c_void) -> ::nserror::nsresult,

    /* void setCharset (in ACString aCharset); */
    pub SetCharset: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setWrapColumn (in unsigned long aWrapColumn); */
    pub SetWrapColumn: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aWrapColumn: u32) -> ::nserror::nsresult,

    /* readonly attribute AString mimeType; */
    pub GetMimeType: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aMimeType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void encodeToStream (in nsIOutputStream aStream); */
    pub EncodeToStream: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aStream: *const nsIOutputStream) -> ::nserror::nsresult,

    /* AString encodeToString (); */
    pub EncodeToString: unsafe extern "system" fn (this: *const nsIDocumentEncoder, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString encodeToStringWithContext (out AString aContextString, out AString aInfoString); */
    pub EncodeToStringWithContext: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aContextString: *mut ::nsstring::nsAString, aInfoString: *mut ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString encodeToStringWithMaxLength (in unsigned long aMaxLength); */
    pub EncodeToStringWithMaxLength: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aMaxLength: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void setNodeFixup (in nsIDocumentEncoderNodeFixup aFixup); */
    pub SetNodeFixup: unsafe extern "system" fn (this: *const nsIDocumentEncoder, aFixup: *const nsIDocumentEncoderNodeFixup) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocumentEncoder {
    /// ```text
    /// /**
    ///    * Output only the selection (as opposed to the whole document).
    ///    */
    /// ```
    ///

    pub const OutputSelectionOnly: i64 = 1;

    /// ```text
    /// /** Plaintext output: Convert html to plaintext that looks like the html.
    ///     * Can't be used in conjunction with `OutputPreformatted`.
    ///     * Implies wrap (except inside <pre>), since html wraps.
    ///     * HTML, XHTML and XML output: do prettyprinting, ignoring existing formatting.
    ///     * XML output : it doesn't implicitly wrap
    ///     */
    /// ```
    ///

    pub const OutputFormatted: i64 = 2;

    /// ```text
    /// /** Don't do prettyprinting. Don't do any wrapping that's not in the existing
    ///    * HTML/XML source. This option overrides OutputFormatted if both are set.
    ///    * HTML/XHTML output: If neither are set, there won't be prettyprinting too, but
    ///    * long lines will be wrapped.
    ///    * Supported also in XML and Plaintext output.
    ///    * @note This option does not affect entity conversion.
    ///    */
    /// ```
    ///

    pub const OutputRaw: i64 = 4;

    /// ```text
    /// /**
    ///    * Do not print html head tags.
    ///    * XHTML/HTML output only.
    ///    */
    /// ```
    ///

    pub const OutputBodyOnly: i64 = 8;

    /// ```text
    /// /**
    ///    * Output as though the content is preformatted
    ///    * (e.g. maybe it's wrapped in a PRE or PRE_WRAP style tag)
    ///    * Plaintext output only.
    ///    * Can't be used together with `OutputFormatted`/`OutputFormatFlowed`.
    ///    * XXXbz How does this interact with OutputRaw?
    ///    */
    /// ```
    ///

    pub const OutputPreformatted: i64 = 16;

    /// ```text
    /// /**
    ///    * Wrap even if we're not doing formatted output (e.g. for text fields).
    ///    * Supported in XML, XHTML, HTML and Plaintext output.
    ///    * Set implicitly in HTML/XHTML output when no OutputRaw.
    ///    * Ignored when OutputRaw.
    ///    * XXXLJ: set implicitly in HTML/XHTML output, to keep compatible behaviors
    ///    *        for old callers of this interface
    ///    * XXXbz How does this interact with OutputFormatFlowed?
    ///    */
    /// ```
    ///

    pub const OutputWrap: i64 = 32;

    /// ```text
    /// /**
    ///    * Output for format flowed (RFC 2646). This is used when converting
    ///    * to text for mail sending. This differs just slightly
    ///    * but in an important way from normal formatted, and that is that
    ///    * lines are space stuffed. This can't (correctly) be done later.
    ///    * PlainText output only.
    ///    * If this flag is set, `OutputFormat` has to be set too.
    ///    * XXXbz How does this interact with OutputRaw/OutputWrap?
    ///    */
    /// ```
    ///

    pub const OutputFormatFlowed: i64 = 64;

    /// ```text
    /// /**
    ///    * Convert links, image src, and script src to absolute URLs when possible.
    ///    * XHTML/HTML output only.
    ///    */
    /// ```
    ///

    pub const OutputAbsoluteLinks: i64 = 128;

    /// ```text
    /// /**
    ///    * LineBreak processing: if this flag is set than CR line breaks will
    ///    * be written. If neither this nor OutputLFLineBreak is set, then we
    ///    * will use platform line breaks. The combination of the two flags will
    ///    * cause CRLF line breaks to be written.
    ///    */
    /// ```
    ///

    pub const OutputCRLineBreak: i64 = 512;

    /// ```text
    /// /**
    ///    * LineBreak processing: if this flag is set than LF line breaks will
    ///    * be written. If neither this nor OutputCRLineBreak is set, then we
    ///    * will use platform line breaks. The combination of the two flags will
    ///    * cause CRLF line breaks to be written.
    ///    */
    /// ```
    ///

    pub const OutputLFLineBreak: i64 = 1024;

    /// ```text
    /// /**
    ///    * Output the content of noscript elements (only for serializing
        ///    * to plaintext).
    ///    */
    /// ```
    ///

    pub const OutputNoScriptContent: i64 = 2048;

    /// ```text
    /// /**
    ///    * Output the content of noframes elements (only for serializing
        ///    * to plaintext). (Used only internally in the plain text serializer;
        ///    * ignored if passed by the caller.)
    ///    */
    /// ```
    ///

    pub const OutputNoFramesContent: i64 = 4096;

    /// ```text
    /// /**
    ///    * Don't allow any formatting nodes (e.g. <br>, <b>) inside a <pre>.
    ///    * This is used primarily by mail. XHTML/HTML output only.
    ///    */
    /// ```
    ///

    pub const OutputNoFormattingInPre: i64 = 8192;

    /// ```text
    /// /**
    ///    * Encode entities when outputting to a string.
    ///    * E.g. If set, we'll output &nbsp; if clear, we'll output 0xa0.
    ///    * The basic set is just &nbsp; &amp; &lt; &gt; &quot; for interoperability
    ///    * with older products that don't support &alpha; and friends.
    ///    * HTML output only.
    ///    */
    /// ```
    ///

    pub const OutputEncodeBasicEntities: i64 = 16384;

    /// ```text
    /// /**
    ///    * Normally &nbsp; is replaced with a space character when
    ///    * encoding data as plain text, set this flag if that's
    ///    * not desired.
    ///    * Plaintext output only.
    ///    */
    /// ```
    ///

    pub const OutputPersistNBSP: i64 = 131072;

    /// ```text
    /// /**
    ///    * Normally when serializing the whole document using the HTML or
    ///    * XHTML serializer, the encoding declaration is rewritten to match.
    ///    * This flag suppresses that behavior.
    ///    */
    /// ```
    ///

    pub const OutputDontRewriteEncodingDeclaration: i64 = 262144;

    /// ```text
    /// /**
    ///    * When using the HTML or XHTML serializer, skip elements that are not
    ///    * visible when this flag is set.  Elements are not visible when they
    ///    * have CSS style display:none or visibility:collapse, for example.
    ///    */
    /// ```
    ///

    pub const SkipInvisibleContent: i64 = 524288;

    /// ```text
    /// /**
    ///    * Output for delsp=yes (RFC 3676). This is used with OutputFormatFlowed
    ///    * when converting to text for mail sending.
    ///    * PlainText output only.
    ///    */
    /// ```
    ///

    pub const OutputFormatDelSp: i64 = 1048576;

    /// ```text
    /// /**
    ///    * Drop <br> elements considered "invisible" by the editor. OutputPreformatted
    ///    * implies this flag.
    ///    */
    /// ```
    ///

    pub const OutputDropInvisibleBreak: i64 = 2097152;

    /// ```text
    /// /**
    ///    * Don't check for _moz_dirty attributes when deciding whether to
    ///    * pretty-print if this flag is set (bug 599983).
    ///    */
    /// ```
    ///

    pub const OutputIgnoreMozDirty: i64 = 4194304;

    /// ```text
    /// /**
    ///    * Serialize in a way that is suitable for copying a plaintext version of the
    ///    * document to the clipboard.  This can for example cause line endings to be
    ///    * injected at preformatted block element boundaries.
    ///    */
    /// ```
    ///

    pub const OutputForPlainTextClipboardCopy: i64 = 33554432;

    /// ```text
    /// /**
    ///    * Include ruby annotations and ruby parentheses in the output.
    ///    * PlainText output only.
    ///    */
    /// ```
    ///

    pub const OutputRubyAnnotation: i64 = 67108864;

    /// ```text
    /// /**
    ///    * Disallow breaking of long character strings. This is important
    ///    * for serializing e-mail which contains CJK strings. These must
    ///    * not be broken just as "normal" longs strings aren't broken.
    ///    */
    /// ```
    ///

    pub const OutputDisallowLineBreaking: i64 = 134217728;

    /// ```text
    /// /**
    ///    * Release reference of Document after using encodeTo* method to recycle
    ///    * this encoder without holding Document. To use this encoder again,
    ///    * we have to call init again.
    ///    */
    /// ```
    ///

    pub const RequiresReinitAfterOutput: i64 = 268435456;

    /// ```text
    /// /**
    ///    * Initialize with a pointer to the document and the mime type.
    ///    * Resets wrap column to 72 and resets node fixup.
    ///    * @param aDocument Document to encode.
    ///    * @param aMimeType MimeType to use. May also be set by SetMimeType.
    ///    * @param aFlags Flags to use while encoding. May also be set by SetFlags.
    ///    */
    /// ```
    ///

    /// `void init (in Document aDocument, in AString aMimeType, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn Init(&self, aDocument: *const libc::c_void, aMimeType: *const ::nsstring::nsAString, aFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aDocument, aMimeType, aFlags)
    }



    /// `[noscript] void nativeInit (in Document aDocument, in AString aMimeType, in unsigned long aFlags);`
    #[inline]
    pub unsafe fn NativeInit(&self, aDocument: *const libc::c_void, aMimeType: *const ::nsstring::nsAString, aFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).NativeInit)(self, aDocument, aMimeType, aFlags)
    }


    /// ```text
    /// /**
    ///    *  If the selection is set to a non-null value, then the
    ///    *  selection is used for encoding, otherwise the entire
    ///    *  document is encoded.
    ///    * @param aSelection The selection to encode.
    ///    */
    /// ```
    ///

    /// `void setSelection (in Selection aSelection);`
    #[inline]
    pub unsafe fn SetSelection(&self, aSelection: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetSelection)(self, aSelection)
    }


    /// ```text
    /// /**
    ///    *  If the range is set to a non-null value, then the
    ///    *  range is used for encoding, otherwise the entire
    ///    *  document or selection is encoded.
    ///    * @param aRange The range to encode.
    ///    */
    /// ```
    ///

    /// `void setRange (in Range aRange);`
    #[inline]
    pub unsafe fn SetRange(&self, aRange: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetRange)(self, aRange)
    }


    /// ```text
    /// /**
    ///    *  If the node is set to a non-null value, then the
    ///    *  node is used for encoding, otherwise the entire
    ///    *  document or range or selection is encoded.
    ///    * @param aNode The node to encode.
    ///    */
    /// ```
    ///

    /// `void setNode (in Node aNode);`
    #[inline]
    pub unsafe fn SetNode(&self, aNode: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetNode)(self, aNode)
    }


    /// ```text
    /// /**
    ///    *  If the container is set to a non-null value, then its
    ///    *  child nodes are used for encoding, otherwise the entire
    ///    *  document or range or selection or node is encoded.
    ///    *  @param aContainer The node which child nodes will be encoded.
    ///    */
    /// ```
    ///

    /// `void setContainerNode (in Node aContainer);`
    #[inline]
    pub unsafe fn SetContainerNode(&self, aContainer: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetContainerNode)(self, aContainer)
    }


    /// ```text
    /// /**
    ///    *  Documents typically have an intrinsic character set,
    ///    *  but if no intrinsic value is found, the platform character set
    ///    *  is used. This function overrides both the intrinisc and platform
    ///    *  charset.
    ///    *  @param aCharset Overrides the both the intrinsic or platform
    ///    *  character set when encoding the document.
    ///    *
    ///    *  Possible result codes: NS_ERROR_NO_CHARSET_CONVERTER
    ///    */
    /// ```
    ///

    /// `void setCharset (in ACString aCharset);`
    #[inline]
    pub unsafe fn SetCharset(&self, aCharset: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCharset)(self, aCharset)
    }


    /// ```text
    /// /**
    ///    *  Set a wrap column.  This may have no effect in some types of encoders.
    ///    * @param aWrapColumn Column to which to wrap. If 0, wrapping is disabled.
    ///    */
    /// ```
    ///

    /// `void setWrapColumn (in unsigned long aWrapColumn);`
    #[inline]
    pub unsafe fn SetWrapColumn(&self, aWrapColumn: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetWrapColumn)(self, aWrapColumn)
    }


    /// ```text
    /// /**
    ///    *  The mime type preferred by the encoder.  This piece of api was
    ///    *  added because the copy encoder may need to switch mime types on you
    ///    *  if you ask it to copy html that really represents plaintext content.
    ///    *  Call this AFTER Init() and SetSelection() have both been called.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString mimeType;`
    #[inline]
    pub unsafe fn GetMimeType(&self, aMimeType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetMimeType)(self, aMimeType)
    }


    /// ```text
    /// /**
    ///    *  Encode the document and send the result to the nsIOutputStream.
    ///    *
    ///    *  Possible result codes are the stream errors which might have
    ///    *  been encountered.
    ///    * @param aStream Stream into which to encode.
    ///    */
    /// ```
    ///

    /// `void encodeToStream (in nsIOutputStream aStream);`
    #[inline]
    pub unsafe fn EncodeToStream(&self, aStream: *const nsIOutputStream) -> ::nserror::nsresult {
        ((*self.vtable).EncodeToStream)(self, aStream)
    }


    /// ```text
    /// /**
    ///    * Encode the document into a string.
    ///    *
    ///    * @return The document encoded into a string.
    ///    */
    /// ```
    ///

    /// `AString encodeToString ();`
    #[inline]
    pub unsafe fn EncodeToString(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).EncodeToString)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Encode the document into a string. Stores the extra context information
    ///    * into the two arguments.
    ///    * @param [OUT] aContextString The string where the parent hierarchy
    ///    *              information will be stored.
    ///    * @param [OUT] aInfoString The string where extra context info will
    ///    *              be stored.
    ///    * @return The document encoded as a string.
    ///    *
    ///    */
    /// ```
    ///

    /// `AString encodeToStringWithContext (out AString aContextString, out AString aInfoString);`
    #[inline]
    pub unsafe fn EncodeToStringWithContext(&self, aContextString: *mut ::nsstring::nsAString, aInfoString: *mut ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).EncodeToStringWithContext)(self, aContextString, aInfoString, _retval)
    }


    /// ```text
    /// /**
    ///    * Encode the document into a string of limited size.
    ///    * @param aMaxLength After aMaxLength characters, the encoder will stop
    ///    *                   encoding new data.
    ///    *                   Only values > 0 will be considered.
    ///    *                   The returned string may be slightly larger than
    ///    *                   aMaxLength because some serializers (eg. HTML)
    ///    *                   may need to close some tags after they stop
    ///    *                   encoding new data, or finish a line (72 columns
        ///    *                   by default for the plain text serializer).
    ///    *
    ///    * @return The document encoded into a string.
    ///    */
    /// ```
    ///

    /// `AString encodeToStringWithMaxLength (in unsigned long aMaxLength);`
    #[inline]
    pub unsafe fn EncodeToStringWithMaxLength(&self, aMaxLength: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).EncodeToStringWithMaxLength)(self, aMaxLength, _retval)
    }


    /// ```text
    /// /**
    ///    * Set the fixup object associated with node persistence.
    ///    * @param aFixup The fixup object.
    ///    */
    /// ```
    ///

    /// `void setNodeFixup (in nsIDocumentEncoderNodeFixup aFixup);`
    #[inline]
    pub unsafe fn SetNodeFixup(&self, aFixup: *const nsIDocumentEncoderNodeFixup) -> ::nserror::nsresult {
        ((*self.vtable).SetNodeFixup)(self, aFixup)
    }


}


